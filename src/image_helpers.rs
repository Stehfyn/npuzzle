/**
 * @file image_helpers.rs
 *
 * @brief This is the image helpers module which implements unified logic for handling the two types of image
 * data NPuzzle encounters at runtime.
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */
use egui_extras::RetainedImage;
use image::DynamicImage;
use std::io::Cursor;
use std::sync::Arc;

pub enum ImageDataWrapper<'a> {
    #[allow(unused)]
    Bytes(Arc<[u8]>),
    VecRef(&'a Vec<u8>),
}

impl<'a> ImageDataWrapper<'a> {
    fn as_slice(&self) -> &[u8] {
        match self {
            ImageDataWrapper::Bytes(arc) => &arc[..],
            ImageDataWrapper::VecRef(vec) => &vec[..],
        }
    }
}
pub fn bytes_to_dynamic(bytes: ImageDataWrapper<'_>) -> Option<DynamicImage> {
    match image::load_from_memory(&bytes.as_slice()) {
        Ok(image) => Some(image),
        Err(_e) => None,
    }
}
pub fn dynamic_to_retained(dynamic: &DynamicImage) -> Option<RetainedImage> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut writer = Cursor::new(&mut buffer);

    match dynamic.write_to(&mut writer, image::ImageFormat::Jpeg) {
        Ok(_) => Some(egui_extras::RetainedImage::from_image_bytes("img", &buffer).unwrap()),
        Err(_e) => None,
    }
}
pub fn from_response_to_image_wrapper<'a>(
    response: &'a ehttp::Response,
) -> Option<ImageDataWrapper<'a>> {
    let content_type = response.content_type().unwrap_or_default();

    if content_type.starts_with("image/") {
        Some(ImageDataWrapper::VecRef(&response.bytes))
    } else {
        None
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct SubImage {
    uid: String,
    ptid: egui::TextureId,
    draw_region: egui::Rect,
    uv_quad: egui::Rect,
    index: usize,
}

impl Default for SubImage {
    fn default() -> Self {
        Self {
            uid: "".to_owned(),
            ptid: egui::TextureId::default(),
            index: 0,
            draw_region: egui::Rect::from([egui::Pos2::default(), egui::Pos2::default()]),
            uv_quad: egui::Rect::from([egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)]),
        }
    }
}

impl SubImage {
    pub fn new(
        id: String,
        tid: egui::TextureId,
        ind: usize,
        region: &egui::Rect,
        uv: &egui::Rect,
    ) -> Self {
        Self {
            uid: id,
            ptid: tid,
            index: ind,
            draw_region: (*region).clone(),
            uv_quad: (*uv).clone(),
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_center(&self) -> egui::Pos2 {
        self.draw_region.center()
    }

    pub fn contains(&self, pos: egui::Pos2) -> bool {
        self.draw_region.contains(pos)
    }

    pub fn region(&mut self, region: &egui::Rect) {
        self.draw_region = (*region).clone();
    }

    pub fn padding(&mut self, x: f32, y: f32) {
        let pad_x = x / 2.;
        let pad_y = y / 2.;

        self.draw_region.min.x += pad_x;
        self.draw_region.min.y += pad_y;

        self.draw_region.max.x -= pad_x;
        self.draw_region.max.y -= pad_y;
    }

    pub fn drag(&mut self, v: egui::Vec2) {
        self.draw_region.min += v;
        self.draw_region.max += v;
    }

    pub fn paint(&mut self, ui: &mut egui::Ui, order: &mut egui::Order, is_dragging: bool) {
        if is_dragging {
            *order = egui::Order::Foreground;
        }

        let painter = ui.ctx().layer_painter(egui::LayerId::new(
            order.clone(),
            egui::Id::new(&self.uid[..]),
        ));

        painter.image(
            self.ptid,
            self.draw_region,
            self.uv_quad,
            egui::Color32::WHITE,
        );

        if is_dragging {
            painter.rect_stroke(
                self.draw_region,
                10.0,
                egui::Stroke::new(5.0, egui::Color32::RED),
            );
        }
    }

    pub fn debug_paint(&mut self, ui: &mut egui::Ui, order: &mut egui::Order, is_dragging: bool) {
        self.paint(ui, order, is_dragging);

        let painter = ui.ctx().layer_painter(egui::LayerId::new(
            egui::Order::Debug,
            egui::Id::new(format!("debug_{}", &self.uid[..])),
        ));

        painter.text(
            self.draw_region.min,
            egui::Align2::LEFT_TOP,
            format!("{}", self.index),
            egui::FontId::proportional(36.),
            egui::Color32::RED,
        );
    }
}
