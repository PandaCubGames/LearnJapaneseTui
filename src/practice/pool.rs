// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Picks entries in random order, each once per round.

use super::Rng;

/// Picks entries in random order, each once per round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pool {
    len: usize,
    left: Vec<usize>,
    last: Option<usize>,
}

impl Pool {
    pub fn new(len: usize) -> Pool {
        Pool {
            len,
            left: Vec::new(),
            last: None,
        }
    }

    /// The next entry, an index in `0..len`; `None` when `len` is 0.
    pub fn next(&mut self, rng: &mut Rng) -> Option<usize> {
        if self.len == 0 {
            return None;
        }
        if self.left.is_empty() {
            self.left = (0..self.len).collect();
        }
        let mut i = rng.below(self.left.len());
        if self.left.len() > 1 && Some(self.left[i]) == self.last {
            let j = rng.below(self.left.len() - 1);
            i = if j >= i { j + 1 } else { j };
        }
        let entry = self.left.swap_remove(i);
        self.last = Some(entry);
        Some(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pool_gives_nothing() {
        assert_eq!(Pool::new(0).next(&mut Rng::new(0)), None);
    }

    #[test]
    fn one_entry_every_time() {
        let mut pool = Pool::new(1);
        let mut rng = Rng::new(0);
        for _ in 0..5 {
            assert_eq!(pool.next(&mut rng), Some(0));
        }
    }

    #[test]
    fn every_entry_once_per_round() {
        for seed in 0..20 {
            let mut pool = Pool::new(3);
            let mut rng = Rng::new(seed);
            for _ in 0..3 {
                let mut round: Vec<_> = (0..3).map(|_| pool.next(&mut rng).unwrap()).collect();
                round.sort_unstable();
                assert_eq!(round, [0, 1, 2]);
            }
        }
    }

    #[test]
    fn no_entry_twice_in_a_row() {
        for len in 2..=5 {
            for seed in 0..20 {
                let mut pool = Pool::new(len);
                let mut rng = Rng::new(seed);
                let picks: Vec<_> = (0..50).map(|_| pool.next(&mut rng).unwrap()).collect();
                assert!(picks.windows(2).all(|pair| pair[0] != pair[1]), "{picks:?}");
            }
        }
    }

    #[test]
    fn same_seed_same_order() {
        let order = |seed| {
            let mut pool = Pool::new(4);
            let mut rng = Rng::new(seed);
            (0..12).map(|_| pool.next(&mut rng)).collect::<Vec<_>>()
        };
        assert_eq!(order(9), order(9));
    }
}
