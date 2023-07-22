use crate::user_interface::message_manager::MessageManager;
use crate::user_interface::styles::ContainerStyle;
use crate::user_interface::Event;
use iced::widget::{column, row, Button, Column, Container, Text};
use iced::{theme, Element, Length};

/// Channel selector is a sidebar that shows all the channels
/// that the user is in. It also allows the user to switch
/// between channels.
pub struct ChannelSelector;

impl ChannelSelector {
    pub fn view(message_manager: &MessageManager) -> Element<Event> {
        let column: Column<'_, Event, iced::Renderer> = column(
            message_manager
                .get_active_channels()
                .iter()
                .map(|channel_id| {
                    let channel_name = message_manager
                        .get_channel_by_id(*channel_id)
                        .unwrap()
                        .name
                        .clone();
                    row![Button::new(Text::new(channel_name))
                        .on_press(Event::ChannelSelected(*channel_id))]
                    .padding(10.0)
                    .into()
                })
                .collect(),
        )
        .width(Length::Fixed(200.0))
        .align_items(iced::Alignment::Center);

        Container::new(column)
            .style(theme::Container::Custom(Box::new(ContainerStyle)))
            .height(Length::Fill)
            .into()
    }

    pub fn on_event(event: &Event, message_manager: &mut MessageManager) {
        if let Event::ChannelSelected(channel_id) = event {
            message_manager.current_channel = Some(*channel_id);
        }
    }
}
