/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: MIT / Apache License 2.0
 **********************************************************************************************************************/

//! # Lazy Singleton initialization
//!

use core::cell::UnsafeCell;
use ruspiro_lock::Spinlock;

/// A wrapper that enables lazy initialization of the value stored within the `Singleton`
pub struct LazyValue<T: 'static + Sized> {
  /// the actual value that shall be provided as a singleton
  inner: UnsafeCell<Option<T>>,
  /// the closure used to initialize the singleton
  init: Option<&'static dyn Fn() -> T>,
  /// A lock that secures the lazy update of the inner value in case it happens across cores
  lock: Spinlock,
}

impl<T: 'static + Sized> LazyValue<T> {
  /// create a new [LazySingleton] where the value is already available
  pub const fn with_value(value: T) -> Self {
    Self {
      inner: UnsafeCell::new(Some(value)),
      init: None,
      lock: Spinlock::new(),
    }
  }

  /// create a new [LazySingleton] where the actual value will be lazily created at first access
  pub const fn with_init<F>(init: &'static F) -> Self
  where
    F: Fn() -> T,
  {
    Self {
      inner: UnsafeCell::new(None),
      init: Some(init),
      lock: Spinlock::new(),
    }
  }

  fn set(&self, value: T) -> Result<(), T> {
    let inner = unsafe { &*self.inner.get() };
    if inner.is_some() {
      return Err(value);
    }
    // update the actual value of LazyValue. This is safe as this is the
    // only place this is updated and we checked the value is actually None
    // before
    let inner = unsafe { &mut *self.inner.get() };
    *inner = Some(value);

    Ok(())
  }

  fn init(&self) {
    // locking the spinlock to ensure the initialization really happens
    // exclusively
    self.lock.aquire();
    // if we could aquire the lock there is a probability that the initialization was kind of a longer running
    // task and thus has already happened while waiting for the lock. So check if the value is still not initialized
    if unsafe { &*self.inner.get() }.is_none() {
      let init = self.init.unwrap();
      let value = init();
      assert!(self.set(value).is_ok(), "LazyValue initialized twice");
    }
    self.lock.release();
  }

  pub fn get(&self) -> &T {
    if let Some(inner) = unsafe { &*self.inner.get() }.as_ref() {
      inner
    } else {
      self.init();
      unsafe { &*self.inner.get() }.as_ref().unwrap()
    }
  }

  pub fn get_mut(&self) -> &mut T {
    if let Some(inner) = unsafe { &mut *self.inner.get() }.as_mut() {
      inner
    } else {
      self.init();
      unsafe { &mut *self.inner.get() }.as_mut().unwrap()
    }
  }
}
