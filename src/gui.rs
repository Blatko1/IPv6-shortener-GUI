use eframe::{
    egui::{self, Button, RichText, TextEdit},
    epaint::{FontFamily, FontId},
};

pub struct GUI {
    input: [InputChunkBuffer; 8],
    output: ResultOutput,
}

const EMPTY_INPUT: InputChunkBuffer = InputChunkBuffer::new();

impl GUI {
    pub fn new() -> Self {
        Self {
            input: [EMPTY_INPUT; 8],
            output: ResultOutput::new(),
        }
    }
}

const HEADING_FONT: FontId = FontId::new(20.0, FontFamily::Monospace);
const INPUT_FONT: FontId = FontId::new(17.0, FontFamily::Monospace);
const TEXT_FONT: FontId = FontId::new(14.0, FontFamily::Monospace);

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("IPv6 shortener")
                            .strong()
                            .underline()
                            .font(HEADING_FONT),
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
                            .font(TEXT_FONT),
                    );
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(10.0);
                for i in 0..7 {
                    TextEdit::singleline(&mut self.input[i])
                        .desired_width(40.0)
                        .hint_text("0000")
                        .font(INPUT_FONT)
                        .show(ui);
                    ui.label(RichText::new(":").font(INPUT_FONT).strong());
                }
                TextEdit::singleline(&mut self.input[7])
                    .desired_width(40.0)
                    .hint_text("0000")
                    .font(INPUT_FONT)
                    .show(ui);
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.add_space(crate::WIN_WIDTH / 2.0 - 55.0);
                if ui.add(Button::new("Click to Shorten")).clicked() {
                    println!("mama ti");
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                TextEdit::multiline(&mut self.output)
                    .desired_rows(2)
                    .hint_text("Shortened IPv6 address will appear here.")
                    .frame(true)
                    .font(INPUT_FONT)
                    .show(ui);
            });

            ui.add_space(13.0);

            ui.label("Select to copy...");
        });
    }
}

struct ResultOutput {
    result: String,
}

impl ResultOutput {
    fn new() -> Self {
        Self {
            result: String::new(),
        }
    }
}

impl eframe::egui::TextBuffer for ResultOutput {
    // Result output text is never mutable.
    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        &self.result
    }

    fn insert_text(&mut self, _text: &str, _char_index: usize) -> usize {
        0 // Buffer is immutable
    }

    fn delete_char_range(&mut self, _char_range: std::ops::Range<usize>) {
        // Buffer is immutable
    }
}

const INPUT_CHUNK_MAX_CHARS: usize = 4;

/// InputChunkBuffer holds a input String buffer with length of 4 hexadecimal characters (e.g. f6a2).
#[derive(Debug, Default)]
struct InputChunkBuffer {
    data: String,
}

impl InputChunkBuffer {
    const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl eframe::egui::TextBuffer for InputChunkBuffer {
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
