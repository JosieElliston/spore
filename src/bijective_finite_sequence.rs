use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::my_rng::Rng;

// https://stackoverflow.com/questions/53755017/can-i-randomly-sample-from-a-hashset-efficiently
/// like a bidict but one of the types is usize
/// also the indices are private and unspecified and unstable and shouldn't leak to the user
/// the derived eq requires same insertion/removal order
#[derive(Clone, Debug)]
pub struct BijectiveFiniteSequence<T: Eq + Copy + Hash> {
    values: Vec<T>,
    value_to_index: HashMap<T, usize, ahash::RandomState>,
}
impl<T: Eq + Copy + Hash> BijectiveFiniteSequence<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            value_to_index: HashMap::default(),
        }
    }

    pub fn contains(&self, val: &T) -> bool {
        self.value_to_index.contains_key(val)
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.values.len(), self.value_to_index.len());
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn insert(&mut self, val: T) {
        let l = self.len();
        let entry = self.value_to_index.entry(val);
        entry.or_insert_with(|| {
            self.values.push(val);
            l
        });
    }

    /// is None iff `self.is_empty()`
    pub fn get_random(&self, rng: &mut Rng) -> Option<&T> {
        // println!("get_random with len {}", self.len());
        if self.is_empty() {
            None
        } else {
            self.values.get(rng.next_u32_n(self.len() as u32) as usize)
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    pub fn remove(&mut self, value: &T) -> Option<T> {
        let index = self.value_to_index.remove(value)?;
        let last_value = self.values.pop().unwrap();
        if index != self.values.len() {
            self.values[index] = last_value;
            self.value_to_index.insert(last_value, index);
        }
        Some(*value)

        // let index = self.value_to_index.remove(value)?;
        // if index + 1 == self.values.len() {
        //     self.values.pop();
        // } else {
        //     self.values.swap_remove(index);
        //     self.value_to_index.insert(self.values[index], index);
        // }
        // Some(*value)
    }

    pub fn validate(&self) {
        assert_eq!(
            self.values.len(),
            self.values.iter().copied().collect::<HashSet<_>>().len()
        );
        for (index, value) in self.values.iter().enumerate() {
            assert!(self.value_to_index.contains_key(value));
            assert_eq!(index, *self.value_to_index.get(value).unwrap());
        }
        for (value, index) in &self.value_to_index {
            let index_2 = self.values.iter().position(|r| r == value).unwrap();
            assert_eq!(index, &index_2);
        }
    }
}

// equality that is not insertion/removal order dependent
// impl<T: Eq + Copy + Hash> PartialEq for BijectiveFiniteSequence<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.index_to_value.iter().collect::<HashSet<_>>()
//             == other.index_to_value.iter().collect::<HashSet<_>>()
//     }
// }
