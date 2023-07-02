use std::io::{Read, Write};
use std::net::TcpStream;

use super::user_interface::{FromNetworkingEvent, ToNetworkingEvent};
use crate::user_interface::FromNetworkingEvent::SenderInitialized;
use crate::user_interface::{ChannelId, Message, MessageId};
use iced::Result;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{unbounded_channel, Sender, UnboundedReceiver};

pub struct ClientNetworking {
    stream: Option<TcpStream>,
    curr_message_id: u64,
}

impl ClientNetworking {
    pub const fn new() -> Self {
        Self {
            stream: None,
            curr_message_id: 0,
        }
    }

    pub async fn manage_connection(
        &mut self,
        mut event_sender: iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
    ) -> Result {
        let (to_event_sender, mut to_event_receiver) =
            mpsc::unbounded_channel::<ToNetworkingEvent>();
        event_sender
            .try_send(SenderInitialized(to_event_sender))
            .unwrap();

        self.stream =
            Some(TcpStream::connect("localhost:8080").expect("Couldnt connect to server!"));

        println!("Established connection");
        let tmp: ChannelId = ChannelId { id: 1 };

        loop {
            self.send_message(&mut event_sender, &mut to_event_receiver)
                .await;
            self.listen_server(&mut event_sender, tmp).await;
            tokio::time::sleep(core::time::Duration::from_millis(10)).await;
        }

        //println!("Terminated.");
    }

    pub async fn listen_server(
        &mut self,
        event_sender: &mut iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
        channel_id: ChannelId,
    ) {
        let mut buf = [0; 512];
        let bytes_read = self.stream.as_ref().unwrap().read(&mut buf).unwrap();
        if bytes_read == 0 {
            return;
        }
        println!("Got it");
        event_sender
            .try_send(FromNetworkingEvent::Message(
                MessageId::new(self.curr_message_id),
                Message {
                    contents: std::str::from_utf8(buf.get(..bytes_read).unwrap())
                        .unwrap()
                        .to_owned(),
                    sender: "Other guy".to_owned(),
                },
            ))
            .unwrap();

        event_sender
            .try_send(FromNetworkingEvent::MessageReceived(
                channel_id,
                MessageId::new(self.curr_message_id),
            ))
            .unwrap();
        self.curr_message_id += 1;
    }

    pub async fn send_message(
        &mut self,
        event_sender: &mut iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
        to_event_receiver: &mut UnboundedReceiver<ToNetworkingEvent>,
    ) {
        while let Some(message) = to_event_receiver.recv().await {
            match message {
                ToNetworkingEvent::MessageSent(msg, channel_id) => {
                    //send message to yourself
                    event_sender
                        .try_send(FromNetworkingEvent::Message(
                            MessageId::new(self.curr_message_id),
                            Message {
                                contents: msg.clone(),
                                sender: "You".to_owned(),
                            },
                        ))
                        .unwrap();
                    event_sender
                        .try_send(FromNetworkingEvent::MessageReceived(
                            channel_id,
                            MessageId::new(self.curr_message_id),
                        ))
                        .unwrap();

                    self.curr_message_id += 1;

                    //send to server
                    self.stream
                        .as_ref()
                        .unwrap()
                        .write_all(msg.as_bytes())
                        .unwrap();
                    println!("Sending message to server,awaiting reply...");
                    //await reply
                    self.listen_server(event_sender, channel_id).await;
                }
            }
        }
    }
}
