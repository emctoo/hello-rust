use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};
use std::collections::hash_map::Entry;

use tokio::{
    sync::{broadcast, Mutex},
    task::JoinHandle,
};

pub struct Channel {
    pub name: String,
    tx: broadcast::Sender<String>,
    inner_user: Mutex<Vec<String>>,
    user_count: AtomicU32,
}

/// this struct is used for managing multiple channels at once
/// ## how it works
/// everything is managed by a mutex
/// there is a hashmap of channels
/// a hashmap of user receiver
/// and a hashmap of user task
/// when a user joins a channel a task will be created and all of that channels
/// message will be forwarded to user receiver
/// and then user can listen to its own user receiver and receive message from all joined channels
pub struct ChannelManager {
    inner: Mutex<HashMap<String, Channel>>,
    user_channel_map: Mutex<HashMap<String, Vec<UserTask>>>,
    user_receiver_map: Mutex<HashMap<String, broadcast::Sender<String>>>,
}

#[derive(Debug)]
pub enum ChannelError {
    /// channel does not exist
    NotFound,
    /// can not send message to this channel
    MessageSendFail,
    /// you have not called init_user
    NotInitiated,
}

struct UserTask {
    channel_name: String,
    task: JoinHandle<()>,
}

impl Channel {
    /// creates new channel with a given name
    /// capacity is the underlying channel capacity and its default is 100
    pub fn new(name: String, capacity: Option<usize>) -> Channel {
        let (tx, _rx) = broadcast::channel(capacity.unwrap_or(100));

        Channel {
            name,
            tx,
            inner_user: Mutex::new(vec![]),
            user_count: AtomicU32::new(0),
        }
    }

    /// join the channels with a unique user
    /// if user has joined before, it just returns the sender
    pub async fn join(&self, user: String) -> broadcast::Sender<String> {
        let mut inner = self.inner_user.lock().await;

        if !inner.contains(&user) {
            inner.push(user);

            self.user_count.fetch_add(1, Ordering::SeqCst);
        }

        self.tx.clone()
    }

    pub async fn leave(&self, user: String) {
        let mut inner = self.inner_user.lock().await;
        if let Some(pos) = inner.iter().position(|x| *x == user) {
            inner.swap_remove(pos);
            self.user_count.fetch_sub(1, Ordering::SeqCst);
        }
    }

    /// this method will join the user and return a receiver
    pub async fn receive(&self, user: String) -> broadcast::Receiver<String> {
        self.join(user).await.subscribe()
    }

    /// check if user is in the channel
    pub async fn contains_user(&self, user: &String) -> bool {
        let inner = self.inner_user.lock().await;

        inner.contains(user)
    }

    /// checks if channel is empty
    pub fn is_empty(&self) -> bool {
        self.user_count.load(Ordering::SeqCst) == 0
    }

    /// get sender without joining channel
    pub fn get_sender(&self) -> broadcast::Sender<String> {
        self.tx.clone()
    }

    ///send message to channel
    pub fn send(&self, data: String) -> Result<usize, broadcast::error::SendError<String>> {
        self.tx.send(data)
    }

    pub async fn users(&self) -> tokio::sync::MutexGuard<Vec<String>> {
        self.inner_user.lock().await
    }

