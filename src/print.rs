use std::fmt::Display;

use colored::{ColoredString, Colorize};

pub const DEFAULT_PADDING: usize = 10;
pub const SEPARATOR_COUNT: usize = 60;
pub const SEPARATOR_DASH: &str = "─";
pub const SEPARATOR_SPACE: &str = " ";

pub trait PadAlign {
    /// Align all string tuples in `lines` with a minimum of `padding` between them.
    ///
    /// # Example
    /// ```rust
    /// let lines = pad_align(
    ///     &[
    ///         ("This is the first line:", "Success"),
    ///         ("Second line:", "Fail"),
    ///         ("Third line incoming:", "Neutral"),
    ///     ],
    ///     5,
    /// );
    ///
    /// println!("{}", lines);
    ///
    /// ```
    /// This will print:
    /// ```text
    /// This is the first line:     Success
    /// Second line:                Fail
    /// Third line incoming:        Neutral
    /// ```
    fn pad_align(&self, padding: usize) -> String;

    /// Same as `pad_align(&self, padding: usize)` with a predefined padding
    /// of `DEFAULT_PADDING`.
    fn pad_align_default(&self) -> String;
}

impl PadAlign for [(&str, Box<dyn Display>)] {
    fn pad_align(&self, padding: usize) -> String {
        let left_max = self.iter().map(|(lhs, _)| lhs.len()).max().unwrap_or(0);

        self.iter()
            .map(|(lhs, rhs)| {
                let total_width = left_max + padding;
                format!("{lhs:<total_width$}{rhs}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn pad_align_default(&self) -> String {
        self.pad_align(DEFAULT_PADDING)
    }
}

/// Stringifies an `Option<T>`, representing `None` as the string `"None"`.
pub fn string_option<T: Display>(opt: Option<T>) -> String {
    match opt {
        Some(display_value) => display_value.to_string(),
        None => "None".to_string(),
    }
}

pub fn print_header<B, T>(text: &str, border_styler: B, text_styler: T)
where
    B: Fn(&str) -> ColoredString,
    T: Fn(&str) -> ColoredString,
{
    let trimmed = text.trim();

    let spacer = SEPARATOR_SPACE.repeat(SEPARATOR_COUNT);
    let spaced_text = format!(" {trimmed} ");
    let header_line = format!("{spaced_text: ^SEPARATOR_COUNT$}");

    let styled_spacer = border_styler(&spacer);
    let styled_text = text_styler(&header_line);

    println!("{styled_spacer}");
    println!("{styled_text}");
    println!("{styled_spacer}");
    println!();
}

/// Styles an `Option<T>` into a `ColoredString` using custom closures.
///
/// # Arguments
///
/// * `opt`: The `Option<T>` to style.
/// * `some_styler`: A closure that styles the `Some(T)` value.
/// * `none_styler`: A closure that provides the styled output for the `None` case.
pub fn styled_option<T, S, N>(opt: Option<T>, some_styler: S, none_styler: N) -> ColoredString
where
    T: Display,
    S: Fn(&str) -> ColoredString,
    N: Fn() -> ColoredString,
{
    match opt {
        Some(display_value) => {
            let s = display_value.to_string();
            some_styler(&s)
        }
        None => none_styler(),
    }
}

/// Creates a list (`Vec`) of `(&str, Box<dyn Display>)` tuples that can be
/// pretty-printed using the `PadAlign` trait.
///
/// This macro handles the repetitive `Box::new()` wrapping.
///
/// # Example
/// ```rust
/// use print::{pretty_list, PadAlign};
///
/// let lines = pretty_list![
///     ("Name", "Arthur Dent"),
///     ("Answer", 42),
///     ("Towel", true),
/// ];
///
/// println!("{}", lines.pad_align(5));
/// ```
#[macro_export]
macro_rules! pretty_list {
    ($(($key:expr, $value:expr)),* $(,)?) => {
        {
       vec![
            $(
                ($key, Box::new($value) as Box<dyn Display>)
            ),*
        ]
        }
    };
}
