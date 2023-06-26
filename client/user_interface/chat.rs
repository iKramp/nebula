use super::selectable_text::SelectableText;
use crate::user_interface::{Event, FromNetworkingEvent, Message, ToNetworkingEvent};
use iced::widget::scrollable::{snap_to, RelativeOffset};
use iced::widget::{column, row, scrollable, text::Text, text_input::TextInput, Column, Space};
use iced::{theme, Alignment, Command, Element, Length};
use tokio::sync::mpsc::UnboundedSender;

pub struct ChatModule {
    /// Holds all the messages that are shown.
    messages: Vec<Message>,
    /// Holds the id of a scrollable element that holds the messages.
    messages_scrollable_id: scrollable::Id,
    /// Holds the position of the scrollable element that holds the messages.
    messages_scroll_position: f32,
    /// Holds the text that the user has typed in the message box.
    current_message: String,
}

impl ChatModule {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            messages_scrollable_id: scrollable::Id::unique(),
            messages_scroll_position: 0.0,
            current_message: String::new(),
        }
    }

    pub fn on_event(
        &mut self,
        event: Event,
        sender: &mut UnboundedSender<ToNetworkingEvent>,
    ) -> Command<Event> {
        match event {
            Event::Networking(FromNetworkingEvent::Message(_id, msg)) => {
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
                self.current_message = msg;
                Command::none()
            }

            Event::MessageSubmitted => {
                sender
                    .send(ToNetworkingEvent::MessageSent(self.current_message.clone()))
                    .unwrap();
                self.current_message.clear();
                Command::none()
            }

            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Event> {
        let messages_column: Column<Event, _> = column(
            self.messages
                .iter()
                .map(|msg| {
                    column![
                        Text::new(msg.sender.clone()).size(15),
                        row![
                            Space::new(Length::Fixed(5.0), Length::Fixed(0.0)),
                            TextInput::new("", &msg.contents)
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
            TextInput::new("", &self.current_message)
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
}
