use super::MAX_WRAP;
use crate::fd::FileDialog;
use crate::image_helpers;
use egui_extras::RetainedImage;
use image::DynamicImage;
use log::{debug, error, info};
use poll_promise::Promise;
use rand::Rng;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;

#[allow(unused)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct GalleryPanel {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub open: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    fd: FileDialog,
    #[cfg_attr(feature = "serde", serde(skip))]
    img_srcs: Vec<Option<String>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    img_raw: Vec<Option<image::DynamicImage>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    img: Vec<Option<RetainedImage>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    dropped_files: Vec<egui::DroppedFile>,
    #[cfg_attr(feature = "serde", serde(skip))]
    picked_path: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip))]
    selected_img: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    last_selected_img: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    selection_changed: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    random_promise: Option<poll_promise::Promise<Result<Option<DynamicImage>, String>>>,

    #[cfg_attr(feature = "serde", serde(skip))]
    upload_ui_rects: Vec<egui::Rect>,
    #[cfg_attr(feature = "serde", serde(skip))]
    upload_ui_font_size: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    upload_label: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    random_label: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    gallery_img_width: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    gallery_img_width_min: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    gallery_img_width_max: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    init_slider_bounds: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    init_image: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    pickup_init: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    last_requested_url: String,
}

impl Default for GalleryPanel {
    fn default() -> Self {
        Self {
            // Example stuff:
            open: false,
            fd: FileDialog::default(),
            img_srcs: Vec::default(),
            img_raw: Vec::default(),
            img: Vec::default(),
            picked_path: None,
            dropped_files: Vec::default(),
            selected_img: 0,
            last_selected_img: 0,
            selection_changed: false,
            random_promise: None,
            upload_ui_rects: Vec::default(),
            upload_ui_font_size: 16.0,
            upload_label: "Upload Image".to_owned(),
            random_label: "Random Image".to_owned(),
            gallery_img_width: 146.,
            gallery_img_width_min: 146.,
            gallery_img_width_max: 146.,
            init_slider_bounds: false,
            init_image: true,
            pickup_init: false,
            last_requested_url: "".to_owned(),
        }
    }
}

impl GalleryPanel {
    #[allow(unused)]
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.selection_changed = false;
        self.detect_files_being_dropped(ctx);
        self.add_dropped_file_images(ctx);
        self.fetch_if_initial(ctx);
        self.selection_changed = self.selected_img != self.last_selected_img;
        self.last_selected_img = self.selected_img;
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.upload_ui(ui, frame);

        ui.separator();

