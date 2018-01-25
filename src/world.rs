#![allow(unused_imports)]

use std::collections::HashMap;
use super::locations2::*;
use super::medallions;
use super::items::Item;

pub type Assignments = HashMap<Location2, Item>;

#[derive(Eq, PartialEq)]
pub struct World {
  assignments: Assignments,
  medallions: medallions::EntranceConfig,
}

use std::fmt;
impl fmt::Debug for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "World {{\n\tmedallons: {:?}\n\tassignments={{", self.medallions)?;
    for &loc in get_all_locations().iter() {
      let item = match self.assignments.get(&loc) {
        Some(it) => format!("{:?}", it),
        None => format!("<none>"),
      };
      writeln!(f, "\t\t{:?}={:?}", loc, item)?;
    }
    write!(f, "\t}}\n}}")
  }
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

  // meh
  pub fn num_assignments(&self) -> usize {
    self.assignments.len()
  }

  pub fn assign(&mut self, loc: Location2, item: Item) {
    if self.assignments.contains_key(&loc) {
      debug!("About to panic:\n\titem={:?}\n\tloc={:?}\n\tcurrent={:?}", item, loc, self.assignments.get(&loc));
      panic!("Trying to overwrite an item assignment");
    }

    info!("Filling {:?} with {:?}", loc, item);
    self.assignments.insert(loc, item);
  }
}
