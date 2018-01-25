#![allow(non_camel_case_types)]

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Zone {
  TempEastLightWorld,
  EP1,
  EP2,
  EP3,
  EP4,
  EP5,
  EP6,
  EP56,
  TH1,
  TH2,
  TH3,
  TH4,
  TH12,
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

#[allow(dead_code)]
pub const ALL_ZONES: &[Zone] = &[
  TempEastLightWorld,
  EP1,
  EP2,
  EP3,
  EP4,
  EP5,
  TH1,
  TH2,
  TH3,
  TH4,
  TH12,
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
];
