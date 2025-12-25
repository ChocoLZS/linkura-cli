use indicatif::{ProgressBar, ProgressStyle};

#[allow(dead_code)]
pub const TICK_CLOCK: [&str; 12] = [
    "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ", "🕛 ",
];

#[derive(Debug)]
pub struct SpinnerManager {
    quiet: bool,
}

impl SpinnerManager {
    pub fn new(quiet: bool) -> Self {
        Self { quiet }
    }

    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        if self.quiet {
            // Create a hidden progress bar that doesn't output to console
            let pb = ProgressBar::hidden();
            pb.set_message(message.to_string());
            pb
        } else {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message(message.to_string());
            pb
        }
    }

    pub fn create_spinner_with_color(&self, message: &str, color: &str) -> ProgressBar {
        if self.quiet {
            let pb = ProgressBar::hidden();
            pb.set_message(message.to_string());
            pb
        } else {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template(&format!("{{spinner:.{color}}} {{msg}}"))
                    .unwrap(),
            );
            pb.set_message(message.to_string());
            pb
        }
    }
}
