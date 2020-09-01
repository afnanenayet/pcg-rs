//! # Synopsis
//!
//! The PCG crate is a port of the C/C++ PCG library for generating random
//! numbers. It implements the `RngCore` trait so all of the standard Rust methods
//! for generating random numbers are available. You can find a reference on the methods provided
//! by the `Rng` trait here: https://rust-random.github.io/rand/rand/trait.Rng.html
//!
//! _Note: you must use the `rand` crate if you want to use the methods provided
//! by the `Rng` trait._
//!
//! ```
//! use rand::prelude::*;
//! use pcg::Pcg;
//!
//! // Create the PCG struct with state
//! let mut pcg = Pcg::default();
//!
//! // Generate arbitrary random values
//! let mut some_bool: bool = pcg.gen();
//! let mut some_f32: f32 = pcg.gen();
//! let mut some_u32: u32 = pcg.gen();
//! ```

use crate::consts::{INCREMENTOR, INIT_INC, INIT_STATE};
use core::hash::{Hash, Hasher};
use core::num::Wrapping;
use rand_core::{impls, Error, RngCore, SeedableRng};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod consts;

/// The `Pcg` state struct contains state information for use by the random
/// number generating functions.
///
/// The internals are private and shouldn't be modified by
/// anything other than the member functions. Note that the random number
/// generating functions will modify the state of this struct, so you must
/// initialize `Pcg` as mutable in order to use any of its functionality.
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pcg {
    state: u64,
    inc: u64,
}

impl Pcg {
    /// Constructs a new PCG state struct with a particular seed and sequence.
    ///
    /// The function returns a struct with state information for the PCG RNG.  The `seed` param
    /// supplies an initial state for the RNG, and the `seq` param functionally acts as a stream
    /// ID. If you're unsure of which params to initialize this struct with, construct the default
    /// struct.
    ///
    /// If you can't think of a seed and a state to initialize this with, just use the default
    /// struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use pcg::Pcg;
    ///
    /// let mut rng = Pcg::new(0, 0);
    /// ```
    pub fn new(seed: u64, seq: u64) -> Pcg {
        Pcg {
            state: seed,
            inc: (seq << 1) | 1,
        }
    }
}

impl Default for Pcg {
    fn default() -> Self {
        Pcg {
            state: INIT_STATE,
            inc: INIT_INC,
        }
    }
}

impl RngCore for Pcg {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let old_state = self.state;
        self.state = (Wrapping(old_state) * Wrapping(INCREMENTOR) + Wrapping(self.inc)).0;
        let xor_shifted = (old_state >> 18) ^ old_state >> 27;

        // need to cast to i64 to allow the `-` operator (casting between integers of
        // the same size is a no-op)
        let rot = (old_state >> 59) as i64;
        (xor_shifted >> rot as u64) | (xor_shifted << ((-rot) & 31))
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

/// The number of 8-bit buckets that the seed is made of
const N: usize = 8;

/// A wrapper type for the PcgSeed
///
/// This wrapper allows us to implement a `SeedableRng` for `Pcg`.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PcgSeed(pub [u8; N]);

/// A bit mask for u8
const MASK: u8 = 0b11111111;

impl Default for PcgSeed {
    fn default() -> Self {
        Self([0; N])
    }
}

impl AsMut<[u8]> for PcgSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Hash for PcgSeed {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // create a vector from the array
        let seed_vec: Vec<u8> = self.0.iter().cloned().collect();
        seed_vec.hash(state)
    }
}

impl From<u64> for PcgSeed {
    fn from(init: u64) -> Self {
        let mut seed: [u8; N] = [0; N];

        for i in 0..N {
            let shift_factor = i * 8;
            let section = (init >> shift_factor) as u8;
            seed[0] = section & MASK
        }
        PcgSeed(seed)
    }
}

impl SeedableRng for Pcg {
    type Seed = PcgSeed;

    fn from_seed(seed: Self::Seed) -> Pcg {
        let mut seed_bytes: u64 = 0;

        // Iterate through each set of 8 bytes to fill in a section of the seed
        for (i, byte) in seed.0.iter().enumerate() {
            seed_bytes |= (byte << (i * 8)) as u64;
        }
        Pcg::new(seed_bytes, INIT_INC)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let _rng = Pcg::new(0, 0);
    }
}
