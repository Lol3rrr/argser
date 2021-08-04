# Argser
A library to handle configuration for Programs

## Examples
### Simple Use-Case
* `name`: The Name
#### Code
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

### Using Subcategories
* `name`: The Name
* `con.domain`: The Domain
* `con.port`: The Port
#### Code
```rust no_run
use argser::argser;

#[argser]
struct Options {
	name: String,
	#[argser(subcategory)]
	con: Connection,
}

#[argser]
struct Connection {
	domain: String,
	port: u16,
}

fn main() {
  let opts: Options = argser::parse_cli().unwrap();

  println!("Hello {}", opts.name);
	println!("Connecting to {}:{}", opts.con.domain, opts.con.port);
}
```
