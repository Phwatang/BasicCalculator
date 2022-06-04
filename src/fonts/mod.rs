// Custom font import needed as the default font within egui does not
// support many strange characters
use eframe::egui;

const FONT_NAME: &str = "Inter";

/// Prioritise NotoSans font to be the default text on the GUI
pub fn set_font(ctx: &egui::Context) {
    let mut font_config = egui::FontDefinitions::default();
    // import NotoSans
    font_config.font_data.insert(
        FONT_NAME.to_owned(),
        egui::FontData::from_static(include_bytes!("Inter-Regular.ttf"))
    );

    // prioritize NotoSans
    font_config.families.get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, FONT_NAME.to_owned());
    // set font data onto ctx
    ctx.set_fonts(font_config);
}