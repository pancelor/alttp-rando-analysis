//! Various conversion functions
#![allow(unused_imports)]
#![allow(dead_code)]

#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons, connections};
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::dungeons::Dungeon;
use super::locations2::Location2;
use super::items::Item;

pub fn keyfrontier_from_dungeon(dungeon: Dungeon) -> HashSet<KeyDoor> {
  zones_from_dungeon(dungeon).iter()
    .flat_map(|&zone| keyfrontier_from_zone(zone))
    .collect()
}

pub fn dungeon_from_key(key: Item) -> Dungeon {
  use super::items::*;
  use super::dungeons::*;
  match key {
    KeyH1 => panic!("idk"),
    KeyH2 => panic!("idk"),
    KeyP1 => EasternPalace,
    KeyP2 => DesertPalace,
    KeyP3 => TowerOfHera,
    KeyD1 => PalaceOfDarkness,
    KeyD2 => SwampPalace,
    KeyD3 => SkullWoods,
    KeyD4 => ThievesTown,
    KeyD5 => IcePalace,
    KeyD6 => MiseryMire,
    KeyD7 => TurtleRock,
    KeyA1 => panic!("idk"),
    KeyA2 => panic!("idk"),
    _     => panic!("bad arg"), // TODO: rm?
  }
}

pub fn dungeon_from_zone(zone: Zone) -> Dungeon {
  use super::dungeons::*;
  use super::zones::*;
  match zone {
    TempEastLightWorld => Overworld,
    POD1 |
    POD2 |
    POD3 |
    POD4 |
    POD5 |
    POD6 |
    POD7 |
    POD8 |
    POD9 |
    POD10 |
    POD47 |
    POD29A |
    POD29B => PalaceOfDarkness,
  }
}

pub fn zones_from_dungeon(dungeon: Dungeon) -> HashSet<Zone> {
  zones::all().into_iter()
    .filter(|&zone| dungeon_from_zone(zone) == dungeon)
    .collect()
}

pub fn keyfrontier_from_zone(zone: Zone) -> HashSet<KeyDoor> {
  connections::ALL_KEYDOORS.clone().into_iter() // TODO: ew ew awful perf here
    .filter(|&idoor| idoor.zone1 == zone || idoor.zone2 == zone)
    .map(|&x| x)
    .collect()
}

pub fn itemfrontier_from_zone(zone: Zone) -> Vec<ItemDoor> {
  connections::ALL_ITEMDOORS.clone().into_iter() // TODO: ew ew awful perf here
    .filter(|&idoor| idoor.zone1 == zone || idoor.zone2 == zone)
    .map(|&x| x)
    .collect()
}

pub fn locations_from_zone(zone: Zone) -> HashSet<Location2> {
  use super::zones::*;
  use super::locations2::*;
  match zone {
    TempEastLightWorld => hashset!{
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
