// stop terminal window from appearing when executing
// in windows
#![windows_subsystem = "windows"]

use eframe::{run_native, epi::App, egui};
mod button_layout;
mod number_display;
mod expression_evaluate;

const START_WINDOW_X: f32 = 562.0;
const START_WINDOW_Y: f32 = 484.0;

// Stores a past calculation
struct Calculation {
    expression: String,
    answer: String
}

struct Calculator {
    current_entry: String,
    past_entries: Vec<Calculation>
}
impl Calculator {
    pub fn new() -> Self {
        return Self {
            current_entry: String::new(),
            past_entries: Vec::new()
        };
    }
}

impl App for Calculator {
    // on gui startup
    fn setup(
        &mut self,
        ctx: &egui::CtxRef, 
        _frame: &mut eframe::epi::Frame<'_>, 
        _storage: Option<&dyn eframe::epi::Storage>
    ) {
        number_display::set_font(ctx);
    }
    // for each frame
    fn update(
        &mut self, 
        ctx: &eframe::egui::CtxRef, 
        frame: &mut eframe::epi::Frame<'_>) 
    {
        // reset recent_press
        let mut recent_press: String = String::from("");
        // create left viewing area
        egui::SidePanel::left("main_area")
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_min_width(262.0);
            ui.visuals_mut().clip_rect_margin = 0.0;
            // set vertical layout
            ui.vertical(|ui| {
                // display buttons on bottom
                button_layout::show_buttons(ui, &mut recent_press);
                if recent_press != "" {
                    match recent_press.as_str() {
                        "=" => {
                            let potential_answer: Option<f64> = expression_evaluate::evaluate_RPN(
                                &expression_evaluate::postfix_to_RPN(&self.current_entry)
                            );
                            let expression: String = self.current_entry.clone();
                            if potential_answer.is_some() {
                                self.current_entry = potential_answer.unwrap().to_string();
                                self.past_entries.push(Calculation{expression: expression, answer: self.current_entry.clone()})
                            }
                            else {
                                self.current_entry.clear();
                                self.past_entries.push(Calculation{expression: expression, answer: String::from("Error")});
                            }
                        },
                        "C" => self.current_entry.clear(),
                        "←" => {let _ = self.current_entry.pop();},
                        _ => self.current_entry.push_str(&recent_press)
                    }
                }
                // display number area to fill up top
                egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show_inside(ui, |ui| {
                    number_display::show_number_screen(ui, &self.current_entry);
                });
            });
        });
        // create viewing area filling up rest of right side
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(250.0);
                // show historic calculations
                for entry in &self.past_entries {
                   ui.label(&entry.expression);
                   ui.strong(&entry.answer);
                   ui.separator();
               } 
            });
        });
    }
    // name of application
    fn name(&self) -> &str { 
        "Calculator"
    }
}

fn main() {
    let app: Calculator = Calculator::new();
    let mut win_options = eframe::NativeOptions::default();
    win_options.initial_window_size = Some(egui::Vec2{x: START_WINDOW_X, y: START_WINDOW_Y});
    run_native(Box::new(app), win_options);
}
