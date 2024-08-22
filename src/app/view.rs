use iced::Element;

use super::*;

impl App {
    pub(super) fn loaded_view<'a>(
        &'a self,
        state: &'a LoadedState,
    ) -> Element<<App as iced::Application>::Message> {
        let grid = state.grid.view();
        return grid;
    }
}
