# pcg-rs

[![Build Status](https://travis-ci.org/afnanenayet/pcg-rs.svg?branch=master)](https://travis-ci.org/afnanenayet/pcg-rs)

## Synopsis

This is a port of the PCG random number generation libary, made for C and C++,
to Rust.

The library implements the `RngCore` trait, which automatically implements the
`Rng` trait, providing a standard interface to generate and sample random numbers.

_Note_: with the 1.0.0 release of pcg-rs, the old sampling methods have been deprecated,
please use the sampling methods implemented via the `Rng` trait instead.

http://www.pcg-random.org