        self.image_gallery_ui(ui, frame);
    }

    #[allow(unused_variables)]
    pub fn end_of_frame(&mut self, ctx: &egui::Context) {
        if let Some(promise) = &mut self.random_promise {
            if let Some(result) = promise.ready_mut() {
                match result {
                    Ok(resource) => {
                        if let Some(dimg) = resource {
                            self.img_raw.insert(self.img_raw.len(), Some(dimg.clone()));
                            if let Some(rimg) = image_helpers::dynamic_to_retained(dimg) {
                                self.img.insert(self.img.len(), Some(rimg));
                                self.img_srcs.insert(
                                    self.img_srcs.len(),
                                    Some(self.last_requested_url.clone()),
                                );
                                self.pickup_init = true;
                            }
                            self.random_promise = None;
                        }
                    }
                    Err(error) => {
                        // This should only happen if the fetch API isn't available or something similar.
                        self.random_promise = None;
                    }
                }
            } else {
                //ui.spinner();
            }
        }
    }

    fn detect_files_being_dropped(&mut self, ctx: &egui::Context) {
        // Preview hovering files:
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let mut text = "Dropping files:\n".to_owned();
            for file in &ctx.input(|i| i.raw.hovered_files.clone()) {
                if let Some(path) = &file.path {
                    text += &format!("\n{}", path.display());

                    self.picked_path = Some(path.as_path().to_string_lossy().to_string());
                } else if !file.mime.is_empty() {
                    text += &format!("\n{}", file.mime);
                } else {
                    text += "\n???";
                }
            }

            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("file_drop_target"),
            ));

            let screen_rect = ctx.input(|i| i.screen_rect());
            painter.rect_filled(screen_rect, 0.0, egui::Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::monospace(14.0),
                egui::Color32::WHITE,
            );
        }

        // Collect dropped files:
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            self.dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
        }
    }

    #[allow(unused_variables)]
    fn add_dropped_file_images(&mut self, ctx: &egui::Context) {
        if !self.dropped_files.is_empty() {
            debug!("file was dropped");

            for file in self.dropped_files.clone() {
                let mut info = if let Some(path) = &file.path {
                    path.display().to_string()
                } else if !file.name.is_empty() {
                    file.name.clone()
                } else {
                    "???".to_owned()
                };

                debug!("{}", info.as_str());
                #[cfg(target_arch = "wasm32")]
                if let Some(bytes) = &file.bytes {
                    info += &format!(" ({} bytes)", bytes.len());
                    self.try_add_image_to_gallery(
                        image_helpers::ImageDataWrapper::Bytes(bytes.clone()),
                        "default".to_owned(),
                    );
                } else {
                    debug!("Unable to read file!");
                }

                #[cfg(not(target_arch = "wasm32"))]
                match fs::read(info.clone()) {
                    Ok(bytes) => {
                        info += &format!(" ({} bytes)", bytes.len());
                        self.try_add_image_to_gallery(
                            image_helpers::ImageDataWrapper::VecRef(&bytes),
                            "default".to_owned(),
                        );
                    }

                    Err(err) => {
                        eprintln!("Error reading file: {}", err);
                    }
                }
            }
        }
        self.dropped_files.clear();
    }

    fn fetch_if_initial(&mut self, ctx: &egui::Context) {
        if self.init_image {
            self.init_image = false;
            self.fetch_image(ctx);
        }
    }

    #[allow(unused_variables)]
    fn upload_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            self.calc_upload_ui_rects(ui);
            let bw = (ui.available_width() / 2.) - (ui.ctx().style().spacing.item_spacing.x);
            let bh = self.calc_upload_ui_button_height() * 2.;
            ui.add_space((ui.ctx().style().spacing.item_spacing.x) / 2.);

            let button = egui::Button::new(self.upload_label.clone());

            let mut style = (*ui.ctx().style()).clone();
            style.text_styles = [(
                egui::TextStyle::Button,
                egui::FontId::new(self.upload_ui_font_size, egui::FontFamily::Proportional),
            )]
            .into();
            ui.style_mut().text_styles = style.text_styles;
            #[cfg(target_arch = "wasm32")]
            {
                if ui
                    .add_sized([bw, bh], egui::Button::new(self.upload_label.clone()))
                    .clicked()
                {
                    self.fd.open();
                }
                if let Some(bytes) = self.fd.get() {
                    self.try_add_image_to_gallery(
                        image_helpers::ImageDataWrapper::VecRef(&bytes),
                        "User Upload".to_owned(),
                    );
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let mut fd = FileDialog::default();
                if ui.add_sized([bw, bh], button).clicked() {
                    fd.open();
                    if let Some(bytes) = fd.get() {
                        self.try_add_image_to_gallery(
                            image_helpers::ImageDataWrapper::VecRef(&bytes),
                            "User Upload".to_owned(),
                        );
                    }
                }
            }

            {
                let mut trigger_fetch = false;
                let button2 = egui::Button::new(self.random_label.clone());

                if ui.add_sized([bw, bh], button2).clicked() {
                    trigger_fetch = true;
                }

                if trigger_fetch {
                    self.fetch_image(ui.ctx());
                }
            }
        });
    }

    #[allow(unused_variables)]
    fn image_gallery_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let texture_ids_and_sizes: Vec<_> = self
            .img
            .iter()
            .filter_map(|img_option| img_option.as_ref())
            .map(|img| (img.texture_id(ui.ctx()), img.size_vec2()))
            .collect();
        ui.horizontal(|ui| {
            ui.spacing_mut().slider_width = ui.available_width();
            ui.add(
                egui::Slider::new(
                    &mut self.gallery_img_width,
                    std::ops::RangeInclusive::new(
                        self.gallery_img_width_min,
                        self.gallery_img_width_max,
                    ),
                )
                .show_value(false)
                .trailing_fill(true),
            );
        });

        ui.separator();

        let max = ui.available_width()
            - (ui.ctx().style().spacing.item_spacing.x * 2.)
            - ui.ctx().style().spacing.scroll_bar_width;
        let mut bid = ui.next_auto_id();
        let w_for_3 = (ui.available_width() / 3.)
            - ui.ctx().style().spacing.button_padding.x
            - ui.ctx().style().spacing.item_spacing.x
            - ui.ctx().style().spacing.window_margin.right;
        self.gallery_img_width_min = w_for_3;

        let w_for_1 = ui.available_width()
            - ui.ctx().style().spacing.button_padding.x
            - ui.ctx().style().spacing.item_spacing.x
            - ui.ctx().style().spacing.item_spacing.x
            - ui.ctx().style().spacing.scroll_bar_width
            - ui.ctx().style().spacing.window_margin.right;
        self.gallery_img_width_max = w_for_1;

        if !self.init_slider_bounds {
            self.gallery_img_width = self.gallery_img_width_min;
            self.init_slider_bounds = true;
        }

        egui::ScrollArea::new([false, true])
            .auto_shrink([false, false])
            .max_width(ui.available_width())
            .max_height(ui.available_height() - self.calc_clear_ui_height(ui))
            .drag_to_scroll(true)
            .show(ui, |ui| {
                egui::Grid::new("neato").show(ui, |ui| {
                    let mut row_w_accum = 0.;

                    #[allow(unused_variables)]
                    for (i, img) in self.img.iter_mut().enumerate() {
                        if let Some((texture_id, size)) = texture_ids_and_sizes.get(i) {
                            let _bid = ui.next_auto_id();
                            if i == self.selected_img {
                                bid = _bid;
                            }
                            let w = self.gallery_img_width;
                            row_w_accum += w + ui.ctx().style().spacing.item_spacing.x;
                            if row_w_accum >= max {
                                row_w_accum = w + ui.ctx().style().spacing.item_spacing.x;
                                ui.end_row();
                            }
                            if ui
                                .add(egui::ImageButton::new(*texture_id, [w, w]))
                                .clicked()
                            {
                                bid = _bid;
                                self.selected_img = i;
                            };
                        } else {
                            ui.label("Err");
                        }
                    }
                });
            });

        if self.selected_img < self.img.len() {
            ui.ctx().highlight_widget(bid)
        }

        ui.separator();

        ui.horizontal(|ui| {
            let mut style = (*ui.ctx().style()).clone();
            style.text_styles = [(
                egui::TextStyle::Button,
                egui::FontId::new(self.upload_ui_font_size, egui::FontFamily::Proportional),
            )]
            .into();
            ui.style_mut().text_styles = style.text_styles;

            if ui
                .add_sized(
                    [
                        ui.available_width(),
                        self.calc_upload_ui_button_height() * 2.,
                    ],
                    egui::Button::new("Clear Gallery"),
                )
                .clicked()
            {
                self.img.clear();
                self.img_raw.clear();
                self.selected_img = 0;
                self.last_selected_img = 0;
            };
        });

        ui.separator();

        #[allow(deprecated)]
        ui.centered(|ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                egui::RichText::new("(source code)").size(12.),
                "https://github.com/Stehfyn/cs481/blob/main/src/gallery_panel.rs",
            ));
        });
    }

    fn get_random_picsum_image_url(&self) -> String {
        let seed: f64 = rand::thread_rng().gen_range(0.0..1.3e4); //rand::task_rng().gen_range(0.0, 1.3e4);
        let side = 640;
        let url = format!("https://picsum.photos/seed/{seed}/{side}");
        return url;
    }

    // Launch an async fetch request to picsum for an image.
    fn fetch_image(&mut self, ctx: &egui::Context) {
        let url = self.get_random_picsum_image_url();
        if let Some(_) = self.random_promise {
        } else {
            self.last_requested_url = url.clone();
        }
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(&url);
        let nctx = ctx.clone();
        ehttp::fetch(request, move |response| {
            nctx.request_repaint(); // wake up UI thread
            let resource = response.map(|response| {
                if let Some(bytes) = image_helpers::from_response_to_image_wrapper(&response) {
                    image_helpers::bytes_to_dynamic(bytes)
                } else {
                    None
                }
            });
            sender.send(resource);
        });

        self.random_promise = Some(promise);
    }

    // Try and convert from sequence of bytes -> DynamicImage -> RetainedImage, then add it to our containers.
    fn try_add_image_to_gallery(
        &mut self,
        bytes: image_helpers::ImageDataWrapper<'_>,
        src: String,
    ) {
        if let Some(dimg) = image_helpers::bytes_to_dynamic(bytes) {
            if let Some(rimg) = image_helpers::dynamic_to_retained(&dimg) {
                self.img_srcs.insert(self.img_srcs.len(), Some(src.clone()));
                self.img_raw.insert(self.img_raw.len(), Some(dimg));
                self.img.insert(self.img.len(), Some(rimg));
            }
        }
    }

    // Has the gallery image selection changed?
    pub fn has_selection_changed(&self) -> bool {
        self.selection_changed
    }

    // Do we have an auto-fetched image to pickup?
    pub fn pickup_init(&mut self) -> bool {
        if self.pickup_init {
            self.pickup_init = false;
            return true;
        } else {
            return false;
        }
    }

    // Get the DynamicImage object of currently selected gallery image.
    pub fn get_selected_image_raw(&self) -> Option<image::DynamicImage> {
        if self.img.len() > 0 && (self.img.len() == self.img_raw.len()) {
            self.img_raw[self.selected_img].clone()
        } else {
            None
        }
    }

    pub fn get_selected_image_src(&self) -> Option<String> {
        if self.img_srcs.len() > 0 {
            self.img_srcs[self.selected_img].clone()
        } else {
            None
        }
    }

    pub fn get_dynamic_image_count(&self) -> usize {
        self.img_raw.len()
    }

    pub fn get_retained_image_count(&self) -> usize {
        self.img.len()
    }

    fn calc_clear_ui_height(&mut self, ui: &egui::Ui) -> f32 {
        let clear_gallery_button_height = ui
            .painter()
            .layout(
                "Clear Gallery".to_owned(),
                egui::FontId::new(self.upload_ui_font_size, egui::FontFamily::Proportional),
                egui::Color32::default(),
                MAX_WRAP,
            )
            .rect
            .height();

        let source_code_hyperlink_height = ui
            .painter()
            .layout(
                "(source code)".to_owned(),
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
                egui::Color32::default(),
                MAX_WRAP,
            )
            .rect
            .height();
        2. * (clear_gallery_button_height
            + source_code_hyperlink_height
            + (ui.ctx().style().spacing.item_spacing.y * 3.))
    }

    fn calc_upload_ui_rects(&mut self, ui: &egui::Ui) {
        self.upload_ui_rects.clear();

        self.upload_ui_rects.push(
            ui.painter()
                .layout(
                    self.upload_label.clone(),
                    egui::FontId::new(self.upload_ui_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.upload_ui_rects.push(
            ui.painter()
                .layout(
                    self.random_label.clone(),
                    egui::FontId::new(self.upload_ui_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );
    }

    #[allow(unused)]
    pub fn calc_upload_ui_button_widths(&mut self, style: &egui::Style) -> f32 {
        let mut width = 0.;
        let item_spacing_x = style.spacing.item_spacing.x;
        let button_padding_x = style.spacing.button_padding.x;

        for r in self.upload_ui_rects.iter() {
            width += r.width();
            width += button_padding_x * 2.;
            width += item_spacing_x;
        }

        width
    }

    fn calc_upload_ui_button_height(&mut self) -> f32 {
        // Look at the tallest button
        self.upload_ui_rects
            .iter()
            .map(|r| r.height())
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}
