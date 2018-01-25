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
}
