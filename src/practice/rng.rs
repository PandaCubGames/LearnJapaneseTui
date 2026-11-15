// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! A small random number generator, so the order of questions can be fixed in tests.

/// SplitMix64: small and fast, good enough to shuffle questions. Not for cryptography.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A number in `0..n`; 0 when `n` is 0.
    pub fn below(&mut self, n: usize) -> usize {
        let n = u64::try_from(n).unwrap_or(u64::MAX);
        let wide = u128::from(self.next_u64()) * u128::from(n);
        usize::try_from(wide >> 64).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_zero_matches_the_reference_values() {
        let mut rng = Rng::new(0);
        assert_eq!(rng.next_u64(), 0xe220_a839_7b1d_cdaf);
        assert_eq!(rng.next_u64(), 0x6e78_9e6a_a1b9_65f4);
        assert_eq!(rng.next_u64(), 0x06c4_5d18_8009_454f);
    }

    #[test]
    fn same_seed_same_numbers() {
        let mut a = Rng::new(7);
        let mut b = Rng::new(7);
        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_differ() {
        assert_ne!(Rng::new(1).next_u64(), Rng::new(2).next_u64());
    }

    #[test]
    fn below_stays_in_range() {
        let mut rng = Rng::new(3);
        for n in 1..=50 {
            for _ in 0..200 {
                assert!(rng.below(n) < n);
            }
        }
    }

    #[test]
    fn below_reaches_every_value() {
        let mut rng = Rng::new(4);
        let mut seen = [false; 5];
        for _ in 0..200 {
            seen[rng.below(5)] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn below_zero_and_one_give_zero() {
        let mut rng = Rng::new(5);
        assert_eq!(rng.below(0), 0);
        assert_eq!(rng.below(1), 0);
    }
}
