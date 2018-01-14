#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use multiset::HashMultiSet;
use super::locations;
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
  POD47,
  POD29A,
  POD29B,
}
pub use self::Zone::*;



type CanAccessClosure = Fn(&HashMultiSet<Item>) -> bool + Sync;

pub struct ZoneConnection {
  zone1: Zone,
  zone2: Zone,
  one_way: bool,
  can_pass_callback: &'static CanAccessClosure,
}

impl ZoneConnection {
  pub fn can_pass(&self, items: &HashMultiSet<Item>) -> bool {
    (self.can_pass_callback)(&items)
  }
}

use std::fmt;
impl fmt::Debug for ZoneConnection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let op : &str =
      if (self.one_way) {
        "==>"
      } else {
        "<=>"
      };
    write!(f, "{:?} {} {:?}", self.zone1, op, self.zone2)
  }
}



macro_rules! cxn {
  // Creates a ZoneConnection in a compact form
  ($z1:ident ==> $z2:ident: $cb:expr) => (
    ZoneConnection {
      zone1: $z1,
      zone2: $z2,
      one_way: true,
      can_pass_callback: $cb,
    }
  );
  ($z1:ident <=> $z2:ident: $cb:expr) => (
    ZoneConnection {
      zone1: $z1,
      zone2: $z2,
      one_way: false,
      can_pass_callback: $cb,
    }
  );
  ($z1:ident <k> $z2:ident) => (
    cxn!($z1 <=> $z2: &|ref items| {
      // TODO: check to be sure items has enough keys
      //   hmm... i'll need to change the signature...
      //   need to think more about this ASAP
      todo(items)
    })
  );
}



fn todo(_: &HashMultiSet<Item>) -> bool { true } // for warning suppression

pub static PODCXNS : &[ZoneConnection] = &[
  cxn!(POD1   <k> POD2),
  cxn!(POD1   <=> POD8:   &|ref items| {todo(items) /*Bow!*/}),
  cxn!(POD8   ==> POD2:   &|ref items| {todo(items) /*Hammer*/}),
  cxn!(POD2   <k> POD3),
  cxn!(POD2   <k> POD4),
  cxn!(POD4   <k> POD47),
  cxn!(POD47  <=> POD7:   &|ref items| {todo(items) /*Lamp*/}),
  cxn!(POD4   <=> POD6:   &|ref items| {todo(items) /*Lamp*/}),
  cxn!(POD4   <k> POD5),
  cxn!(POD2   <=> POD29A: &|ref items| {todo(items) /*Bow!, Lamp, Hammer*/}),
  cxn!(POD29A <k> POD29B),
  cxn!(POD29B <=> POD9:   &|ref items| {todo(items) /*BigKeyPOD*/}),
];
