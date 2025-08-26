use std::io::{self, Write, stdout};

/// get user's input. return String.
///
/// ## how to get input without message.
/// ```
/// let input = get_input("");
/// ```
///
/// ## how to get input with message.
/// ```
/// let input = get_input("please title>");
/// ```
///
/// ```bash
/// // 👇console
/// please title>foo 👈foo is user's input.
/// // input == "foo"
/// ```
pub fn get_input<S: AsRef<str>>(message: S) -> Result<String, io::Error> {
    print!("{}", message.as_ref());
    stdout().flush()?;
    let mut word = String::new();
    std::io::stdin().read_line(&mut word)?;
    Ok(word.trim().to_string())
}

/// if user's input is "y" or "yes", return true.
pub fn yes_no<S: AsRef<str>>(message: S) -> bool {
    let input = match get_input(message.as_ref()) {
        Ok(v) => v,
        Err(_) => return false,
    };
    input.is_empty() || matches!(input.as_ref(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        assert_eq!(get_input("hello?>").unwrap(), "hoge".to_string());
    }

    #[test]
    fn yes_no_test() {
        println!("{}", yes_no("message"));
    }
}
