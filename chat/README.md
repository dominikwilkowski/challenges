## The Problem

Build a TCP chat server where multiple clients connect and messages are broadcast to all connected clients.

## Requirements
1. Listen on `127.0.0.1:8080` using `tokio::net::TcpListener`
2. When a client connects, assign them a name (e.g., User1, User2, etc.)
3. Broadcast a join message to all other clients: "User3 has joined the chat"
4. When a client sends a message, broadcast it to all other connected clients (not back to the sender) in the format: "User3: hello everyone"
5. When a client disconnects, broadcast a leave message: "User3 has left the chat"
6. Use `tokio::sync::broadcast` channel for message distribution
7. Handle errors gracefully — one client disconnecting should never crash the server or affect other clients

How to test it:
- Run your server with cargo run --bin hw3
- Open 2-3 terminals and connect with: telnet 127.0.0.1 8080 (or `ncat`, or Test-NetConnection + a simple TCP client)
- Type messages in one terminal and see them appear in the others

Hints:
- `TcpListener::accept()` in a loop to accept new connections
- `tokio::spawn` a new task per client
- `tokio::io::AsyncBufReadExt` gives you `lines()` on a BufReader for reading line-by-line
- `tokio::io::AsyncWriteExt` gives you `write_all()` for sending
- `TcpStream` can be split into a reader and writer with `tcp_stream.into_split()`
- `broadcast::channel(capacity)` — each receiver gets all messages sent after it subscribes
- Use `tokio::select!` to simultaneously wait for:
	- (a) a new line from the client
	- (b) a new broadcast message to send to the client
