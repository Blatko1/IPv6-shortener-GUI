use eframe::{
    egui::{self, Button, RichText, TextBuffer, TextEdit},
    epaint::{FontFamily, FontId},
};

const WIN_WIDTH: f32 = 600.0;
const WIN_HEIGHT: f32 = 240.0;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIN_WIDTH, WIN_HEIGHT)),
        //icon_data: todo!(),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "IPv6 Shortener",
        options,
        Box::new(|cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    input: [InputChunkBuffer; 8],
    output: ResultOutput
}

const EMPTY_INPUT: InputChunkBuffer = InputChunkBuffer::new();

impl MyApp {
    fn new() -> Self {
        Self {
            input: [EMPTY_INPUT; 8],
            output: ResultOutput::new()
        }
    }
}

const HEADING: &str = "IPv6 shortener";
const HEADING_FONT_ID: FontId = FontId::new(20.0, FontFamily::Monospace);
const INPUT_FONT_ID: FontId = FontId::new(17.0, FontFamily::Monospace);
const TEXT_FONT_ID: FontId = FontId::new(14.0, FontFamily::Monospace);
const INPUT_BOX_WIDTH: f32 = 40.0;
const INPUT_HINT_TEXT: &str = "0000";
const OUTPUT_HINT_TEXT: &str = "Shortened IPv6 will appear here.";

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new(HEADING)
                            .strong()
                            .underline()
                            .font(HEADING_FONT_ID),
                    );
                });
            });

            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("Enter IPv6 address:")
                            .strong()
                            .font(TEXT_FONT_ID),
                    );
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(10.0);
                for i in 0..7 {
                    TextEdit::singleline(&mut self.input[i])
                        .desired_width(INPUT_BOX_WIDTH)
                        .hint_text(INPUT_HINT_TEXT)
                        .font(INPUT_FONT_ID)
                        .show(ui);
                    ui.label(RichText::new(":").font(INPUT_FONT_ID).strong());
                }
                TextEdit::singleline(&mut self.input[7])
                    .desired_width(INPUT_BOX_WIDTH)
                    .hint_text(INPUT_HINT_TEXT)
                    .font(INPUT_FONT_ID)
                    .show(ui);
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(WIN_WIDTH / 2.0 - 55.0);
                if ui.add(Button::new("Click to Shorten")).clicked() {
                    println!("mama ti");
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                TextEdit::multiline(&mut self.output).desired_rows(2)
                .hint_text(OUTPUT_HINT_TEXT).frame(true).font(INPUT_FONT_ID).show(ui);
            });

            ui.add_space(13.0);

            if ui.button("Copy to clipboard").clicked() {

            }
        });
    }
}

struct ResultOutput {
    result: String
}

impl ResultOutput {
    pub fn new() -> Self {
        Self {
            result: String::from("mama tiiiaidiamdio eaiofuwe")
        }
    }
}

impl TextBuffer for ResultOutput {
    // Result output text is never mutable.
    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        &self.result
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        0 // Buffer is immutable
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        // Buffer is immutable
    }
}

const INPUT_CHUNK_MAX_CHARS: usize = 4;

/// InputChunkBuffer holds a input String buffer with length of 4 hexadecimal characters.
#[derive(Debug, Default)]
struct InputChunkBuffer {
    data: String,
}

impl InputChunkBuffer {
    pub const fn new() -> Self {
        Self {
            data: String::new()
        }
    }
}

impl TextBuffer for InputChunkBuffer {
    // InputChunk is always mutable.
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.data
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let mut inserted = 0;
        for c in text.chars().rev() {
            if self.data.len() < INPUT_CHUNK_MAX_CHARS {
                if let Some(_) = c.to_digit(16) {
                    self.data.insert(char_index, c);
                    inserted += 1;
                }
                continue;
            }
            break;
        }
        inserted
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.data.delete_char_range(char_range);
    }
}
