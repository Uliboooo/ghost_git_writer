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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SemVerPart {
    Major,
    Minor,
    Patch,
}

impl SemVerPart {
    fn label(self) -> &'static str {
        match self {
            SemVerPart::Major => "MAJOR",
            SemVerPart::Minor => "MINOR",
            SemVerPart::Patch => "PATCH",
        }
    }
}

impl std::str::FromStr for SemVerPart {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "major" => Ok(SemVerPart::Major),
            "minor" => Ok(SemVerPart::Minor),
            "patch" => Ok(SemVerPart::Patch),
            other => Err(format!("unknown SemVer part: {other}")),
        }
    }
}

pub struct SemVerSelector {
    selected: SemVerPart,
}

impl SemVerSelector {
    pub fn new(selected: SemVerPart) -> Self {
        Self { selected }
    }
}

impl Display for SemVerSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts = [SemVerPart::Major, SemVerPart::Minor, SemVerPart::Patch];
        let mut top = String::new();
        let mut mid = String::new();
        let mut bot = String::new();

        for (i, &part) in parts.iter().enumerate() {
            let label = part.label();
            let w = get_str_len(label);
            let is_sel = part == self.selected;

            // Add a column separator before this item (except for the first).
            // Skip when one of the adjacent items is selected — the box wall serves as the separator.
            if i > 0 && !(parts[i - 1] == self.selected) && !is_sel {
                top.push(' ');
                mid.push('│');
                bot.push(' ');
            }

            if is_sel {
                top.push_str(&format!("╭{}╮", mul_str(&"─", w + 2)));
                mid.push_str(&format!("│ {label} │"));
                bot.push_str(&format!("╰{}╯", mul_str(&"─", w + 2)));
            } else {
                top.push_str(&mul_str(&" ", w + 2));
                mid.push_str(&format!(" {label} "));
                bot.push_str(&mul_str(&" ", w + 2));
            }
        }

        write!(f, "{top}\n{mid}\n{bot}")
    }
}

#[cfg(test)]
mod tests {
    use crate::cli_helper::{Printer, SemVerPart, SemVerSelector, mul_str};

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
    fn sem_ver_part_from_str_test() {
        assert_eq!("major".parse::<SemVerPart>(), Ok(SemVerPart::Major));
        assert_eq!("Minor".parse::<SemVerPart>(), Ok(SemVerPart::Minor));
        assert_eq!("PATCH".parse::<SemVerPart>(), Ok(SemVerPart::Patch));
        assert!("unknown".parse::<SemVerPart>().is_err());
    }

    #[test]
    fn sem_ver_selector_test() {
        // MAJOR selected (first item)
        assert_eq!(
            format!("{}", SemVerSelector::new(SemVerPart::Major)),
            "╭───────╮               \n│ MAJOR │ MINOR │ PATCH \n╰───────╯               "
        );
        // MINOR selected (middle item)
        assert_eq!(
            format!("{}", SemVerSelector::new(SemVerPart::Minor)),
            "       ╭───────╮       \n MAJOR │ MINOR │ PATCH \n       ╰───────╯       "
        );
        // PATCH selected (last item)
        assert_eq!(
            format!("{}", SemVerSelector::new(SemVerPart::Patch)),
            "               ╭───────╮\n MAJOR │ MINOR │ PATCH │\n               ╰───────╯"
        );
    }
}
