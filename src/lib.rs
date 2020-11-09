/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: MIT / Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-singleton/||VERSION||")]
#![no_std]
#![feature(const_fn)]

//! # Singleton pattern implementation
//!
//! Provide a cross core synchronisation safe singleton implementation pattern. The `Singleton` is intended to be used
//! to declare crate level static variables that require safe access accross cores. This is helpful where the data
//! structure used within the `Singleton` represents a peripheral where the crate shall only hand out a single instance
//! to safely represent to unique existance of the peripheral.
//!
//! # HINT
//! Safe lazy initialization is ensured using atomics. On the Raspberry Pi atmomic operations require the *MMU* to be 
//! configured and active. Otherwise the executing CPU core will hang when trying to execute the atomic operation.
//!
//! # Example
//! ```no_run
//! # use ruspiro_singleton::*;
//! // define the static variable with an inizialization closure
//! static FOO:Singleton<u32> = Singleton::new(20);
//!
//! // define the static variable with an inizialization closure
//! static DEMO:Singleton<Box<Demo>> = Singleton::lazy(&|| {
//!     Box::new(
//!         Demo::new()
//!     )
//! });
//!
//! // define the type to be accessible as singleton
//! struct Demo {
//!     pub count: u32,
//! }
//!
//! // implement the type that should provided as singlton
//! impl Demo {
//!     pub const fn new() -> Self {
//!         Demo {
//!             count: 0,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     // safely use the singleton inside the closure passed to [with_mut] to update it's contents
//!     DEMO.with_mut(|d| {
//!         d.count += 10;
//!     });
//!
//!     // safely use the singleton inside the closure passed to [with_ref] if read-only access is required
//!     DEMO.with_mut(|d| {
//!         println!("Value: {}", d.count);
//!     });
//!
//!     // you may also return a value from the singleton to work with it after the safe singleton access
//!     let val = DEMO.with_ref(|d| {
//!         if d.count != 0 {
//!             true
//!         } else {
//!             false
//!         }
//!     });
//! }
//! ```

mod lazy;

use lazy::LazyValue;
use ruspiro_lock::RWLock;

/// The Singleton wrapper stores any type
pub struct Singleton<T: 'static> {
    /// the inner value wrapping the contained data for safe read/write access
    inner: RWLock<LazyValue<T>>,
}

// The Singleton need to implement Send & Sync to ensure cross core compile check mechanics
// this is safe as the inner RWLock ensures cross core safety
unsafe impl<T> Sync for Singleton<T> {}
unsafe impl<T> Send for Singleton<T> {}

impl<T: 'static> Singleton<T> {
    /// Create a new [Singleton] instance to be used in a static variable. Only ``const fn`` constructors are allowed
    /// here.
    /// # Example
    /// ```no_run
    /// # use ruspiro_singleton::*;
    /// static FOO: Singleton<u32> = Singleton::new(20);
    /// # fn main() {}
    /// ```
    pub const fn new(value: T) -> Self {
        Singleton {
            inner: RWLock::new(LazyValue::with_value(value)),
        }
    }

    /// Create a new [Singleton] instance passing a closure that will be evaluated at first access to the contents of
    /// the singleton that will provide its value
    /// # Example
    /// ```no_run
    /// # use ruspiro_singleton::*;
    /// static FOO: Singleton<String> = Singleton::lazy(&|| String::from("foo"));
    /// # fn main() {}
    /// ```
    pub const fn lazy<F>(init: &'static F) -> Self
    where
        F: Fn() -> T,
    {
        Self {
            inner: RWLock::new(LazyValue::with_init(init)),
        }
    }

    /// Take the stored singleton for whatever operation and prevent usage by other cores
    /// Safe access to the singleton mutable instance is guarantied inside the given closure.
    ///
    pub fn with_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let inner = self.inner.lock();
        // use write lock to mutably access the inner value of the singleton. As long
        // as the write lock exists no other write or read lock is possible
        let r = f(inner.get_mut());

        // explicitly release the lock befor providing the result of the closure to the caller
        drop(inner);

        r
    }

    /// Immutable access to a singleton for a specific operation.
    /// This access does not enforce any lock nor guarantees safe atomic access to the instance. However, it is usefull
    /// in read-only access scenarios like inside interrupt handlers.
    ///
    pub fn with_ref<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let inner = self.inner.read();
        // multiple read locks are possible when accessing the inner data of the singleton
        // all read locks are required to be release before the next write lock could happen
        let r = f(inner.get());

        // explicitly release the lock befor providing the result of the closure to the caller
        drop(inner);

        r
    }
}
