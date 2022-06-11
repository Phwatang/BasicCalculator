// Stop terminal window from appearing when executing
// in windows
#![windows_subsystem = "windows"]

use eframe::{run_native, epi::App, egui};
mod expression_evaluate;
mod button_layout;
mod number_display;
mod history_panel;
mod fonts;

const NUM_DISPLAY_MIN_HEIGHT: f32 = 85.0;
const MIN_WINDOW_X: f32 = 450.0;
const MIN_WINDOW_Y: f32 = 400.0;

struct Calculator {
    curr_expression: String,
    past_entries: Vec<history_panel::Calculation>,
    button_area: button_layout::CalculatorButtons,
    num_display_height: f32
}
impl Calculator {
    pub fn new() -> Self {
        return Self {
            curr_expression: String::new(),
            past_entries: Vec::new(),
            button_area: button_layout::CalculatorButtons::new(),
            num_display_height: 0.0
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
        let expansion: f32 = 
            button_layout::CalculatorButtons::width_needed_for_height(
                (self.num_display_height - NUM_DISPLAY_MIN_HEIGHT)
                + self.button_area.height
            );

        // Bottom area
        egui::TopBottomPanel::bottom("main_area")
        .max_height(self.button_area.height)
        .show(ctx, |ui| {
            // Display calculator buttons on bottom right
            egui::SidePanel::right("input_buttons")
            .frame(egui::Frame::none())
            .width_range(button_layout::MIN_WIDTH_NEEDED ..= expansion)
            .show_inside(ui, |ui| {
                self.button_area.show_buttons(ui);
                // Button behaviour
                match self.button_area.recent_press.as_deref() {
                    None => (),
                    Some("=") => { // Evaluate expression
                        let potential_answer: Option<f64> = expression_evaluate::evaluate_postfix(
                            &expression_evaluate::infix_to_postfix(&self.curr_expression)
                        );
                        let expression: String = self.curr_expression.clone();
                        if potential_answer.is_some() { // For no error, add expression and result into history
                            self.curr_expression = potential_answer.unwrap().to_string();
                            self.past_entries.push(history_panel::Calculation{expression: expression, answer: self.curr_expression.clone()})
                        }
                        else {
                            self.curr_expression.clear(); // On error, clear number display and place error into history
                            self.past_entries.push(history_panel::Calculation{expression: expression, answer: String::from("Error")});
                        }
                    },
                    Some("C") => self.curr_expression.clear(), // Clear expression
                    Some("⌫") => {let _ = self.curr_expression.pop();}, // Backspace
                    _ => self.curr_expression.push_str(self.button_area.recent_press.as_ref().unwrap().as_str()) // Push number/operator onto expression
                }
            });

            // Fill up rest of bottom left with list of past calculations
            egui::CentralPanel::default().show_inside(ui, |ui| {
                // ui.set_min_width(RIGHT_PANEL_MIN_WIDTH);
                history_panel::show_calculations(ui, &self.past_entries);
            });
        });

        // Fill up rest of top section with the answer display
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            number_display::show_number_screen(ui, &self.curr_expression);
        });

        self.num_display_height = ctx.used_size().y - self.button_area.height;
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
