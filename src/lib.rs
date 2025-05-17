#![warn(clippy::all)]

use core::{fmt::Display, str::FromStr};

use egui::{Color32, TextBuffer};

// macro_rules! valtext_shcema {
//     ($fn:fn) => {
        
//     };
// }

/// A mutable TextBuffer that will validate it's contents when changed.
/// And check an input before adding it to the text.
///
/// The default validator will simply attempt to parse the text as `T`,
/// but a custom validator function can be provided.
pub struct ValText<T, E> {
    text: String,
    parsed_val: Option<Result<T, E>>,
    #[allow(clippy::type_complexity)]
    value_parser: Box<dyn Fn(&str) -> Result<T, E>>,
    #[allow(clippy::type_complexity)]
    /// Whether a user input should be added to the string at index
    input_validator: Box<dyn Fn(&str, &str, usize) -> bool>,
}

impl<T, E> ValText<T, E> {
    pub fn with_parser(validator: impl Fn(&str) -> Result<T, E> + 'static) -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(validator),
            input_validator: Box::new(|_, _, _| true),
        }
    }

    /// Only chars in `charset` can be input
    pub fn with_parser_fixed_charset(validator: impl Fn(&str) -> Result<T, E> + 'static, charset: &'static [char]) -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(validator),
            input_validator: Box::new(|_, s, _| !s.chars().any(|c| !charset.contains(&c))),
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

impl ValText<Color32, egui::ecolor::ParseHexColorError> {
    pub fn color_hex() -> Self {
        Self {
            text: String::new(),
            parsed_val: Some(Err(egui::ecolor::ParseHexColorError::MissingHash)),
            value_parser: Box::new(|str| {
                Color32::from_hex(str)
            }),
            input_validator: Box::new(|_, s, i| {
                if i == 0 {
                    return s.starts_with('#');
                }
                s.chars().all(|c| c.is_ascii_hexdigit())
            }),
        }
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

impl<T: FromStr> ValText<T, T::Err> {
    /// Only allows (0,1,2,3,4,5,6,7,8,9,.) and (-,+) at the beginning
    pub fn number() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                str.parse()
            }),
            input_validator: Box::new(|current_text, s, i| {
                let current_has_no_dot = !current_text.contains('.');
                (if i == 0 {
                    s.starts_with('+') || s.starts_with('-')
                } else { false })
                || s.chars().all(|c| {
                    (if current_has_no_dot { c == '.' } else { false })
                    || c.is_ascii_digit()
                })
            })
        }
    }

    /// Only allows (0,1,2,3,4,5,6,7,8,9) and (-,+) at the beginning
    pub fn number_int() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                str.parse()
            }),
            input_validator: Box::new(|_, s, i| {
                (if i == 0 {
                    s.starts_with('+') || s.starts_with('-')
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
                str.parse()
            }),
            input_validator: Box::new(|_, s, i| {
                (if i == 0 {
                    s.starts_with('+')
                } else { false })
                || s.chars().all(|c| c.is_ascii_digit())
            })
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
            text: Default::default(),
            parsed_val: Default::default(),
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
