mod channel_selector;
mod chat;
mod message_manager;
mod selectable_text;
mod styles;
mod tests;

use crate::networking::ClientNetworking;

use crate::user_interface::channel_selector::ChannelSelector;
use crate::user_interface::chat::ChatModule;
use iced::widget::row;
use iced::{
    executor, subscription, window, Application, Command, Element, Result, Settings, Subscription,
    Theme,
};
use message_manager::MessageManager;
use tokio::sync::mpsc::UnboundedSender;

/// This is the main iced application struct.
struct NebulaApp {
    /// Sender to the networking thread.
    sender: Option<UnboundedSender<ToNetworkingEvent>>,
    /// Module responsible for handling messages and channels
    message_manager: MessageManager,
    /// Module responsible for the chat ui.
    chat_module: ChatModule,
    // Module responsible for the channel selector ui.
    //channel_selector: ChannelSelector, the object is not yet needed
}

/// Message id is a 64 bit integer.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MessageId {
    id: u64,
}

impl MessageId {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
}

/// This is a message struct that is used to
/// represent a message in the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub contents: String,
    pub sender: String,
}

/// Channel id is a 64 bit integer.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChannelId {
    pub id: u64,
}

impl ChannelId {
    pub const fn new(id: u64) -> Self {
        Self { id }
    }
}

/// This is a channel struct that is used to
/// represent a channel in the application. Channels
/// are usually private messages between two users,
/// group messages, or server messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Channel {
    pub name: String,
    pub messages: Vec<MessageId>,
}

/// Events are used to communicate between ui elements.
#[derive(Debug, Clone)]
pub enum Event {
    /// Any event that was sent from the networking thread.
    Networking(FromNetworkingEvent),
    /// Called when the user types in the message box.
    TextInputted(String),
    /// Called when the user scrolls the messages.
    ScrollingMessages(f32),
    /// Called when the user presses the send button.
    MessageSubmitted,
    /// Called when the user selects a channel.
    ChannelSelected(ChannelId),
    /// Used when a function needs to return an Event, but it has nothing to return.
    Nothing,
}

/// These events are used to communicate from ui to networking.
#[derive(Debug, Clone)]
pub enum ToNetworkingEvent {
    /// Called when the user sends a message.
    MessageSent(String, ChannelId),
}

/// These events are used to communicate from networking to ui.
#[derive(Debug, Clone)]
pub enum FromNetworkingEvent {
    /// Called when the networking thread has been initialized and is ready to receive messages.
    SenderInitialized(UnboundedSender<ToNetworkingEvent>),
    /// Called when the networking thread has received a message or a message has changed.
    Message(MessageId, Message),
    /// Called when active channel id list is received/updated.
    ChannelList(Vec<ChannelId>),
    /// Called when a channel is received/updated.
    Channel(ChannelId, Channel),
    /// Called when a message has been received. Message is received from the front (newest message).
    /// This is just a more efficient way of calling Channel, since it doesn't have to load all the messages.
    MessageReceived(ChannelId, MessageId),
    /// Called when a message has been loaded. Message is loaded from the back (oldest message).
    /// This is just a more efficient way of calling Channel, since it doesn't have to load all the messages.
    #[allow(dead_code)]
    MessageLoaded(ChannelId, MessageId),
}

impl Application for NebulaApp {
    type Executor = executor::Default;
    type Message = Event;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Event>) {
        let mut message_manager = MessageManager::new();
        // TODO: Remove this, this is just for testing.
        // add some test channels to active channels
        message_manager.on_event(FromNetworkingEvent::ChannelList(vec![
            ChannelId::new(0),
            ChannelId::new(1),
            ChannelId::new(2),
        ]));
        message_manager.on_event(FromNetworkingEvent::Channel(
            ChannelId::new(0),
            Channel {
                name: String::from("Channel 0"),
                messages: vec![],
            },
        ));
        message_manager.on_event(FromNetworkingEvent::Channel(
            ChannelId::new(1),
            Channel {
                name: String::from("Channel 1"),
                messages: vec![],
            },
        ));
        message_manager.on_event(FromNetworkingEvent::Channel(
            ChannelId::new(2),
            Channel {
                name: String::from("Channel 2"),
                messages: vec![],
            },
        ));
        message_manager.current_channel = Some(ChannelId::new(0));
        (
            Self {
                sender: None,
                message_manager,
                chat_module: ChatModule::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nebula")
    }

    fn update(&mut self, event: Event) -> Command<Event> {
        // If the networking thread has been initialized, save the sender.
        if let Event::Networking(event) = event.clone() {
            if let FromNetworkingEvent::SenderInitialized(sender) = event.clone() {
                self.sender = Some(sender);
            }
            self.message_manager.on_event(event);
        }

        ChannelSelector::on_event(&event, &mut self.message_manager);

        // Propagate the event to the chat module.
        let commands = vec![self.chat_module.on_event(
            event,
            self.sender.as_mut().unwrap(),
            &self.message_manager,
        )];

        Command::batch(commands)
    }

    fn view(&self) -> Element<Event> {
        let chat_view = self.chat_module.view(&self.message_manager);
        let channel_selector_view = ChannelSelector::view(&self.message_manager);
        row![channel_selector_view, chat_view].into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Event> {
        // create a subscription that will be polled for new messages
        struct NetworkingWorker;
        subscription::channel(
            std::any::TypeId::of::<NetworkingWorker>(),
            100,
            |sender| async move {
                let mut net = ClientNetworking::new();
                net.manage_connection(sender).await.unwrap(); // TODO: uncomment and fix
                panic!("Networking worker died");
            },
        )
        .map(Event::Networking)
    }
}

pub fn start() -> Result {
    let settings = Settings {
        id: None,
        antialiasing: true,
        exit_on_close_request: true,
        window: window::Settings {
            size: (700, 500),
            resizable: true,
            min_size: Some((400, 300)),
            ..window::Settings::default()
        },
        flags: (),
        default_font: None,
        default_text_size: 20.0,
        text_multithreading: false,
        try_opengles_first: false,
    };

    NebulaApp::run(settings)?;

    Ok(())
}
