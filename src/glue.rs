//! Various conversion functions
#![allow(unused_imports)]

use std::collections::BTreeSet;
use super::{medallions, logic, locations2, items, zones, dungeons, connections};
use super::zones::Zone;
use super::connections::{KeyDoor, ItemDoor};
use super::dungeons::Dungeon;
use super::locations2::Location2;
use super::items::Item;

pub fn keyfrontier_from_dungeon(dungeon: Dungeon) -> BTreeSet<KeyDoor> {
  zones_from_dungeon(dungeon).iter()
    .flat_map(|&zone| keyfrontier_from_zone(zone))
    .collect()
}

#[allow(dead_code)] // TODO: rm?
pub fn dungeon_from_key(key: Item) -> Option<Dungeon> {
  use super::items::*;
  use super::dungeons::*;
  match key {
    KeyH1 => None, // TODO: change later
    KeyH2 => None, // TODO: change later
    KeyP1 => Some(EasternPalace),
    KeyP2 => Some(DesertPalace),
    KeyP3 => Some(TowerOfHera),
    KeyD1 => Some(PalaceOfDarkness),
    KeyD2 => Some(SwampPalace),
    KeyD3 => Some(SkullWoods),
    KeyD4 => Some(ThievesTown),
    KeyD5 => Some(IcePalace),
    KeyD6 => Some(MiseryMire),
    KeyD7 => Some(TurtleRock),
    KeyA1 => None, // TODO: change later
    KeyA2 => None, // TODO: change later
    _     => panic!("bad arg"), // TODO: rm? return None?
  }
}

pub fn key_from_dungeon(dungeon: Dungeon) -> Item {
  use super::items::*;
  use super::dungeons::*;
  match dungeon {
    EasternPalace => KeyP1,
    DesertPalace => KeyP2,
    TowerOfHera => KeyP3,
    PalaceOfDarkness => KeyD1,
    SwampPalace => KeyD2,
    SkullWoods => KeyD3,
    ThievesTown => KeyD4,
    IcePalace => KeyD5,
    MiseryMire => KeyD6,
    TurtleRock => KeyD7,
  }
}

pub fn dungeon_from_zone(zone: Zone) -> Option<Dungeon> {
  use super::dungeons::*;
  use super::zones::*;
  match zone {
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
    POD29B => Some(PalaceOfDarkness),
    _ => None,
  }
}

pub fn zones_from_dungeon(dungeon: Dungeon) -> BTreeSet<Zone> {
  zones::all().into_iter()
    .filter(|&zone| dungeon_from_zone(zone) == Some(dungeon))
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
    .filter(|&idoor| idoor.zone1 == zone || (idoor.zone2 == zone && idoor.reversible))
    .cloned()
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
  dungeon_from_zone(keydoor.zone1).expect("your keydoor is outside")
}
