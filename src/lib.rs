/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-singleton/0.3.1")]
#![no_std]
//! # Singleton pattern implementation
//!
//! Provide a cross core synchronisation safe singleton implementation pattern
//!
//! # Example
//! ```
//! use ruspiro_singleton::*;
//!
//! static FOO: Singleton<Foo> = Singleton::new(Foo::new(0));
//!
//! struct Foo {
//!     count: u32,
//! }
//!
//! impl Foo {
//!     pub const fn new(initial_count: u32) -> Self {
//!         Foo {
//!             count: initial_count,
//!         }
//!     }
//!
//!     pub fn count(&self) -> u32 {
//!         self.count    
//!     }
//!
//!     pub fn add_count(&mut self, value: u32) -> u32 {
//!         self.count += value;
//!         self.count
//!     }
//! }
//!
//! fn main () {
//!     let counter = FOO.take_for( |foo| {
//!         println!("secure access to the singleton");
//!         // do something with the singleton, it is mutable inside 'take_for'
//!         let c = foo.add_count(1);
//!         // and return any value, the return value of take_for is inferred from the return
//!         // value of the closure given to this function.
//!         c
//!     });
//!
//!     println!("successfull {}", counter);
//! }
//! ```
//!
//! In case only immutable access to the contents of the singleton is required the ``use_for`` function
//! can be used.
//! ```
//! # use ruspiro_singleton::*;
//! # static FOO: Singleton<Foo> = Singleton::new(Foo::new(0));
//! # struct Foo {
//! #     count: u32,
//! # }
//! # impl Foo {
//! #     pub const fn new(initial_count: u32) -> Self {
//! #         Foo {
//! #             count: initial_count,
//! #         }
//! #     }
//! #
//! #     pub fn count(&self) -> u32 {
//! #         self.count    
//! #     }
//! # }
//!
//! fn main () {
//!     let counter = FOO.use_for( |foo| {
//!             foo.count()
//!         });
//!
//!     println!("current counter: {}", counter);
//! }
//! ```
//!
use core::cell::UnsafeCell;
use ruspiro_lock::Spinlock;

/// The Singleton wrapper stores any type
pub struct Singleton<T: 'static> {
    inner: UnsafeCell<T>,
    lock: Spinlock,
}

// The Singleton need to implement Sync to ensure cross core sync compile check mechanisms
#[doc(hidden)]
unsafe impl<T> Sync for Singleton<T> {}

#[doc(hidden)]
unsafe impl<T> Send for Singleton<T> {}

impl<T: 'static> Singleton<T> {
    /// Create a new singleton instance to be used in a static variable. Only ``const fn`` constructors are allowed here.
    /// If this is not sufficient the ``Singleton`` may be further wrapped by a ``lazy_static!`` available as
    /// external crate from [crates.io](https://crates.io/crates/lazy_static)
    pub const fn new(data: T) -> Singleton<T> {
        Singleton {
            inner: UnsafeCell::new(data),
            lock: Spinlock::new(),
        }
    }

    /// Take the stored singleton for whatever operation and prevent usage by other cores
    /// Safe access to the singleton mutable instance is guarantied inside the given closure.
    ///
    /// # Example
    /// ```
    /// # use ruspiro_singleton::*;
    /// # static FOO: Singleton<u32> = Singleton::new(0);
    /// # fn main() {
    ///     FOO.take_for(|foo| {
    ///         // do something mutable with [foo]
    ///     });
    /// # }
    /// ```
    pub fn take_for<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        // to ensure atomic access to the singleton wrapped resource we aquire a lock before allowing to access
        // the same. While the lock is aquired interrupts are disabled. This ensures there are now deadlocks
        // possible when a lock is interrupted and the handler tries to aquire the same lock
        self.lock.aquire();

        let r = f(unsafe { &mut *self.inner.get() });

        // after processing we can release the lock so other cores can access the singleton as well
        // this also re-enables interrupts
        self.lock.release();
        r
    }

    /// Immutable access to a singleton for a specific operation.
    /// This access does not enforce any lock nor guarantees safe atomic access to the instance. However, it is usefull
    /// in read-only access scenarios like inside interrupt handlers.
    ///
    /// # Example
    /// ```
    /// # use ruspiro_singleton::*;
    /// # static FOO: Singleton<u32> = Singleton::new(0);
    /// # fn main() {
    ///     FOO.use_for(|foo| {
    ///         // do something immutable with [foo]
    ///     });
    /// # }
    /// ```
    pub fn use_for<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(unsafe { &*self.inner.get() })
    }
}
