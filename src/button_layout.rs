/// The input buttons of the calculator

use eframe::egui;

// The button "titles" to be displayed.
// Layout shown is the intended display layout.
const BUTTON_TEXTS: [&str; 24] = [
    "C", "←", "π", "e",
    "√", "(", ")", "^",
    "7", "8", "9", "/",
    "4", "5", "6", "*",
    "1", "2", "3", "-",
    "0", ".", "=", "+"
];
// Grid formating consts
const N_COL: usize = 4;
const N_ROW: usize = 6;
const CELL_MIN_SIZE: f32 = 64.0;
const SPACING: egui::Vec2 = egui::Vec2 {x: 2.0, y: 2.0};
const TOTAL_HEIGHT: f32 = (N_ROW as f32*CELL_MIN_SIZE) + ((N_ROW-1) as f32)*SPACING.y;
const FONT_SIZE: f32 = 32.0; // Font size for the text on each button

/// Creates a bottom panel within the ui and places a grid of buttons
/// onto the panel.
/// 
/// Returns the text of any button clicked.
/// Returns None if no buttons are clicked.
pub fn show_buttons(ui: &mut egui::Ui) -> Option<&str> {
    let mut button_pressed: Option<&str> = None;
    // Formatting button grid dimensions and spacing
    let button_grid = egui::Grid::new("Stuff")
        .min_row_height(CELL_MIN_SIZE)
        .min_col_width(CELL_MIN_SIZE)
        .spacing(SPACING);
    // Show grid to ui
    egui::TopBottomPanel::bottom("main_area_buttons")
    .frame(egui::Frame::none())
    .show_inside(ui, |ui| {
        ui.set_min_height(TOTAL_HEIGHT);
        button_grid.show(ui, |ui| {
            // Iterate through grid
            for i in 0..N_ROW {
                for j in 0..N_COL {
                    // Calculate index
                    let index: usize = (i*N_COL + j) as usize;
                    // Create button
                    ui.centered_and_justified(|ui| {
                        if ui.button(egui::RichText::new(BUTTON_TEXTS[index]).size(FONT_SIZE)).clicked() {
                            button_pressed = Some(BUTTON_TEXTS[index]);
                        }
                    });
                }
                ui.end_row();
            }
        });
        // Add padding to very bottom of grid
        ui.add_space(2.0);
    });
    return button_pressed;
}