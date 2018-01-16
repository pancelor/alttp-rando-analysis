//! Various conversion functions
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons};
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::dungeons::Dungeon;
use super::items::Item;

pub fn keyfrontier_from_dungeon(dungeon: Dungeon) -> HashSet<KeyDoor> {
  hashset!{} // TODO: impl
}

pub fn dungeon_from_key(key: Item) -> Dungeon {
  if !items::all_small_keys().contains(key) {
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

pub fn itemfrontier_from_zone(zone: Zone) -> HashSet<ItemDoor> {
  hashset!{} // TODO: impl
}



pub fn dungeon_from_keydoor(keydoor: zones::KeyDoor) -> Dungeon {
  dungeon_from_zone(keydoor.zone1)
}
