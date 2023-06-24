use super::networking;
use iced::widget::scrollable::{snap_to, RelativeOffset};
use iced::widget::text_input::Appearance;
use iced::widget::{column, row, scrollable, text, text_input, Column, Space};
use iced::{
    executor, subscription, theme, window, Alignment, Application, Background, Color, Command,
    Element, Length, Result, Settings, Subscription, Theme,
};
use tokio::sync::mpsc::UnboundedSender;

struct NebulaApp {
    sender: Option<UnboundedSender<ToNetworkingEvent>>,
    messages: Vec<Message>,
    messages_scrollable_id: scrollable::Id,
    messages_scroll_position: f32,
    curr_message: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub message: String,
    pub sender: String,
}

#[derive(Debug, Clone)]
enum Event {
    Networking(FromNetworkingEvent),
    TextInputted(String),
    ScrollingMessages(f32),
    MessageSubmitted,
    Nothing,
}

#[derive(Debug, Clone)]
pub enum ToNetworkingEvent {
    MessageSent(String),
}

#[derive(Debug, Clone)]
pub enum FromNetworkingEvent {
    SenderInitialized(UnboundedSender<ToNetworkingEvent>),
    MessageReceived(Message),
}

struct SelectableText;

impl text_input::StyleSheet for SelectableText {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            icon_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 0.0)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(1.0, 1.0, 1.0, 1.0)
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgba(0.0, 0.0, 0.0, 0.0)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.3, 0.3, 1.0)
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }
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
                messages: Vec::new(),
                messages_scrollable_id: scrollable::Id::unique(),
                messages_scroll_position: 0.0,
                curr_message: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nebula")
    }

    fn update(&mut self, message: Event) -> Command<Event> {
        let res = match message {
            Event::Networking(event) => match event {
                FromNetworkingEvent::MessageReceived(msg) => {
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
                FromNetworkingEvent::SenderInitialized(sender) => {
                    self.sender = Some(sender);
                    Command::none()
                }
            },

            Event::ScrollingMessages(scroll) => {
                self.messages_scroll_position = scroll;
                Command::none()
            }

            Event::TextInputted(msg) => {
                self.curr_message = msg;
                Command::none()
            }

            Event::MessageSubmitted => {
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(ToNetworkingEvent::MessageSent(self.curr_message.clone()))
                    .unwrap();
                self.curr_message.clear();
                Command::none()
            }

            Event::Nothing => Command::none(),
        };
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
                            text_input::TextInput::new("", &msg.message)
                                .on_input(|_| Event::Nothing)
                                .size(20)
                                .style(theme::TextInput::Custom(Box::new(SelectableText))),
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
                .on_submit(Event::MessageSubmitted)
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
