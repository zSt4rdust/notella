pub mod text;

use iced::{
    widget::{self, Column},
    Application, Element, Vector,
};

use crate::app::App;

pub trait CardKind {
    fn view(&self) -> Element<<App as Application>::Message>;
}

pub struct Card {
    pub position: Vector,
    pub title: Option<String>,
    pub kind: Box<dyn CardKind>,
}

impl Card {
    pub fn view(&self) -> Element<<App as Application>::Message> {
        let mut column = Column::new();

        if let Some(title) = &self.title {
            column = column.push(widget::text(title));
        }

        column.push(self.kind.view()).into()
    }
}