    pub async fn user_count(&self) -> u32 {
        self.user_count.load(Ordering::SeqCst)
    }
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            inner: Mutex::new(HashMap::new()),
            user_channel_map: Mutex::new(HashMap::new()),
            user_receiver_map: Mutex::new(HashMap::new()),
        }
    }

    pub async fn new_channel(&self, name: String, capacity: Option<usize>) {
        let mut channels = self.inner.lock().await;
        channels.insert(name.clone(), Channel::new(name, capacity));
    }

    pub async fn channel_exists(&self, name: &str) -> bool {
        let channels = self.inner.lock().await;

        match channels.get(name) {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn join_or_create(&self, user: String, channel: String) -> Result<broadcast::Sender<String>, ChannelError> {
        match self.channel_exists(&channel).await {
            true => self.join_channel(channel, user).await,
            false => {
                self.new_channel(channel.clone(), None).await;
                self.join_channel(channel, user).await
            }
        }
    }

    /// send a message to a channel
    /// it will fail if there are no users in the channel or
    /// if channel does not exist
    pub async fn send_message_to_channel(&self, name: String, data: String) -> Result<usize, ChannelError> {
        let channels = self.inner.lock().await;
        channels
            .get(&name)
            .ok_or(ChannelError::NotFound)?
            .send(data)
            .map_err(|_| ChannelError::MessageSendFail)
    }

    /// call this at first of your code to initialize user notifier
    pub async fn register(&self, user: String, capacity: Option<usize>) {
        let mut user_receiver = self.user_receiver_map.lock().await;

        match user_receiver.entry(user) {
            Entry::Occupied(_) => {}
            Entry::Vacant(vacant) => {
                let (tx, _rx) = broadcast::channel(capacity.unwrap_or(100));
                vacant.insert(tx);
            }
        }
    }

    /// call this at end of your code to remove user from all channels
    pub async fn deregister(&self, user: String) {
        let channels = self.inner.lock().await;
        let mut user_channel = self.user_channel_map.lock().await;
        let mut user_receiver = self.user_receiver_map.lock().await;

        match user_channel.entry(user.clone()) {
            Entry::Occupied(o) => {
                let user_channels = o.get();
                for task in user_channels {
                    let channel = channels.get(&task.channel_name);
                    if let Some(channel) = channel {
                        channel.leave(user.clone()).await;
                    }
                    task.task.abort();
                }
                o.remove();
            }
            Entry::Vacant(_) => {}
        }

        match user_receiver.entry(user.clone()) {
            Entry::Occupied(o) => {
                o.remove();
            }
            Entry::Vacant(_) => {}
        }
    }

    /// join user to channel
    pub async fn join_channel(&self, name: String, user: String) -> Result<broadcast::Sender<String>, ChannelError> {
        let channels = self.inner.lock().await;
        let mut users = self.user_channel_map.lock().await;
        let user_receiver = self.user_receiver_map.lock().await;

        let sender = channels
            .get(&name)
            .ok_or(ChannelError::NotFound)?
            .join(user.clone())
            .await;

        let user_receiver = user_receiver
            .get(&user)
            .ok_or(ChannelError::NotInitiated)?
            .clone();

        let mut task_recv = sender.subscribe();

        let task = tokio::spawn(async move {
            while let Ok(data) = task_recv.recv().await {
                let _ = user_receiver.send(data);
            }
        });

        match users.entry(user.clone()) {
            Entry::Occupied(mut o) => {
                let channels = o.get_mut();
                let has = channels.iter().any(|x| x.channel_name == name);
                if !has {
                    channels.push(UserTask { channel_name: name, task });
                }
            }
            Entry::Vacant(v) => {
                v.insert(vec![UserTask { channel_name: name, task }]);
            }
        };

        Ok(sender)
    }

    pub async fn remove_channel(&self, channel: String) {
        let mut channels = self.inner.lock().await;
        let mut users = self.user_channel_map.lock().await;

        match channels.entry(channel.clone()) {
            Entry::Vacant(_) => {}
            Entry::Occupied(el) => {
                for user in el.get().users().await.iter() {
                    if let Entry::Occupied(mut user_task) =
                        users.entry(user.into())
                    {
                        let vector = user_task.get_mut();

                        vector.retain(|task| {
                            if task.channel_name == channel {
                                task.task.abort();
                            }
                            task.channel_name != channel
                        });
                    }
                }

                el.remove();
            }
        }
    }

    pub async fn leave_channel(&self, name: String, user: String) -> Result<(), ChannelError> {
        let channels = self.inner.lock().await;
        let mut users = self.user_channel_map.lock().await;

        channels
            .get(&name)
            .ok_or(ChannelError::NotFound)?
            .leave(user.clone())
            .await;

        match users.entry(user.clone()) {
            Entry::Occupied(mut o) => {
                let vector = o.get_mut();
                vector.retain(|task| {
                    if task.channel_name == name {
                        task.task.abort();
                    }
                    task.channel_name != name
                });
            }
            Entry::Vacant(_) => {}
        }

        Ok(())
    }

    pub async fn is_channel_empty(&self, name: String) -> Result<bool, ChannelError> {
        let channels = self.inner.lock().await;
        Ok(channels.get(&name).ok_or(ChannelError::NotFound)?.is_empty())
    }

    pub async fn channels_count(&self) -> usize {
        let channels = self.inner.lock().await;
        channels.len()
    }

    pub async fn get_user_receiver(&self, name: String) -> Result<broadcast::Receiver<String>, ChannelError> {
        let rx = self.user_receiver_map.lock().await;
        let rx = rx.get(&name).ok_or(ChannelError::NotInitiated)?.subscribe();
        Ok(rx)
    }
}

impl Default for ChannelManager {
    fn default() -> Self {
        Self::new()
    }
}
