use iced::widget::scrollable::{snap_to, Id, RelativeOffset};
use iced::widget::{column, row, scrollable, text, text_input, Column, Space};
use iced::{
    subscription, window, Alignment, Application, Command, Element, Length, Result, Settings,
    Subscription,
};
use std::cell::RefCell;
use tokio::sync::mpsc;

struct NebulaApp {
    receiver: RefCell<Option<mpsc::UnboundedReceiver<Event>>>,
    sender: crossbeam_channel::Sender<Event>,
    messages: Vec<Message>,
    messages_scrollable_id: Id,
    messages_scroll_position: f32,
    curr_message: String,
}

#[derive(Debug, Clone)]
struct Message {
    pub message: String,
    pub sender: String,
}

#[derive(Debug, Clone)]
enum Event {
    MessageReceived(Message),
    MessageSent(String),
    TextInputted(String),
    ScrollingMessages(f32),
}

struct Flags {
    receiver: mpsc::UnboundedReceiver<Event>,
    sender: crossbeam_channel::Sender<Event>,
}

impl Application for NebulaApp {
    type Executor = iced::executor::Default;
    type Message = Event;
    type Theme = iced::Theme;
    type Flags = Flags;

    fn new(flags: Flags) -> (Self, Command<Event>) {
        (
            Self {
                receiver: RefCell::new(Some(flags.receiver)),
                sender: flags.sender,
                messages: Vec::new(),
                messages_scrollable_id: Id::unique(),
                messages_scroll_position: 0.0,
                curr_message: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nebula")
    }

    fn update(&mut self, mut message: Event) -> Command<Event> {
        let res = match message.clone() {
            Event::MessageReceived(msg) => {
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

            Event::ScrollingMessages(scroll) => {
                self.messages_scroll_position = scroll;
                Command::none()
            }

            Event::TextInputted(msg) => {
                self.curr_message = msg;
                Command::none()
            }

            Event::MessageSent(_) => {
                message = Event::MessageSent(self.curr_message.clone());
                self.curr_message.clear();
                Command::none()
            }
        };
        self.sender.send(message).unwrap();
        res
    }

    fn view(&self) -> Element<Event> {
        let messages_column: Column<Event, _> = column(
            self.messages
                .iter()
                .map(|msg| {
                    column![
                        text::Text::new(msg.sender.clone()).size(15),
                        row![
                            Space::new(Length::Fixed(5.0), Length::Fixed(0.0)),
                            text::Text::new(msg.message.clone()).size(20),
                        ]
                    ]
                    .into()
                })
                .collect::<Vec<Element<_>>>(),
        );

        let messages_scrollable = scrollable(messages_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .id(self.messages_scrollable_id.clone())
            .on_scroll(move |scroll| Event::ScrollingMessages(scroll.y));

        let chat_column: Column<_, _> = column![
            messages_scrollable,
            text_input::TextInput::new("", &self.curr_message)
                .on_input(Event::TextInputted)
                .on_submit(Event::MessageSent(String::new()))
                .padding(5)
                .size(20),
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
    fn subscription(&self) -> Subscription<Event> {
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

async fn background_task(
    event_sender: mpsc::UnboundedSender<Event>,
    event_receiver: crossbeam_channel::Receiver<Event>,
) {
    loop {
        while let Ok(message) = event_receiver.recv() {
            if let Event::MessageSent(msg) = message {
                event_sender
                    .send(Event::MessageReceived(Message {
                        message: msg,
                        sender: "You".to_owned(),
                    }))
                    .unwrap();
                tokio::time::sleep(core::time::Duration::from_millis(500)).await;
                event_sender
                    .send(Event::MessageReceived(Message {
                        message: "Ok".to_owned(),
                        sender: "Other guy".to_owned(),
                    }))
                    .unwrap();
            }
        }
        tokio::time::sleep(core::time::Duration::from_millis(1)).await;
    }
}

#[allow(clippy::unused_async)]
pub async fn start() -> Result {
    let (sender, receiver) = mpsc::unbounded_channel();
    let (sender2, receiver2) = crossbeam_channel::unbounded();

    tokio::spawn(background_task(sender, receiver2));

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
        flags: Flags {
            receiver,
            sender: sender2,
        },
        default_font: None,
        default_text_size: 20.0,
        text_multithreading: false,
        try_opengles_first: false,
    };

    NebulaApp::run(settings)?;

    Ok(())
}
