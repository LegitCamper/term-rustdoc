use super::Scrollable;
use crate::Result;
use ratatui::layout::Rect;
use std::{fmt, ops::Deref};
use term_rustdoc::tree::Text as StyledText;

mod parse;

/// Scrollable text area for displaying markdown.
pub type ScrollText = Scrollable<StyledLines>;

pub struct StyledLine {
    line: Vec<StyledText>,
}

impl AsRef<[StyledText]> for StyledLine {
    fn as_ref(&self) -> &[StyledText] {
        &self.line
    }
}

#[derive(Default)]
pub struct StyledLines {
    lines: Vec<StyledLine>,
}

impl fmt::Debug for StyledLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StyledLines")
            .field("lines-len", &self.lines.len())
            .finish()
    }
}

impl Deref for StyledLines {
    type Target = [StyledLine];

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl StyledLines {
    pub fn new(doc: &str) -> Self {
        parse::md(doc)
    }
}

impl ScrollText {
    pub fn new_text(doc: &str, area: Rect) -> Result<Self> {
        // TODO:max_windth and text wrap for markdown
        Ok(Scrollable {
            lines: StyledLines::new(doc),
            area,
            ..Default::default()
        })
    }
}
