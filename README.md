# RusPiRo Singleton crate

This crate provides an easy to use singleton pattern that is safe to be used across cores.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-singleton.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-singleton)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-singleton.svg)](https://crates.io/crates/ruspiro-singleton)
[![Documentation](https://docs.rs/ruspiro-singleton/badge.svg)](https://docs.rs/ruspiro-singleton)
[![License](https://img.shields.io/crates/l/ruspiro-singleton.svg)](https://github.com/RusPiRo/ruspiro-singleton#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-singleton = "0.3"
```

Once done on any rust file you can define a static variable as singleton of any type for safe cross core access like so:

```rust
// define the static variable
static DEMO:Singleton<Demo> = Singleton::new(Demo::new());

// define the type to be accessible as singleton
struct Demo {
    pub count: u32,
}

// implement the type that should provided as singlton
impl Demo {
    pub const fn new() -> Self {
        Demo {
            count: 0,
        }
    };
}

fn main() {
    // safely use the singleton inside the closure passed to [take_for].
    DEMO.take_for(|d| {
        d.count += 10;
    });

    // you may also return a value from the singleton to work with it after the safe singleton access
    let _current = DEMO.take_for(|d| {
        d.count
    });
}
```

If the singleton does only require ``read only`` access a non-blocking function could be used:

```rust
fn main() {
    DEMO.use_for(|d| {
        // d is available with immutable access only in this scenario
        println!("current count: {}", d.count);
    });
}
```

## Limitation
The current version of the implementation does not allow *lazy* initialization. Only ``const fn`` functions can be used to initialize the structure instance that should be wrapped by the ``Singleton``.

## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)