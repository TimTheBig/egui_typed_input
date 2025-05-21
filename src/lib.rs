#![warn(clippy::all)]
#![allow(clippy::type_complexity)]

use core::{fmt::Display, str::FromStr};
use egui::TextBuffer;

mod impls;

/// A mutable `TextBuffer` that will validate it's contents when changed.\
/// And check an input before adding it to the text.
///
/// The default validator will simply attempt to parse the text as `T`,
/// but a custom validator function can be provided.
///
/// ## Usage
/// ```
/// use egui_typed_input::ValText;
///
/// # fn main() {
/// let mut alphabetical_order: ValText<Vec<char>, ()> = ValText::new(
///     // parser
///     (|str| Ok(str.chars().collect::<Vec<_>>())),
///     // input validator
///     (|current_text, input, index| {
///         if input.chars().all(|c| c.is_ascii_alphabetic()) {
///             input.chars().all(|c| {
///                 c.to_ascii_lowercase() >= current_text.chars().skip(index.saturating_sub(1)).take(1).last().unwrap_or('a')
///             })
///         } else { false }
///     }),
/// );
///
/// # eframe::run_simple_native(
/// #    "number input",
/// #    eframe::NativeOptions::default(),
/// #    move |ctx, _frame| {
/// #        egui::CentralPanel::default().show(ctx, |ui| {
/// ui.text_edit_singleline(&mut alphabetical_order);
/// println!("alphabetical_order: {:?}", alphabetical_order.get_val());
/// #        });
/// #    }
/// # ).unwrap();
/// # }
/// ```
/// See hex color example (color_hex.rs) and number examples (number.rs) for more
pub struct ValText<T, E> {
    text: String,
    parsed_val: Option<Result<T, E>>,
    value_parser: Box<dyn Fn(&str) -> Result<T, E>>,
    /// Whether a user input should be added to the string at index
    ///
    /// The signature is `(current_text, input, insertion_index) -> should_add_to_text`
    /// 
    /// Note: insertion_index is a character index, not a byte index.
    input_validator: Box<dyn Fn(&str, &str, usize) -> bool>,
}

impl<T, E> ValText<T, E> {
    #[must_use]
    pub fn new(value_parser: impl Fn(&str) -> Result<T, E> + 'static, input_validator: impl Fn(&str, &str, usize) -> bool + 'static) -> Self {
        ValText {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(value_parser),
            input_validator: Box::new(input_validator),
        }
    }

    #[must_use]
    pub fn new_box(value_parser: Box<dyn Fn(&str) -> Result<T, E>>, input_validator: Box<dyn Fn(&str, &str, usize) -> bool>) -> Self {
        ValText {
            text: String::new(),
            parsed_val: None,
            value_parser,
            input_validator,
        }
    }

    #[must_use]
    pub fn with_parser(validator: impl Fn(&str) -> Result<T, E> + 'static) -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(validator),
            input_validator: Box::new(|_, _, _| true),
        }
    }

    /// Only chars in `charset` can be input
    /// ## Usage
    /// ```
    /// # use egui_typed_input::ValText;
    /// # let _: ValText<_, ()> =
    /// ValText::with_parser_fixed_charset(|str| Ok(str.to_owned()), &['a', 'c']);
    /// ```
    /// Would allow 'a' and 'c' but no others.
    #[must_use]
    pub fn with_parser_fixed_charset(parser: impl Fn(&str) -> Result<T, E> + 'static, charset: &'static [char]) -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(parser),
            input_validator: Box::new(|_, s, _| s.chars().all(|c| charset.contains(&c))),
        }
    }

    /// `ValText` must be used before getting value
    pub const fn get_val(&self) -> Option<Result<&T, &E>> {
        match self.parsed_val.as_ref() {
            Some(res) => Some(res.as_ref()),
            None => None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.parsed_val.as_ref().map(|res| res.is_ok()).unwrap_or(false)
    }
}

impl<T: FromStr> ValText<Option<T>, T::Err> {
    pub fn option_parse() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                if str.is_empty() {
                    Ok(None)
                } else {
                    str.parse::<T>().map(|t| Some(t))
                }
            }),
            input_validator: Box::new(|_, _, _| true),
        }
    }
}

impl<T: Display, E> ValText<T, E> {
    pub fn set_val(&mut self, val: T) {
        self.text = val.to_string();
        self.parsed_val = Some(Ok(val));
    }
}

impl<T: FromStr> Default for ValText<T, T::Err> {
    /// Parse the text using `FromStr`
    fn default() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|text| text.parse()),
            input_validator: Box::new(|_, _, _| true),
        }
    }
}

impl<T, E> TextBuffer for ValText<T, E> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if (self.input_validator)(&self.text, text, char_index) {
            let n = self.text.insert_text(text, char_index);
            self.parsed_val = Some((self.value_parser)(&self.text));
            n
        } else {
            0
        }
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.text.delete_char_range(char_range);
        self.parsed_val = Some((self.value_parser)(&self.text));
    }
}
