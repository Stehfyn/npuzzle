use super::MAX_WRAP;
use crate::about_panel::AboutPanel;
use crate::gallery_panel::GalleryPanel;
use crate::puzzle_panel::PuzzlePanel;
use crate::settings_panel::SettingsPanel;
use egui_extras::RetainedImage;
use log::{debug, error, info};

#[derive(Clone, Copy, Debug)]
#[must_use]
enum Command {
    Nothing,
    ResetImage,
    ResetGrid,
}

#[cfg(target_arch = "wasm32")]
use crate::web_helpers::{isIOS, isMobile};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NPuzzle {
    // Example stuff:
    label: String,
    #[serde(skip)]
    screen_rect: egui::Rect,
    top_panel_font_size: f32,
    top_panel_sep_width: f32,
    gallery_label: String,
    puzzle_label: String,
    about_label: String,
    settings_label: String,
    #[serde(skip)]
    top_panel_rects: Vec<egui::Rect>,
    #[serde(skip)]
    top_panel_height: f32,
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    img: Option<RetainedImage>,
    #[serde(skip)]
    available_height: f32,
    #[serde(skip)]
    gallery_panel: super::gallery_panel::GalleryPanel,
    #[serde(skip)]
    game_panel_open: bool,
    #[serde(skip)]
    puzzle_panel: super::puzzle_panel::PuzzlePanel,
    #[serde(skip)]
    about_panel: super::about_panel::AboutPanel,
    #[serde(skip)]
    settings_panel: super::settings_panel::SettingsPanel,
    #[serde(skip)]
    init_once: bool,
}

impl Default for NPuzzle {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            screen_rect: egui::Rect {
                min: egui::Pos2::default(),
                max: egui::Pos2::default(),
            },
            top_panel_font_size: 20.0,
            top_panel_sep_width: 0.0,
            gallery_label: "📸 Gallery".to_owned(),
            puzzle_label: "🏁 Puzzle".to_owned(),
            about_label: "📖 About".to_owned(),
            settings_label: "⚙".to_owned(),
            top_panel_rects: Vec::default(),
            top_panel_height: 40.,

            value: 2.7,
            img: None,
            available_height: 0.0,
            game_panel_open: true,
            gallery_panel: GalleryPanel::default(),
            puzzle_panel: PuzzlePanel::default(),
            about_panel: AboutPanel::default(),
            settings_panel: SettingsPanel::default(),
            init_once: true,
        }
    }
}

impl NPuzzle {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for NPuzzle {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let Self { label, value } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        ctx.request_repaint();
        self.screen_rect = ctx.screen_rect();

        egui::TopBottomPanel::top("top_panel")
            .exact_height(self.calc_top_panel_button_height())
            .show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    // calc button dimensions:
                    self.calc_top_panel_button_rects(ui);
                    let bw = self.calc_top_panel_button_widths(ui);
                    let bh = self.calc_top_panel_button_height();
                    let offset = (self.screen_rect.width() - bw) / 2.;

                    ui.add_space(offset);
                    let mut button_text = "☀";
                    if ui.visuals().dark_mode {
                        button_text = "🌙";
                    }
                    let mut style = (*ctx.style()).clone();
                    style.text_styles = [(
                        egui::TextStyle::Button,
                        egui::FontId::new(18.0, egui::FontFamily::Proportional),
                    )]
                    .into();
                    ui.style_mut().text_styles = style.text_styles;

