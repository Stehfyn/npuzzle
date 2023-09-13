#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "console")] // hide console window on Windows in release

/**
 * @file main.rs
 *
 * @brief This is the main application driver for both native and wasm compilation targets.
 *
 * This modules's skeleton if boilerplate from:
 * https://github.com/emilk/eframe_template
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };

    eframe::run_native(
        "npuzzle",
        native_options,
        Box::new(|cc| Box::new(npuzzle::NPuzzle::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(npuzzle::NPuzzle::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
