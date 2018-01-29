#![allow(non_camel_case_types)]

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Zone {
  TempEastLightWorld,
  TempPreSP,

  EP1,
  EP2,
  EP3,
  EP4,
  EP5,
  EP6,
  EP56,

  DP1,
  DP2,
  DP3,
  DP4,
  DP5,
  DP15A,
  DP15B,
  DP15C,
  DP15D,

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

  SP0,
  SP1,
  SP12,
  SP2,
  SP3,
  SP4,
  SP25,
  SP5,
  SP56,
  SP6,

  // TODO: sync
}
pub use self::Zone::*;

pub const STARTING_ZONE: Zone = TempEastLightWorld;

#[allow(dead_code)]
pub const ALL_ZONES: &[Zone] = &[
  TempEastLightWorld,
  TempPreSP,

  EP1,
  EP2,
  EP3,
  EP4,
  EP5,

  DP1,
  DP2,
  DP3,
  DP4,
  DP5,
  DP15A,
  DP15B,
  DP15C,
  DP15D,

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

  SP0,
  SP1,
  SP2,
  SP3,
  SP4,
  SP5,
  SP6,
  SP12,
  SP25,
  SP56,

  // TODO: sync
];
