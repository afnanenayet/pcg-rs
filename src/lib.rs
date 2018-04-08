use std::num::Wrapping;

/// The PCG crate is a port of the C/C++ PCG library for generating random
/// numbers.

/// The pcg state struct contains the state and the increment factor for the
/// pcg generator. The internals are private and shouldn't be modified by
/// anything other than the member functions. Note that the random number
/// generating functions will modify the state of this struct, so you must
/// initialize `Pcg` as mutable in order to use any of its functionality.
#[derive(Debug)]
pub struct Pcg {
    state: u64,
    inc: u64,
}

impl Pcg {
    /// Construct a new PCG state struct with a particular seed
    pub fn new(seed: u64, seq: u64) -> Pcg {
        let mut rng = Pcg {
            state: 0,
            inc: (seq << 1) | 1,
        };
        rng.state += seed;
        return rng;
    }

    /// Generates a random unsigned 32 bit integer
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

    /// Generates a random unsigned 32 bit integer bounded between 0 and the bound
    pub fn bounded_rand(&mut self, bound: u32) -> u32 {
        let threshold = (-(bound as i32) % (bound as i32)) as u32;

        // the loop is guaranteed to terminate
        loop {
            let r = self.rand();

            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl Default for Pcg {
    /// Returns a hardcoded default seed/state for the PCG rng. The values were
    /// taken from [here](https://github.com/imneme/pcg-c-basic/blob/master/pcg_basic.h#L49),
    /// the basic C implementation of PCG.
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

    #[test]
    /// Checks that there are no runtime errors when generating random numbers
    /// (such as rust complaining about integer overflow ops)
    fn test_rand() {
        let mut rng = Pcg::default();
        let n = 100000000;

        for _ in 0..n {
            let _rand = rng.rand();
        }
    }

    #[test]
    fn test_bounded_rand() {
        let mut rng = Pcg::default();
        let n = 10000000;

        for _ in 0..n {
            let rand = rng.bounded_rand(10);
            assert!(rand < 10);
        }
    }
}
