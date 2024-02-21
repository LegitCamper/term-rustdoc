mod cache;

use self::cache::{Cache, CacheID};
use crate::{
    dashboard::database::DataBase,
    local_registry::PkgInfo,
    ui::{render_line, Scrollable, Surround},
};
use ratatui::prelude::{Buffer, Color, Rect};
use std::path::PathBuf;
use term_rustdoc::util::xformat;

#[derive(Default)]
pub struct PkgDocs {
    db: DataBase,
    caches: Vec<Cache>,
    /// NOTE: the indices only change when the length of caches changes,
    /// because we need to sort caches for displaying, thus both lengths should equal.
    indices: Vec<CacheID>,
}

impl std::ops::Deref for PkgDocs {
    type Target = [CacheID];

    fn deref(&self) -> &Self::Target {
        &self.indices
    }
}

#[derive(Default)]
pub struct DataBaseUI {
    inner: Scrollable<PkgDocs>,
    border: Surround,
}

impl DataBaseUI {
    pub fn init() -> Self {
        let mut ui = DataBaseUI::default();
        if let Ok(db) = DataBase::init() {
            let caches: Vec<_> = db
                .all_caches()
                .map_err(|err| error!("Failed to read CachedDocInfo:\\n{err}"))
                .map(|v| v.into_iter().map(Cache::new_unloaded).collect())
                .unwrap_or_default();
            ui.pkg_docs().indices = (0..caches.len()).map(CacheID).collect();
            ui.pkg_docs().caches = caches;
            ui.pkg_docs().db = db;
        }
        ui
    }

    fn pkg_docs(&mut self) -> &mut PkgDocs {
        &mut self.inner.lines
    }

    pub fn compile_doc(&mut self, pkg_dir: PathBuf, pkg_info: PkgInfo) {
        if let Some(pkg_key) = self.pkg_docs().db.compile_doc(pkg_dir, pkg_info) {
            let caches = &mut self.pkg_docs().caches;
            let id = CacheID(caches.len());
            caches.push(Cache::new_being_cached(pkg_key));
            self.pkg_docs().indices.push(id);
            self.sort_caches();
        }
    }

    /// Sort the Cache vec because the inner states have changed.
    fn sort_caches(&mut self) {
        self.pkg_docs().caches.sort_unstable();
    }
}

/// Rendering
impl DataBaseUI {
    pub fn set_area(&mut self, surround: Surround) {
        self.inner.area = surround.inner();
        self.border = surround;
    }

    pub fn render(&self, buf: &mut Buffer, current: bool) {
        self.border.render(buf);

        let Some(ids) = self.inner.visible_lines() else {
            return;
        };
        let mut start = self.inner.start + 1;
        let Rect { x, mut y, .. } = self.inner.area;
        let width = self.inner.area.width as usize;

        // render current selected pkg
        let text = &self.inner;
        if current && text.get_line_of_current_cursor().is_some() {
            let row = text.area.y + text.cursor.y;
            for col in x..text.area.width + x {
                buf.get_mut(col, row).set_bg(Color::from_u32(0x005DA063)); // #5DA063
            }
        }

        let pkgs = &text.lines.caches;
        for id in ids {
            let num = xformat!("{start:02}. ");
            let [(kind, style_kind), (name, style_name), (ver, style_ver)] = pkgs[id.0].line();
            let line = [
                (kind, style_kind),
                (" ", style_kind),
                (&*num, style_name),
                (name, style_name),
                (" v", style_ver),
                (ver, style_ver),
            ];
            render_line(line, buf, x, y, width);
            start += 1;
            y += 1;
        }
    }

    pub fn load_doc(&mut self) {
        if let Some(id) = self.inner.get_line_of_current_cursor().map(|id| id.0) {
            if self.inner.lines.caches[id].loadable() {
                let unloaded = self.inner.lines.caches.remove(id);
                let loaded = unloaded.load_doc();
                self.inner.lines.caches.push(loaded);
                self.sort_caches();
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.all_lines().is_empty()
    }

    pub fn scroll_text(&mut self) -> &mut Scrollable<PkgDocs> {
        &mut self.inner
    }
}