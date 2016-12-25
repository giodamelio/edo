# Edo
[![Crates.io Version](https://img.shields.io/crates/v/edo.svg)](https://crates.io/crates/edo) [![Build Status](https://travis-ci.org/giodamelio/edo.svg?branch=master)](https://travis-ci.org/giodamelio/edo) [![Dependency Status](https://dependencyci.com/github/giodamelio/edo/badge)](https://dependencyci.com/github/giodamelio/edo)

A super simple templating library for Rust.

[Documentation](https://docs.rs/edo)

# Examples

You can use a simple static replacement.

```rust
use edo::Edo;

let mut template: Edo<&str> = Edo::new("Hello {name}").unwrap();
template.register_static("name", "World!");
let output = template.render("");
assert_eq!(output, "Hello World!");
```

You can also use a handler function to calculate the value.

```rust
use edo::Edo;

let mut template: Edo<&str> = Edo::new("Hello {name}").unwrap();
template.register_handler("name", |_, _| Ok("World!".to_string()));
let output = template.render("");
assert_eq!(output, "Hello World!");
```

Your handlers can also take arguments (As a `Vec<str>`).

```rust
use edo::Edo;

let mut template: Edo<&str> = Edo::new("{say_hello(World)}").unwrap();
template.register_handler("say_hello", |args, _| Ok(format!("Hello {}", args[0])));
let output = template.render("");
assert_eq!(output, "Hello World");
```

Your handlers also take a context argument at render time.

```rust
use edo::Edo;

let mut template: Edo<&str> = Edo::new("{say_hello(World)}").unwrap();
template.register_handler("say_hello", |args, _| Ok(format!("Hello {}", args[0])));
let output = template.render("");
assert_eq!(output, "Hello World");
```

# License

This code is distributed under the MIT license
