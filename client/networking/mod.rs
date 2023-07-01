use std::io::{Read, Write};
use std::net::TcpStream;

use super::user_interface::{FromNetworkingEvent, ToNetworkingEvent};
use crate::user_interface::FromNetworkingEvent::SenderInitialized;
use crate::user_interface::{ChannelId, Message, MessageId};
use iced::futures::channel::mpsc::{Sender, UnboundedReceiver};
use iced::futures::SinkExt;
use iced::Result;
use tokio::sync::mpsc;

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
        mut event_sender: Sender<FromNetworkingEvent>,
    ) -> Result {
        let (to_event_sender, mut to_event_receiver) =
            mpsc::unbounded_channel::<ToNetworkingEvent>();
        event_sender
            .send(SenderInitialized(to_event_sender))
            .await
            .unwrap();



        self.stream =
            Some(TcpStream::connect("localhost:8080").expect("Couldnt connect to server!"));

        println!("Established connection");
        let tmp : ChannelId = ChannelId{
            id : 1
        };

        loop {
            self.send_message(event_sender,to_event_receiver).await;
            self.listen_server(event_sender, tmp).await;
            tokio::time::sleep(core::time::Duration::from_millis(10)).await;    
        }

        //println!("Terminated.");
    }

    pub async fn listen_server(
        &mut self,
        mut event_sender: Sender<FromNetworkingEvent>,
        channel_id: ChannelId,
    ) {
        let mut buf = [0; 512];
        let bytes_read = self.stream.as_ref().unwrap().read(&mut buf).unwrap();
        if bytes_read == 0 {
            return;
        }
        println!("Got it");
        event_sender
            .send(FromNetworkingEvent::Message(
                MessageId::new(self.curr_message_id),
                Message {
                    contents: std::str::from_utf8(buf.get(..bytes_read).unwrap())
                        .unwrap()
                        .to_owned(),
                    sender: "Other guy".to_owned(),
                },
            ))
            .await
            .unwrap();

        event_sender
            .send(FromNetworkingEvent::MessageReceived(
                channel_id,
                MessageId::new(self.curr_message_id),
            ))
            .await
            .unwrap();
        self.curr_message_id += 1;
    }

    pub async fn send_message(&mut self, event_sender : Sender<FromNetworkingEvent>, mut to_event_receiver : UnboundedReceiver<ToNetworkingEvent>) {
        while let Some(message) = to_event_receiver.recv().await {
            match message {
                ToNetworkingEvent::MessageSent(msg, channel_id) => {
                    //send message to yourself
                    event_sender
                        .send(FromNetworkingEvent::Message(
                            MessageId::new(self.curr_message_id),
                            Message {
                                contents: msg.clone(),
                                sender: "You".to_owned(),
                            },
                        ))
                        .await
                        .unwrap();
                    event_sender
                        .send(FromNetworkingEvent::MessageReceived(
                            channel_id,
                            MessageId::new(self.curr_message_id),
                        ))
                        .await
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
                    self.listen_server(event_sender.clone(), channel_id).await;
                }
            }
        }

    }
}
