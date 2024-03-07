#![allow(clippy::redundant_static_lifetimes)]
use crate::{
    color::NEW,
    ui::{render_line, LineState, Scroll, Surround},
};
use ratatui::{
    prelude::{Buffer, Rect},
    widgets::{Block, BorderType, Borders},
};
use rustdoc_types::ItemEnum;
use term_rustdoc::tree::{CrateDoc, ID};

#[derive(Default)]
struct NaviOutlineInner {
    /// Selected item that has inner data of a kind like fields/variants/impls.
    selected: Option<Selected>,
    lines: &'static [&'static str],
}

impl std::ops::Deref for NaviOutlineInner {
    type Target = [&'static str];

    fn deref(&self) -> &Self::Target {
        self.lines
    }
}

impl LineState for &'static str {
    type State = &'static str;

    fn state(&self) -> Self::State {
        self
    }

    fn is_identical(&self, state: &Self::State) -> bool {
        self == state
    }
}

#[derive(Default)]
pub struct NaviOutline {
    display: Scroll<NaviOutlineInner>,
    border: Surround,
}

struct Selected {
    id: ID,
    kind: Kind,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Struct,
    Enum,
    Trait,
    Union,
}

impl Kind {
    fn new(id: &str, doc: &CrateDoc) -> Option<Kind> {
        doc.get_item(id).and_then(|item| {
            Some(match &item.inner {
                ItemEnum::Struct(_) => Kind::Struct,
                ItemEnum::Enum(_) => Kind::Enum,
                ItemEnum::Trait(_) => Kind::Trait,
                ItemEnum::Union(_) => Kind::Union,
                ItemEnum::Import(reexport) => {
                    let id = reexport.id.as_ref()?;
                    match &doc.get_item(&id.0)?.inner {
                        ItemEnum::Struct(_) => Kind::Struct,
                        ItemEnum::Enum(_) => Kind::Enum,
                        ItemEnum::Trait(_) => Kind::Trait,
                        ItemEnum::Union(_) => Kind::Union,
                        _ => return None,
                    }
                }
                _ => return None,
            })
        })
    }

    fn lines(self) -> &'static [&'static str] {
        match self {
            Kind::Struct | Kind::Union => STRUCT,
            Kind::Enum => ENUM,
            Kind::Trait => TRAIT,
        }
    }
}

const STRUCT: &'static [&'static str] = &["Fields", "Impls"];
const ENUM: &'static [&'static str] = &["Varaints", "Impls"];
const TRAIT: &'static [&'static str] = &["Implementors"];

pub fn height() -> u16 {
    [STRUCT.len(), ENUM.len(), TRAIT.len()]
        .into_iter()
        .max()
        .unwrap_or(0) as u16
        + 1u16
}

pub fn width() -> u16 {
    [STRUCT, ENUM, TRAIT]
        .map(|val| val.iter().map(|s| s.len()))
        .into_iter()
        .flatten()
        .max()
        .unwrap_or(0) as u16
        + 5u16
}

impl NaviOutline {
    pub fn set_item_inner(&mut self, id: Option<&str>, doc: &CrateDoc) -> Option<ID> {
        let inner = &mut self.display.lines;
        inner.selected = id.and_then(|id| {
            Kind::new(id, doc).map(|kind| Selected {
                id: id.into(),
                kind,
            })
        });

        let ret = if let Some(selected) = &inner.selected {
            *self.border.block_mut() = block();
            inner.lines = selected.kind.lines();
            Some(selected.id.clone())
        } else {
            *self.border.block_mut() = Default::default();
            None
        };
        // border changes, thus inner area should change
        self.display.area = self.border.inner();

        ret
    }

    pub fn update_area(&mut self, area: Rect) {
        if let Some(inner) = self.border.update_area(area) {
            self.display.area = inner;
        }
    }

    fn inner(&mut self) -> &mut NaviOutlineInner {
        &mut self.display.lines
    }

    fn inner_ref(&self) -> &NaviOutlineInner {
        &self.display.lines
    }

    fn kind(&self) -> Option<Kind> {
        self.inner_ref().selected.as_ref().map(|v| v.kind)
    }

    pub fn render(&self, buf: &mut Buffer) {
        self.border.render(buf);

        if let Some(lines) = self.display.visible_lines() {
            let width = self.display.area.width as usize;
            let Rect { x, mut y, .. } = self.display.area;
            for &line in lines {
                let line = ["👉 ", line].map(|s| (s, NEW));
                render_line(line, buf, x, y, width);
                y += 1;
            }
        }
    }

    /// Returns true if user clicks on the item to ask for update of outline.
    pub fn update_outline(&mut self, y: u16) -> bool {
        self.display.get_line_on_screen(y).is_some()
    }
}

fn block() -> Block<'static> {
    Block::new()
        .borders(Borders::TOP)
        .border_type(BorderType::Thick)
}