                    if ui
                        .add_sized(
                            [self.top_panel_rects[0].width(), bh],
                            egui::Button::new(button_text),
                        )
                        .clicked()
                    {
                        let visuals = if ui.visuals().dark_mode {
                            egui::Visuals::light()
                        } else {
                            egui::Visuals::dark()
                        };
                        ctx.set_visuals(visuals);
                    }
                    ui.add(egui::Separator::default().spacing(self.top_panel_sep_width));
                    if ui
                        .add_sized(
                            [self.top_panel_rects[1].width(), bh],
                            egui::SelectableLabel::new(
                                self.gallery_panel.open,
                                self.gallery_label.clone(),
                            ),
                        )
                        .clicked()
                    {
                        self.gallery_panel.open = !self.gallery_panel.open;
                        #[cfg(target_arch = "wasm32")]
                        if isIOS() || isMobile() {
                            if self.gallery_panel.open {
                                self.puzzle_panel.open = false;
                                self.settings_panel.open = false;
                                self.about_panel.open = false;
                            } else {
                                self.puzzle_panel.open = true;
                                self.settings_panel.open = true;
                            }
                        }
                    }
                    if ui
                        .add_sized(
                            [self.top_panel_rects[2].width(), bh],
                            egui::SelectableLabel::new(
                                self.puzzle_panel.open,
                                self.puzzle_label.clone(),
                            ),
                        )
                        .clicked()
                    {
                        self.puzzle_panel.open = true;
                        self.settings_panel.open = true;
                        self.gallery_panel.open = false;
                        self.about_panel.open = false;
                    }
                    if ui
                        .add_sized(
                            [self.top_panel_rects[3].width(), bh],
                            egui::SelectableLabel::new(
                                self.about_panel.open,
                                self.about_label.clone(),
                            ),
                        )
                        .clicked()
                    {
                        self.about_panel.open = !self.about_panel.open;
                        #[cfg(target_arch = "wasm32")]
                        if isIOS() || isMobile() {
                            self.gallery_panel.open = false;
                            self.puzzle_panel.open = !self.about_panel.open;
                            self.settings_panel.open = !self.about_panel.open;
                        }
                    }

                    ui.add(egui::Separator::default().spacing(self.top_panel_sep_width));

                    ui.scope(|ui| {
                        #[allow(unused_mut)]
                        let mut disable_settings_button = self.puzzle_panel.is_playing();
                        #[cfg(target_arch = "wasm32")]
                        if isIOS() || isMobile() {
                            if self.about_panel.open || self.gallery_panel.open {
                                disable_settings_button = true;
                            }
                        }
                        ui.set_enabled(!disable_settings_button);
                        if ui
                            .add_sized(
                                [self.top_panel_rects[4].width(), bh],
                                egui::SelectableLabel::new(
                                    self.settings_panel.open,
                                    self.settings_label.clone(),
                                ),
                            )
                            .clicked()
                        {
                            self.settings_panel.open = !self.settings_panel.open;
                        }
                    });
                });
            });

        self.gallery_panel.update(ctx, _frame);
        #[allow(unused_assignments)]
        let mut cmd = Command::Nothing;
        cmd = self.gallery_panel(ctx, _frame);
        self.gallery_panel.end_of_frame(ctx);

        match cmd {
            Command::ResetImage => {
                self.puzzle_panel
                    .set_puzzle_image_and_rebuild(self.gallery_panel.get_selected_image_raw());
            }
            _ => {}
        }

        self.about_panel.update(ctx, _frame);
        self.about_panel(ctx, _frame);

        self.settings_panel.update(ctx, _frame);
        cmd = self.settings_panel(ctx, _frame);

        match cmd {
            Command::ResetGrid => {
                self.puzzle_panel.set_mn(self.settings_panel.get_mn());
            }
            _ => {}
        }

        self.puzzle_panel
            .set_debug_paint(self.settings_panel.is_debug_overlay_active());
        self.puzzle_panel.update(ctx, _frame);
        self.puzzle_panel(ctx, _frame);
        self.pass_data();
    }
}

