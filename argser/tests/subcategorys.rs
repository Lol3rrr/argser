use argser::{argser, FromArgs};

#[test]
fn one_subcategory() {
    #[argser]
    #[derive(Debug, PartialEq)]
    struct Options {
        name: String,
        #[argser(subcategory)]
        con: Con,
    }

    #[argser]
    #[derive(Debug, PartialEq)]
    struct Con {
        port: u16,
    }

    let fixed_provider = {
        let mut tmp = argser::provider::Fixed::empty();

        tmp.add_arg("name", "test-name");
        tmp.add_arg("con.port", "123");

        tmp
    };

    let expected = Options {
        name: "test-name".to_owned(),
        con: Con { port: 123 },
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
            name: "con.port".to_owned(),
            required: true,
            description: "".to_owned(),
        },
    ];
    assert_eq!(expected_args, Options::arguments());
}
