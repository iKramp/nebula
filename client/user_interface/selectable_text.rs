use iced::widget::text_input;
use iced::{Background, Color, Theme};

pub struct SelectableText;

impl text_input::StyleSheet for SelectableText {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            icon_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
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

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        self.active(style)
    }
}
