use std::collections::HashMap;

use argser::{argser, FromArgs};

#[test]
fn map_primitive() {
    #[argser]
    #[derive(Debug, PartialEq)]
    struct Options {
        name: String,
        #[argser(map)]
        con: u16,
    }

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();

        tmp.add_arg("name", "test-name");
        tmp.add_arg("con.port", "123");
        tmp.add_arg("con.test", "234");

        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        con: {
            let mut tmp = HashMap::new();
            tmp.insert("port".to_string(), 123u16);
            tmp.insert("test".to_string(), 234u16);
            tmp
        },
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
            name: "con.{name}".to_owned(),
            required: false,
            description: "".to_owned(),
        },
    ];
    assert_eq!(expected_args, Options::arguments());
}

#[test]
fn map_subcategory() {
    #[argser]
    #[derive(Debug, PartialEq)]
    struct Options {
        name: String,
        #[argser(map(subcategory))]
        con: Con,
    }

    #[argser]
    #[derive(Debug, PartialEq)]
    struct Con {
        ip: String,
        port: u16,
    }

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();

        tmp.add_arg("name", "test-name");
        tmp.add_arg("con.test1.ip", "example.com");
        tmp.add_arg("con.test1.port", "123");
        tmp.add_arg("con.test2.ip", "example.com");
        tmp.add_arg("con.test2.port", "234");

        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        con: {
            let mut tmp = HashMap::new();
            tmp.insert(
                "test1".to_owned(),
                Con {
                    ip: "example.com".to_owned(),
                    port: 123,
                },
            );
            tmp.insert(
                "test2".to_owned(),
                Con {
                    ip: "example.com".to_owned(),
                    port: 234,
                },
            );
            tmp
        },
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
            name: "con.{name}.ip".to_owned(),
            required: false,
            description: "".to_owned(),
        },
        argser::ArgumentDetail {
            name: "con.{name}.port".to_owned(),
            required: false,
            description: "".to_owned(),
        },
    ];
    assert_eq!(expected_args, Options::arguments());
}
