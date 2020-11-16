# Changelog

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
