/// The history panel of the calculator

use eframe::egui;
const FONT_SIZE: f32 = 32.0;

// Stores a past calculation
pub struct Calculation {
    pub expression: String,
    pub answer: String
}

/// Displays the vec of Calculations given in a list format
pub fn show_calculations(ui: &mut egui::Ui, calcs: &Vec<Calculation>) {
    // Set vertical scrollable panel
    egui::ScrollArea::vertical().show(ui, |ui| {
        for calc in calcs {
            ui.label(egui::RichText::new(&calc.expression).size(FONT_SIZE));
            ui.strong(egui::RichText::new(&calc.answer).size(FONT_SIZE));
            ui.separator();
        } 
    });
}