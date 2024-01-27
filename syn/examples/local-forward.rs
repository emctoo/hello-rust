use tokio_ssh2::AsyncSession;
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    let tcp = std::net::TcpStream::connect("127.0.0.1:22")?;
    let mut session = AsyncSession::new(tcp)?;

    session.handshake().await?;
    session.userauth_password("maple", "asdf").await?;

    let mut channel = session.channel_session().await?;
    channel.request_pty("xterm-256color", None, None).await?;
    channel.shell().await?;

    let mut stream = channel.stream(0)?;
    let mut line = String::new();

    stream.write_all("ls".as_bytes()).await?;
    println!("ls");

    stream.read_to_string(&mut line).await?;

    println!("{}", line);

    Ok(())
}