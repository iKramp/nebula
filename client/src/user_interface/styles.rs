use iced::widget::container;
use iced::widget::container::Appearance;
use iced::{Background, Color, Theme};

pub struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 1.0))),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
        }
    }
}
