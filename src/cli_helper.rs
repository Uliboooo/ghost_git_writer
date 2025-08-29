use indicatif::{ProgressBar, ProgressStyle};
use std::{fmt::Display, time::Duration};
use unicode_width::UnicodeWidthChar;

// pub enum Error {
//     Io(std::io::Error),
// }
//
// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::Io(e) => write!(f, "io error: {}", e),
//         }
//     }
// }
//
// impl From<std::io::Error> for Error {
//     fn from(value: std::io::Error) -> Self {
//         Self::Io(value)
//     }
// }

pub struct Spinner {
    pb: ProgressBar,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.blue} {msg}")
                .unwrap(),
        );
        pb.set_message(message.to_string());
        Self { pb }
    }

    pub fn stop(&self, message: &str) {
        self.pb.finish_with_message(message.to_string());
    }
}

#[derive(Debug)]
pub struct Printer {
    content: String,
}

impl Printer {
    pub fn new<T: AsRef<str>>(content: T) -> Self {
        Self {
            content: content.as_ref().to_string(),
        }
    }
}

impl From<String> for Printer {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&String> for Printer {
    fn from(value: &String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Printer {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Display for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_chars = get_max_len(&self.content);
        let start_l = format!("╭{}╮", mul_str(&"─", max_chars + 2));
        let end_l = format!("╰{}╯", mul_str(&"─", max_chars + 2));

        let mut res = String::new();
        res.push_str(&start_l);
        for l in self.content.lines() {
            let rem = max_chars - get_str_len(l);
            let fill_space = mul_str(&" ", rem);
            res.push_str(format!("\n│ {l}{fill_space} │").as_str());
        }
        res.push_str(format!("\n{}", end_l.as_str()).as_str());

        write!(f, "{res}")
    }
}

fn get_str_len<T: AsRef<str>>(strg: T) -> u32 {
    let mut len = 0;
    for c in strg.as_ref().chars() {
        len += c.width().unwrap_or(0);
    }
    len as u32
}

fn mul_str<T: AsRef<str>>(msg: &T, mul: u32) -> String {
    let mut res = String::new();
    for _ in 0..mul {
        res.push_str(msg.as_ref());
    }
    res
}

fn get_max_len<T: AsRef<str>>(strg: T) -> u32 {
    let mut max = 0;
    for l in strg.as_ref().lines() {
        let len = get_str_len(l);
        if max < len {
            max = len;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::cli_helper::{Printer, mul_str};

    #[test]
    fn str_mul_test() {
        let c = "hi";
        let res = mul_str(&c, 10);
        assert_eq!("hihihihihihihihihihi".to_string(), res);
    }

    #[test]
    fn print_test() {
        let test_str = [
            (
                "line1\nline2line2line2\nline3line3",
                "╭─────────────────╮\n│ line1           │\n│ line2line2line2 │\n│ line3line3      │\n╰─────────────────╯",
            ),
            (
                "line全角21\nli全角ne2line2line2\nline3line3",
                "╭─────────────────────╮\n│ line全角21          │\n│ li全角ne2line2line2 │\n│ line3line3          │\n╰─────────────────────╯",
            ),
        ];

        for r in test_str {
            assert_eq!(format!("{}", Printer::new(r.0)), r.1.to_string());
        }
    }
}
