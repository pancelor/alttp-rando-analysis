#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use multiset::HashMultiSet;
use super::locations;
use super::dungeons;
use super::items::Item;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Zone {
  TempOverworld,
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
}
pub use self::Zone::*;


pub type Zone = HashSet<Location2>;
type CanPassClosure = Fn(&HashMultiSet<Item>) -> bool + Sync;

#[derive(Eq, PartialEq, Hash)]
pub struct ItemDoor {
  zone1: Zone,
  zone2: Zone,
  one_way: bool,
  can_pass_callback: &'static CanPassClosure,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct KeyDoor {
  z1: Zone,
  z2: Zone,
  dungeon: dungeons::Dungeon,
}

impl ItemDoor {
  pub fn can_pass(&self, items: &HashMultiSet<Item>) -> bool {
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



macro_rules! cxn {
  // Creates a ItemDoor in a compact form
  ($z1:ident ==> $z2:ident: $cb:expr) => (
    ItemDoor {
      zone1: $z1,
      zone2: $z2,
      one_way: true,
      can_pass_callback: $cb,
    }
  );
  ($z1:ident <=> $z2:ident: $cb:expr) => (
    ItemDoor {
      zone1: $z1,
      zone2: $z2,
      one_way: false,
      can_pass_callback: $cb,
    }
  );
  ($z1:ident ==> $z2:ident) => (
    cxn!($z1 ==> $z2: &|ref _items| true)
  );
  ($z1:ident <=> $z2:ident) => (
    cxn!($z1 <=> $z2: &|ref _items| true)
  );
  ($z1:ident <k> $z2:ident) => (
    KeyDoor {
      zone1: $z1,
      zone2: $z2,
      dungeon: dungeons::EasternPalace, // TODO
    }
  );
}



fn todo(_: &HashMultiSet<Item>) -> bool { true } // for warning suppression

pub static ITEMDOORS : &[ItemDoor] = &[
  cxn!(TempOverworld <=> POD1),
  cxn!(POD1   <=> POD8:   &|ref items| {todo(items) /*Bow!*/}),
  cxn!(POD8   ==> POD2:   &|ref items| {todo(items) /*Hammer*/}),
  cxn!(POD47  <=> POD7:   &|ref items| {todo(items) /*Lamp*/}),
  cxn!(POD7   <=> POD10:  &|ref items| {todo(items) /*BigKeyPOD*/}),
  cxn!(POD4   <=> POD6:   &|ref items| {todo(items) /*Lamp*/}),
  cxn!(POD2   <=> POD29A: &|ref items| {todo(items) /*Bow!, Lamp, Hammer*/}),
  cxn!(POD29B <=> POD9:   &|ref items| {todo(items) /*BigKeyPOD*/}),
];

pub static KEYDOORS : &[KeyDoor] = &[
  cxn!(POD1   <k> POD2),
  cxn!(POD2   <k> POD3),
  cxn!(POD2   <k> POD4),
  cxn!(POD4   <k> POD47),
  cxn!(POD4   <k> POD5),
  cxn!(POD29A <k> POD29B),
];
