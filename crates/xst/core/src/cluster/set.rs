use crate::shard::SetData;

/// Sorted, disjoint, non-adjacent inclusive Unicode scalar intervals.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CharSet(pub Vec<(u32, u32)>);

impl CharSet {
    pub fn new(ranges: impl IntoIterator<Item = (u32, u32)>) -> Self {
        let mut ranges: Vec<_> = ranges
            .into_iter()
            .flat_map(|(lo, hi)| {
                // Never include surrogate code points, even for ranges spanning them.
                [(lo, hi.min(0xd7ff)), (lo.max(0xe000), hi.min(0x10ffff))]
                    .into_iter()
                    .filter(|(lo, hi)| lo <= hi)
            })
            .collect();
        ranges.sort_unstable();
        let mut merged: Vec<(u32, u32)> = Vec::new();
        for (lo, hi) in ranges {
            if let Some(last) = merged.last_mut() {
                if lo <= last.1 + 1 {
                    last.1 = last.1.max(hi);
                    continue;
                }
            }
            merged.push((lo, hi));
        }
        Self(merged)
    }

    pub fn from_data(set: &SetData) -> Self {
        let positive = Self::new(
            set.range
                .iter()
                .map(|r| (*r.start() as u32, *r.end() as u32)),
        );
        if !set.negated {
            return positive;
        }
        let mut complement = Vec::new();
        let mut next = 0;
        for (lo, hi) in positive.0 {
            if next < lo {
                complement.push((next, lo - 1));
            }
            next = hi + 1;
        }
        if next <= 0x10ffff {
            complement.push((next, 0x10ffff));
        }
        Self::new(complement)
    }

    pub fn union(&mut self, other: &Self) -> bool {
        let merged = Self::new(self.0.iter().chain(&other.0).copied());
        if *self == merged {
            false
        } else {
            *self = merged;
            true
        }
    }

    pub fn matches(&self, ch: char) -> bool {
        let ch = ch as u32;
        let index = self.0.partition_point(|&(_, hi)| hi < ch);
        self.0.get(index).is_some_and(|&(lo, _)| lo <= ch)
    }
}
