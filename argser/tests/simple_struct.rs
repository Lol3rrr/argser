use argser::{argser, FromArgs};

#[test]
fn primitives() {
    #[argser]
    #[derive(Debug, PartialEq)]
    struct Options {
        name: String,
        port: u16,
    }

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();

        tmp.add_arg("name", "test-name");
        tmp.add_arg("port", "123");

        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        port: 123,
    };
    assert_eq!(
        Ok(expected),
        argser::parse_args_from_providers(&[&fixed_provider])
    );
    let expected_args = vec![
        argser::ArgumentDetail {
            name: "name".to_owned(),
            required: true,
            description: "".to_owned(),
        },
        argser::ArgumentDetail {
            name: "port".to_owned(),
            required: true,
            description: "".to_owned(),
        },
    ];
    assert_eq!(expected_args, Options::arguments());
}

#[test]
fn primitives_with_default_func() {
    #[argser]
    #[derive(Debug, PartialEq)]
    struct Options {
        name: String,
        #[argser(default_func(default_port))]
        port: u16,
    }

    fn default_port() -> u16 {
        10
    }

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();
        tmp.add_arg("name", "test-name");
        tmp.add_arg("port", "123");
        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        port: 123,
    };
    assert_eq!(
        Ok(expected),
        argser::parse_args_from_providers(&[&fixed_provider])
    );

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();
        tmp.add_arg("name", "test-name");
        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        port: 10,
    };
    assert_eq!(
        Ok(expected),
        argser::parse_args_from_providers(&[&fixed_provider])
    );

    let expected_args = vec![
        argser::ArgumentDetail {
            name: "name".to_owned(),
            required: true,
            description: "".to_owned(),
        },
        argser::ArgumentDetail {
            name: "port".to_owned(),
            required: false,
            description: "".to_owned(),
        },
    ];
    assert_eq!(expected_args, Options::arguments());
}

#[test]
fn primitives_with_default() {
    #[argser]
    #[derive(Debug, PartialEq)]
    struct Options {
        name: String,
        #[argser(default)]
        port: Option<u16>,
    }

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();
        tmp.add_arg("name", "test-name");
        tmp.add_arg("port", "123");
        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        port: Some(123),
    };
    assert_eq!(
        Ok(expected),
        argser::parse_args_from_providers(&[&fixed_provider])
    );

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();
        tmp.add_arg("name", "test-name");
        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        port: None,
    };
    assert_eq!(
        Ok(expected),
        argser::parse_args_from_providers(&[&fixed_provider])
    );

    let expected_args = vec![
        argser::ArgumentDetail {
            name: "name".to_owned(),
            required: true,
            description: "".to_owned(),
        },
        argser::ArgumentDetail {
            name: "port".to_owned(),
            required: false,
            description: "".to_owned(),
        },
    ];
    assert_eq!(expected_args, Options::arguments());
}
