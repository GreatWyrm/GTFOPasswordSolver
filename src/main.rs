use eframe::egui;

// Goal: Password solver
// Up to 4 characters as an input - filter through a list and pick the right matching word

impl eframe::App for PasswordSolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Center align heading
            ui.vertical_centered(|ui| {
                ui.heading("GTFO Password Solver")
            });
            // TODO: Shift downwards a bit
            ui.vertical_centered(|ui| {
                ui.heading("Inputs")
            });
            // TODO, possibly change the size of the selection box & center these
            // Width does properly change the width, height does not appear to work
            // TODO: allow keyboard inputs to pick the character
            egui::ComboBox::from_label("Character 1")
                .selected_text(format!("{}", self.password_parts[0]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(&mut self.password_parts[0], character, &String::from(character));
                    }
                });
            egui::ComboBox::from_label("Character 2")
                .selected_text(format!("{}", self.password_parts[1]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(&mut self.password_parts[1], character, &String::from(character));
                    }
                });
            egui::ComboBox::from_label("Character 3")
                .selected_text(format!("{}", self.password_parts[2]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(&mut self.password_parts[2], character, &String::from(character));
                    }
                });
            egui::ComboBox::from_label("Character 4")
                .selected_text(format!("{}", self.password_parts[3]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(&mut self.password_parts[3], character, &String::from(character));
                    }
                });
            ui.label(format!("Selected Password: {}{}{}{}", self.password_parts[0], self.password_parts[1], self.password_parts[2], self.password_parts[3]));
        });
    }
}

struct PasswordSolverApp {
    // Selected password characters
    password_parts: [char; 4],
    // Constant list of possible characters, includes '-' for unselected and the entire english alphabet
    combo_box_options: [char; 27]
}

impl Default for PasswordSolverApp {
    fn default() -> Self {
        Self {
            password_parts: ['-', '-', '-', '-'],
            combo_box_options: ['-', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']
        }
    }
}

pub fn filled_triangle(
    ui: &egui::Ui,
    rect: egui::Rect,
    visuals: &egui::style::WidgetVisuals,
    _is_open: bool,
) {
    let rect = egui::Rect::from_center_size(
        rect.center(),
        egui::vec2(rect.width() * 0.6, rect.height() * 0.4),
    );
    ui.painter().add(egui::Shape::convex_polygon(
        vec![rect.left_top(), rect.right_top(), rect.center_bottom()],
        visuals.fg_stroke.color,
        visuals.fg_stroke,
    ));
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GTFO Password Solver",
        options,
        Box::new(|_cc| {
            Ok(Box::<PasswordSolverApp>::default())
        }),
    )
}