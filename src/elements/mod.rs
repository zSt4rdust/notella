pub mod styles;

use iced::{
    theme,
    widget::{button, text_input, Button, TextInput},
    Application, Element, Renderer,
};

use crate::app::App;

pub fn notella_button<'a>(
    content: impl Into<
        Element<'a, <App as Application>::Message, <App as Application>::Theme, Renderer>,
    >,
    style: theme::Button,
) -> Button<'a, <App as Application>::Message, <App as Application>::Theme, Renderer> {
    button(content).style(theme::Button::Custom(Box::new(
        styles::buttons::NotellaButton::new(style),
    )))
}

pub fn notella_text_input<'a>(
    placeholder: &str,
    value: &str,
) -> TextInput<'a, <App as Application>::Message, <App as Application>::Theme, Renderer> {
    text_input(placeholder, value).style(theme::TextInput::Custom(Box::new(
        styles::text_input::NotellaTextInput,
    )))
}
