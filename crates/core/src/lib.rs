// Copyright (C) 2023 Lily Lyons
//
// This file is part of Luminol.
//
// Luminol is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luminol is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luminol.  If not, see <http://www.gnu.org/licenses/>.
//
//     Additional permission under GNU GPL version 3 section 7
//
// If you modify this Program, or any covered work, by linking or combining
// it with Steamworks API by Valve Corporation, containing parts covered by
// terms of the Steamworks API by Valve Corporation, the licensors of this
// Program grant you additional permission to convey the resulting work.

use std::sync::Arc;

mod tab;
pub use tab::{EditTabs, Tab, Tabs};

mod window;
pub use window::{EditWindows, Window, Windows};

pub mod modal;
pub use modal::Modal;

mod data_cache;
pub use data_cache::Data;

/// Toasts to be displayed for errors, information, etc.
mod toasts;
pub use toasts::Toasts;

pub struct UpdateState<'res> {
    #[cfg(not(target_arch = "wasm32"))]
    pub audio: &'res mut luminol_audio::Audio,
    #[cfg(target_arch = "wasm32")]
    pub audio: &'res mut luminol_audio::AudioWrapper,

    pub graphics: Arc<luminol_graphics::GraphicsState>,
    pub filesystem: &'res mut luminol_filesystem::project::FileSystem, // FIXME: this is probably wrong
    pub data: &'res mut Data, // FIXME: this is also probably wrong

    // TODO: look into std::any?
    // we're using generics here to allow specialization on the type of window
    // this is fucntionality not really used atm but maybe in the future..?
    pub edit_windows: &'res mut EditWindows,
    pub edit_tabs: &'res mut EditTabs,
    pub toasts: &'res mut Toasts,

    pub project_config: &'res mut Option<luminol_config::project::Config>,
    pub global_config: &'res mut luminol_config::global::Config,

    pub toolbar: &'res mut ToolbarState,
}

#[allow(missing_docs)]
#[derive(Default)]
pub struct ToolbarState {
    /// The currently selected pencil.
    pub pencil: Pencil,
}

#[derive(Default, strum::EnumIter, strum::Display, PartialEq, Eq, Clone, Copy)]
#[allow(missing_docs)]
pub enum Pencil {
    #[default]
    Pen,
    Circle,
    Rectangle,
    Fill,
}

impl<'res> UpdateState<'res> {
    pub(crate) fn reborrow_with_edit_window<'this>(
        &'this mut self,
        edit_windows: &'this mut window::EditWindows,
    ) -> UpdateState<'this> {
        UpdateState {
            audio: self.audio,
            graphics: self.graphics.clone(),
            filesystem: self.filesystem,
            data: self.data,
            edit_tabs: self.edit_tabs,
            edit_windows,
            toasts: self.toasts,
            project_config: self.project_config,
            global_config: self.global_config,
            toolbar: self.toolbar,
        }
    }

    pub(crate) fn reborrow_with_edit_tabs<'this>(
        &'this mut self,
        edit_tabs: &'this mut tab::EditTabs,
    ) -> UpdateState<'this> {
        UpdateState {
            audio: self.audio,
            graphics: self.graphics.clone(),
            filesystem: self.filesystem,
            data: self.data,
            edit_tabs,
            edit_windows: self.edit_windows,
            toasts: self.toasts,
            project_config: self.project_config,
            global_config: self.global_config,
            toolbar: self.toolbar,
        }
    }
}