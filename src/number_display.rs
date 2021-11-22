use eframe::egui;

/// Configure fonts for the context given
pub fn set_font(ctx: &egui::CtxRef) {
    let mut font_config = egui::FontDefinitions::default();
    // import NotoSans
    font_config.font_data.insert(
        String::from("NotoSans"),
        std::borrow::Cow::Borrowed(include_bytes!("../fonts/NotoSans-Regular.ttf"))
    );
    // set font sizes
    font_config.family_and_size.insert(
        egui::TextStyle::Body,
        (egui::FontFamily::Proportional, 48.0)
    );
    font_config.family_and_size.insert(
        egui::TextStyle::Button,
        (egui::FontFamily::Proportional, 32.0)
    );
    // prioritize NotoSans
    font_config.fonts_for_family.get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, String::from("NotoSans"));
    // set font data onto ctx
    ctx.set_fonts(font_config);
}

/// Place label onto the UI where numbers will be displayed
pub fn show_number_screen(ui: &mut egui::Ui, text: &String) {
    let display = egui::Label::new(text).strong().wrap(true);
    ui.spacing_mut().item_spacing = egui::vec2(0.0,0.0);
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.with_layout(egui::Layout::right_to_left(), |ui| {
            ui.add(display);
        });
    });
}