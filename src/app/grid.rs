use iced::{
    mouse::{self, Cursor},
    widget::{
        canvas::{self, Frame, Text},
        Canvas,
    },
    Color, Element, Length, Point, Renderer, Size, Theme, Vector,
};

use crate::utils::{PointExt, Record};

#[derive(Debug, Clone)]
pub enum GridMessage {}

pub struct Grid {
    pub pixels_per_unit: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_sensitivity: f32,
}

pub struct GridState {
    translation: Vector,
    dragging: bool,

    zoom: f32,
    cursor_pos: Record<Point>,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            translation: Vector::ZERO,
            dragging: false,

            zoom: 1.0,
            cursor_pos: Record::new(Point::ORIGIN),
        }
    }
}

impl Grid {
    pub fn view(&self) -> Element<GridMessage> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<GridMessage> for Grid {
    type State = GridState;

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> (canvas::event::Status, Option<GridMessage>) {
        match event {
            canvas::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        let cursor_world_before = state.cursor_pos.cur.to_world(
                            self.pixels_per_unit,
                            state.zoom,
                            state.translation,
                            bounds.size(),
                        );

                        let y = if y < 0.0 { -1.0 } else { 1.0 };
                        state.zoom += y * (self.zoom_sensitivity * state.zoom);
                        state.zoom = state.zoom.clamp(self.min_zoom, self.max_zoom);

                        let cursor_world_after = state.cursor_pos.cur.to_world(
                            self.pixels_per_unit,
                            state.zoom,
                            state.translation,
                            bounds.size(),
                        );

                        state.translation.x -= cursor_world_after.x - cursor_world_before.x;
                        state.translation.y -= cursor_world_after.y - cursor_world_before.y;

                        (canvas::event::Status::Captured, None)
                    }
                },
                mouse::Event::ButtonPressed(mouse::Button::Middle) => {
                    state.dragging = true;
                    (canvas::event::Status::Captured, None)
                }
                mouse::Event::ButtonReleased(mouse::Button::Middle) => {
                    state.dragging = false;
                    (canvas::event::Status::Captured, None)
                }
                mouse::Event::CursorMoved { position } => {
                    state.cursor_pos.set(position);

                    if state.dragging {
                        let space_between = self.pixels_per_unit * state.zoom;

                        let delta = position - state.cursor_pos.prev;
                        let units_delta = Vector {
                            x: delta.x / space_between,
                            y: delta.y / space_between,
                        };

                        state.translation = Vector {
                            x: state.translation.x - units_delta.x,
                            y: state.translation.y + units_delta.y,
                        };
                    }

                    (canvas::event::Status::Captured, None)
                }
                _ => (canvas::event::Status::Captured, None),
            },
            _ => (canvas::event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<<Renderer as canvas::Renderer>::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let space_between = self.pixels_per_unit * state.zoom;

        let offset_x = (state.translation.x * space_between - frame.width() / 2.0) % space_between;
        let offset_y = (state.translation.y * space_between + frame.height() / 2.0) % space_between;

        for x in 0..=((frame.width() + space_between) / space_between) as u32 {
            let x = x as f32 * space_between;

            for y in 0..=((frame.height() + space_between) / space_between) as u32 {
                let y = y as f32 * space_between;

                let point = Point {
                    x: x - offset_x,
                    y: y + offset_y,
                };

                frame.fill_rectangle(
                    point,
                    Size::new(2.0, 2.0),
                    Color::from_rgba(1.0, 1.0, 1.0, (0.05 * state.zoom).clamp(0.0, 0.15)),
                );
            }
        }

        #[cfg(debug_assertions)]
        {
            frame.fill_rectangle(
                Point::new(frame.width() / 2.0 - 1.0, 0.0),
                Size::new(2.0, frame.height()),
                Color::from_rgba(1.0, 1.0, 1.0, 0.03),
            );

            frame.fill_rectangle(
                Point::new(0.0, frame.height() / 2.0 - 1.0),
                Size::new(frame.width(), 2.0),
                Color::from_rgba(1.0, 1.0, 1.0, 0.03),
            );

            frame.fill_text(Text {
                content: format!("x: {}, y: {}", state.translation.x, state.translation.y),
                color: Color::WHITE,
                ..Default::default()
            });

            if let Cursor::Available(cursor) = cursor {
                let cursor_world = cursor.to_world(
                    self.pixels_per_unit,
                    state.zoom,
                    state.translation,
                    bounds.size(),
                );

                frame.fill_text(Text {
                    position: Point {
                        x: cursor.x,
                        y: cursor.y - 20.0,
                    },
                    content: format!(
                        "x: {}, y: {}",
                        (cursor_world.x * 10.0).round() / 10.0,
                        (cursor_world.y * 10.0).round() / 10.0
                    ),
                    color: Color::WHITE,
                    ..Default::default()
                });
            }
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if state.dragging {
            return mouse::Interaction::Grab;
        }

        return mouse::Interaction::default();
    }
}
