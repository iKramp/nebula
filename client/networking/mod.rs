use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use super::user_interface::{FromNetworkingEvent, ToNetworkingEvent};
use crate::user_interface::FromNetworkingEvent::SenderInitialized;
use crate::user_interface::{Message, MessageId};
use iced::futures::channel::mpsc::Sender;
use iced::futures::SinkExt;
use iced::Result;
use tokio::sync::mpsc;

pub async fn background_task(mut event_sender: Sender<FromNetworkingEvent>) -> Result {
    let (to_event_sender, mut to_event_receiver) = mpsc::unbounded_channel::<ToNetworkingEvent>();
    event_sender
        .send(SenderInitialized(to_event_sender))
        .await
        .unwrap();

    manage_connection();

    loop {
        while let Some(message) = to_event_receiver.recv().await {
            match message {
                ToNetworkingEvent::MessageSent(msg) => {
                    event_sender
                        .send(FromNetworkingEvent::Message(
                            MessageId::new(0),
                            Message {
                                contents: msg,
                                sender: "You".to_owned(),
                            },
                        ))
                        .await
                        .unwrap();
                    tokio::time::sleep(core::time::Duration::from_millis(500)).await;
                    event_sender
                        .send(FromNetworkingEvent::Message(
                            MessageId::new(0),
                            Message {
                                contents: "Ok".to_owned(),
                                sender: "Other guy".to_owned(),
                            },
                        ))
                        .await
                        .unwrap();
                }
            }
        }
        tokio::time::sleep(core::time::Duration::from_millis(10)).await;
    }
}

pub fn manage_connection() {
    match TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8080");

            let msg = b"Hello!";

            stream.write_all(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0_u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {text}");
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {e}");
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {e}");
        }
    }
    println!("Terminated.");
}
