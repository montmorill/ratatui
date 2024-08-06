use indoc::indoc;

use crate::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

/// A fun example of using half block characters to draw a logo
pub struct RatatuiLogo;

impl Widget for RatatuiLogo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let logo = indoc! {"
            █▀▀▄ ▄▀▀▄ ▀█▀ ▄▀▀▄ ▀█▀ █  █ █
            █▀▀▄ █▀▀█  █  █▀▀█  █  ▀▄▄▀ █
        "};
        Text::raw(logo).render(area, buf);
    }
}
