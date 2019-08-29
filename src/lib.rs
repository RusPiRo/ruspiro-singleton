/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-singleton/0.2.1")]
#![no_std]
#![feature(asm)]

//! # Singleton pattern implementation
//! 
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
//! fn some_function () {
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
//! }
//! ```
//! 
//! In case only immutable access to the contents of the singleton is required the ``use_for`` function 
//! can be used.
//! ```
//! fn some_other_function() {
//!     let counter = MY_SINGLETON.use_for( |s| {
//!             s.count()
//!         });
//! 
//!     println!("current counter: {}", counter);
//! }
//! ```
//! 
use core::cell::RefCell;
use ruspiro_lock::Spinlock;

/// The Singleton wrapper stores any type
pub struct Singleton<T: 'static> {
    inner: RefCell<T>,
    lock: Spinlock,
}

// The Singleton need to implement Sync to ensure cross core sync compile check mechanisms
#[doc(hidden)]
unsafe impl<T> Sync for Singleton<T> { }

impl<T: 'static> Singleton<T> {
    
    /// Create a new singleton instance to be used in a static variable. Only ``const fn`` constructors are allowed here.
    /// If this is not sufficient the ``Singleton`` may be further wrapped by a ``lazy_static!`` available as 
    /// external crate from [crates.io](https://crates.io/crates/lazy_static)
    pub const fn new(data: T) -> Singleton<T> {
        Singleton {
            inner: RefCell::new(data),
            lock: Spinlock::new(),
        }
    }

    /// Take the stored singleton for whatever operation and prevent usage by other cores
    /// Safe access to the singleton mutable instance is guarantied inside the given closure.
    /// 
    /// # Example
    /// ```
    /// # fn doc() {
    ///     MY_SINGLETON.take_for(|my| {
    ///         // do something with [my]
    ///         my.any_mutable_function();
    /// # }
    /// ```
    pub fn take_for<F, R>(&self, f: F) -> R
        where F: FnOnce(&mut T) -> R 
    {
            // to ensure atomic access to the singleton wrapped resource we aquire a lock before allowing to access
            // the same. While the lock is aquired interrupts are disabled. This ensures there are now deadlocks 
            // possible when a lock is interrupted and the handler tries to aquire the same lock
            self.lock.aquire();
            
            let r = f( &mut *self.inner.borrow_mut() );
            
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
    /// # fn doc() {
    ///     MY_SINGLETON.use_for(|my| {
    ///         // do something with [my]
    ///         let _ = my.any_immutable_function();
    ///     });
    /// # }
    /// ```
    pub fn use_for<F, R>(&self, f: F) -> R
        where F: FnOnce(&T) -> R
    {
            f( & *self.inner.borrow() )
    }
}