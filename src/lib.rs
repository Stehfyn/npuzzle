#![warn(clippy::all, rust_2018_idioms)]
/**
 * @file lib.rs
 *
 * @brief This is the library file collecting all crate-wide modules.
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */
mod app;
pub use app::NPuzzle;
mod about_panel;
mod fd;
mod gallery_panel;
mod image_helpers;
mod npuzzle;
mod puzzle_panel;
mod settings_panel;
mod web_helpers;
const MAX_WRAP: f32 = 1000.0;
