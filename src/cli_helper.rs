use indicatif::{ProgressBar, ProgressStyle};
// use unicode_width::UnicodeWidthChar;
use std::{fmt::Display, time::Duration};

pub enum Error {
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

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

// fn get_str_len<T: AsRef<str>>(strg: T) -> u32{
//     let mut len = 0;
//     for c in strg.as_ref().chars() {
//         len += c.width().unwrap_or(0);
//     }
//     len as u32
// }
//
// fn mut_str<T: AsRef<str>, U: Into<u32>>(msg: &T, mul: U) -> String {
//     let mut res = String::new();
//     for _ in 0..mul.into() {
//         res.push_str(msg.as_ref());
//     }
//     res
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::cli_helper::mut_str;
//
//     #[test]
//     fn str_mul_test() {
//         let c = "hi";
//         let res = mut_str(&c, 10);
//         assert_eq!("hihihihihihihihihihi".to_string(), res);
//     }
// }
