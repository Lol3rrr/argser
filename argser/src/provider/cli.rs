use crate::ArgProvider;

/// This ArgProvider collects all the CLI-Arguments passed to the Program
///
/// # Accepts
/// This Provider accepts all the CLI-Flags that are in one of these Formats:
/// * "-{name} {value}"
/// * "-{name}={value}"
pub struct Cli {}

impl Cli {
    /// Creates a new Instance of the ArgProvider
    pub fn new() -> Self {
        Self {}
    }

    fn parse_vars<I>(mut iter: I) -> Vec<(String, String)>
    where
        I: Iterator<Item = String>,
    {
        let mut result = Vec::new();

        loop {
            let item = match iter.next() {
                Some(i) => i,
                None => break,
            };

            if !item.starts_with('-') {
                continue;
            }

            let item = item.trim_start_matches('-');

            match item.split_once('=') {
                Some((key, value)) => {
                    result.push((key.to_string(), value.to_string()));
                    continue;
                }
                None => {
                    let key = item;

                    let value = match iter.next() {
                        Some(v) => v,
                        None => break,
                    };

                    result.push((key.to_string(), value.to_string()));
                    continue;
                }
            };
        }

        result
    }
}
impl ArgProvider for Cli {
    fn get_args(&self) -> Vec<(String, String)> {
        let vars = std::env::args();

        Self::parse_vars(vars)
    }
}
impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vars_1_pair() {
        let input = vec!["-p".to_owned(), "8080".to_owned()];

        let result = Cli::parse_vars(input.into_iter());
        let expected = vec![("p".to_owned(), "8080".to_owned())];

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_vars_single_pair() {
        let input = vec!["-p=8080".to_owned()];

        let result = Cli::parse_vars(input.into_iter());
        let expected = vec![("p".to_owned(), "8080".to_owned())];

        assert_eq!(expected, result);
    }
}
