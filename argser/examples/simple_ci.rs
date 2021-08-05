use argser::argser;

#[argser]
#[derive(Debug)]
struct CLI {
    name: String,
    #[argser(subcategory, rename("other_name"))]
    sub: SubCategory,
    #[argser(default)]
    optional: Option<String>,
    #[argser(default_func(default_names))]
    names: Vec<String>,
    #[argser(map)]
    con_map: u16,
}

#[argser]
#[derive(Debug)]
struct SubCategory {
    other: String,
}

fn default_names() -> Vec<String> {
    vec!["test_default".to_string()]
}

pub fn main() {
    let cli: CLI = argser::parse_args_from_providers(&[&argser::provider::Cli::new()]).unwrap();

    println!("Cli: {:?}", cli);
}
