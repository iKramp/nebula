use iced::widget::scrollable::{snap_to, Id, RelativeOffset};
use iced::widget::{column, scrollable, text, Column};
use iced::{
    subscription, window, Alignment, Application, Command, Element, Length, Result, Settings,
    Subscription,
};
use std::cell::RefCell;
use tokio::sync::mpsc;

struct NebulaApp {
    receiver: RefCell<Option<mpsc::UnboundedReceiver<Message>>>,
    messages: Vec<String>,
    messages_scrollable_id: Id,
    messages_scroll_position: f32,
}

#[derive(Debug, Clone)]
enum Message {
    MessageReceived(String),
    ScrollingMessages(f32),
}

struct Flags {
    receiver: mpsc::UnboundedReceiver<Message>,
}

impl Application for NebulaApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = Flags;

    fn new(flags: Flags) -> (Self, Command<Message>) {
        (
            Self {
                receiver: RefCell::new(Some(flags.receiver)),
                messages: Vec::new(),
                messages_scrollable_id: Id::unique(),
                messages_scroll_position: 0.0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nebula")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MessageReceived(msg) => {
                self.messages.push(msg);
                if self.messages_scroll_position > 0.999 {
                    snap_to(
                        self.messages_scrollable_id.clone(),
                        RelativeOffset { y: 1.0, x: 0.0 },
                    )
                } else {
                    Command::none()
                }
            }

            Message::ScrollingMessages(scroll) => {
                self.messages_scroll_position = scroll;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let messages_column: Column<Message, _> = column(
            self.messages
                .iter()
                .map(|msg| text::Text::new(msg).size(20).into())
                .collect::<Vec<Element<_>>>(),
        );

        let messages_scrollable = scrollable(messages_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .id(self.messages_scrollable_id.clone())
            .on_scroll(move |scroll| Message::ScrollingMessages(scroll.y));

        let chat_column: Column<_, _> = column![
            text::Text::new("Nebula").size(50),
            text::Text::new("Messages:").size(20),
            messages_scrollable,
        ];

        chat_column
            .padding(5)
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }

    // create a subscription that will be polled for new messages
    fn subscription(&self) -> Subscription<Message> {
        subscription::unfold(
            "external messages",
            self.receiver.take(),
            move |mut receiver| async move {
                let num = receiver.as_mut().unwrap().recv().await.unwrap();
                (num, receiver)
            },
        )
    }
}

async fn background_task(event_sender: mpsc::UnboundedSender<Message>) {
    loop {
        let random_number = rand::random::<u32>() % 100;
        event_sender
            .send(Message::MessageReceived(format!(
                "I send you a random number: {random_number}"
            )))
            .unwrap();
        tokio::time::sleep(core::time::Duration::from_millis(200)).await;
    }
}

#[allow(clippy::unused_async)]
pub async fn start() -> Result {
    let (sender, receiver) = mpsc::unbounded_channel();

    tokio::spawn(background_task(sender));

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
        flags: Flags { receiver },
        default_font: None,
        default_text_size: 20.0,
        text_multithreading: false,
        try_opengles_first: false,
    };

    NebulaApp::run(settings)?;

    Ok(())
}
