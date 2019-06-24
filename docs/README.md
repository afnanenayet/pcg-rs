# pcg-rs

[![Build Status](https://travis-ci.org/afnanenayet/pcg-rs.svg?branch=master)](https://travis-ci.org/afnanenayet/pcg-rs)
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

