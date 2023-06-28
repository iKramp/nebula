use crate::user_interface::Event;
use iced::widget::{column, Column};
use iced::{Element, Length};

/// Channel selector is a sidebar that shows all the channels
/// that the user is in. It also allows the user to switch
/// between channels.
pub struct ChannelSelector;

impl ChannelSelector {
    pub const fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Event> {
        let column: Column<'_, _, _> = column![];
        column.width(Length::Fixed(200.0)).into()
    }
}
