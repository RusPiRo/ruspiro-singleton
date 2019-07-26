# RusPiRo Singleton crate
This crate provides an easy to use singleton pattern that is safe to be used across cores.

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-singleton = { git = 'https://github.com/RusPiRo/ruspiro-singleton' }
```

Once done on any rust file you can define a static variable as singleton of any type for safe cross core access like so:
```
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

    // you may also return a value from the singleton to work with it outside the safe singleton access
    // however, this only works for Copy semantics. Move semantics are not allowed and the compiler will complain.
    let _current = DEMO.take_for(|d| {
        d.count
    });
}
```

## License
This crate is licensed with the Appache License 2.0
