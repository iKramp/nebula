use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

use super::user_interface::{FromNetworkingEvent, ToNetworkingEvent};
use crate::user_interface::FromNetworkingEvent::SenderInitialized;
use crate::user_interface::{ChannelId, Message, MessageId};
use iced::Result;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct ClientNetworking {
    stream_manager: network_manager::NetworkManager,
    curr_message_id: u64,
    id: u64,
}

impl ClientNetworking {
    pub async fn new() -> Self {
        let stream = TcpStream::connect("localhost:8080").expect("Couldnt connect to server!");
        Self {
            stream_manager: network_manager::NetworkManager::new(stream).await,
            curr_message_id: 0,
            id: 0,
        }
    }

    fn request_id(&mut self) -> u64 {
        println!("requesting id...");
        let data = kvptree::ValueType::LIST(HashMap::from([(
            "request_type_id".to_owned(),
            kvptree::ValueType::STRING("1".to_owned()),
        )]));
        self.stream_manager.send_message(
                kvptree::to_byte_vec(data)
            );
        let id_message = self.stream_manager.wait_for_message().unwrap();
        println!("got message");
        let data = kvptree::from_byte_vec(id_message).unwrap();
        data.get_str("answer.client_id")
            .unwrap()
            .parse::<u64>()
            .unwrap()
        /*m[0]*/
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

        println!("Established connection");
        let tmp: ChannelId = ChannelId { id: 1 };
        self.id = self.request_id();
        println!("my id is {:}", &self.id);

        loop {
            self.send_message(&mut event_sender, &mut to_event_receiver)
                .await;
            self.get_new_messages(/*&mut event_sender, */ tmp, 0).await; //add a way to get last message id
            tokio::time::sleep(core::time::Duration::from_millis(1000)).await;
        }

        //println!("Terminated.");
    }

    #[allow(clippy::unused_async)]
    pub async fn get_new_messages(
        &mut self,
        //event_sender: &mut iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
        channel_id: ChannelId,
        last_message_id: u64,
    ) {
        //send to server
        let data = kvptree::ValueType::LIST(HashMap::from([
            //i need to implement an easier way to do this...
            (
                "request_type_id".to_owned(),
                kvptree::ValueType::STRING("3".to_owned()),
            ),
            (
                "request".to_owned(),
                kvptree::ValueType::LIST(HashMap::from([
                    (
                        "channel_id".to_owned(),
                        kvptree::ValueType::STRING(channel_id.id.to_string()),
                    ),
                    (
                        "last_message".to_owned(),
                        kvptree::ValueType::STRING(last_message_id.to_string()),
                    ),
                ])),
            ),
        ]));
        let buf = kvptree::to_byte_vec(data);

        self.stream_manager.send_message(buf);
        println!("Requesting new messages from server...");

        /*let msg = self.read_from_server();idk what this is so i just commented it


        println!("Got it");
        event_sender
            .try_send(FromNetworkingEvent::Message(
                MessageId::new(self.curr_message_id),
                Message {
                    contents: std::str::from_utf8(&msg)
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
        self.curr_message_id += 1;*/
    }

    fn read_from_server(&mut self) -> Option<Vec<u8>> {
        self.stream_manager.get_message()
    }

    pub async fn send_message(
        //TODO: still shit implementation but at least it shouldn't block
        &mut self,
        event_sender: &mut iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
        to_event_receiver: &mut UnboundedReceiver<ToNetworkingEvent>,
    ) {

            if let Ok(message) = to_event_receiver.try_recv() {
                match message {
                    ToNetworkingEvent::MessageSent(msg, channel_id) => {
                        //send message to yourself
                        event_sender
                            .try_send(FromNetworkingEvent::Message(
                                MessageId::new(self.curr_message_id),
                                Message {
                                    contents: msg.clone(),
                                    sender: self.id.to_string(),
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
                        let data = kvptree::ValueType::LIST(HashMap::from([
                            //i need to implement an easier way to do this...
                            (
                                "request_type_id".to_owned(),
                                kvptree::ValueType::STRING("2".to_owned()),
                            ),
                            (
                                "request".to_owned(),
                                kvptree::ValueType::LIST(HashMap::from([(
                                    "message".to_owned(),
                                    kvptree::ValueType::STRING(msg),
                                )])),
                            ),
                        ]));
                        let buf = kvptree::to_byte_vec(data);
                        self.stream_manager.send_message(buf);
                        println!("Sending message to server...");
                    }
                }
            }
    }
}
