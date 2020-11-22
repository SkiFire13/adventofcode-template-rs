#![allow(dead_code)]

pub use std::cell::{Cell, RefCell};
pub use std::cmp::{max, min, Ordering, Reverse};
pub use std::collections::{
    BinaryHeap, BTreeMap, BTreeSet, HashMap, HashSet, VecDeque,
    hash_map::Entry,
};
pub use std::convert::{TryFrom, TryInto};
pub use std::iter;
pub use std::mem::{self, replace, swap, take};
pub use std::rc::Rc;
pub use std::sync::Arc;

pub use arrayvec::ArrayVec;
pub use derive_more::{Add, AddAssign, Constructor};
pub use itertools::Itertools;
pub use num::integer::{gcd_lcm, sqrt};
pub use ord_by_key::ord_eq_by_key_selector as ord_by_key;
pub use parse_display::{Display, FromStr};
pub use regex::Regex;
pub use rayon::prelude::*;

pub use grid::*;
mod grid {
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Grid<T> {
        pub vec: Vec<T>,
        pub width: usize,
    }

    impl<T> Grid<T> {
        pub fn height(&self) -> usize {
            self.vec.len() / self.width
        }
        pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
            if x >= self.width { return None; }
            self.vec.get(x + self.width * y)
        }
    }
    
    impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
        type Output = T;
        fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
            &self.vec[x + self.width * y]
        }
    }
    
    impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
        fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
            &mut self.vec[x + self.width * y]
        }
    }
}
