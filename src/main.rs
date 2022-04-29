// Stop terminal window from appearing when executing
// in windows
#![windows_subsystem = "windows"]

use eframe::{run_native, epi::App, egui};
mod expression_evaluate;
mod button_layout;
mod number_display;
mod history_panel;
mod fonts;

const LEFT_PANEL_WIDTH: f32 = 262.0;
const RIGHT_PANEL_MIN_WIDTH: f32 = 300.0;
const MIN_WINDOW_X: f32 = LEFT_PANEL_WIDTH + RIGHT_PANEL_MIN_WIDTH;
const MIN_WINDOW_Y: f32 = 484.0;

struct Calculator {
    curr_expression: String,
    past_entries: Vec<history_panel::Calculation>
}
impl Calculator {
    pub fn new() -> Self {
        return Self {
            curr_expression: String::new(),
            past_entries: Vec::new()
        };
    }
}

impl App for Calculator {
    // On gui startup
    fn setup(
        &mut self,
        ctx: &egui::Context, 
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>
    ) {
        fonts::set_font(ctx);
        ctx.set_visuals(egui::Visuals::dark());
    }
    // for each frame
    fn update(
        &mut self, 
        ctx: &egui::Context, 
        _frame: &eframe::epi::Frame) 
    {
        // Create left viewing area
        egui::SidePanel::left("main_area")
        .min_width(LEFT_PANEL_WIDTH)
        .resizable(false)
        .show(ctx, |ui| {
            ui.visuals_mut().clip_rect_margin = 0.0;
            // Set to vertical layout
            ui.vertical(|ui| {
                // Display calculator buttons on bottom
                let recent_press = button_layout::show_buttons(ui);
                // React on press of button
                match recent_press {
                    None => (),
                    Some("=") => {
                        let potential_answer: Option<f64> = expression_evaluate::evaluate_RPN(
                            &expression_evaluate::postfix_to_RPN(&self.curr_expression)
                        );
                        let expression: String = self.curr_expression.clone();
                        if potential_answer.is_some() {
                            self.curr_expression = potential_answer.unwrap().to_string();
                            self.past_entries.push(history_panel::Calculation{expression: expression, answer: self.curr_expression.clone()})
                        }
                        else {
                            self.curr_expression.clear();
                            self.past_entries.push(history_panel::Calculation{expression: expression, answer: String::from("Error")});
                        }
                    },
                    Some("C") => self.curr_expression.clear(),
                    Some("←") => {let _ = self.curr_expression.pop();},
                    _ => self.curr_expression.push_str(recent_press.unwrap())
                }
                // Fill up the top of the left panel with the answer display
                egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show_inside(ui, |ui| {
                    number_display::show_number_screen(ui, &self.curr_expression);
                });
            });
        });
        // Fill up rest of right side with list of past calculations
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_width(RIGHT_PANEL_MIN_WIDTH);
            history_panel::show_calculations(ui, &self.past_entries);
        });
    }
    // Name of application
    fn name(&self) -> &str { 
        "Calculator"
    }
}

fn main() {
    let app: Calculator = Calculator::new();
    let mut win_options = eframe::NativeOptions::default();
    win_options.min_window_size = Some(egui::Vec2{x: MIN_WINDOW_X, y: MIN_WINDOW_Y});
    win_options.initial_window_size = Some(egui::Vec2{x: MIN_WINDOW_X, y: MIN_WINDOW_Y});
    run_native(Box::new(app), win_options);
}
