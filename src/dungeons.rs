#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::locations;
use super::items::Item;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Dungeon {
  Overworld,

  EasternPalace,
  DesertPalace,
  TowerOfHera,
  SkullWoods,
  ThievesTown,
  MiseryMire,
  SwampPalace,
  IcePalace,
  PalaceOfDarkness,
  TurtleRock,

  GanonSTower,
  // TODO: aga tower, escape (apparently there are 4 nondungeon keys - A1, A2, H1, and H2)
  // TODO: sync with `all`
}
pub use self::Dungeon::*;

pub const ALL: &[Dungeon] = &[
  Overworld,

  EasternPalace,
  DesertPalace,
  TowerOfHera,
  SkullWoods,
  ThievesTown,
  MiseryMire,
  SwampPalace,
  IcePalace,
  PalaceOfDarkness,
  TurtleRock,

  GanonSTower,
  // TODO: sync with enum list
];
