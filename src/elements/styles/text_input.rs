use iced::{theme, widget::text_input, Border, Theme};

use super::SHARED_CORNER_RADIUS;

pub struct NotellaTextInput;

impl text_input::StyleSheet for NotellaTextInput {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let default_appearance = style.active(&theme::TextInput::Default);

        text_input::Appearance {
            border: Border {
                radius: SHARED_CORNER_RADIUS.into(),
                ..default_appearance.border
            },
            ..default_appearance
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let default_appearance = style.focused(&theme::TextInput::Default);

        text_input::Appearance {
            border: Border {
                radius: SHARED_CORNER_RADIUS.into(),
                ..default_appearance.border
            },
            ..default_appearance
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        style.placeholder_color(&theme::TextInput::Default)
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        style.value_color(&theme::TextInput::Default)
    }

    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        style.disabled_color(&theme::TextInput::Default)
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        style.selection_color(&theme::TextInput::Default)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let default_appearance = style.disabled(&theme::TextInput::Default);

        text_input::Appearance {
            border: Border {
                radius: SHARED_CORNER_RADIUS.into(),
                ..default_appearance.border
            },
            ..default_appearance
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let default_appearance = style.hovered(&theme::TextInput::Default);

        text_input::Appearance {
            border: Border {
                radius: SHARED_CORNER_RADIUS.into(),
                ..default_appearance.border
            },
            ..default_appearance
        }
    }
}
