//! Various conversion functions
#![allow(unused_imports)]
#![allow(dead_code)]

#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons};
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::zones::Zone::*;
use super::dungeons::Dungeon;
use super::locations2::Location2;
use super::locations2::Location2::*;
use super::items::Item;

pub fn keyfrontier_from_dungeon(dungeon: Dungeon) -> HashSet<KeyDoor> {
  hashset!{} // TODO: impl
}

pub fn dungeon_from_key(key: Item) -> Dungeon {
  if !items::is_key(key) {
    panic!("bad arg"); // TODO: rm?
  }
  dungeons::EasternPalace // TODO: impl FRD
}

pub fn dungeon_from_zone(zone: Zone) -> Dungeon {
  // TODO: type check to be sure it's an actual dungeon and not overworld?
  dungeons::EasternPalace // TODO: impl FRD
}

pub fn keyfrontier_from_zone(zone: Zone) -> HashSet<KeyDoor> {
  hashset!{} // TODO: impl
}

pub fn itemfrontier_from_zone(zone: Zone) -> Vec<ItemDoor> {
  match zone {
    // ah crap
  }
  vec![] // TODO: impl
}

pub fn locations_from_zone(zone: Zone) -> HashSet<Location2> {
  match zone {
    Overworld => hashset!{
      TempOverworld1,
      TempOverworld2,
      TempOverworld3,
      TempOverworld4,
      TempOverworld5,
    },
    POD1 => hashset!{
      PalaceOfDarknessShooterRoom,
    },
    POD2 => hashset!{
      PalaceOfDarknessStalfosBasement,
      PalaceOfDarknessTheArenaBridge,
    },
    POD3 => hashset!{
      PalaceOfDarknessBigKeyChest,
    },
    POD4 => hashset!{
      PalaceOfDarknessCompassChest,
    },
    POD5 => hashset!{
      PalaceOfDarknessHarmlessHellway,
    },
    POD6 => hashset!{
      PalaceOfDarknessDarkBasementLeft,
      PalaceOfDarknessDarkBasementRight,
    },
    POD7 => hashset!{
      PalaceOfDarknessDarkMazeTop,
      PalaceOfDarknessDarkMazeBottom,
    },
    POD8 => hashset!{
      PalaceOfDarknessTheArenaLedge,
      PalaceOfDarknessMapChest,
    },
    POD9 => hashset!{
      PalaceOfDarknessHelmasaurKing,
      PalaceOfDarknessPrize,
    },
    POD10 => hashset!{
      PalaceOfDarknessBigChest,
    },
    POD47 => hashset!{},
    POD29A => hashset!{},
    POD29B => hashset!{},
  }
}



pub fn dungeon_from_keydoor(keydoor: KeyDoor) -> Dungeon {
  dungeon_from_zone(keydoor.zone1)
}
