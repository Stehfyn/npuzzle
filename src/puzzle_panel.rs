use crate::image_helpers;
use crate::npuzzle::*;
#[cfg(target_arch = "wasm32")]
use crate::web_helpers::{isIOS, isMobile};
use log::{debug, error, info};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct PuzzlePanel {
    pub open: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    m: i32,
    #[cfg_attr(feature = "serde", serde(skip))]
    n: i32,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_image: Option<image::DynamicImage>,
    #[cfg_attr(feature = "serde", serde(skip))]
    drag_delta: Option<egui::Vec2>,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_image_r: Option<egui_extras::RetainedImage>,
    #[cfg_attr(feature = "serde", serde(skip))]
    puzzle_subimages: Vec<image_helpers::SubImage>,
    #[cfg_attr(feature = "serde", serde(skip))]
    delay_repaint: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    debug_paint: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    constrained_width: f32,
    #[cfg_attr(feature = "serde", serde(skip))]
    board: NBoard,
    #[cfg_attr(feature = "serde", serde(skip))]
    regen: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    missing_index: usize,
    #[cfg_attr(feature = "serde", serde(skip))]
    in_play: bool,
}

impl Default for PuzzlePanel {
    fn default() -> Self {
        Self {
            open: true,
            m: 3,
            n: 3,
            puzzle_image: None,
            drag_delta: Some(egui::Vec2::ZERO),
            puzzle_image_r: None,
            puzzle_subimages: Vec::default(),
            delay_repaint: false,
            debug_paint: true,
            constrained_width: 0.,
            board: NBoard::new(3),
            regen: false,
            missing_index: (3 * 3) + 1,
            in_play: false,
        }
    }
}

impl PuzzlePanel {
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.regen {
            self.generate_puzzle_board();
            self.in_play = true;
            self.regen = false;
        }
        if self.in_play {
            if self.board.check_win() {
                debug!("win!!!");
            }
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.game_canvas_ui(ui, frame);
    }

    fn game_canvas_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // calc available width, item_spacing.y is used as we will use it for both
        // x and y to create a uniform grid, and by default y spacing is smaller ...
        let item_spacing_x = ui.style().spacing.item_spacing.x;
        let item_spacing_y = ui.style().spacing.item_spacing.y;
        let new_item_spacing_xy = ui.style().spacing.item_spacing.y / 2.;

        ui.style_mut().spacing.item_spacing.x = new_item_spacing_xy;
        ui.style_mut().spacing.item_spacing.y = new_item_spacing_xy;

        let button_padding = ui.style().spacing.button_padding;
        ui.style_mut().spacing.button_padding =
            egui::Vec2::new(button_padding.x / 2., button_padding.y / 2.);

        let avail_w = ui.available_width() - ui.ctx().style().spacing.window_margin.left;

        let mut button_side = (avail_w / (self.m as f32)) - ui.ctx().style().spacing.item_spacing.y;

        // we first assume we don't need to offset the starting x coord of our puzzle_panel
        let mut w_offset = 0.;

        // check projected height which is just avail_w pointing downward
        // settings_panel is guaranteed 25% of screen rect height

        let mut avail_h = ui.ctx().screen_rect().height()
            - ui.next_widget_position().y
            - (ui.ctx().screen_rect().height() * 0.25)
            - ui.ctx().style().spacing.window_margin.bottom;
        avail_h = avail_h
            - ui.ctx().style().spacing.window_margin.top
            - ui.ctx().style().spacing.window_margin.bottom
            - 12.0 * 2.;

        self.constrained_width = avail_w;
        // we need to fit our puzzle_panel with the settings_panel, thus we must fix dimensions
        // and set the proper x coord offset if we don't fit by avail_w
        if avail_w > avail_h {
            button_side = (avail_h / (self.m as f32)) - ui.ctx().style().spacing.item_spacing.y;
            w_offset = (avail_w
                - ((self.m as f32) * (button_side + ui.ctx().style().spacing.item_spacing.y)))
                / 2.;

            w_offset += (ui.ctx().style().spacing.item_spacing.y * 3.)
                - (((6. - self.m as f32) / 4.) * ui.ctx().style().spacing.item_spacing.y);

            self.constrained_width = avail_h;
        }

