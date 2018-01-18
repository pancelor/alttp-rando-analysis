#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, BTreeSet};
use super::locations;
use super::dungeons;
use super::items::*;
use super::zones::*;


type CanPassClosure = Fn(&Vec<Item>) -> bool + Sync;

#[derive(Copy, Clone)]
pub struct ItemDoor {
  pub zone1: Zone,
  pub zone2: Zone,
  pub reversible: bool,
  pub can_pass_callback: &'static CanPassClosure,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
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
impl fmt::Debug for KeyDoor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} <k> {:?}", self.zone1, self.zone2)
  }
}

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

macro_rules! cxn {
  ($z1:ident ==> $z2:ident: $cb:expr) => ({
    ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: false,
      can_pass_callback: $cb,
    }
  });
  ($z1:ident <=> $z2:ident: $cb:expr) => ({
    ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: true,
      can_pass_callback: $cb,
    }
  });
  ($z1:ident ==> $z2:ident) => (
    cxn!($z1 ==> $z2: &|ref _items| true);
  );
  ($z1:ident <=> $z2:ident) => (
    cxn!($z1 <=> $z2: &|ref _items| true);
  );
  ($z1:ident <k> $z2:ident) => ({
    KeyDoor {
      zone1: $z1,
      zone2: $z2,
      // dungeon: dungeons::EasternPalace, // TODO?
    }
  });
}



// fn todo(_: &Vec<Item>) -> bool { true } // for warning suppression

pub static ALL_ITEMDOORS : &[ItemDoor] = &[
  cxn!(TempEastLightWorld <=> POD1),
  cxn!(POD1   <=> POD8:   &|ref items| { items.contains(&Bow) }),
  cxn!(POD8   ==> POD2:   &|ref items| { items.contains(&Hammer) }),
  cxn!(POD47  <=> POD7:   &|ref items| { items.contains(&Lamp) }),
  cxn!(POD7   <=> POD10:  &|ref items| { items.contains(&BigKeyD1) }),
  cxn!(POD4   <=> POD6:   &|ref items| { items.contains(&Lamp) }),
  cxn!(POD2   <=> POD29A: &|ref items| { items.contains(&Bow) && items.contains(&Lamp) && items.contains(&Hammer) }),
  cxn!(POD29B <=> POD9:   &|ref items| { items.contains(&BigKeyD1) }),
];

pub static ALL_KEYDOORS : &[KeyDoor] = &[
  cxn!(POD1   <k> POD2),
  cxn!(POD2   <k> POD3),
  cxn!(POD2   <k> POD4),
  cxn!(POD4   <k> POD47),
  cxn!(POD4   <k> POD5),
  cxn!(POD29A <k> POD29B),
];
