use std::collections::HashMap;

use crate::ParseError;

/// Defines the Interface to parse a Collection of provided Arguments into a
/// single concrete Struct.
///
/// This Trait will most likely be automatically implemented for you by using
/// the [`argser`](crate::argser)-Attribute-Macro
pub trait FromArgs
where
    Self: Sized,
{
    /// Parses the given Collection of Arguments into a valid instance of Self
    fn parse(args: HashMap<String, Vec<String>>) -> Result<Self, ParseError>;
}

/// Defines the interface to parse a List-Argument Values into single Conecrete
/// Value for a Field in a CLI-Struct
pub trait ParseFromArgs
where
    Self: Sized,
{
    /// Parses the given Raw-Values into a single Value of the Type
    fn parse(value: Vec<String>) -> Result<Self, ParseError>;

    /// Parses the given Raw-Values using [`ParseFromArgs::parse`] or if
    /// that fails for whatever reason, it will fallback to using the
    /// default_func
    fn parse_with_default_fn<F>(value: Vec<String>, default_func: F) -> Self
    where
        F: Fn() -> Self,
    {
        match ParseFromArgs::parse(value) {
            Ok(v) => v,
            Err(_) => default_func(),
        }
    }

    /// Parses the given Raw-Values using [`ParseFromArgs::parse`] or if that
    /// fails for whatever reason, it will fallback to using [`Default::default`]
    fn parse_with_default(value: Vec<String>) -> Self
    where
        Self: Default,
    {
        match ParseFromArgs::parse(value) {
            Ok(v) => v,
            Err(_) => Default::default(),
        }
    }
}

impl ParseFromArgs for String {
    fn parse(mut value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }
        Ok(value.remove(0))
    }
}
impl ParseFromArgs for u64 {
    fn parse(mut value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }
        match value.remove(0).parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ParseError::InvalidValue),
        }
    }
}
impl ParseFromArgs for u32 {
    fn parse(mut value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }
        match value.remove(0).parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ParseError::InvalidValue),
        }
    }
}
impl ParseFromArgs for u16 {
    fn parse(mut value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }
        match value.remove(0).parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ParseError::InvalidValue),
        }
    }
}
impl ParseFromArgs for u8 {
    fn parse(mut value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }
        match value.remove(0).parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(ParseError::InvalidValue),
        }
    }
}
impl ParseFromArgs for bool {
    fn parse(value: Vec<String>) -> Result<Self, ParseError> {
        match value.get(0) {
            Some(value) if value == "true" => Ok(true),
            Some(value) if value == "false" => Ok(false),
            None => Err(ParseError::MissingValue),
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl<T> ParseFromArgs for Option<T>
where
    T: ParseFromArgs,
{
    fn parse(value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }

        let value = T::parse(value)?;
        Ok(Some(value))
    }
}
impl<T> ParseFromArgs for Vec<T>
where
    T: ParseFromArgs,
{
    fn parse(value: Vec<String>) -> Result<Self, ParseError> {
        if value.len() < 1 {
            return Err(ParseError::MissingValue);
        }

        value
            .into_iter()
            .map(|raw| ParseFromArgs::parse(vec![raw]))
            .filter(|r| r.is_ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_parse() {
        assert_eq!(
            Ok("test_str".to_owned()),
            ParseFromArgs::parse(vec!["test_str".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Result::<String, ParseError>::Err(ParseError::MissingValue),
            ParseFromArgs::parse(Vec::new())
        );
    }

    #[test]
    fn u64_parse() {
        assert_eq!(
            Ok(10u64),
            ParseFromArgs::parse(vec!["10".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Result::<u64, ParseError>::Err(ParseError::InvalidValue),
            ParseFromArgs::parse(vec!["other".to_owned()])
        );
        assert_eq!(
            Result::<u64, ParseError>::Err(ParseError::MissingValue),
            ParseFromArgs::parse(Vec::new())
        );
    }
    #[test]
    fn u32_parse() {
        assert_eq!(
            Ok(10u32),
            ParseFromArgs::parse(vec!["10".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Result::<u32, ParseError>::Err(ParseError::InvalidValue),
            ParseFromArgs::parse(vec!["other".to_owned()])
        );
        assert_eq!(
            Result::<u32, ParseError>::Err(ParseError::MissingValue),
            ParseFromArgs::parse(Vec::new())
        );
    }
    #[test]
    fn u16_parse() {
        assert_eq!(
            Ok(10u16),
            ParseFromArgs::parse(vec!["10".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Result::<u16, ParseError>::Err(ParseError::InvalidValue),
            ParseFromArgs::parse(vec!["other".to_owned()])
        );
        assert_eq!(
            Result::<u16, ParseError>::Err(ParseError::MissingValue),
            ParseFromArgs::parse(Vec::new())
        );
    }
    #[test]
    fn u8_parse() {
        assert_eq!(
            Ok(10u8),
            ParseFromArgs::parse(vec!["10".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Result::<u8, ParseError>::Err(ParseError::InvalidValue),
            ParseFromArgs::parse(vec!["other".to_owned()])
        );
        assert_eq!(
            Result::<u8, ParseError>::Err(ParseError::MissingValue),
            ParseFromArgs::parse(Vec::new())
        );
    }

    #[test]
    fn bool_parse() {
        assert_eq!(
            Ok(true),
            ParseFromArgs::parse(vec!["true".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Ok(false),
            ParseFromArgs::parse(vec!["false".to_owned(), "other".to_owned()])
        );
        assert_eq!(
            Result::<bool, ParseError>::Err(ParseError::InvalidValue),
            ParseFromArgs::parse(vec!["other".to_owned()])
        );
        assert_eq!(
            Result::<bool, ParseError>::Err(ParseError::MissingValue),
            ParseFromArgs::parse(Vec::new())
        );
    }
}
