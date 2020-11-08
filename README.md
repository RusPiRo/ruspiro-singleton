# RusPiRo Singleton crate

This crate provides an easy to use singleton pattern that is safe to be used across cores.

[![Travis-CI Status](https://api.travis-ci.com/RusPiRo/ruspiro-singleton.svg?branch=master)](https://travis-ci.com/RusPiRo/ruspiro-singleton)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-singleton.svg)](https://crates.io/crates/ruspiro-singleton)
[![Documentation](https://docs.rs/ruspiro-singleton/badge.svg)](https://docs.rs/ruspiro-singleton)
[![License](https://img.shields.io/crates/l/ruspiro-singleton.svg)](https://github.com/RusPiRo/ruspiro-singleton#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-singleton = "||VERSION||"
```

Once done on any rust file you can define a static variable as `Singleton` of any type for safe cross core access in two different ways. The first variant requires to provide an instance of the data to be wrapped inside the singleton while defining the same.

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
    }
}
```

The second variant allows to pass a closure to the initialization of the `Singleton` that will be evaluated at first access to the contents of it.

> !HINT!
> Safe lazy initialization is ensured using atomics. On the Raspberry Pi atmomic operations require the *MMU* to be configured and active. Otherwise the executing CPU core will hang when trying to execute the atomic operation.

```rust
// define the static variable with an inizialization closure
static DEMO:Singleton<Box<Demo>> = Singleton::lazy(&|| {
    Box::new(
        Demo::new()
    )
});

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
```

To use the data sealed with the `Singleton` call either of the two methods `with_ref` and `with_mut` providing a closure accessing the data immuable or mutable.

```rust
fn main() {
    // safely use the singleton inside the closure passed to [with_mut] to update it's contents
    DEMO.with_mut(|d| {
        d.count += 10;
    });

    // safely use the singleton inside the closure passed to [with_ref] if read-only access is required
    DEMO.with_mut(|d| {
        println!("Value: {}", d.count);
    });

    // you may also return a value from the singleton to work with it after the safe singleton access
    let val = DEMO.with_ref(|d| {
        if d.count != 0 {
            true
        } else {
            false
        }
    });
}
```

## Limitation

The current version of the implementation does not allow *lazy* initialization. Only ``const fn`` functions can be used to initialize the structure instance that should be wrapped by the ``Singleton``.

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
