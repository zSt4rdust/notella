use iced::{widget::text, Application, Element};

use crate::app::App;

use super::CardKind;

pub struct TextCard {
    pub text: String,
}

impl CardKind for TextCard {
    fn view(&self) -> Element<<App as Application>::Message> {
        text(self.text.clone()).into()
    }
}
