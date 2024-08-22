use iced::{theme, widget::button, Border, Theme};

use super::SHARED_CORNER_RADIUS;

pub struct NotellaButton {
    style: theme::Button,
}

impl NotellaButton {
    pub fn new(style: theme::Button) -> Self {
        Self { style }
    }
}

impl button::StyleSheet for NotellaButton {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border::with_radius(SHARED_CORNER_RADIUS),
            ..style.active(&self.style)
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border::with_radius(SHARED_CORNER_RADIUS),
            ..style.hovered(&self.style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border::with_radius(SHARED_CORNER_RADIUS),
            ..style.pressed(&self.style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border: Border::with_radius(SHARED_CORNER_RADIUS),
            ..style.disabled(&self.style)
        }
    }
}
