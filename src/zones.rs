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

POD1   <=> POD2:   KeyPOD
POD1   <=> POD8:   Bow
POD8    => POD2:   Hammer
POD2   <=> POD3:   KeyPOD
POD2   <=> POD4:   KeyPOD
POD4   <=> POD47:  KeyPOD
POD47  <=> POD7:   Lamp
POD4   <=> POD6:   Lamp
POD4   <=> POD5:   KeyPOD
POD2   <=> POD29A: Bow and Lamp and Hammer
POD29A <=> POD29B: KeyPOD
POD29B <=> POD9:   BigKeyPOD
