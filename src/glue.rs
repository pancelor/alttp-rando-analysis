//! Various conversion functions
#![allow(unused_imports)]
#![allow(dead_code)]

#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons};
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::dungeons::Dungeon;
use super::locations2::Location2;
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
  vec![] // TODO: impl
}

pub fn locations_from_zone(zone: Zone) -> HashSet<Location2> {
  hashset!{} // TODO: impl
}



pub fn dungeon_from_keydoor(keydoor: KeyDoor) -> Dungeon {
  dungeon_from_zone(keydoor.zone1)
}
