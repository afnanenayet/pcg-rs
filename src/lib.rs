/// The PCG crate is a port of the C/C++ PCG library for generating random
/// numbers.

extern crate rand_core;

#[cfg(test)]
extern crate rand;

use std::num::Wrapping;
use rand_core::{RngCore, Error, impls};

/// The `Pcg` state struct contains state information for use by the random
/// number generating functions.
///
/// The internals are private and shouldn't be modified by
/// anything other than the member functions. Note that the random number
/// generating functions will modify the state of this struct, so you must
/// initialize `Pcg` as mutable in order to use any of its functionality.
#[derive(Debug)]
pub struct Pcg {
    state: u64,
    inc: u64,
}

impl Pcg {
    /// Constructs a new PCG state struct with a particular seed and sequence.
    ///
    /// The function returns a struct with state information for the PCG RNG.
    /// The `seed` param supplies an initial state for the RNG, and the `seq`
    /// param functionally acts as a stream ID. If you're unsure of which
    /// params to initialize this struct with, construct the default struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use pcg::Pcg;
    ///
    /// let mut rng = Pcg::new(0, 0);
    /// ```
    pub fn new(seed: u64, seq: u64) -> Pcg {
        let mut rng = Pcg {
            state: 0,
            inc: (seq << 1) | 1,
        };
        rng.state += seed;
        return rng;
    }

    /// Generates a random unsigned 32 bit integer
    ///
    /// The function generates a random unsignd 32-bit integer that is bounded
    /// from 0 to `std::u32::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pcg::Pcg;
    ///
    /// let mut rng = Pcg::default();
    /// let random_number = rng.rand();
    /// ```
    #[deprecated(since="1.0.0", note="Please use the methods provided by the `Rng` trait instead.")]
    pub fn rand(&mut self) -> u32 {
        let old_state = self.state;
        self.state = (Wrapping(old_state) * Wrapping(6364136223846793005) + Wrapping(self.inc)).0;
        let xor_shifted = (old_state >> 18) ^ old_state >> 27;
        // need to cast to i64 to allow the `-` operator (casting between integers of
        // the same size is a no-op)
        let rot = (old_state >> 59) as i64;
        let res = (xor_shifted >> rot as u64) | (xor_shifted << ((-rot) & 31));
        res as u32
    }

    /// Generates a random unsigned 32 bit integer bounded between 0 and the
    /// upper `bound`.
    ///
    /// The range for the function is [0, `bound`), so every number output by
    /// this function will be strictly less than the `bound`).
    ///
    /// # Examples
    ///
    /// ```
    /// use pcg::Pcg;
    ///
    /// let mut rng = Pcg::default();
    ///
    /// // create a random number in the range [0, 10)
    /// let bounded_random_number = rng.bounded_rand(10);
    /// ```
    #[deprecated(
        since = "1.0.0", note =
        "Please use the `gen_range` or uniform distribution methods provided by the `Rng` trait
        instead."
    )]
    pub fn bounded_rand(&mut self, bound: u32) -> u32 {
        let threshold = (-(bound as i32) % (bound as i32)) as u32;

        // the loop is guaranteed to terminate
        loop {
            let r = self.next_u32();

            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl RngCore for Pcg {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let old_state = self.state;
        self.state = (Wrapping(old_state) * Wrapping(6364136223846793005) + Wrapping(self.inc)).0;
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

impl Default for Pcg {
    /// Creates a PCG RNG state struct with default values.
    ///
    /// Returns a hardcoded default seed/state for the PCG RNG. The values are
    /// taken from [here](https://github.com/imneme/pcg-c-basic/blob/master/pcg_basic.h#L49),
    /// the basic C implementation of PCG.
    ///
    /// # Examples
    ///
    /// ```
    /// use pcg::Pcg;
    ///
    /// let mut rng = Pcg::default();
    /// ```
    fn default() -> Pcg {
        Pcg {
            state: 0x853c49e6748fea9b,
            inc: 0xda3e39cb94b95bdb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_init() {
        let _rng = Pcg::new(0, 0);
    }

    #[test]
    fn test_init_default() {
        let _rng: Pcg = Default::default();
    }

    #[test]
    /// Checks that there are no runtime errors when generating random numbers
    /// (such as rust complaining about integer overflow ops)
    fn test_rand() {
        let mut rng = Pcg::default();
        let n = 100000000;

        for _ in 0..n {
            let _rand = rng.next_u32();
        }
    }

    #[test]
    fn test_bounded_rand() {
        let mut rng = Pcg::default();
        let n = 10000000;
        let mut v = vec![0 as u32; 10];

        for _ in 0..n {
            let rand = rng.gen_range(0, 10);
            assert!(rand < 10);
            v[rand as usize] += 1;
        }
        print!("{:?}", v);

        let mut v = vec![0 as u32; 2];
        for _ in 0..n {
            let rand = rng.gen_range(0, 2);
            assert!(rand < 2);
            v[rand as usize] += 1;
        }
        print!("{:?}", v);
    }
}
