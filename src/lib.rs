#![warn(clippy::all)]

use core::{fmt::Display, str::FromStr};

use egui::{Color32, TextBuffer};

/// A mutable TextBuffer that will validate it's contents when changed.
/// And check an input before adding it to the text.
///
/// The default validator will simply attempt to parse the text as `T`,
/// but a custom validator function can be provided.
pub struct ValText<T> {
    text: String,
    parsed_val: Option<T>,
    #[allow(clippy::type_complexity)]
    value_parser: Box<dyn Fn(&str) -> Option<T>>,
    #[allow(clippy::type_complexity)]
    /// Whether a user input should be added to the string at index
    input_validator: Box<dyn Fn(&str, usize) -> bool>,
}

impl<T> ValText<T> {
    pub fn with_parser(validator: impl Fn(&str) -> Option<T> + 'static) -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(validator),
            input_validator: Box::new(|_, _| true),
        }
    }

    pub const fn get_val(&self) -> Option<&T> {
        self.parsed_val.as_ref()
    }

    pub const fn is_valid(&self) -> bool {
        self.parsed_val.is_some()
    }
}

impl ValText<Color32> {
    pub fn color_hex() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                Color32::from_hex(str).ok()
            }),
            input_validator: Box::new(|s, i| {
                if i == 0 {
                    return s.starts_with('#');
                }
                s.chars().all(|c| c.is_ascii_hexdigit())
            }),
        }
    }
}

impl<T: FromStr> ValText<Option<T>> {
    pub fn option_parse() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                if str.is_empty() {
                    Some(None)
                } else {
                    str.parse::<T>().map(|t| Some(t)).ok()
                }
            }),
            input_validator: Box::new(|_, _| true),
        }
    }
}

impl<T: FromStr> ValText<T> {
    /// Only allows (0,1,2,3,4,5,6,7,8,9,.) and (-,+) at the beginning
    pub fn number() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                str.parse().ok()
            }),
            input_validator: Box::new(|s, i| {
                (if i == 0 {
                    s.as_bytes()[0] == b'+' || s.as_bytes()[0] == b'-'
                } else { false })
                || s.chars().all(|c| c == '.' || c.is_ascii_digit())
            })
        }
    }

    /// Only allows (0,1,2,3,4,5,6,7,8,9) and (-,+) at the beginning
    pub fn number_int() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                str.parse().ok()
            }),
            input_validator: Box::new(|s, i| {
                (if i == 0 {
                    s.as_bytes()[0] == b'+' || s.as_bytes()[0] == b'-'
                } else { false })
                || s.chars().all(|c| c.is_ascii_digit())
            })
        }
    }

    /// Only allows (0,1,2,3,4,5,6,7,8,9) and (+) at the beginning
    pub fn number_uint() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                str.parse().ok()
            }),
            input_validator: Box::new(|s, i| {
                (if i == 0 {
                    s.as_bytes()[0] == b'+'
                } else { false })
                || s.chars().all(|c| c.is_ascii_digit())
            })
        }
    }
}

impl<T: Display> ValText<T> {
    pub fn set_val(&mut self, val: T) {
        self.text = val.to_string();
        self.parsed_val = Some(val);
    }
}

impl<T: FromStr> Default for ValText<T> {
    /// Parse the text using `FromStr`
    fn default() -> Self {
        Self {
            text: Default::default(),
            parsed_val: Default::default(),
            value_parser: Box::new(|text| text.parse().ok()),
            input_validator: Box::new(|_, _| true),
        }
    }
}

impl<T> TextBuffer for ValText<T> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if (self.input_validator)(text, char_index) {
            let n = self.text.insert_text(text, char_index);
            self.parsed_val = (self.value_parser)(&self.text);
            n
        } else {
            0
        }
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        self.text.delete_char_range(char_range);
        self.parsed_val = (self.value_parser)(&self.text);
    }
}
