use egui::Image;

use super::MAX_WRAP;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    Dfs,
    Bfs,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameMode {
    TimeAttack,
    Outsmart,
    Race,
}

#[cfg(target_arch = "wasm32")]
use crate::web_helpers::{isIOS, isMobile};

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::TimeAttack => write!(f, "Time Attack"),
            GameMode::Outsmart => write!(f, "Outsmart"),
            GameMode::Race => write!(f, "Race"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct SettingsPanel {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub open: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    m: i32,
    #[cfg_attr(feature = "serde", serde(skip))]
    n: i32,
    #[cfg_attr(feature = "serde", serde(skip))]
    run_mode: RunMode,
    #[cfg_attr(feature = "serde", serde(skip))]
    mn_has_changed: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    last_mn: i32,
    #[cfg_attr(feature = "serde", serde(skip))]
    mn_slider_float: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    debug_menu_open: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_panel_constrained_width: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    game_mode: GameMode,
    #[cfg_attr(feature = "serde", serde(skip))]
    button_ui_font_size: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    button_ui_rects: Vec<egui::Rect>,
    #[cfg_attr(feature = "serde", serde(skip))]
    agent_settings_label: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    debug_menu_label: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    agent_settings_menu_open: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    debug_overlay_active: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_subimage_count: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    gallery_retained_image_count: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_retained_image_count: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    gallery_dynamic_image_count: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_dynamic_image_count: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    selected_image_src: Option<String>,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self {
            open: true,
            m: 3,
            n: 3,
            run_mode: RunMode::Dfs,
            mn_has_changed: false,
            last_mn: 3,
            mn_slider_float: 3.,
            debug_menu_open: false,
            puzzle_panel_constrained_width: 0.,
            game_mode: GameMode::TimeAttack,
            button_ui_font_size: 16.0,
            button_ui_rects: Vec::default(),
            agent_settings_label: "Agent Settings".to_owned(),
            debug_menu_label: "Debug Menu".to_owned(),
            debug_overlay_active: false,
            agent_settings_menu_open: false,
            puzzle_retained_image_count: 0,
            puzzle_dynamic_image_count: 0,
            puzzle_subimage_count: 0,
            gallery_dynamic_image_count: 0,
            gallery_retained_image_count: 0,
            selected_image_src: None,
        }
    }
}

