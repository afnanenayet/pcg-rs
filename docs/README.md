# pcg-rs

![Build status](https://github.com/afnanenayet/pcg-rs/workflows/Rust/badge.svg?branch=master)
[![crates badge](https://meritbadge.herokuapp.com/pcg)](https://crates.io/crates/pcg)
[![Documentation](https://docs.rs/pcg/badge.svg)](https://docs.rs/pcg)
![License](https://img.shields.io/crates/l/pcg/3.0.0.svg)

## Synopsis

This is a port of the PCG random number generation libary, made for C and C++,
to Rust.

The library implements the `RngCore` trait, which automatically implements the
`Rng` trait, providing a standard interface to generate and sample random numbers.

_Note_: with the 1.0.0 release of pcg-rs, the old sampling methods have been deprecated,
please use the sampling methods implemented via the `Rng` trait instead.

http://www.pcg-random.org

## Usage

This crate offers `no_std` compatibility through the `std` feature. It is
enabled by default, but if you want to use `no_std`, you can add the package
like this:

```toml
[dependencies.pcg]
version = "4.0"
default-features = false
```

This crate also has optional support for `serde`, which you can enable as a
feature:

```toml
[dependencies.pcg]
version = "4.0"
features = ["std", "serde"]
```

## Example Usage

```rust
use rand::prelude::*;
use pcg::Pcg;

// Initialize the default PCG rng state
let mut rng = Pcg::default();

// Generate some boolean using the standard `gen()` method, which generates the
// appropriate type with type inference
let random_bool: bool = rng.gen();
```
