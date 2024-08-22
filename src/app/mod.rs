pub mod grid;
pub mod update;
pub mod view;

use std::vec;

use iced::{event, executor, Application, Command, Element, Event, Theme, Vector};

use crate::cards::{text::TextCard, Card};

pub struct App {
    pub state: AppState,
}

pub enum AppState {
    Board(LoadedState),
}

pub struct LoadedState {
    pub grid: grid::Grid,
    pub cards: Vec<Card>,
}

impl Default for LoadedState {
    fn default() -> Self {
        Self {
            grid: grid::Grid {
                pixels_per_unit: 25.0,
                min_zoom: 0.5,
                max_zoom: 4.0,
                zoom_sensitivity: 0.05,
            },
            cards: vec![
                Card {
                    position: Vector::new(10.0, 5.0),
                    title: None,
                    kind: Box::new(TextCard {
                        text: "Hello, World! 1".to_owned(),
                    }),
                },
                Card {
                    position: Vector::new(5.0, 10.0),
                    title: None,
                    kind: Box::new(TextCard {
                        text: "Hello, World! 2".to_owned(),
                    }),
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    EventOccurred(Event),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::Board(LoadedState::default()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Notella".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &mut self.state {
            AppState::Board(state) => Self::loaded_update(message, state),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            AppState::Board(state) => self.loaded_view(state),
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Nord
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(AppMessage::EventOccurred)
    }
}
