#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use super::locations;
use super::dungeons;
use super::items::Item;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Zone {
  TempEastLightWorld,
  POD1,
  POD2,
  POD3,
  POD4,
  POD5,
  POD6,
  POD7,
  POD8,
  POD9,
  POD10,
  POD47,
  POD29A,
  POD29B,
  // TODO: sync
}
pub use self::Zone::*;

pub fn all() -> HashSet<Zone> {
  hashset!{
    TempEastLightWorld,
    POD1,
    POD2,
    POD3,
    POD4,
    POD5,
    POD6,
    POD7,
    POD8,
    POD9,
    POD10,
    POD47,
    POD29A,
    POD29B,
    // TODO: sync
  }
}

type CanPassClosure = Fn(&Vec<Item>) -> bool + Sync;

#[derive(Copy, Clone)]
pub struct ItemDoor {
  pub zone1: Zone,
  pub zone2: Zone,
  pub reversible: bool,
  pub can_pass_callback: &'static CanPassClosure,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct KeyDoor {
  pub zone1: Zone,
  pub zone2: Zone,
  // pub dungeon: dungeons::Dungeon,
}

impl ItemDoor {
  pub fn can_pass(&self, items: &Vec<Item>) -> bool {
    (self.can_pass_callback)(&items)
  }
}

use std::fmt;
impl fmt::Debug for ItemDoor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let op : &str =
      if self.reversible {
        "<=>"
      } else {
        "==>"
      };
    write!(f, "{:?} {} {:?}", self.zone1, op, self.zone2)
  }
}
