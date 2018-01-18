//! Various conversion functions
#![allow(unused_imports)]
#![allow(dead_code)]

#![allow(unused_variables)]

use std::collections::{HashMap, BTreeSet};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons, connections};
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::dungeons::Dungeon;
use super::locations2::Location2;
use super::items::Item;

pub fn keyfrontier_from_dungeon(dungeon: Dungeon) -> BTreeSet<KeyDoor> {
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

pub fn zones_from_dungeon(dungeon: Dungeon) -> BTreeSet<Zone> {
  zones::all().into_iter()
    .filter(|&zone| dungeon_from_zone(zone) == dungeon)
    .collect()
}

pub fn keyfrontier_from_zone(zone: Zone) -> BTreeSet<KeyDoor> {
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

pub fn locations_from_zone(zone: Zone) -> BTreeSet<Location2> {
  use super::zones::*;
  use super::locations2::*;
  match zone {
    TempEastLightWorld => btreeset!{
      TempOverworld1,
      TempOverworld2,
      TempOverworld3,
      TempOverworld4,
      TempOverworld5,
    },
    POD1 => btreeset!{
      PalaceOfDarknessShooterRoom,
    },
    POD2 => btreeset!{
      PalaceOfDarknessStalfosBasement,
      PalaceOfDarknessTheArenaBridge,
    },
    POD3 => btreeset!{
      PalaceOfDarknessBigKeyChest,
    },
    POD4 => btreeset!{
      PalaceOfDarknessCompassChest,
    },
    POD5 => btreeset!{
      PalaceOfDarknessHarmlessHellway,
    },
    POD6 => btreeset!{
      PalaceOfDarknessDarkBasementLeft,
      PalaceOfDarknessDarkBasementRight,
    },
    POD7 => btreeset!{
      PalaceOfDarknessDarkMazeTop,
      PalaceOfDarknessDarkMazeBottom,
    },
    POD8 => btreeset!{
      PalaceOfDarknessTheArenaLedge,
      PalaceOfDarknessMapChest,
    },
    POD9 => btreeset!{
      PalaceOfDarknessHelmasaurKing,
      PalaceOfDarknessPrize,
    },
    POD10 => btreeset!{
      PalaceOfDarknessBigChest,
    },
    POD47 => btreeset!{},
    POD29A => btreeset!{},
    POD29B => btreeset!{},
  }
}



pub fn dungeon_from_keydoor(keydoor: KeyDoor) -> Dungeon {
  dungeon_from_zone(keydoor.zone1)
}
