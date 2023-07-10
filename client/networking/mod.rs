use std::io::{Read, Write};
use std::net::TcpStream;

use super::user_interface::{FromNetworkingEvent, ToNetworkingEvent};
use crate::networking;
use crate::user_interface::FromNetworkingEvent::SenderInitialized;
use crate::user_interface::{ChannelId, Message, MessageId};
use iced::Result;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{unbounded_channel, Sender, UnboundedReceiver};

pub struct ClientNetworking {
    stream: Option<TcpStream>,
    curr_message_id: u64,
    id : u8
}

impl ClientNetworking {
    pub const fn new() -> Self {
        Self {
            stream: None,
            curr_message_id: 0,
            id : 0,
        }
    }

    fn request_id(&mut self) -> u8 {
        println!("requesting id...");
        let msg = [1;1];
        self.stream
            .as_ref()
            .unwrap()
            .write_all(&msg);
        let m= self.read_from_server();
        m[0]
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
        self.id = self.request_id();
        println!("my id is {:}", &self.id);

        loop {
            self.send_message(&mut event_sender, &mut to_event_receiver)
                .await;
            self.get_new_messages(&mut event_sender, tmp, 0).await;//add a way to get last message id
            tokio::time::sleep(core::time::Duration::from_millis(1000)).await;
        }

        //println!("Terminated.");
    }

    pub async fn get_new_messages(
        &mut self,
        event_sender: &mut iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
        channel_id: ChannelId,
        last_message_id: u64,
    ) {

        let mut buf = Vec::new();
        buf.push(3 as u8);//id of requesting new messages
        buf.append(&mut channel_id.id.to_be_bytes().to_vec());
        buf.append(&mut last_message_id.to_be_bytes().to_vec());

        self.stream
            .as_ref()
            .unwrap()
            .write_all(&buf)
            .unwrap();
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

    fn read_from_server(&mut self) -> Vec<u8>{
        let mut buf = [0; 2048];
        let bytes_read = self.stream.as_ref().unwrap().read(&mut buf).unwrap();
        buf[..bytes_read].to_vec()
    }


    pub async fn send_message(//TODO: bocchi rethink how this is implemented. it currently blocks the thread until a message is sent from the client.
                              // Before my change it just blocked the thread forever. Cause is awaiting to_event_reciever.recv, which blocks until there is a new message. It returns None only after the sender is dropped or something
        &mut self,
        event_sender: &mut iced::futures::channel::mpsc::Sender<FromNetworkingEvent>,
        to_event_receiver: &mut UnboundedReceiver<ToNetworkingEvent>,
    ) {
        if let Some(message) = to_event_receiver.recv().await {
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
                    let mut buf = Vec::new();
                    buf.push(2 as u8);//id of sending a message
                    buf.append(&mut msg.as_bytes().to_vec());
                    self.stream
                        .as_ref()
                        .unwrap()
                        .write_all(&buf)
                        .unwrap();
                    println!("Sending message to server...");
                }
            }
        }
    }
}