impl SettingsPanel {
    #[allow(unused)]
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.mn_has_changed = false;
        if self.last_mn != self.m {
            self.mn_has_changed = true;
            self.last_mn = self.m;
        }
    }

    #[allow(unused)]
    pub fn end_of_frame(&mut self, ctx: &egui::Context) {}

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.puzzle_settings_ui(ui, frame)
    }

    #[allow(unused)]
    fn puzzle_settings_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            let slider_width = (self.puzzle_panel_constrained_width / 2.)
                - ui.ctx().style().spacing.item_spacing.x;
            ui.spacing_mut().slider_width = slider_width;
            ui.add_space(
                (ui.available_width() / 2.) - (self.puzzle_panel_constrained_width / 2.)
                    + ui.ctx().style().spacing.window_margin.left,
            );
            ui.add(
                egui::Slider::new(
                    &mut self.mn_slider_float,
                    std::ops::RangeInclusive::new(2.0, 6.0),
                )
                .show_value(false)
                .trailing_fill(true), //.text("== N"),
            );
            self.m = self.mn_slider_float.round() as i32;
            self.n = self.m;

            ui.spacing_mut().combo_width = slider_width;

            egui::ComboBox::from_label("")
                .selected_text(
                    egui::RichText::new(format!("  {}", self.game_mode.to_string())).size(18.0),
                )
                .show_ui(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.selectable_value(
                            &mut self.game_mode,
                            GameMode::TimeAttack,
                            egui::RichText::new("Time Attack").size(18.0),
                        );
                    });
                    ui.vertical_centered(|ui| {
                        ui.selectable_value(
                            &mut self.game_mode,
                            GameMode::Outsmart,
                            egui::RichText::new("Outsmart").size(18.0),
                        );
                    });
                    ui.vertical_centered(|ui| {
                        ui.selectable_value(
                            &mut self.game_mode,
                            GameMode::Race,
                            egui::RichText::new("Race").size(18.0),
                        );
                    });
                });
        });

        ui.separator();
        let mut start_pos = ui.cursor().left_top();
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            self.calc_button_ui_rects(ui);
            let (avail_w, offset_w) = if ui.available_width() > self.puzzle_panel_constrained_width
            {
                start_pos.y = ui.cursor().top();
                //start_pos.x = start;
                (
                    self.puzzle_panel_constrained_width,
                    (ui.available_width() / 2.) - (self.puzzle_panel_constrained_width / 2.)
                        + (ui.ctx().style().spacing.window_margin.left / 2.),
                )
            } else {
                (ui.available_width(), 0.)
            };
            let bw = (avail_w / 2.) - ui.ctx().style().spacing.item_spacing.x;
            let bh = self.calc_button_ui_height() * 2.;

            let start_y = (ui.ctx().screen_rect().height() - ui.cursor().left_top().y) * 2.5;
            #[cfg(target_arch = "wasm32")]
            {
                if (isIOS() || isMobile()) {
                    start_pos.x = ui.ctx().style().spacing.item_spacing.x;
                } else {
                    start_pos.x += offset_w
                        + ui.ctx().style().spacing.item_spacing.x
                        + ui.ctx().style().spacing.item_spacing.x * 0.5;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                start_pos.x += offset_w
                    + ui.ctx().style().spacing.item_spacing.x
                    + ui.ctx().style().spacing.item_spacing.x * 0.5;
            }

            start_pos.y -= start_y;

            ui.add_space(offset_w);
            if ui
                .add_sized(
                    [bw, bh],
                    egui::Button::new(
                        egui::RichText::new("Agent Settings").size(self.button_ui_font_size),
                    ),
                )
                .clicked()
            {
                self.agent_settings_menu_open = !self.agent_settings_menu_open;
                if self.agent_settings_menu_open {
                    self.debug_menu_open = false;
                }
            };
            if ui
                .add_sized(
                    [bw, bh],
                    egui::Button::new(
                        egui::RichText::new(&self.debug_menu_label).size(self.button_ui_font_size),
                    ),
                )
                .clicked()
            {
                self.debug_menu_open = !self.debug_menu_open;
                if self.debug_menu_open {
                    self.agent_settings_menu_open = false;
                }
            };

            //calc menu dimensions
            // exceeds 600
            let max_w = 600.;
            let menu_w = if ui.ctx().screen_rect().width() < max_w {
                ui.ctx().screen_rect().width()
                    - (ui.spacing().window_margin.left / 2.)
                    - ui.spacing().window_margin.left
                    - ui.spacing().window_margin.left
                    - ui.spacing().window_margin.right
                    - ui.spacing().window_margin.right
            } else {
                max_w
            };
            let menu_h = ui.ctx().screen_rect().height()
                - start_pos.y
                - (ui.spacing().window_margin.bottom * 2.)
                - (self.calc_button_ui_height() * 6.)
                - self.calc_button_ui_height() * 0.25;

            if self.debug_menu_open {
                egui::Window::new("Debug Menu")
                    .current_pos(start_pos)
                    .open(&mut self.debug_menu_open)
                    .show(ui.ctx(), |ui| {
                        ui.set_width(menu_w);
                        ui.set_height(menu_h);
                        ui.label(egui::RichText::new(format!(
                            "Retained Images (Puzzle Panel): {}",
                            self.puzzle_retained_image_count
                        )));
                        ui.label(egui::RichText::new(format!(
                            "Dynamic Images (Puzzle Panel): {}",
                            self.puzzle_dynamic_image_count
                        )));
                        ui.label(egui::RichText::new(format!(
                            "Subimages (Puzzle Panel): {}",
                            self.puzzle_subimage_count
                        )));
                        ui.label(egui::RichText::new(format!(
                            "Retained Images (Gallery Panel): {}",
                            self.gallery_retained_image_count
                        )));
                        ui.label(egui::RichText::new(format!(
                            "Dynamic Images (Gallery Panel): {}",
                            self.gallery_dynamic_image_count
                        )));
                        if let Some(src) = &self.selected_image_src {
                            #[allow(deprecated)]
                            ui.centered(|ui| {
                                ui.add(egui::Hyperlink::new(src));
                            });
                        }
                        #[allow(deprecated)]
                        ui.centered(|ui| {
                            if ui
                                .button(egui::RichText::new("Toggle Debug Overlay").size(16.0))
                                .clicked()
                            {
                                self.debug_overlay_active = !self.debug_overlay_active;
                            }
                        });
                    });
            }

            if self.agent_settings_menu_open {
                egui::Window::new("Agent Settings")
                    .current_pos(start_pos)
                    .open(&mut self.agent_settings_menu_open)
                    .show(ui.ctx(), |ui| {
                        ui.set_width(menu_w);
                        ui.set_height(menu_h);
                        if ui.button("Search").clicked() {}
                        ui.radio_value(&mut self.run_mode, RunMode::Dfs, "DFS");
                        ui.radio_value(&mut self.run_mode, RunMode::Bfs, "BFS");
                    });
            }
        });
        ui.separator();

        #[allow(deprecated)]
        ui.centered(|ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                egui::RichText::new("(source code)").size(12.),
                "https://github.com/Stehfyn/cs481/blob/main/src/settings_panel.rs",
            ));
        });
    }
}

