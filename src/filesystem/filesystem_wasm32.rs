// Copyright (C) 2022 Lily Lyons
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
#![allow(missing_docs)]

use std::cell::RefCell;
use std::io::Cursor;
use std::path::PathBuf;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::{data::data_cache::DataCache, UpdateInfo};

// Javascript interface for filesystem
#[wasm_bindgen(module = "/assets/filesystem.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn js_open_project() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn js_read_file(path: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn js_read_bytes(path: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn js_dir_children(path: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn js_save_data(path: JsValue, data: JsValue) -> Result<JsValue, JsValue>;

    fn js_filesystem_supported() -> bool;
}

pub struct Filesystem {
    project_path: RefCell<Option<PathBuf>>,
}

impl Default for Filesystem {
    fn default() -> Self {
        if !js_filesystem_supported() {
            rfd::MessageDialog::new()
                .set_description("Filesystem not supported on this browser")
                .show();
            panic!("Filesystem not supported on this browser");
        }
        Self {
            project_path: RefCell::new(None),
        }
    }
}

impl Filesystem {
    pub fn unload_project(&self) {
        *self.project_path.borrow_mut() = None;
    }

    pub fn project_loaded(&self) -> bool {
        self.project_path.borrow().is_some()
    }

    pub fn project_path(&self) -> Option<PathBuf> {
        self.project_path.borrow().clone()
    }

    pub async fn load_project(
        &self,
        path: JsValue,
        cache: &'static DataCache,
    ) -> Result<(), String> {
        *self.project_path.borrow_mut() = Some(PathBuf::from(path.as_string().unwrap()));
        cache.load(self).await.map_err(|e| {
            *self.project_path.borrow_mut() = None;
            e
        })
    }

    pub async fn dir_children(&self, path: &str) -> Result<Vec<String>, String> {
        js_dir_children(JsValue::from_str(path))
            .await
            .map(|ref children| {
                js_sys::Array::from(children)
                    .iter()
                    .map(|child| child.as_string().unwrap())
                    .collect()
            })
            .map_err(|s| format!("JS Error {:#?}", s))
    }

    pub async fn bufreader(&self, path: &str) -> Result<Cursor<Vec<u8>>, String> {
        Ok(Cursor::new(self.read_bytes(path).await?))
    }

    pub async fn read_data<T>(&self, path: &str) -> Result<T, String>
    where
        T: serde::de::DeserializeOwned,
    {
        let str = js_read_file(JsValue::from_str(&format!("Data_RON/{}", path)))
            .await
            .map(|s| s.as_string().unwrap())
            .map_err(|s| format!("JS Error {:#?}", s))?;

        ron::from_str(&str).map_err(|e| e.to_string())
    }

    pub async fn read_bytes(&self, path: &str) -> Result<Vec<u8>, String> {
        js_read_bytes(JsValue::from_str(path))
            .await
            .map(|bytes| js_sys::Uint8Array::try_from(bytes).unwrap().to_vec())
            .map_err(|s| format!("JS Error {:#?}", s))
    }

    pub async fn save_data(&self, path: &str, data: &str) -> Result<(), String> {
        js_save_data(
            JsValue::from_str(&format!("Data_RON/{}", path)),
            JsValue::from_str(data),
        )
        .await
        .map(|_| ())
        .map_err(|s| format!("JS Error {:#?}", s))
    }

    pub async fn save_cached(&self, data_cache: &'static DataCache) -> Result<(), String> {
        data_cache.save(self).await
    }

    pub async fn try_open_project(&self, info: &'static UpdateInfo) -> Result<(), String> {
        let handle = js_open_project()
            .await
            .map_err(|_| "Cancelled loading project".to_string())?;

        self.load_project(handle, &info.data_cache).await
    }
}
