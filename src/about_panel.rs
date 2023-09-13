/**
 * @file about_panel.rs
 *
 * @brief This is the About Panel module which details relavent topic information and attributions.
 *
 * @author Stephen Foster
 * Contact: stephenfoster@nevada.unr.edu
 *
 */
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
    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.centered(|ui| {
            ui.heading(egui::RichText::new("Game Modes").size(24.0).underline());
        });
        ui.label(egui::RichText::new("\n").size(6.0));
        ui.centered(|ui| {
            ui.label(egui::RichText::new("TimeAttack").italics().size(18.0));
        });

        ui.label(egui::RichText::new("\n").size(2.0));
        ui.centered(|ui| {
        ui.label(egui::RichText::new("Complete the puzzle in a race against the clock! Hints are available via A* search.").size(14.0));
        });
        ui.centered(|ui| {
            ui.label(egui::RichText::new("Outsmart").italics().size(18.0));
        });
        ui.label(egui::RichText::new("\n").size(2.0));
        ui.centered(|ui| {
        ui.label(egui::RichText::new("Try and outwit the computer! Submit an insolvable puzzle to win. Solutions are checked with A* search.").size(14.0));
        });
        ui.label(egui::RichText::new("\n").size(6.0));
        ui.centered(|ui| {
            ui.label(
                egui::RichText::new("A* (Manhattan Distance)")
                    .italics()
                    .size(18.0),
            );
        });
        ui.code(
            egui::RichText::new(
                "pub fn a_star_solve(&self) -> Option<Vec<usize>> {
            let mut visited: HashSet<String> = HashSet::new();
            let mut heap = BinaryHeap::new();
    
            heap.push(State {
                cost: 0,
                board: self.clone(),
                steps: Vec::new(),
            });
    
            while let Some(State { cost, board, steps }) = heap.pop() {
                if board.check_win() {
                    return Some(steps);
                }
    
                let state_str = board.to_string();
                if visited.contains(&state_str) {
                    continue;
                }
                visited.insert(state_str);
    
                for swappable_index in board.get_swappable() {
                    let mut new_board = board.clone();
                    new_board.swap(swappable_index);
    
                    let mut new_steps = steps.clone();
                    new_steps.push(swappable_index);
    
                    heap.push(State {
                        cost: new_steps.len() + new_board.manhattan_distance(),
                        board: new_board,
                        steps: new_steps,
                    });
                }
            }
            None
        }",
            )
            .size(10.)
            .background_color(egui::Color32::BLACK),
        );
        ui.add_sized(
            [1f32, ((ui.available_height() - (48. * 2.)) as f32)],
            egui::Label::new(""),
        );
        ui.centered(|ui| {
            ui.add(egui::Label::new(
                egui::RichText::new("Stephen Foster").size(12.),
            ));
        });
        ui.centered(|ui| {
            ui.add(egui::Label::new(
                egui::RichText::new("CS 481 - AI in Games").size(12.),
            ));
        });
        ui.centered(|ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                egui::RichText::new("stephenfoster@nevada.unr.edu").size(12.),
                "stephenfoster@nevada.unr.edu",
            ));
        });
        ui.centered(|ui| {
            ui.add(egui::Hyperlink::from_label_and_url(
                egui::RichText::new("@Stehfyn").size(12.),
                "https://github.com/Stehfyn/",
            ));
        });
    }
}
