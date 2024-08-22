use super::*;

impl App {
    pub(super) fn loaded_update(
        _message: <App as iced::Application>::Message,
        _state: &mut LoadedState,
    ) -> Command<<App as iced::Application>::Message> {
        Command::none()
    }
}