impl SettingsPanel {
    pub fn mn_has_changed(&mut self) -> bool {
        self.mn_has_changed
    }

    pub fn get_mn(&mut self) -> i32 {
        self.m //could just as well be self.n
    }

    pub fn set_gallery_retained_image_count(&mut self, count: usize) {
        self.gallery_retained_image_count = count;
    }

    pub fn set_gallery_dynamic_image_count(&mut self, count: usize) {
        self.gallery_dynamic_image_count = count;
    }

    pub fn set_puzzle_dynamic_image_count(&mut self, count: usize) {
        self.puzzle_dynamic_image_count = count;
    }

    pub fn set_puzzle_retained_image_count(&mut self, count: usize) {
        self.puzzle_retained_image_count = count;
    }

    pub fn set_puzzle_subimage_count(&mut self, count: usize) {
        self.puzzle_subimage_count = count;
    }

    pub fn set_puzzle_panel_constrained_width(&mut self, width: f32) {
        self.puzzle_panel_constrained_width = width;
    }

    pub fn set_selected_image_src(&mut self, src: Option<String>) {
        self.selected_image_src = src;
    }

    pub fn is_debug_overlay_active(&self) -> bool {
        self.debug_overlay_active
    }

    fn calc_button_ui_rects(&mut self, ui: &egui::Ui) {
        self.button_ui_rects.clear();

        self.button_ui_rects.push(
            ui.painter()
                .layout(
                    self.agent_settings_label.clone(),
                    egui::FontId::new(self.button_ui_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.button_ui_rects.push(
            ui.painter()
                .layout(
                    self.debug_menu_label.clone(),
                    egui::FontId::new(self.button_ui_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );
    }

    #[allow(unused)]
    pub fn calc_button_ui_widths(&mut self, style: &egui::Style) -> f32 {
        let mut width = 0.;
        let item_spacing_x = style.spacing.item_spacing.x;
        let button_padding_x = style.spacing.button_padding.x;

        for r in self.button_ui_rects.iter() {
            width += r.width();
            width += button_padding_x * 2.;
            width += item_spacing_x;
        }

        width
    }

    fn calc_button_ui_height(&mut self) -> f32 {
        // Look at the tallest button
        self.button_ui_rects
            .iter()
            .map(|r| r.height())
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }

    #[allow(unused)]
    pub fn calc_panel_ui_height(&mut self) -> f32 {
        self.calc_button_ui_height() * 7.35
    }
}