impl NPuzzle {
    fn gallery_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Command {
        // The backend-panel can be toggled on/off.
        // We show a little animation when the user switches it.
        let is_open = self.gallery_panel.open || ctx.memory(|mem| mem.everything_is_visible());

        let mut cmd = Command::Nothing;
        let gw = self.calc_gallery_panel_width(&ctx.style());

        egui::SidePanel::left("gallery_panel")
            .exact_width(gw - ctx.style().spacing.item_spacing.x * 2.)
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(self.gallery_label.clone());
                });

                ui.separator();
                self.gallery_panel_contents(ui, frame, &mut cmd);
            });

        if self.gallery_panel.has_selection_changed() {
            cmd = Command::ResetImage;
        }
        if self.gallery_panel.pickup_init() && self.init_once {
            self.init_once = false;
            cmd = Command::ResetImage;
        }

        cmd
    }

    fn puzzle_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.puzzle_panel.open {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.puzzle_panel_contents(ui, frame);
            });
        }
    }

    fn settings_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) -> Command {
        let mut is_open = self.settings_panel.open || ctx.memory(|mem| mem.everything_is_visible());
        if self.puzzle_panel.is_playing() {
            is_open = false;
        }
        let mut cmd = Command::Nothing;
        egui::TopBottomPanel::bottom("settings_panel")
            .exact_height(self.settings_panel.calc_panel_ui_height())
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                let mut style = (*ctx.style()).clone();
                style.text_styles = [(
                    egui::TextStyle::Button,
                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                )]
                .into();
                ui.vertical_centered(|ui| {
                    ui.heading("⚙ Puzzle Settings");
                });
                self.settings_panel_contents(ui, frame);
            });
        if self.settings_panel.mn_has_changed() {
            cmd = Command::ResetGrid;
        }
        cmd
    }

    fn about_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let is_open = self.about_panel.open || ctx.memory(|mem| mem.everything_is_visible());

        let gw = self.calc_gallery_panel_width(&ctx.style());

        egui::SidePanel::right("about_panel")
            .exact_width(gw)
            .resizable(false)
            .show_animated(ctx, is_open, |ui| {
                let mut style = (*ctx.style()).clone();
                style.text_styles = [(
                    egui::TextStyle::Button,
                    egui::FontId::new(18.0, egui::FontFamily::Proportional),
                )]
                .into();
                ui.vertical_centered(|ui| {
                    ui.heading(self.about_label.clone());
                });

                ui.separator();
                self.about_panel_contents(ui, frame);
            });
    }

    #[allow(unused_variables)]
    fn gallery_panel_contents(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        cmd: &mut Command,
    ) {
        self.gallery_panel.ui(ui, frame);

        if false {
            ui.separator();

            ui.horizontal(|ui| {
                if ui
                    .button("Reset egui")
                    .on_hover_text("Forget scroll, positions, sizes etc")
                    .clicked()
                {
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
            });
        }
    }

    fn puzzle_panel_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.puzzle_panel.ui(ui, frame);
    }

    fn settings_panel_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.settings_panel.ui(ui, frame);
    }

    fn about_panel_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.about_panel.ui(ui, frame);
    }
}

impl NPuzzle {
    fn pass_data(&mut self) {
        self.settings_panel
            .set_puzzle_panel_constrained_width(self.puzzle_panel.get_constrained_width());
        self.settings_panel
            .set_puzzle_subimage_count(self.puzzle_panel.get_puzzle_subimage_count());
        self.settings_panel
            .set_puzzle_retained_image_count(self.puzzle_panel.get_puzzle_retained_image_count());
        self.settings_panel
            .set_puzzle_dynamic_image_count(self.puzzle_panel.get_puzzle_dynamic_image_count());
        self.settings_panel
            .set_gallery_dynamic_image_count(self.gallery_panel.get_dynamic_image_count());
        self.settings_panel
            .set_gallery_retained_image_count(self.gallery_panel.get_retained_image_count());
        self.settings_panel
            .set_selected_image_src(self.gallery_panel.get_selected_image_src());
        self.puzzle_panel
            .set_game_mode(self.settings_panel.get_game_mode());
    }

    fn calc_top_panel_button_rects(&mut self, ui: &egui::Ui) {
        self.top_panel_rects.clear();

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    "☀".to_owned(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.gallery_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.puzzle_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.about_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.settings_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );
    }

    fn calc_top_panel_button_widths(&mut self, ui: &egui::Ui) -> f32 {
        let mut width = 0.;
        let item_spacing_x = ui.style().spacing.item_spacing.x;
        let button_padding_x = ui.style().spacing.button_padding.x;

        for r in self.top_panel_rects.iter() {
            width += r.width();
            width += button_padding_x * 2.;
            width += item_spacing_x;
        }

        width += item_spacing_x;

        width
    }

    fn calc_top_panel_button_height(&mut self) -> f32 {
        // Look at the tallest button
        self.top_panel_rects
            .iter()
            .map(|r| r.height())
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }

    #[allow(unused_variables)]
    fn calc_gallery_panel_width(&mut self, style: &egui::Style) -> f32 {
        #[cfg(target_arch = "wasm32")]
        if isMobile() || isIOS() {
            return self.screen_rect.width();
        } else {
            let one_third = self.screen_rect.width() / 3.;
            let min_panel_width = self.gallery_panel.calc_upload_ui_button_widths(style);
            if one_third > min_panel_width {
                if one_third < 480. {
                    return one_third;
                } else {
                    return 480.;
                }
            } else {
                return min_panel_width;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        480.
    }
}
