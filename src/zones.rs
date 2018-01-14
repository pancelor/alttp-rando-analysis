#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::{locations, items};

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


cxn!(POD1   <=key=> POD2)
cxn!(POD1   <=> POD8:   &|ref items| {return true; /*Bow!*/})
cxn!(POD8    => POD2:   &|ref items| {return true; /*Hammer*/})
cxn!(POD2   <=key=> POD3)
cxn!(POD2   <=key=> POD4)
cxn!(POD4   <=key=> POD47)
cxn!(POD47  <=> POD7:   &|ref items| {return true; /*Lamp*/})
cxn!(POD4   <=> POD6:   &|ref items| {return true; /*Lamp*/})
cxn!(POD4   <=key=> POD5)
cxn!(POD2   <=> POD29A: &|ref items| {return true; /*Bow!, Lamp, Hammer*/})
cxn!(POD29A <=key=> POD29B)
cxn!(POD29B <=> POD9:   &|ref items| {return true; /*BigKeyPOD*/})

pub struct ZoneConnection

macro_rules! cxn {
  // Creates a ZoneConnection in a compact form
  ($z1:ident => $z2:ident: $cb:expr) => {
    pub static $loc_name : Location2 = Location2 {
      name: stringify!($loc_name),
      zone: $zone,
      can_access_callback: $cb,
    };
  }
}