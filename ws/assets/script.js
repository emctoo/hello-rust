const socket = new WebSocket('ws://localhost:3000/ws');

function sendMessage(ws, content) {
    // const blob = new Blob([JSON.stringify({content}, null, 2)]);
    // ws.send(blob, {type: "application/json"})
    ws.send(JSON.stringify({content}, null, 2));
    console.log("message sent: ", content)
}

function wsSendMessage(content) {
    sendMessage(socket, content);
}

socket.onopen = function (ev) {
    console.log('connection ready')
    // socket.send('Hello Server!');
    wsSendMessage("Hi")
};

socket.onmessage = function (ev) {
    console.log('message from server: ', ev.data);
};

let interval = setInterval(() => {
    wsSendMessage("time: " + new Date().toISOString());
}, 1000 * 3);

function cleanupWs(socket) {
    clearInterval(interval);
}

socket.onerror = function (ev) {
    console.error("ws error: ", ev);
    cleanupWs(socket);
}

socket.onclose = function (ev) {
    console.info("got close event: ", ev);
    cleanupWs(socket);
}

// setTimeout(() => {
//     wsSendMessage("Hello, world!");
// }, 1000);

// setTimeout(() => {
//     socket.send('About done here...');
//     console.log("Sending close over websocket");
//     socket.close(3000, "Crash and Burn!");
// }, 3000);