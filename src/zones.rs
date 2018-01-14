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
}

pub use self::Zone::*;
