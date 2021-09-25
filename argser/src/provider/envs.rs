use crate::ArgProvider;

/// The ArgProvider for collecting and using Environment-Variables
pub struct Env {
    to_lowercase: bool,
}

impl Env {
    /// Creates a new Instance of the ArgProvider with the given Configuration
    pub fn new(to_lowercase: bool) -> Self {
        Self { to_lowercase }
    }

    /// Whether or not the Names of the Environment-Variables are converted to
    /// lowercase before being passed on
    pub fn converts_to_lowercase(&self) -> bool {
        self.to_lowercase
    }

    /// Updates the `to_lowercase` Option for this Instance
    pub fn convert_to_lowercase(&mut self, nvalue: bool) {
        self.to_lowercase = nvalue;
    }

    fn parse<I>(&self, iter: I) -> Vec<(String, String)>
    where
        I: Iterator<Item = (String, String)>,
    {
        if self.to_lowercase {
            iter.map(|(key, value)| (key.to_lowercase(), value))
                .collect()
        } else {
            iter.collect()
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new(false)
    }
}

impl ArgProvider for Env {
    fn get_args(&self) -> Vec<(String, String)> {
        let raw = std::env::vars();

        self.parse(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_to_lowercase() {
        let input = vec![
            ("TEST-1".to_owned(), "value-1".to_owned()),
            ("TEST-2".to_owned(), "value-2".to_owned()),
        ];

        let expected = vec![
            ("test-1".to_owned(), "value-1".to_owned()),
            ("test-2".to_owned(), "value-2".to_owned()),
        ];

        let env = Env::new(true);
        let result = env.parse(input.into_iter());

        assert_eq!(expected, result);
    }

    #[test]
    fn parse_withoiut_to_lowercase() {
        let input = vec![
            ("TEST-1".to_owned(), "value-1".to_owned()),
            ("TEST-2".to_owned(), "value-2".to_owned()),
        ];

        let expected = vec![
            ("TEST-1".to_owned(), "value-1".to_owned()),
            ("TEST-2".to_owned(), "value-2".to_owned()),
        ];

        let env = Env::new(false);
        let result = env.parse(input.into_iter());

        assert_eq!(expected, result);
    }
}
