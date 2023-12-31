use crate::user_interface::{Channel, ChannelId, FromNetworkingEvent, Message, MessageId};
use std::collections::HashMap;

/// `MessageManager` is responsible for keeping track of
/// messages and channels. It is also responsible for
/// receiving events from the networking thread.
#[derive(Debug)]
pub struct MessageManager {
    /// The current channel that the user is in.
    pub current_channel: Option<ChannelId>,

    /// Holds all the channels that the user is in.
    active_channels: Vec<ChannelId>,

    /// has all channels by id.
    channels: HashMap<ChannelId, Channel>,

    /// has all messages by id.
    messages: HashMap<MessageId, Message>,
}

impl MessageManager {
    pub fn new() -> Self {
        Self {
            current_channel: None,
            active_channels: Vec::new(),
            channels: HashMap::new(),
            messages: HashMap::new(),
        }
    }

    /// Handle an event from the networking thread.
    pub fn on_event(&mut self, event: FromNetworkingEvent) {
        match event {
            FromNetworkingEvent::SenderInitialized(_) => {} // ignore

            FromNetworkingEvent::Message(id, message) => {
                self.messages.insert(id, message);
            }

            FromNetworkingEvent::ChannelList(channels) => {
                self.active_channels = channels;
            }

            FromNetworkingEvent::Channel(id, channel) => {
                self.channels.insert(id, channel);
            }

            FromNetworkingEvent::MessageReceived(id, message) => {
                self.channels.get_mut(&id).unwrap().messages.push(message);
            }

            FromNetworkingEvent::MessageLoaded(id, message) => {
                self.channels
                    .get_mut(&id)
                    .unwrap()
                    .messages
                    .insert(0, message);
            }
        }
    }

    /// Get reference to all active channels.
    pub const fn get_active_channels(&self) -> &Vec<ChannelId> {
        &self.active_channels
    }

    /// Get channel by id.
    pub fn get_channel_by_id(&self, id: ChannelId) -> Option<&Channel> {
        self.channels.get(&id)
    }

    /// Get message by id.
    pub fn get_message_by_id(&self, id: MessageId) -> Option<&Message> {
        self.messages.get(&id)
    }
}
