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

const SEM_VER_ITEMS: [&str; 3] = ["MAJOR", "MINOR", "PATCH"];

pub struct SemVerSelector {
    selected: String,
}

impl SemVerSelector {
    pub fn new<T: AsRef<str>>(selected: T) -> Self {
        Self {
            selected: selected.as_ref().to_string(),
        }
    }
}

impl Display for SemVerSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let selected_lower = self.selected.trim().to_lowercase();
        let sel_idx = SEM_VER_ITEMS
            .iter()
            .position(|s| s.to_lowercase() == selected_lower)
            .unwrap_or(1);

        let mut top = String::new();
        let mut mid = String::new();
        let mut bot = String::new();

        for (i, item) in SEM_VER_ITEMS.iter().enumerate() {
            let w = get_str_len(item);

            // Add column separator before this item (except for the first item).
            // When neither adjacent item is selected, add an explicit │ separator.
            // When one adjacent item is selected, the box wall already provides it.
            if i > 0 {
                let prev_is_sel = (i - 1) == sel_idx;
                let curr_is_sel = i == sel_idx;
                if !prev_is_sel && !curr_is_sel {
                    top.push(' ');
                    mid.push('│');
                    bot.push(' ');
                }
            }

            if i == sel_idx {
                top.push_str(&format!("╭{}╮", mul_str(&"─", w + 2)));
                mid.push_str(&format!("│ {} │", item));
                bot.push_str(&format!("╰{}╯", mul_str(&"─", w + 2)));
            } else {
                let spaces = mul_str(&" ", w + 2);
                top.push_str(&spaces);
                mid.push_str(&format!(" {} ", item));
                bot.push_str(&spaces);
            }
        }

        write!(f, "{top}\n{mid}\n{bot}")
    }
}

#[cfg(test)]
mod tests {
    use crate::cli_helper::{Printer, SemVerSelector, mul_str};

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

    #[test]
    fn sem_ver_selector_test() {
        // MAJOR selected (first item)
        assert_eq!(
            format!("{}", SemVerSelector::new("MAJOR")),
            "╭───────╮               \n│ MAJOR │ MINOR │ PATCH \n╰───────╯               "
        );
        // Case-insensitive match
        assert_eq!(
            format!("{}", SemVerSelector::new("major")),
            "╭───────╮               \n│ MAJOR │ MINOR │ PATCH \n╰───────╯               "
        );
        // MINOR selected (middle item)
        assert_eq!(
            format!("{}", SemVerSelector::new("MINOR")),
            "       ╭───────╮       \n MAJOR │ MINOR │ PATCH \n       ╰───────╯       "
        );
        // Mixed-case match (as LLM might return "Minor")
        assert_eq!(
            format!("{}", SemVerSelector::new("Minor")),
            "       ╭───────╮       \n MAJOR │ MINOR │ PATCH \n       ╰───────╯       "
        );
        // PATCH selected (last item)
        assert_eq!(
            format!("{}", SemVerSelector::new("PATCH")),
            "               ╭───────╮\n MAJOR │ MINOR │ PATCH │\n               ╰───────╯"
        );
        // Unknown value defaults to MINOR
        assert_eq!(
            format!("{}", SemVerSelector::new("unknown")),
            "       ╭───────╮       \n MAJOR │ MINOR │ PATCH \n       ╰───────╯       "
        );
    }
}
