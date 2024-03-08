use super::navi::NaviAction;
use crate::ui::scrollable::ScrollTreeLines;
use ratatui::prelude::{Buffer, Rect};
use term_rustdoc::tree::{CrateDoc, TreeLines, ID};

#[derive(Default)]
pub struct OutlineInner {
    kind: OutlineKind,
    modules: ScrollTreeLines,
    setu: Setu,
}

impl std::fmt::Debug for OutlineInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutlineInner")
            .field("kind", &self.kind)
            .finish()
    }
}

impl OutlineInner {
    pub fn new(doc: &CrateDoc) -> Self {
        let modules = match ScrollTreeLines::new_tree_lines(doc.clone().into()) {
            Ok(lines) => lines,
            Err(err) => {
                error!("Failed to init module Outline:\n{err}");
                return OutlineInner::default();
            }
        };
        OutlineInner {
            modules,
            ..Default::default()
        }
    }

    pub fn is_module_tree(&self) -> bool {
        matches!(self.kind, OutlineKind::Modules)
    }

    pub fn display(&mut self) -> &mut ScrollTreeLines {
        match self.kind {
            OutlineKind::Modules => &mut self.modules,
            OutlineKind::InnerItem => &mut self.setu.display,
        }
    }

    pub fn display_ref(&self) -> &ScrollTreeLines {
        match self.kind {
            OutlineKind::Modules => &self.modules,
            OutlineKind::InnerItem => &self.setu.display,
        }
    }

    pub fn update_area(&mut self, area: Rect) {
        self.modules.area = area;
        self.setu.update_area(area);
    }

    pub fn render(&self, buf: &mut Buffer) {
        match self.kind {
            OutlineKind::Modules => self.modules.render(buf),
            OutlineKind::InnerItem => self.setu.render(buf),
        };
    }
}

/// Action from Navi
impl OutlineInner {
    pub fn set_setu_id(&mut self, id: ID) {
        self.setu.outer_item = id;
    }

    pub fn action(&mut self, action: NaviAction) {
        match action {
            NaviAction::BackToHome => self.back_to_home(),
            x => {
                self.setu.update_lines(&self.modules, x);
                self.kind = OutlineKind::InnerItem;
            }
        };
    }

    fn back_to_home(&mut self) {
        self.kind = OutlineKind::Modules;
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum OutlineKind {
    #[default]
    Modules,
    InnerItem,
}

/// Stands for struct/enum/trait/union.
#[derive(Default)]
pub struct Setu {
    outer_item: ID,
    display: ScrollTreeLines,
}

impl Setu {
    pub fn update_area(&mut self, area: Rect) {
        self.display.area = area;
    }

    pub fn update_lines(&mut self, modules: &ScrollTreeLines, action: NaviAction) {
        let doc = modules.lines.doc();
        self.display.lines = TreeLines::new_with(doc, |map| {
            let id = &self.outer_item;
            let dmod = map.dmodule();
            match action {
                NaviAction::ITABImpls => dmod.impl_tree(id, map),
                NaviAction::Item => dmod.item_inner_tree(id, map),
                _ => dmod.item_inner_tree(id, map),
            }
            .unwrap_or_default()
        })
        .0;
        if self.display.total_len() == 0 {
            let path = modules.lines.doc_ref().path(&self.outer_item);
            error!("{path} generated unexpected empty TreeLines");
        }
        self.display.update_maxwidth();
    }

    pub fn render(&self, buf: &mut Buffer) {
        if self.display.lines.is_empty() {
            return;
        }
        self.display.render(buf);
    }
}
