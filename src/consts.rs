//! Contains arbitrary constants for use in the `pcg-rs` crate.

/// The initial/default state to initialize the Pcg struct with
pub const INIT_STATE: u64 = 0x853c_49e6_748f_ea9b;

/// The initial/default incrementing value to initialize the Pcg struct with
pub const INIT_INC: u64 = 0xda3e_39cb_94b9_5bdb;

/// The value to multiply the state with when a random number is generated in order to
/// alter the random number generator's state
pub const INCREMENTOR: u64 = 6_364_136_223_846_793_005;
