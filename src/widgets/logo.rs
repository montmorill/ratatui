use indoc::indoc;

use crate::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

/// A fun example of using half block characters to draw a logo
pub struct RatatuiLogo {
    size: Size,
}

/// The size of the logo
#[non_exhaustive]
pub enum Size {
    /// A tiny logo
    Tiny,
    /// A small logo
    Small,
}

impl Widget for RatatuiLogo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let logo = match self.size {
            Size::Tiny => Self::tiny(),
            Size::Small => Self::small(),
        };
        Text::raw(logo).render(area, buf);
    }
}

impl RatatuiLogo {
    /// Create a new Ratatui logo widget
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    fn tiny() -> &'static str {
        indoc! {"
            ▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
            ▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
        "}
    }

    fn small() -> &'static str {
        indoc! {"
            █▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █
            █▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
        "}
    }
}
