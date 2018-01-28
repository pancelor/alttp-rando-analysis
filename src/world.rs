#![allow(unused_imports)]

use std::collections::HashMap;
use super::locations2::*;
use super::medallions;
use super::items::Item;
use super::items;

#[derive(Eq, PartialEq)]
pub struct World {
  assignments: HashMap<Location2, Item>,
  medallions: medallions::EntranceConfig,
}

use std::fmt;
impl fmt::Debug for World {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "World {{\n\tmedallons: {:?}\n\tassignments={{", self.medallions)?;
    let mut write_later1 = Vec::new();
    let mut write_later2 = Vec::new();
    for &loc in get_all_locations().iter() {
      match self.assignments.get(&loc) {
        Some(&item) if item == items::TEMP_JUNK => write_later1.push(loc),
        Some(&item) => writeln!(f, "\t\t{:?}={:?}", loc, item)?,
        None => write_later2.push(loc),
      }
    }
    for &loc in write_later1.iter() {
      writeln!(f, "\t\t{:?}={:?}", loc, "-")?;
    }
    for &loc in write_later2.iter() {
      writeln!(f, "\t\t{:?}={:?}", loc, "***none***")?;
    }
    write!(f, "\t}}\n}}")
  }
}

impl World {
  pub fn new(medallions: medallions::EntranceConfig) -> Self {
    let assignments = HashMap::new();
    Self {
      assignments,
      medallions
    }
  }

  pub fn get(&self, loc: &Location2) -> Option<&Item> {
    self.assignments.get(loc)
  }

  // "key" as in HashMap.contains_key; not a zelda item
  pub fn contains_key(&self, loc: &Location2) -> bool {
    self.assignments.contains_key(loc)
  }

  pub fn fill_presets(&mut self, presets: &HashMap<Location2, Item>) {
    for (&loc, &item) in presets {
      self.assign(loc, item);
    }
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


// misfit functions


impl World {
  pub fn num_assignments(&self) -> usize {
    self.assignments.len()
  }
}
