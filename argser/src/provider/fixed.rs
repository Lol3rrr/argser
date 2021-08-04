use crate::ArgProvider;

/// This contains a List of Arguments that it will provide to the Parser.
///
/// This is mostly intended for testing to provide a way to make sure
/// that certain Arguments are always present
pub struct Fixed {
    args: Vec<(String, String)>,
}

impl Fixed {
    /// Creates a new Empty List of Fixed-Arguments
    pub fn empty() -> Self {
        Self { args: Vec::new() }
    }

    /// Adds the given Key-Value Pair to the List of Arguments
    pub fn add_arg<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.args.push((key.into(), value.into()));
    }
}

impl ArgProvider for Fixed {
    fn get_args(&self) -> Vec<(String, String)> {
        self.args.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let fixed = Fixed::empty();
        assert_eq!(Vec::<(String, String)>::new(), fixed.get_args());
    }

    #[test]
    fn empty_add_arg() {
        let mut fixed = Fixed::empty();
        assert_eq!(Vec::<(String, String)>::new(), fixed.get_args());

        fixed.add_arg("test-key".to_owned(), "test-value".to_owned());
        assert_eq!(
            vec![("test-key".to_owned(), "test-value".to_owned())],
            fixed.get_args()
        );
    }
}
