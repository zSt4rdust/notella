pub mod grid;
pub mod update;
pub mod view;

use iced::{event, executor, Application, Command, Element, Event, Size, Theme};

pub struct App {
    pub state: AppState,
}

pub enum AppState {
    Loaded(LoadedState),
}

pub struct LoadedState {
    pub input_value: String,
    pub window_size: Size<u32>,
    pub grid: grid::Grid,
}

impl Default for LoadedState {
    fn default() -> Self {
        Self {
            input_value: String::new(),
            window_size: Size::new(0, 0),
            grid: grid::Grid {
                pixels_per_unit: 25.0,
                min_zoom: 0.5,
                max_zoom: 4.0,
                zoom_sensitivity: 0.05,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    EventOccurred(Event),
    Grid(grid::GridMessage),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: AppState::Loaded(LoadedState::default()),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Notella".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &mut self.state {
            AppState::Loaded(state) => Self::loaded_update(message, state),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match &self.state {
            AppState::Loaded(state) => self.loaded_view(state),
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Nord
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(AppMessage::EventOccurred)
    }
}
