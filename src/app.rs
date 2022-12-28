use eframe::{
    egui::{self, Button, RichText, TextBuffer, TextEdit},
    epaint::{FontFamily, FontId},
};

use crate::error::Error;

pub struct IPv6ShortenApp {
    input: [InputChunkBuffer; 8],
    output: OutputMsg,
}

const EMPTY_INPUT: InputChunkBuffer = InputChunkBuffer::new();

impl IPv6ShortenApp {
    pub fn new() -> Self {
        Self {
            input: [EMPTY_INPUT; 8],
            output: OutputMsg::new(),
        }
    }

    fn shorten_button_action(&mut self) {
        match self.validate_input() {
            Ok(_) => {
                self.output.update(format!("Result: {}", crate::shortener::shorten_ipv6(&self.input)));
            }
            Err(msg) => self.output.update(msg.to_string()),
        }
    }

    fn validate_input(&self) -> Result<(), Error> {
        for (i, input) in self.input.iter().enumerate() {
            // Validating characters is not required since invalid characters
            // are disabled at the user input stage.
            let len = input.len();
            if len != 4 && len != 0 {
                return Err(Error::NotEnoughChars(
                    "Required 4 characters at input box",
                    i + 1,
                ));
            }
            // assert!(len < 5);
        }

        Ok(())
    }
}

const HEADING_FONT: FontId = FontId::new(20.0, FontFamily::Monospace);
const INPUT_FONT: FontId = FontId::new(17.0, FontFamily::Monospace);
const TEXT_FONT: FontId = FontId::new(14.0, FontFamily::Monospace);

impl eframe::App for IPv6ShortenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Heading
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

            // First label
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

            // Input boxes
            ui.horizontal(|ui| {
                ui.add_space(4.0);
                for i in 0..7 {
                    TextEdit::singleline(&mut self.input[i])
                        .desired_width(41.0)
                        .hint_text("0000")
                        .font(INPUT_FONT)
                        .show(ui);
                    ui.label(RichText::new(":").font(INPUT_FONT).strong());
                }
                TextEdit::singleline(&mut self.input[7])
                    .desired_width(41.0)
                    .hint_text("0000")
                    .font(INPUT_FONT)
                    .show(ui);
            });

            ui.add_space(10.0);

            // 'Shorten' button
            ui.horizontal(|ui| {
                ui.add_space(crate::WIN_WIDTH / 2.0 - 55.0);
                if ui.add(Button::new("Click to shorten")).clicked() {
                    self.shorten_button_action();
                }
            });

            ui.add_space(10.0);

            // Result output box
            ui.horizontal(|ui| {
                ui.centered_and_justified(|ui| {
                    TextEdit::multiline(&mut self.output)
                        .desired_rows(2)
                        .hint_text("Shortened IPv6 address will appear here.")
                        .frame(true)
                        .font(INPUT_FONT)
                        .show(ui);
                });
            });

            ui.add_space(13.0);

            // Info
            ui.label("Select to copy...");
        });
    }
}

struct OutputMsg {
    msg: String,
}

impl OutputMsg {
    fn new() -> Self {
        Self { msg: String::new() }
    }

    fn update(&mut self, msg: String) {
        self.msg = msg;
    }
}

impl eframe::egui::TextBuffer for OutputMsg {
    // Result output text is never mutable.
    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        &self.msg
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
pub struct InputChunkBuffer {
    pub buf: String,
}

impl InputChunkBuffer {
    const fn new() -> Self {
        Self { buf: String::new() }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}

impl TextBuffer for InputChunkBuffer {
    // InputChunk is always mutable.
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        &self.buf
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let mut inserted = 0;
        for c in text.chars().rev() {
            if self.buf.len() < INPUT_CHUNK_MAX_CHARS {
                if let Some(_) = c.to_digit(16) {
                    self.buf.insert(char_index, c);
                    inserted += 1;
                }
                continue;
            }
            break;
        }
        inserted
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.buf.delete_char_range(char_range);
    }
}
