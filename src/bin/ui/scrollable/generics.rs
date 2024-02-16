use ratatui::{buffer::Buffer, style::Style};
use std::ops::Deref;
use term_rustdoc::{tree::TreeLine, util::XString};

pub trait Lines: Deref<Target = [Self::Line]> {
    type Line: LineState;
}

impl<L: LineState, Ls: Deref<Target = [L]>> Lines for Ls {
    type Line = L;
}

pub trait LineState {
    type State: PartialEq + Default;
    fn state(&self) -> Self::State;
    fn is_identical(&self, state: &Self::State) -> bool;
}

impl LineState for TreeLine {
    type State = Option<XString>;

    fn state(&self) -> Self::State {
        self.id.clone()
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self.id == *state
    }
}
// worry too much about what features they should pick. If you're unsure, we suggest\n
pub fn render_line<'t, I>(line: I, buf: &mut Buffer, mut x: u16, y: u16, width: usize)
where
    I: IntoIterator<Item = (&'t str, Style)>,
{
    let mut used_width = 0usize;
    for (text, style) in line {
        // stop rendering once it hits the end of width
        if used_width > width {
            info!("{text:?} truncated in row {y} col {x} (used {used_width}, maximum {width})");
            return;
        }
        let (x_pos, _) = buf.set_stringn(x, y, text, width, style);
        used_width += x_pos.saturating_sub(x) as usize;
        x = x_pos;
    }
}

pub fn render_line_fill_gap<'t, I>(
    line: I,
    style: Style,
    buf: &mut Buffer,
    mut x: u16,
    y: u16,
    width: usize,
    gap_str: &mut XString,
) where
    I: IntoIterator<Item = &'t str>,
{
    let mut used_width = 0usize;
    for text in line {
        // stop rendering once it hits the end of width
        if used_width > width {
            info!("{text:?} truncated in row {y} col {x} (used {used_width}, maximum {width})");
            return;
        }
        let (x_pos, _) = buf.set_stringn(x, y, text, width, style);
        used_width += x_pos.saturating_sub(x) as usize;
        x = x_pos;
    }
    if let Some(gap) = width.checked_sub(used_width).filter(|gap| *gap > 0) {
        gap_str.clear();
        (0..gap).for_each(|_| gap_str.push(' '));
        buf.set_stringn(x, y, gap_str, gap, style);
    }
}
