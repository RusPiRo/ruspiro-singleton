# Changelog

## :melon: v0.4.3

This is a maintenance release ensuring successful build of the crate with the latest (2021-09-05) nightly compiler version.

- ### :wrench: Maintenance

  - introduce the new `#![feature(const_fn_trait_bound)]` as a replacement for the former more generic `const_fn` feature.
  - build this crate with `aarch64-unknown-none` standard build target.

## :peach: v0.4.2

This is a maintenance release migrating the build pipeline to github actions.

- ### :wrench: Maintenance

  - Migrate the build pipeline to github actions.
  - Introduce custom build target for crate build.

## :peach: v0.4.1

- ### :detective: Fixes

  - remove soundness when the interior type of the `Singleton` is not `Send` and not `Sync` like a Cell.

## :peach: v0.4.0

- ### :bulb: Features

  - Introduce the ability to lazylie initialize the value stored inside the `Singleton` using a closure. The initialization is evaluated on first access to the `Singleton` contents.

- ### :wrench: Maintenance

  - Enable proper and stable pipeline to support release and publishing process

## :banana: v0.3.1

- ### :detective: Fixes

  - remove `asm!` macro usages and replace with `llvm_asm!`
  - use `cargo make` to stabilize cross-platform builds
