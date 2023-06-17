use iced::widget::{button, column, text};
use iced::{
    subscription, window, Alignment, Application, Command, Element, Length, Result, Settings,
    Subscription,
};
use std::cell::RefCell;
use tokio::sync::mpsc;

struct Counter {
    value: i32,
    receiver: RefCell<Option<mpsc::UnboundedReceiver<Message>>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

struct Flags {
    receiver: mpsc::UnboundedReceiver<Message>,
}

impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = Flags;

    fn new(flags: Flags) -> (Self, Command<Message>) {
        (
            Self {
                value: 0,
                receiver: RefCell::new(Some(flags.receiver)),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let column = column![
            button("Increment")
                .on_press(Message::IncrementPressed)
                .padding(30),
            text(self.value).size(50),
            button("Decrement")
                .on_press(Message::DecrementPressed)
                .padding(30),
        ];

        column![column.padding(50).align_items(Alignment::Center)]
            .padding(50)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // create a subscription that will be polled for new messages
    fn subscription(&self) -> Subscription<Message> {
        subscription::unfold(
            "led changes",
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
        event_sender.send(Message::IncrementPressed).unwrap();
        tokio::time::sleep(core::time::Duration::from_secs(5)).await;
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

    Counter::run(settings)?;

    Ok(())
}
