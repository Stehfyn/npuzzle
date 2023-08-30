#![warn(clippy::all, rust_2018_idioms)]
mod app;
pub use app::TemplateApp;
mod about_panel;
mod fd;
mod gallery_panel;
mod image_helpers;
mod puzzle_panel;
mod settings_panel;
mod web_helpers;
const MAX_WRAP: f32 = 1000.0;
