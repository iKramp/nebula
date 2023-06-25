use super::user_interface::{FromNetworkingEvent, ToNetworkingEvent};
use crate::user_interface::FromNetworkingEvent::SenderInitialized;
use crate::user_interface::Message;
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

    loop {
        while let Some(message) = to_event_receiver.recv().await {
            match message {
                ToNetworkingEvent::MessageSent(msg) => {
                    event_sender
                        .send(FromNetworkingEvent::MessageReceived(Message {
                            message: msg,
                            sender: "You".to_owned(),
                        }))
                        .await
                        .unwrap();
                    tokio::time::sleep(core::time::Duration::from_millis(500)).await;
                    event_sender
                        .send(FromNetworkingEvent::MessageReceived(Message {
                            message: "Ok".to_owned(),
                            sender: "Other guy".to_owned(),
                        }))
                        .await
                        .unwrap();
                }
            }
        }
        tokio::time::sleep(core::time::Duration::from_millis(1)).await;
    }
}
