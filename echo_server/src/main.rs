use std::net::{TcpListener, TcpStream};
use std::thread;

use std::io::Write;
use std::io::Read;

fn handle_client(mut stream: TcpStream) {
	let mut buf = [0; 512];
	loop {
		let n = match stream.read(&mut buf) {
			Err(e) => panic!("Got an error: {}", e),
			Ok(m) => {
				if m == 0 {
					// We got an EOF
					break;
				}
				m
			},
		};

		match stream.write(&buf[0:n]) {
			Err(_) => break,
			Ok(_) => continue,
		}
	}
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:4444").unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move|| {
					handle_client(stream);
				});
			},
			Err(e) => { println!("failed: {}", e) },
		}
	}

	drop(listener);
}