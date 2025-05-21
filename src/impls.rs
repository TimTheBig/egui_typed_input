use core::str::FromStr;
use egui::Color32;

use crate::ValText;

impl ValText<Color32, egui::ecolor::ParseHexColorError> {
    /// A hex color starting with `#`, parsed using [`Color32::from_hex`].\
    /// Supports the 3, 4, 6, and 8-digit formats.
    #[must_use]
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

impl<T: FromStr> ValText<T, T::Err> {
    /// Only allows (0,1,2,3,4,5,6,7,8,9,.) and (-,+) at the beginning
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

#[derive(Debug, thiserror::Error)]
pub enum PercentageParseError {
    /// > 100
    #[error("number is more then 100")]
    OutOfRangeHigh,
    /// < 0
    #[error("number is less then 0")]
    Neg,
    #[error(transparent)]
    ParseFloat(#[from] core::num::ParseFloatError),
    #[error(transparent)]
    ParseInt(#[from] core::num::ParseIntError),
}

impl ValText<f64, PercentageParseError> {
    // todo unit test
    /// A numarical percentage in the range of 0-100.\
    /// Only allows (0,1,2,3,4,5,6,7,8,9,.) and (+) at the beginning
    #[must_use]
    pub fn percentage() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                let num = str.parse();
                match num {
                    Ok(num) => {
                        if num > 100.0 {
                            Err(PercentageParseError::OutOfRangeHigh)
                        } else if num < 0.0 {
                            Err(PercentageParseError::Neg)
                        } else {
                            Ok(num)
                        }
                    },
                    Err(e) => Err(e.into()),
                }
            }),
            input_validator: Box::new(|current_text, s, i| {
                let current_text_no_des_len = current_text.split_once('.')
                    .map(|(pre_dot, _)| pre_dot.len())
                    .unwrap_or(current_text.len());
                if current_text_no_des_len + s.len() > 3 && !current_text.contains('.') { return false; }

                let current_has_no_dot = !current_text.contains('.');
                let all_num_or_dot = s.chars().all(|c| {
                    (if current_has_no_dot { c == '.' } else { false })
                    || c.is_ascii_digit()
                });

                if !current_text.is_empty() && current_text.as_bytes()[i.saturating_sub(1)] == b'.' && all_num_or_dot {
                    return true;
                }

                // only allow therd char if others are 00
                if current_text_no_des_len == 2 {
                    if s.starts_with('.') && all_num_or_dot {
                        return true;
                    } else if s == "0" {
                        return current_text.starts_with("10") && !current_text.contains('.');
                    }
                    return false;
                }

                (if i == 0 {
                    s.starts_with('+')
                } else { false })
                || all_num_or_dot
            })
        }
    }
}

impl ValText<f32, PercentageParseError> {
    /// A numarical percentage in the range of 0-100.\
    /// Only allows (0,1,2,3,4,5,6,7,8,9,.) and (+) at the beginning
    #[must_use]
    pub fn percentage() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                let num = str.parse();
                match num {
                    Ok(num) => {
                        if num > 100.0 {
                            Err(PercentageParseError::OutOfRangeHigh)
                        } else if num < 0.0 {
                            Err(PercentageParseError::Neg)
                        } else {
                            Ok(num)
                        }
                    },
                    Err(e) => Err(e.into()),
                }
            }),
            input_validator: Box::new(|current_text, s, i| {
                let current_text_no_des_len = current_text.split_once('.')
                    .map(|(pre_dot, _)| pre_dot.len())
                    .unwrap_or(current_text.len());
                if current_text_no_des_len + s.len() > 3 && !current_text.contains('.') { return false; }
                
                let current_has_no_dot = !current_text.contains('.');
                let all_num_or_dot = s.chars().all(|c| {
                    (if current_has_no_dot { c == '.' } else { false })
                    || c.is_ascii_digit()
                });

                if !current_text.is_empty() && current_text.as_bytes()[i.saturating_sub(1)] == b'.' && all_num_or_dot {
                    return true;
                }

                // only allow therd char if others are 00
                if current_text_no_des_len == 2 {
                    if s.starts_with('.') && all_num_or_dot {
                        return true;
                    } else if s == "0" {
                        return current_text.starts_with("10") && !current_text.contains('.');
                    }
                    return false;
                }

                (if i == 0 {
                    s.starts_with('+')
                } else { false })
                || all_num_or_dot
            })
        }
    }
}

impl ValText<u32, PercentageParseError> {
    /// A numarical percentage in the range of 0-100.\
    /// Only allows (0,1,2,3,4,5,6,7,8,9) and (+) at the beginning
    #[must_use]
    pub fn percentage_uint() -> Self {
        Self {
            text: String::new(),
            parsed_val: None,
            value_parser: Box::new(|str| {
                let num = str.parse();
                match num {
                    Ok(num) => {
                        if num > 100 {
                            Err(PercentageParseError::OutOfRangeHigh)
                        } else {
                            Ok(num)
                        }
                    },
                    Err(e) => Err(e.into()),
                }
            }),
            input_validator: Box::new(|current_text, s, i| {
                if current_text.len() + s.len() > 3 { return false; }

                // only allow therd char if others are 00
                if current_text.len() == 2 {
                    if s == "0" {
                        return current_text.starts_with("10");
                    }
                    return false;
                }

                (if i == 0 {
                    s.starts_with('+')
                } else { false })
                || s.chars().all(|c| c.is_ascii_digit())
            })
        }
    }
}
