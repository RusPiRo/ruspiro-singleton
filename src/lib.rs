/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-singleton/0.0.1")]
#![no_std]

//! # Simgleton implementation
//! Provide a cross core synchronisation safe singleton implementation pattern
//! 
//! # Example
//! ```
//! static MY_SINGLETON: Singleton<MySingleton> = Singleton::new(MySingleton::new(0));
//! 
//! struct MySingleton {
//!     count: u32,
//! }
//! 
//! impl MySingleton {
//!     pub fn new(initial_count: u32) -> Self {
//!         MySingleton {
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
//! # fn main () {
//!     let counter = MY_SINGLETON.take_for( |singleton| {
//!         println!("secure access to the singleton");
//!         // do something with the singleton, it is mutable inside 'take_for'
//!         let c = singleton.add_count(1);
//!         // and return any value, the return value of take_for is inferred from the return
//!         // value of the closure given to this function.
//!         c
//!     });
//! 
//!     println!("successfull {}", counter);
//! # }
//! ```
//! 
use core::cell::UnsafeCell;
use ruspiro_lock::Spinlock;

/// The Singleton wrapper stores any type
pub struct Singleton<T> {
    inner: UnsafeCell<T>,
    lock: Spinlock,
}

// The Singleton need to implement Sync to ensure cross core sync compile check mechanisms
#[doc(hidden)]
unsafe impl<T> Sync for Singleton<T> { }

impl<T> Singleton<T> {
    /// Create a new singleton instance to be used in a static variable. If the singleton intance to be created
    /// does not contain only const fn constructor the singleton may be further wrapped by [lazy_static!] available as 
    /// external crate from crates.io
    pub const fn new(data: T) -> Singleton<T> {
        Singleton {
            inner: UnsafeCell::new(data),
            lock: Spinlock::new(),
        }
    }

    /// Take the stored singleton for whatever operation and prevent usage by other cores
    /// Safe access to the singleton mutable instance is guarantied inside the given closure.
    pub fn take_for<F, R>(&self, f: F) -> R
        where F: FnOnce(&mut T) -> R {
            // to ensure atomic access to the singleton wrapped resource we aquire a lock before allowing to access
            // the same
            self.lock.aquire();
            let r = f(unsafe { &mut *self.inner.get() });
            // after processing we can release the lock so other cores can access the singleton as well
            self.lock.release();
            r
        }

    /// Unsafe weak access to a singleton for a specific operation. Access by other cores is **not** permitted.
    /// This access does not enforce any lock nor guarantees safe atomic access to the instance. However, it is usefull
    /// in read-only access scenarios like inside interrupt handlers to ensure they do not depend on any lock that could
    /// lead to a dead-lock situation. The access to the singleton is imutable to enforce read-only access to the same.
    /// 
    /// # Example
    /// ```
    /// fn sample() {
    ///     unsafe {
    ///         MY_SINGLETON.use_weak_for(|my| {
    ///             // do something with [my]
    ///             let _ = my.any_imutable_function();
    ///         })
    ///     };
    /// }
    /// ```
    pub unsafe fn use_weak_for<F, R>(&self, f: F) -> R
        where F: FnOnce(&T) -> R {
            f( & *self.inner.get() )
        }
}

