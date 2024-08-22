use iced::window;

use super::*;

impl App {
    pub(super) fn loaded_update(
        message: <App as iced::Application>::Message,
        state: &mut LoadedState,
    ) -> Command<<App as iced::Application>::Message> {
        match message {
            AppMessage::EventOccurred(event) => match event {
                Event::Window(id, window::Event::Resized { width, height }) => {
                    if id == window::Id::MAIN {
                        state.window_size = Size::new(width, height);
                    }

                    Command::none()
                }
                _ => Command::none(),
            },
            _ => Command::none(),
        }
    }
}
