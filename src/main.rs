use eframe::egui;
use egui::Vec2;
use regex::Regex;

// Goal: Password solver
// Up to 4 characters as an input - filter through a list and pick the right matching word

impl eframe::App for PasswordSolverApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Center align heading
            ui.vertical_centered(|ui| ui.heading("GTFO Password Solver"));
            // TODO: Shift downwards a bit
            ui.separator();
            ui.vertical_centered(|ui| ui.heading("Inputs"));

            // TODO, center combo boxes on 1 row
            // TODO: allow keyboard inputs to pick the character
            // Adjust interactable shape of combo box
            ui.spacing_mut().interact_size = Vec2::new(100.0, 100.0);
            egui::ComboBox::from_label("Character 1")
                .selected_text(format!("{}", self.password_parts[0]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(
                            &mut self.password_parts[0],
                            character,
                            String::from(character),
                        );
                    }
                });
            egui::ComboBox::from_label("Character 2")
                .selected_text(format!("{}", self.password_parts[1]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(
                            &mut self.password_parts[1],
                            character,
                            String::from(character),
                        );
                    }
                });
            egui::ComboBox::from_label("Character 3")
                .selected_text(format!("{}", self.password_parts[2]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(
                            &mut self.password_parts[2],
                            character,
                            String::from(character),
                        );
                    }
                });
            egui::ComboBox::from_label("Character 4")
                .selected_text(format!("{}", self.password_parts[3]))
                .show_ui(ui, |ui| {
                    for character in self.combo_box_options {
                        ui.selectable_value(
                            &mut self.password_parts[3],
                            character,
                            String::from(character),
                        );
                    }
                });
            ui.label(format!(
                "Selected Password: {}{}{}{}",
                self.password_parts[0],
                self.password_parts[1],
                self.password_parts[2],
                self.password_parts[3]
            ));

            let password_matches =
                PasswordSolverApp::find_matching(self.password_parts, self.possible_codes.clone());
            match password_matches {
                None => ui.label("Please input at least 1 letter"),
                Some(matches) => {
                    match matches.len() {
                        0 => ui.label("No matches found."),
                        1 => ui.label(format!("Only Match: {}", matches[0])),
                        len => {
                            // Show up to 50 matches
                            let amount = len.min(50);
                            let output = matches[0..amount].join(", ");
                            ui.label(format!(
                                "Matches: {}{}",
                                output,
                                if len >= 50 { ", ..." } else { "" }
                            ))
                        }
                    }
                }
            }
        });
    }
}

struct PasswordSolverApp {
    // Selected password characters
    password_parts: [char; 4],
    // Constant list of possible characters, includes '-' for unselected and the entire english alphabet
    combo_box_options: [char; 27],
    possible_codes: Vec<String>,
}

impl PasswordSolverApp {
    pub fn new(codes: Vec<String>) -> Self {
        Self {
            password_parts: ['-', '-', '-', '-'],
            combo_box_options: [
                '-', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
                'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ],
            possible_codes: codes,
        }
    }

    // Returns an empty option if no matching was performed (everything was a wildcard). Otherwise, returns a Vector of all matches
    pub fn find_matching(
        password_parts: [char; 4],
        possible_codes: Vec<String>,
    ) -> Option<Vec<String>> {
        // Build Regex from password parts, '-' maps to wildcard (.)
        // Technically it should be [a-z] instead of '.', but it's fine
        let mut parts: [char; 4] = ['a'; 4];
        let mut wildcards: i32 = 0;

        for (index, password_char) in password_parts.iter().enumerate() {
            if *password_char == '-' {
                parts[index] = '.';
                wildcards += 1;
            } else {
                parts[index] = *password_char;
            }
        }
        if wildcards == 4 {
            // There's no input, we have 4 wildcards, return None to indicate that
            return None;
        }
        let regex = Regex::new(String::from_iter(parts).as_str()).unwrap();
        let matches = possible_codes
            .iter()
            .filter(|code| regex.is_match(code))
            .cloned()
            .collect::<Vec<_>>();

        Some(matches)
    }
}

fn main() -> eframe::Result {
    let mut possible_codes: Vec<String> = Vec::new();
    // Read in codes
    let all_codes = include_str!("../codes.txt");
    for code in all_codes.lines() {
        possible_codes.push(code.to_string());
    }

    // Create window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GTFO Password Solver",
        options,
        Box::new(|_cc| {
            Ok(Box::<PasswordSolverApp>::new(PasswordSolverApp::new(
                possible_codes,
            )))
        }),
    )
}
