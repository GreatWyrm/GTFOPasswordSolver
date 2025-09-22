use eframe::egui;
use regex::Regex;

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

            let password_matches = PasswordSolverApp::find_matching(self.password_parts, self.possible_codes.clone());
            match password_matches {
                None => {
                    ui.label(format!("Please input at least 1 letter"));
                }
                Some(matches) => {
                    if !matches.is_empty() {
                        if matches.len() == 1 {
                            ui.label(format!("Only Match: {}", matches.get(0).unwrap()));
                        } else {
                            // Show up to 50 matches
                            let max_displayed = 50;
                            let num_to_output = std::cmp::min(max_displayed, matches.len());
                            let mut output: String = "".to_string();
                            for i in 0..num_to_output {
                                if output.len() > 0 {
                                    output.push_str(", ");
                                }
                                output.push_str(matches.get(i).unwrap());
                            }
                            if max_displayed <= matches.len() {
                                output.push_str(", ...");
                            }
                            ui.label(format!("Matches: {}", output));
                        }
                    } else {
                        ui.label("No matches found.");
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
    possible_codes: Vec<String>
}

impl PasswordSolverApp {
    pub fn new(codes: Vec<String>) -> Self {
        Self {
            password_parts: ['-', '-', '-', '-'],
            combo_box_options: ['-', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
            possible_codes: codes
        }
    }

    // Returns an empty option if no matching was performed (everything was a wildcard). Otherwise, returns a Vector of all matches
    pub fn find_matching(password_parts: [char; 4], possible_codes: Vec<String>) -> Option<Vec<String>> {
        // Build Regex from password parts, '-' maps to wildcard (.)
        // Technically it should be [a-z] instead of '.', but it's fine
        let mut parts: [char; 4] = ['a'; 4];
        let mut wildcards: i32 = 0;
        let mut i: usize = 0;
        for password_char in password_parts {
            if password_char == '-' {
                parts[i] = '.';
                wildcards += 1;
            } else {
                parts[i] = password_char;
            }
            i += 1;
        }
        if wildcards == 4 {
            // There's no input, we have 4 wildcards, return None to indicate that
            return None;
        }
        let regex = Regex::new(String::from_iter(parts).as_str()).unwrap();
        let mut matches: Vec<String> = Vec::new();
        for code in possible_codes {
            if regex.find(code.as_str()).is_some() {
                matches.push(code);
            }
        }
        return Some(matches);
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
            Ok(Box::<PasswordSolverApp>::new(PasswordSolverApp::new(possible_codes)))
        }),
    )
}