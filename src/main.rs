// The start of witch 17.3.2025

use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::{result, thread};
use std::io::{Read, Write};

type Result<T> = result::Result<T, ()>;

enum Message {
    ClientConnected(Arc<TcpStream>),
    ClientDisconnected(Arc<TcpStream>),
    NewMessage(Vec<u8>)
}

struct Client {
    connection: Arc<TcpStream>
}

fn server(message: Receiver<Message>) -> Result<()> {
    Ok(())
}

fn client(mut stream: Arc<TcpStream>, message: Sender<Message>) -> Result<()> {
    message.send(Message::ClientConnected(stream.clone())).map_err(|e| {
        eprintln!("Error: could not send message: {e}")
    })?;
    
    let mut buf = vec![0; 64];

    loop {
        let n = stream.deref().read(&mut buf).map_err(|e| {
            eprintln!("Error: could not read message send by client: {e}");
            drop(message.send(Message::ClientDisconnected(stream.clone())));
        })?;

        message.send(Message::NewMessage(buf[0..n].to_vec())).map_err(|e| {
            eprintln!("Error: could not send message to server: {e}") ;
        })?;
    }
}

fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    
    let listener = TcpListener::bind(addr).map_err(|e| {
        eprintln!("Error: could not bind {addr}: {e}");
    })?;

    println!("Info: Listening to {addr}");

    let (message_tx, message_rx) = channel::<Message>();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let message_tx = message_tx.clone();
                thread::spawn(|| {
                    client(Arc::new(stream), message_tx);
                });
            },
            Err(e) => {
                eprintln!("Error: couldn't accept connection: {e}")
            },
        }
    }

    Ok(())
}
