/// The input buttons of the calculator

use eframe::egui;

// The button texts and text colours
// Layout shown is the intended display layout.
const ORANG: egui::Color32 = egui::Color32::from_rgb(247, 137, 52);
const WHITE: egui::Color32 = egui::Color32::WHITE;
const LBLUE: egui::Color32 = egui::Color32::from_rgb(132, 151, 245);
struct ButInfo<'a>(&'a str, egui::Color32);
const BUTTON_INFO: [ButInfo; 24] = [
    ButInfo("C", ORANG), ButInfo("π", LBLUE), ButInfo("e", LBLUE), ButInfo("^", ORANG),
    ButInfo("(", ORANG), ButInfo(")", ORANG), ButInfo("√", ORANG), ButInfo("÷", ORANG),
    ButInfo("7", WHITE), ButInfo("8", WHITE), ButInfo("9", WHITE), ButInfo("×", ORANG),
    ButInfo("4", WHITE), ButInfo("5", WHITE), ButInfo("6", WHITE), ButInfo("-", ORANG),
    ButInfo("1", WHITE), ButInfo("2", WHITE), ButInfo("3", WHITE), ButInfo("+", ORANG),
    ButInfo("⌫", ORANG), ButInfo("0", WHITE), ButInfo(".", WHITE), ButInfo("=", ORANG)
];
// Grid formating consts
const N_COL: usize = 4;
const N_ROW: usize = 6;
const SPACING: egui::Vec2 = egui::Vec2 {x: 2.0, y: 2.0};
const FONT_SIZE: f32 = 32.0; // Font size for the text on each button

// Minimum ui width needs for show_buttons method to work properly
pub const MIN_WIDTH_NEEDED: f32 = 170.0;

pub struct CalculatorButtons {
    pub recent_press: Option<String>,
    pub height: f32,
    pub width: f32
}
impl CalculatorButtons {
    pub fn new() -> Self {
        return CalculatorButtons {
            recent_press: None,
            height: 0.0,
            width: 0.0
        }
    }
    /// Places a grid of buttons onto the ui given. Button sizes will
    /// be scaled to ensure the grid spans the entire width of ui given.
    /// Buttons will always be in proportion to a square.
    pub fn show_buttons(&mut self, ui: &mut egui::Ui) {
        self.width = ui.available_width();
        // Lock in ui width (weird behaviour without this)
        ui.set_width(self.width);

        self.recent_press = None;
        let button_width = (self.width - ((N_COL-1) as f32)*SPACING.x) / N_COL as f32;
        // Formatting button grid (sizing and spacing)
        let button_grid = egui::Grid::new("Stuff")
            .min_row_height(button_width)
            .max_col_width(button_width)
            // setting min_col_width seems to be unecessary
            .spacing(SPACING);
        // Place buttons onto the grid
        button_grid.show(ui, |ui| {
            // Iterate through grid
            for i in 0..N_ROW {
                for j in 0..N_COL {
                    // Calculate index
                    let index: usize = (i*N_COL + j) as usize;
                    // Create button and check if its clicked
                    ui.centered_and_justified(|ui| {
                        if ui.add(
                            egui::Button::new(
                                egui::RichText::new(BUTTON_INFO[index].0).size(FONT_SIZE).color(BUTTON_INFO[index].1))
                            .fill(egui::color::Color32::TRANSPARENT))
                        .clicked() {
                            self.recent_press = Some(BUTTON_INFO[index].0.to_string());
                        }
                    });
                }
                ui.end_row();
            }
        });
        // Add padding to very bottom of grid
        ui.add_space(5.0);
        // Calculate height used
        self.height = button_width*(N_ROW as f32) + SPACING.y*((N_ROW-1) as f32) + 5.0;
    }

    /// Calculates ui width needed for show_buttons method to reach a given height.
    pub fn width_needed_for_height(height: f32) -> f32 {
        // Calculation is the inverse of the height calculation used in show_buttons
        return
            ((height-5.0-SPACING.y*((N_ROW-1) as f32)) / (N_ROW as f32)) * (N_COL as f32)
            + ((N_COL-1) as f32) * SPACING.x;
    }
}