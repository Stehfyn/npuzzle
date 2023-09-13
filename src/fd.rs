/**
 * @file fd.rs
 *
 * @brief This is the File Dialog helper module used to facilitate uploading image files to NPuzzle.
 *
 * Various issues used as implementation resources:
 * - https://github.com/emilk/egui/issues/270
 * - https://github.com/emilk/egui/discussions/2010
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */
#[cfg(target_arch = "wasm32")]
use js_sys::{Array, ArrayBuffer, Uint8Array};
#[cfg(target_arch = "wasm32")]
use poll_promise::Promise;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, File, FileReader, HtmlElement, HtmlInputElement, Url};

pub struct FileDialog {
    file: Option<Vec<u8>>,
    #[cfg(target_arch = "wasm32")]
    file_request_future: Option<Promise<Option<Vec<u8>>>>,
    #[allow(unused)]
    #[cfg(not(target_arch = "wasm32"))]
    file_request_future: Option<()>,
}

impl Default for FileDialog {
    fn default() -> Self {
        Self {
            file: None,
            file_request_future: None,
        }
    }
}

impl FileDialog {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(&mut self) {
        let path = rfd::FileDialog::new().pick_file();
        if let Some(path) = path {
            self.file = std::fs::read(path).ok();
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn open(&mut self) {
        self.file_request_future = Some(Promise::spawn_local(Self::open_async()));
    }

    #[cfg(target_arch = "wasm32")]
    async fn open_async() -> Option<Vec<u8>> {
        let res = rfd::AsyncFileDialog::new().pick_file().await;
        match res {
            Some(file) => {
                let data: Vec<u8> = file.read().await;
                Some(data)
            }
            None => None,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get(&mut self) -> Option<Vec<u8>> {
        if let Some(file_async) = &self.file_request_future {
            if let Some(file_result) = file_async.ready() {
                self.file = file_result.clone();
                self.file_request_future = None;
                std::mem::replace(&mut self.file, None)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get(&mut self) -> Option<Vec<u8>> {
        std::mem::replace(&mut self.file, None)
    }

    #[allow(unused)]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self, filename: &str, file: Vec<u8>) {
        let path = rfd::FileDialog::new().set_file_name(filename).save_file();

        if let Some(path) = path {
            std::fs::write(path, file).ok();
        }
    }
}
