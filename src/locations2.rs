#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use multiset::HashMultiSet;
use super::{regions};
use super::items::Item;

type CanAccessClosure = Fn(&HashMultiSet<Item>) -> bool + Sync;

pub struct Location2 {
  name: &'static str,
  can_access_callback: &'static CanAccessClosure,
}

use std::fmt;
impl fmt::Debug for Location2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Location2 {
  pub fn can_access(&self, items: &HashMultiSet<Item>) -> bool {
    (self.can_access_callback)(&items)
  }
}

const TODO : bool = true;
fn temp_allow_unused(_: &HashMultiSet<Item>) {}

pub static DesertPalaceBigChest : Location2 = Location2 {
  name: "DesertPalaceBigChest",
  can_access_callback: &|items: &HashMultiSet<Item>| -> bool {
    temp_allow_unused(&items); TODO
  },
};
