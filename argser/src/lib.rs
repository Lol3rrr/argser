#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use std::collections::HashMap;

pub use argser_macros::argser;

pub mod provider;

mod traits;
pub use traits::*;

/// The Error returned when attempting to Parse the Arguments
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// Some Paramether was missing
    MissingParam {
        /// The Name of the Missing-Parameter
        name: String,
    },
    /// This indicates that there was no Data to be used for parsing
    MissingValue,
    /// This simply indicates that the supplied value was malformed/in an
    /// incorrect format
    InvalidValue,
    /// Some Custom-Error returned by custom implementations for [`ParseFromArgs`]
    Custom(String),
}

/// Defines the interface that needs to be implemented by Argument-Providers,
/// this enables users to source the arguments that should be used from
/// different Parts, like CLI-Args, Environment-Variables, etc.
pub trait ArgProvider {
    /// Get the list of Argument-Pairs from the given Argument-Provider
    fn get_args(&self) -> Vec<(String, String)>;
}

// TODO
// Support command structures like -test.{name}.test
// struct Cli {
//   test: HashMap<String, SubCategory>,
// }
// struct SubCategory {
//   test: String,
// }

/// This will load all the Arguments from the given Providers and then attempt
/// to parse an instance of `T` from that Collection of Arguments
pub fn parse_args_from_providers<T>(providers: &[&dyn ArgProvider]) -> Result<T, ParseError>
where
    T: FromArgs,
{
    let all_args: HashMap<String, Vec<String>> = {
        let mut tmp: HashMap<String, Vec<String>> = HashMap::new();
        for inner_vec in providers.iter().map(|p| p.get_args()) {
            for (key, value) in inner_vec {
                match tmp.get_mut(&key) {
                    Some(previous) => {
                        previous.push(value);
                    }
                    None => {
                        tmp.insert(key, vec![value]);
                    }
                };
            }
        }
        tmp
    };

    T::parse(all_args)
}

/// This is a simple Wrapper for [`parse_args_from_providers`] that
/// automatically uses the [`Cli`](provider::Cli) and [`Env`](provider::Env)
/// ArgProvider to collect Arguments and then Parse them
pub fn parse_cli<T>() -> Result<T, ParseError>
where
    T: FromArgs,
{
    let cli = provider::Cli::default();
    let env = provider::Env::default();

    parse_args_from_providers(&[&cli, &env])
}
