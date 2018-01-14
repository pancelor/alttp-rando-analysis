#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::zones;
use super::zones::*;
use super::items::Item;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Location2 {
  TempOverworld1,
  TempOverworld2,
  TempOverworld3,
  TempOverworld4,
  TempOverworld5,
  TempOverworld6,
  TempOverworld7,
  TempOverworld8,
  TempOverworld9,
  TempOverworld10,
  TempOverworld11,
  TempOverworld12,
  PalaceOfDarknessBigKeyChest,
  PalaceOfDarknessTheArenaLedge,
  PalaceOfDarknessTheArenaBridge,
  PalaceOfDarknessBigChest,
  PalaceOfDarknessCompassChest,
  PalaceOfDarknessHarmlessHellway,
  PalaceOfDarknessStalfosBasement,
  PalaceOfDarknessDarkBasementLeft,
  PalaceOfDarknessDarkBasementRight,
  PalaceOfDarknessMapChest,
  PalaceOfDarknessDarkMazeTop,
  PalaceOfDarknessDarkMazeBottom,
  PalaceOfDarknessShooterRoom,
  PalaceOfDarknessHelmasaurKing,
  PalaceOfDarknessPrize,
}

pub fn get_all_locations() -> Vec<Location2> {
  use self::Location2::*;
  vec![
    TempOverworld1,
    TempOverworld2,
    TempOverworld3,
    TempOverworld4,
    TempOverworld5,
    TempOverworld6,
    TempOverworld7,
    TempOverworld8,
    TempOverworld9,
    TempOverworld10,
    TempOverworld11,
    TempOverworld12,
    PalaceOfDarknessBigKeyChest,
    PalaceOfDarknessTheArenaLedge,
    PalaceOfDarknessTheArenaBridge,
    PalaceOfDarknessBigChest,
    PalaceOfDarknessCompassChest,
    PalaceOfDarknessHarmlessHellway,
    PalaceOfDarknessStalfosBasement,
    PalaceOfDarknessDarkBasementLeft,
    PalaceOfDarknessDarkBasementRight,
    PalaceOfDarknessMapChest,
    PalaceOfDarknessDarkMazeTop,
    PalaceOfDarknessDarkMazeBottom,
    PalaceOfDarknessShooterRoom,
    PalaceOfDarknessHelmasaurKing,
    PalaceOfDarknessPrize,
  ]
}
