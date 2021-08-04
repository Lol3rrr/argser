# Argser
A library to handle configuration for Programs

## Examples
```rust no_run
use argser::argser;

#[argser]
struct Options {
    name: String,
}

fn main() {
    let opts: Options = argser::parse_cli().unwrap();

    println!("Hello {}", opts.name);
}
```
