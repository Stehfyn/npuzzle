#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct AboutPanel {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub open: bool,
}

impl Default for AboutPanel {
    fn default() -> Self {
        Self { open: false }
    }
}

impl AboutPanel {
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
    pub fn end_of_frame(&mut self, ctx: &egui::Context) {}
    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}
