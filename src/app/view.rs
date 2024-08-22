use iced::Element;

use super::*;

impl App {
    pub(super) fn loaded_view<'a>(
        &'a self,
        state: &'a LoadedState,
    ) -> Element<<App as iced::Application>::Message> {
        return state.grid.view().map(move |msg| AppMessage::Grid(msg));
    }
}
