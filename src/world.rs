#![allow(unused_imports)]

use std::collections::HashMap;
use super::locations2::Location2;
use super::medallions;
use super::items::Item;

pub type Assignments = HashMap<Location2, Item>;

#[derive(Eq, PartialEq, Debug)]
pub struct World {
  assignments: Assignments,
  medallions: medallions::EntranceConfig,
}

impl World {
  pub fn new(medallions: medallions::EntranceConfig) -> Self {
    let assignments = hashmap!{};
    Self {
      assignments,
      medallions
    }
  }

  pub fn get(&self, loc: &Location2) -> Option<&Item> {
    self.assignments.get(loc)
  }

  pub fn contains_key(&self, loc: &Location2) -> bool {
    self.get(loc).is_some()
  }

  pub fn assign(&mut self, loc: Location2, item: Item) {
    if self.assignments.contains_key(&loc) {
      debug!("About to panic:\n\titem={:?}\n\tloc={:?}\n\tcurrent={:?}", item, loc, self.assignments.get(&loc));
      panic!("Trying to overwrite an item assignment");
    }

    self.assignments.insert(loc, item);
  }
}

// TODO: add .assign method
//   kill pub assignments
//   make assignments Options
//   preload assignments with keys etc
