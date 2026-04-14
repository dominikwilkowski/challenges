use tokio::{
	io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
	net::TcpListener,
	sync::broadcast,
};

#[derive(Clone, Debug)]
struct ChatMessage {
	sender: String,
	content: String,
}

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").await.expect("failed to bind to 127.0.0.1:8080");

	println!("Chat server listening on 127.0.0.1:8080");

	let (sender, _) = broadcast::channel::<ChatMessage>(100);
	let mut user_counter: u64 = 0;

	loop {
		let (stream, address) = match listener.accept().await {
			Ok(connection) => connection,
			Err(error) => {
				eprintln!("Failed to accept connection: {error}");
				continue;
			},
		};

		user_counter += 1;
		let username = format!("User{user_counter}");
		let sender = sender.clone();
		let mut receiver = sender.subscribe();

		println!("{username} connected from {address}");

		tokio::spawn(async move {
			let (reader, mut writer) = stream.into_split();
			let mut lines = BufReader::new(reader).lines();

			let _ = sender.send(ChatMessage {
				sender: username.clone(),
				content: format!("{username} has joined the chat"),
			});

			loop {
				tokio::select! {
					line_result = lines.next_line() => {
						match line_result {
							Ok(Some(line)) if !line.is_empty() => {
								let _ = sender.send(ChatMessage {
									sender: username.clone(),
									content: format!("{username}: {line}"),
								});
							}
							// Client disconnected or empty EOF
							Ok(_) => break,
							Err(error) => {
								eprintln!("Read error for {username}: {error}");
								break;
							}
						}
					}
					broadcast_result = receiver.recv() => {
						match broadcast_result {
							Ok(message) if message.sender != username => {
								let formatted = format!("{}\n", message.content);
								if writer.write_all(formatted.as_bytes()).await.is_err() {
									break;
								}
							}
							// Skip own messages
							Ok(_) => {}
							// If we fell behind, just keep going
							Err(broadcast::error::RecvError::Lagged(count)) => {
								eprintln!("{username} lagged, missed {count} messages");
							}
							Err(broadcast::error::RecvError::Closed) => break,
						}
					}
				}
			}

			let _ = sender.send(ChatMessage {
				sender: username.clone(),
				content: format!("{username} has left the chat"),
			});

			println!("{username} disconnected");
		});
	}
}
