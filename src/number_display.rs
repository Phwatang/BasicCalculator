/// The number display of the calculator

use eframe::egui;
const FONT_SIZE: f32 = 48.0;

/// Place label onto the UI where numbers will be displayed
pub fn show_number_screen(ui: &mut egui::Ui, text: &String) {
    let number_area = egui::RichText::new(text).size(FONT_SIZE).strong();
    ui.spacing_mut().item_spacing = egui::vec2(0.0,0.0);
    // Vertical scrollable are incase of large numbers
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            ui.add(egui::Label::new(number_area).wrap(true));
        });
    });
}