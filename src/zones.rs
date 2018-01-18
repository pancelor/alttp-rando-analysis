#![allow(non_camel_case_types)]

use std::collections::BTreeSet;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
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

pub fn all() -> BTreeSet<Zone> {
  btreeset!{
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
