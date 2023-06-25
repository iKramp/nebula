mod chat;
mod selectable_text;

use super::networking;
use iced::{
    executor, subscription, window, Application, Command, Element, Result, Settings, Subscription,
    Theme,
};
use tokio::sync::mpsc::UnboundedSender;

/// This is the main iced application struct.
struct NebulaApp {
    /// Sender to the networking thread.
    sender: Option<UnboundedSender<ToNetworkingEvent>>,
    /// Module responsible for the chat ui.
    chat_module: chat::ChatModule,
}

/// This is a message struct that is used to
/// represent a message in the application.
#[derive(Debug, Clone)]
pub struct Message {
    pub message: String,
    pub sender: String,
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
    /// Used when a function needs to return an Event, but it has nothing to return.
    Nothing,
}

/// These events are used to communicate from ui to networking.
#[derive(Debug, Clone)]
pub enum ToNetworkingEvent {
    /// Called when the user sends a message.
    MessageSent(String),
}

/// These events are used to communicate from networking to ui.
#[derive(Debug, Clone)]
pub enum FromNetworkingEvent {
    /// Called when the networking thread has been initialized and is ready to receive messages.
    SenderInitialized(UnboundedSender<ToNetworkingEvent>),
    /// Called when the networking thread has received a message.
    MessageReceived(Message),
}

impl Application for NebulaApp {
    type Executor = executor::Default;
    type Message = Event;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Event>) {
        (
            Self {
                sender: None,
                chat_module: chat::ChatModule::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nebula")
    }

    fn update(&mut self, event: Event) -> Command<Event> {
        if let Event::Networking(FromNetworkingEvent::SenderInitialized(sender)) = event.clone() {
            self.sender = Some(sender);
        }

        let commands = vec![self
            .chat_module
            .on_event(event, self.sender.as_mut().unwrap())];

        Command::batch(commands)
    }

    fn view(&self) -> Element<Event> {
        let chat_view = self.chat_module.view();
        chat_view
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    // create a subscription that will be polled for new messages
    fn subscription(&self) -> Subscription<Event> {
        struct NetworkingWorker;
        subscription::channel(
            std::any::TypeId::of::<NetworkingWorker>(),
            100,
            |sender| async move {
                networking::background_task(sender).await.unwrap();
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
