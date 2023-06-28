// tests for MessageManager

#[cfg(test)]
mod tests {
    use crate::user_interface::message_manager::MessageManager;
    use crate::user_interface::{Channel, ChannelId, FromNetworkingEvent, Message, MessageId};

    #[test]
    fn test_gets_message_event() {
        let mut manager = MessageManager::new();
        let message_id = MessageId::new(2_735_373);
        let message_id2 = MessageId::new(6_253_374);
        assert_eq!(manager.get_message_by_id(message_id), None);
        // add a message.
        manager.on_event(FromNetworkingEvent::Message(
            message_id,
            Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            },
        ));
        // check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id),
            Some(&Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
        // add another message.
        manager.on_event(FromNetworkingEvent::Message(
            message_id2,
            Message {
                contents: "Hello, again!".to_owned(),
                sender: "Alice".to_owned(),
            },
        ));
        // check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id2),
            Some(&Message {
                contents: "Hello, again!".to_owned(),
                sender: "Alice".to_owned(),
            })
        );
        // check if the first message is still there.
        assert_eq!(
            manager.get_message_by_id(message_id),
            Some(&Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
        // check if any other message isn't there.
        assert_eq!(manager.get_message_by_id(MessageId::new(235_252)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_372)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_374)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(6_253_373)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(6_253_375)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(0)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(1)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2)), None);
    }

    #[test]
    fn test_gets_channel_list() {
        let mut manager = MessageManager::new();
        assert_eq!(manager.get_active_channels(), &Vec::<ChannelId>::new());
        // send a channel list.
        manager.on_event(FromNetworkingEvent::ChannelList(vec![
            ChannelId::new(32_543_534),
            ChannelId::new(54_663_634),
            ChannelId::new(34_624_656),
        ]));
        // check if the channel list is there.
        assert_eq!(
            manager.get_active_channels(),
            &vec![
                ChannelId::new(32_543_534),
                ChannelId::new(54_663_634),
                ChannelId::new(34_624_656)
            ]
        );
        // send another channel list.
        manager.on_event(FromNetworkingEvent::ChannelList(vec![
            ChannelId::new(32_543_534),
            ChannelId::new(54_663_634),
            ChannelId::new(34_624_656),
            ChannelId::new(3_465_738_563),
        ]));
        // check if the second channel list is there.
        assert_eq!(
            manager.get_active_channels(),
            &vec![
                ChannelId::new(32_543_534),
                ChannelId::new(54_663_634),
                ChannelId::new(34_624_656),
                ChannelId::new(3_465_738_563)
            ]
        );
    }

    #[test]
    fn test_gets_channel() {
        let mut manager = MessageManager::new();
        let channel_id = ChannelId::new(32_543_534);
        let channel_id2 = ChannelId::new(54_663_634);
        assert_eq!(manager.get_channel_by_id(channel_id), None);
        // Add a channel.
        manager.on_event(FromNetworkingEvent::Channel(
            channel_id,
            Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            },
        ));
        // Check if the channel is there.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            })
        );
        // Add another channel.
        manager.on_event(FromNetworkingEvent::Channel(
            channel_id2,
            Channel {
                name: "Random".to_owned(),
                messages: Vec::new(),
            },
        ));
        // Check if the second channel is there.
        assert_eq!(
            manager.get_channel_by_id(channel_id2),
            Some(&Channel {
                name: "Random".to_owned(),
                messages: Vec::new(),
            })
        );
        // Check if the first channel is still there.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            })
        );
        // Check that any random channel is not there.
        assert_eq!(
            manager.get_channel_by_id(ChannelId::new(3_465_738_563)),
            None
        );
        assert_eq!(manager.get_channel_by_id(ChannelId::new(0)), None);
        assert_eq!(manager.get_channel_by_id(ChannelId::new(1)), None);
        assert_eq!(manager.get_channel_by_id(ChannelId::new(2)), None);
        assert_eq!(
            manager.get_channel_by_id(ChannelId::new(3_453_467_543)),
            None
        );
        assert_eq!(manager.get_channel_by_id(ChannelId::new(65_436_354)), None);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_gets_message_received() {
        let mut manager = MessageManager::new();
        let channel_id = ChannelId::new(32_543_534);
        let message_id = MessageId::new(2_735_373);
        let message_id2 = MessageId::new(6_253_374);
        let message_id3 = MessageId::new(235_252);
        assert_eq!(manager.get_channel_by_id(channel_id), None);
        // Add a channel.
        manager.on_event(FromNetworkingEvent::Channel(
            channel_id,
            Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            },
        ));
        // Check if the channel is there.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            })
        );
        // Check if any other channel is not there.
        assert_eq!(
            manager.get_channel_by_id(ChannelId::new(3_465_738_563)),
            None
        );
        assert_eq!(manager.get_channel_by_id(ChannelId::new(0)), None);
        assert_eq!(manager.get_channel_by_id(ChannelId::new(1)), None);
        assert_eq!(manager.get_channel_by_id(ChannelId::new(2)), None);
        assert_eq!(
            manager.get_channel_by_id(ChannelId::new(3_453_467_543)),
            None
        );
        assert_eq!(manager.get_channel_by_id(ChannelId::new(65_436_354)), None);

        // Add a message received event.
        manager.on_event(FromNetworkingEvent::MessageReceived(channel_id, message_id));
        manager.on_event(FromNetworkingEvent::Message(
            message_id,
            Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            },
        ));
        // Check if the message is in the channel.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id],
            })
        );
        // Check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id),
            Some(&Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
        // Add another message received event.
        manager.on_event(FromNetworkingEvent::MessageReceived(
            channel_id,
            message_id2,
        ));
        manager.on_event(FromNetworkingEvent::Message(
            message_id2,
            Message {
                contents: "Hello, again!".to_owned(),
                sender: "Alice".to_owned(),
            },
        ));
        // Check if the message is in the channel.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id, message_id2],
            })
        );
        // Check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id2),
            Some(&Message {
                contents: "Hello, again!".to_owned(),
                sender: "Alice".to_owned(),
            })
        );
        // Add another message received event.
        manager.on_event(FromNetworkingEvent::MessageReceived(
            channel_id,
            message_id3,
        ));
        manager.on_event(FromNetworkingEvent::Message(
            message_id3,
            Message {
                contents: "Hello, once more!".to_owned(),
                sender: "Bob".to_owned(),
            },
        ));
        // Check if the message is in the channel.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id, message_id2, message_id3],
            })
        );
        // Check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id3),
            Some(&Message {
                contents: "Hello, once more!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
    }

    #[test]
    fn test_gets_message_loaded() {
        let mut manager = MessageManager::new();
        let channel_id = ChannelId::new(32_543_534);
        let message_id = MessageId::new(2_735_373);
        let message_id2 = MessageId::new(6_253_374);
        assert_eq!(manager.get_channel_by_id(channel_id), None);
        // Add a channel.
        manager.on_event(FromNetworkingEvent::Channel(
            channel_id,
            Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            },
        ));
        // Check if the channel is there.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            })
        );
        // Add a message loaded event.
        manager.on_event(FromNetworkingEvent::MessageLoaded(channel_id, message_id));
        manager.on_event(FromNetworkingEvent::Message(
            message_id,
            Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            },
        ));
        // Check if the message is in the channel.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id],
            })
        );
        // Check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id),
            Some(&Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
        // Add another message loaded event.
        manager.on_event(FromNetworkingEvent::MessageLoaded(channel_id, message_id2));
        manager.on_event(FromNetworkingEvent::Message(
            message_id2,
            Message {
                contents: "Hello, again!".to_owned(),
                sender: "Alice".to_owned(),
            },
        ));
        // Check if the message is in the channel.
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id2, message_id],
            })
        );
        // Check if the message is there.
        assert_eq!(
            manager.get_message_by_id(message_id2),
            Some(&Message {
                contents: "Hello, again!".to_owned(),
                sender: "Alice".to_owned(),
            })
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_everything_mixed() {
        let mut manager = MessageManager::new();
        let channel_id = ChannelId::new(32_543_534);
        let channel_id2 = ChannelId::new(3_465_738_563);
        let message_id = MessageId::new(2_735_373);
        let message_id2 = MessageId::new(6_253_374);
        let message_id3 = MessageId::new(235_252);
        let message_id4 = MessageId::new(2_735_242);
        let message_id5 = MessageId::new(6_253_234);
        let message_id6 = MessageId::new(235_232);
        // check that active channel list is empty
        assert_eq!(manager.get_active_channels(), &Vec::<ChannelId>::new());
        // update active channel list
        manager.on_event(FromNetworkingEvent::ChannelList(vec![channel_id]));
        assert_eq!(manager.get_active_channels(), &vec![channel_id]);
        // check that channel is not there
        assert_eq!(manager.get_channel_by_id(channel_id), None);
        // add channel
        manager.on_event(FromNetworkingEvent::Channel(
            channel_id,
            Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            },
        ));
        // check that channel is there
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            })
        );
        // check that any other channel is not there
        assert_eq!(manager.get_channel_by_id(channel_id2), None);
        assert_eq!(manager.get_channel_by_id(ChannelId::new(65_436_152)), None);
        // update active channel list
        manager.on_event(FromNetworkingEvent::ChannelList(vec![
            channel_id,
            channel_id2,
        ]));
        assert_eq!(
            manager.get_active_channels(),
            &vec![channel_id, channel_id2]
        );
        // Add a second channel.
        manager.on_event(FromNetworkingEvent::Channel(
            channel_id2,
            Channel {
                name: "Other".to_owned(),
                messages: Vec::new(),
            },
        ));
        // check that channel is there
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: Vec::new(),
            })
        );
        // check that other channel is there
        assert_eq!(
            manager.get_channel_by_id(channel_id2),
            Some(&Channel {
                name: "Other".to_owned(),
                messages: Vec::new(),
            })
        );
        // add message received event
        manager.on_event(FromNetworkingEvent::MessageReceived(channel_id, message_id));
        // check that message is not there
        assert_eq!(manager.get_message_by_id(message_id), None);
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id],
            })
        );
        // add message
        manager.on_event(FromNetworkingEvent::Message(
            message_id,
            Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            },
        ));
        // check that message is there
        assert_eq!(
            manager.get_message_by_id(message_id),
            Some(&Message {
                contents: "Hello, world!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
        // check that other message is not there
        assert_eq!(manager.get_message_by_id(message_id2), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_246)), None);
        // add message received event
        manager.on_event(FromNetworkingEvent::MessageReceived(
            channel_id,
            message_id2,
        ));
        // check that message is not there
        assert_eq!(manager.get_message_by_id(message_id2), None);
        assert_eq!(
            manager.get_channel_by_id(channel_id),
            Some(&Channel {
                name: "General".to_owned(),
                messages: vec![message_id, message_id2],
            })
        );
        // add message
        manager.on_event(FromNetworkingEvent::Message(
            message_id2,
            Message {
                contents: "Hello, once more!".to_owned(),
                sender: "Bob".to_owned(),
            },
        ));
        // check that message is there
        assert_eq!(
            manager.get_message_by_id(message_id2),
            Some(&Message {
                contents: "Hello, once more!".to_owned(),
                sender: "Bob".to_owned(),
            })
        );
        // check that other message is not there
        assert_eq!(manager.get_message_by_id(message_id3), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_247)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_248)), None);
        // add message received event
        manager.on_event(FromNetworkingEvent::MessageReceived(
            channel_id2,
            message_id3,
        ));
        // check that message is not there
        assert_eq!(manager.get_message_by_id(message_id3), None);
        assert_eq!(
            manager.get_channel_by_id(channel_id2),
            Some(&Channel {
                name: "Other".to_owned(),
                messages: vec![message_id3],
            })
        );
        // add message
        manager.on_event(FromNetworkingEvent::Message(
            message_id3,
            Message {
                contents: "Hello, once more!".to_owned(),
                sender: "Alice".to_owned(),
            },
        ));
        // check that message is there
        assert_eq!(
            manager.get_message_by_id(message_id3),
            Some(&Message {
                contents: "Hello, once more!".to_owned(),
                sender: "Alice".to_owned(),
            })
        );
        // check that other message is not there
        assert_eq!(manager.get_message_by_id(message_id4), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_249)), None);
        assert_eq!(manager.get_message_by_id(MessageId::new(2_735_250)), None);
        // add message loaded event
        manager.on_event(FromNetworkingEvent::MessageLoaded(channel_id2, message_id4));
        // add another message received event
        manager.on_event(FromNetworkingEvent::MessageReceived(
            channel_id2,
            message_id5,
        ));
        // add another message loaded event
        manager.on_event(FromNetworkingEvent::MessageLoaded(channel_id2, message_id6));
        // check that the order is correct
        assert_eq!(
            manager.get_channel_by_id(channel_id2),
            Some(&Channel {
                name: "Other".to_owned(),
                messages: vec![message_id6, message_id4, message_id3, message_id5],
            })
        );
    }
}
