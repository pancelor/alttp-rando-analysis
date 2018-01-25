#![allow(unused_imports)]

use std::collections::HashMap;
use super::locations2::Location2;
use super::items::Item;

pub type Assignments = HashMap<Location2, Item>;

#[derive(Eq, PartialEq, Debug)]
pub struct World {
  pub assignments: Assignments,
}

impl World {
  pub fn new() -> Self {
    let assignments = hashmap!{

      // TODO
    };
    Self {
      assignments,
    }
  }

  pub fn assign(&mut self, item: Item, loc: Location2) {
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
