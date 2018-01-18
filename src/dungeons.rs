#![allow(non_camel_case_types)]

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub enum Dungeon {
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
  // TODO: aga tower, escape (apparently there are 4 nondungeon keys - A1, A2, H1, and H2)
  // TODO: sync
}
pub use self::Dungeon::*;

pub const ALL: &[Dungeon] = &[
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
  // TODO: sync
];