        egui::Grid::new("game_canvas").show(ui, |ui| {
            ui.style_mut().spacing.item_spacing.x = ui.style().spacing.item_spacing.y;

            let mut subimage_index = 0;
            let rebuild_subimages = self.puzzle_subimages.is_empty();

            for i in 0..self.m {
                ui.add_space(w_offset);
                for j in 0..self.n {
                    #[cfg(target_arch = "wasm32")]
                    self.fix_puzzle_offset_for_mobile(ui);

                    if rebuild_subimages {
                        self.rebuild_subimage(j, i, subimage_index, button_side, ui);
                    }

                    if let Some(subimage) = self.puzzle_subimages.get_mut(subimage_index) {
                        if !rebuild_subimages {
                            update_subimage_region(ui, subimage, button_side);
                        }
                        let can_drag_list = self.board.get_swappable();
                        let can_drag = (self.missing_index != subimage_index)
                            && (can_drag_list.contains(&subimage_index));
                        let drag_event = make_drag(
                            &mut self.drag_delta,
                            ui,
                            ui.next_auto_id(),
                            can_drag,
                            |ui| {
                                ui.add_visible_ui(!self.delay_repaint, |ui| {
                                    ui.add_sized(
                                        [button_side, button_side],
                                        egui::Button::new("")
                                            .frame(true)
                                            .sense(egui::Sense::click_and_drag())
                                            .rounding(10.0),
                                    );
                                });
                            },
                        );

                        match drag_event {
                            Some(DragEvent::Dragging(is_dragging)) => {
                                if is_dragging {
                                    if let Some(drag_delta) = self.drag_delta {
                                        subimage.drag(drag_delta);
                                        if !self.delay_repaint {
                                            let mut order = egui::Order::Foreground;
                                            if self.missing_index != subimage_index {
                                                if !self.debug_paint {
                                                    subimage.paint(ui, &mut order, is_dragging);
                                                } else {
                                                    subimage.debug_paint(
                                                        ui,
                                                        &mut order,
                                                        is_dragging,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Some(DragEvent::Released(pos)) => {
                                let missing_index_swap = self.board.missing_index();
                                if let Some(subimage) =
                                    self.puzzle_subimages.get(missing_index_swap)
                                {
                                    if subimage.contains(pos) {
                                        self.missing_index = self.board.swap(subimage_index);
                                        self.puzzle_subimages
                                            .swap(missing_index_swap, subimage_index);
                                    }
                                }
                            }
                            None => {
                                if !self.delay_repaint {
                                    let mut order = egui::Order::Background;
                                    if self.missing_index != subimage_index {
                                        if !self.debug_paint {
                                            subimage.paint(ui, &mut order, false);
                                        } else {
                                            subimage.debug_paint(ui, &mut order, false);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    subimage_index += 1;
                }
                ui.end_row();
            }
        });

        ui.style_mut().spacing.button_padding = button_padding;
        ui.style_mut().spacing.item_spacing.x = item_spacing_x;
        ui.style_mut().spacing.item_spacing.y = item_spacing_y;

        #[allow(deprecated)]
        ui.centered(|ui| {
            if ui.button("Generate").clicked() {
                self.board.generate();
                self.regen = true;
            }
        });

        #[allow(deprecated)]
        ui.centered(|ui| {
            if ui.button("Reset").clicked() {
                self.puzzle_subimages.clear();
                self.missing_index = self.guaranteed_oob_index();
                self.in_play = false;
            }
        });

        #[allow(deprecated)]
        ui.centered(|ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                egui::RichText::new("(source code)").size(12.),
                "https://github.com/Stehfyn/cs481/blob/main/src/puzzle_panel.rs",
            ));
        });

        self.delay_repaint = false;
    }

    fn guaranteed_oob_index(&self) -> usize {
        (self.m * self.n) as usize + 1
    }

    fn generate_puzzle_board(&mut self) {
        let mut new_subimages: Vec<image_helpers::SubImage> = Vec::default();
        for i in 0..self.puzzle_subimages.len() {
            let tile_index = self.board.index_at(i);
            if let Some(simg) = self.puzzle_subimages.get(tile_index) {
                new_subimages.insert(new_subimages.len(), simg.clone())
            }
        }

        self.puzzle_subimages = new_subimages;
        self.missing_index = self.board.missing_index();
    }

    #[cfg(target_arch = "wasm32")]
    fn fix_puzzle_offset_for_mobile(&self, ui: &mut egui::Ui) {
        if isMobile() || isIOS() {
            let x = self.m as f32;
            let input_min = 2.0;
            let input_max = 6.0;
            let output_min = 1.0;
            let output_max = 0.75;

            // Normalize x from [input_min, input_max] to [output_min, output_max]
            let norm =
                (x - input_min) / (input_max - input_min) * (output_max - output_min) + output_min;

            //normalize diff
            ui.add_space(ui.style().spacing.window_margin.left * norm * 0.5);
        }
    }

    fn rebuild_subimage(
        &mut self,
        x: i32,
        y: i32,
        subimage_index: usize,
        button_side: f32,
        ui: &mut egui::Ui,
    ) {
        if let Some(rimg) = &self.puzzle_image_r {
            let next_subimage = build_next_subimage(
                ui,
                format!("sub_img_paint_{}", subimage_index),
                rimg.texture_id(ui.ctx()),
                subimage_index,
                self.m,
                x,
                y,
                button_side,
            );
            self.puzzle_subimages.insert(subimage_index, next_subimage);
        }
    }
}

impl PuzzlePanel {
    pub fn get_puzzle_dynamic_image_count(&self) -> usize {
        #[allow(unused_variables)]
        if let Some(rimg) = &self.puzzle_image {
            return 1;
        } else {
            return 0;
        }
    }

    pub fn get_puzzle_retained_image_count(&self) -> usize {
        #[allow(unused_variables)]
        if let Some(rimg) = &self.puzzle_image_r {
            return 1;
        } else {
            return 0;
        }
    }
    pub fn get_puzzle_subimage_count(&self) -> usize {
        self.puzzle_subimages.len()
    }

    pub fn set_mn(&mut self, mn: i32) {
        self.m = mn;
        self.n = mn;
        self.rebuild_on_next_frame();
    }

    pub fn get_constrained_width(&self) -> f32 {
        self.constrained_width
    }

    pub fn set_debug_paint(&mut self, debug_paint: bool) {
        self.debug_paint = debug_paint;
    }

    pub fn set_puzzle_image_and_rebuild(&mut self, img: Option<image::DynamicImage>) {
        self.puzzle_image = img;
        if let Some(dynamic) = &self.puzzle_image {
            self.puzzle_image_r = image_helpers::dynamic_to_retained(dynamic);
            self.rebuild_on_next_frame();
        }
    }

    pub fn rebuild_on_next_frame(&mut self) {
        self.puzzle_subimages.clear();
        self.reset_board();
        self.delay_repaint = true;
        self.missing_index = self.guaranteed_oob_index();
    }

    fn reset_board(&mut self) {
        self.board = NBoard::new(self.n as usize);
    }
}

pub fn get_next_subimage_region(ui: &mut egui::Ui, button_side: f32) -> egui::Rect {
    let mut start = ui.next_widget_position();
    start.x -= button_side / 2.;
    start.y -= button_side / 2.;
    let end = egui::Pos2::new(start.x + button_side, start.y + button_side);

    egui::Rect {
        min: start,
        max: end,
    }
}

pub fn update_subimage_region(
    ui: &mut egui::Ui,
    subimage: &mut image_helpers::SubImage,
    button_side: f32,
) {
    let next_region = get_next_subimage_region(ui, button_side);
    subimage.region(&next_region);
}

pub fn update_subimage_padding(ui: &mut egui::Ui, subimage: &mut image_helpers::SubImage) {
    subimage.padding(
        ui.ctx().style().spacing.button_padding.x,
        ui.ctx().style().spacing.button_padding.y,
    );
}

pub fn build_next_subimage(
    ui: &mut egui::Ui,
    id: String,
    tid: egui::TextureId,
    ind: usize,
    mn: i32,
    x: i32,
    y: i32,
    button_side: f32,
) -> image_helpers::SubImage {
    let next_region = get_next_subimage_region(ui, button_side);
    let sub_image_rect = egui::Rect::from_two_pos(next_region.min, next_region.max);

    let u_x = (x as f32) * (1.0 / (mn as f32));
    let u_y = ((x + 1) as f32) * (1.0 / (mn as f32));
    let v_x = (y as f32) * (1.0 / (mn as f32));
    let v_y = ((y + 1) as f32) * (1.0 / (mn as f32));

    let uv_rect = egui::Rect::from_x_y_ranges(
        std::ops::RangeInclusive::new(u_x, u_y),
        std::ops::RangeInclusive::new(v_x, v_y),
    );

    let mut subimage = image_helpers::SubImage::new(id, tid, ind, &sub_image_rect, &uv_rect);

    update_subimage_padding(ui, &mut subimage);

    return subimage;
}

pub enum DragEvent {
    Dragging(bool),
    Released(egui::Pos2),
}

pub fn make_drag(
    drag_delta: &mut Option<egui::Vec2>,
    ui: &mut egui::Ui,
    id: egui::Id,
    can_drag: bool,
    ui_closure: impl FnOnce(&mut egui::Ui),
) -> Option<DragEvent> {
    let response = ui.scope(ui_closure).response;
    let response = ui.interact(response.rect, id, egui::Sense::drag());

    let mut drag_event = None;

    if can_drag {
        let mut _is_dragging = false;
        if response.hovered() {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
        }

        if response.drag_started() {
            *drag_delta = Some(egui::Vec2::ZERO);
            _is_dragging = true;
            drag_event = Some(DragEvent::Dragging(true));
        }

        if response.dragged() {
            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
            let delta = drag_delta.map(|s| s + response.drag_delta());

            *drag_delta = delta;
            _is_dragging = true;
            drag_event = Some(DragEvent::Dragging(true));
        }

        if response.drag_released() {
            *drag_delta = None;
            _is_dragging = false;
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                drag_event = Some(DragEvent::Released(pos));
            }
        }
    }
    drag_event
}
