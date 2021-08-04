use crate::ArgProvider;

/// This ArgProvider collects all the CLI-Arguments passed to the Program
pub struct Cli {}

impl Cli {
    /// Creates a new Instance of the ArgProvider
    pub fn new() -> Self {
        Self {}
    }

    fn parse_vars<I>(iter: I) -> Vec<(String, String)>
    where
        I: Iterator<Item = String>,
    {
        let mut result = Vec::new();

        let mut key: Option<String> = None;
        for item in iter {
            if item.chars().nth(0).unwrap() == '-' {
                let item = item.trim_start_matches('-');

                match item.find("=") {
                    Some(split_index) => {
                        let (first, second) = item.split_at(split_index);
                        result.push((first.to_owned(), second[1..].to_owned()));
                    }
                    None => {
                        key = Some(item.to_owned());
                    }
                };
            } else {
                match key.take() {
                    Some(key_val) => {
                        result.push((key_val, item));
                    }
                    None => {}
                };
            }
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
