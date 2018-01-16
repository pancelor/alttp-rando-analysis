#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use rand::{Rng, ThreadRng};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons};
use super::glue::*;
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::items::Item;
use super::locations2::Location2;

// All `Dive`s are greedy wrt items
// `Dive.explore` expands over itemdoor boundaries, but not keylock boundaries (besides the one door passed as an argument)

type Assignments = HashMap<Location2, Item>;

struct Dive {
  zones: HashSet<Zone>,
  items: HashMultiSet<Item>, // includes big/small keys
  openDoors: HashSet<KeyDoor>, // all open keydoors on the entire map, not just the immediately accessible ones
}

impl Dive {
  /// all reachable keydoors
  fn key_frontier(&self) -> HashSet<KeyDoor> {
    self.zones.iter()
      .flat_map(|zone| keyfrontier_from_zone(zone))
      .collect()
  }

  /// all reachable keydoors, filtered to ones that we have keys for
  fn actual_key_frontier(&self) -> HashSet<KeyDoor> {
    let mut dungeons_I_own_keys_for : HashSet<Dungeon> = HashSet::new();
    let all_keys: HashMap<dungeons::Dungeon, Vec<items::Item>> = self.items.iter()
      .filter(|&&item| items::is_key(&item))
      .group_by(|&&item| dungeon_from_key(item))
      .collect();

    let all_doors: HashMap<dungeons::Dungeon, Vec<zones::KeyDoor>> = self.key_frontier().iter()
      .group_by(|&&keydoor| dungeon_from_keydoor(keydoor))
      .collect();

    for (dungeon, keydoors) in all_doors {
      match all_keys.get(&dungeon) {
        Some(keys) => {
          if keydoors.len() < keys.len() {
            dungeons_I_own_keys_for.insert(dungeon)
          }
        },
        None => {},
      }
    }

    all_doors.iter()
      .filter(|&&(dung, keys)| dungeons_I_own_keys_for.contains(&dung))
      .flat_map(|&&(dung, keys)| keys)
      .collect()

    // TODO: use something built on this instead? slower, but idk if the current algo even works lol
    // fn currently_has_key(key: Item, dive: Dive) -> bool {
    //   // TODO: assert key is one of the keys; add a fxn to Item prolly
    //   let num_keys = dive.items.count_of(&key);
    //   let open_dungeon_doors: HashSet<KeyDoor> = keyfrontier_from_dungeon(dungeon_from_key(key)) & dive.openDoors;
    //   num_keys > open_dungeon_doors.len()
    // }
  }

  /// all reachable itemdoors
  fn item_frontier(&self) -> HashSet<ItemDoor> {
    self.zones.iter()
      .flat_map(|zone| itemfrontier_from_zone(zone))
      .collect()
  }

  fn explore_keydoor(&mut self, door: Door, assignments: &Assignments) {
    self.open_keydoor(door);
    self.explore(&assignments);
  }

  fn explore(&mut self, assignments: &Assignments) {
    // assumes self is already greedy (i.e. wont re-explore self.zones)
    // to calculate `frontier: HashSet<KeyDoor | ItemDoor> = self.zones.flat_map(.frontier)`

    let mut item_frontier: VecDeque<ItemDoor> = self.zones.iter()
      .flat_map(|zone| itemfrontier_from_zone(zone))
      .collect();
      // TODO: filter out ones where both sides are already in self.zones?

    while item_frontier.len() > 0 {
      let current_edge: ItemDoor = item_frontier.pop_front();
      if !current_edge.canPass(&self.items) { continue; }
      let zone: Zone = if self.zones.insert(current_edge.zone2) {
        current_edge.zone2
      } else if idoor.reversible && self.zones.insert(current_edge.zone1) {
        current_edge.zone1
      } else {
        continue;
      };
      for idoor in itemfrontier_from_zone(zone) {
        if idoor != current_edge { // not necessary actually; the continue; above will filter it out when we hit it
          item_frontier.push_back(idoor);
        }
      }
      zone.iter()
        .filter_map(|loc| assignments.get(&loc))
        .for_each(|item| self.items.insert(item));
    }
  }

  fn open_keydoor(&mut self, door: KeyDoor) {
    // sanity check door is in frontier
    let key_frontier: HashSet<KeyDoor> = self.zones.iter().flat_map(|z| keyfrontier_from_zone(z)).collect();
    if !key_frontier.contains(door) { panic!("trying to cross through a door not in the frontier"); }

    // sanity check we have a key to open the door
    // TODO
    // let keys_used : usize = self.openDoors.intersect(door.dungeon.keydoors_for()).len();
    // if count()


    // We only want to `insert` the items from unexplored zones (b/c `self.items` is a HashMultiSet)
    let new_zone: Zone = if self.zones.insert(door.zone1) {
      door.zone1
    } else if self.zones.insert(door.zone1)
      door.zone2
    } else {
      panic!("umm something is wrong with the frontier");
    }
    // TODO: make self.items a method, and calc it on the fly from zones? makes for easier debugging... yeah lets do it
    self.zones.insert(new_zone);
    new_zone.iter()
      .filter_map(|loc| assignments.get(&loc))
      .for_each(|item| self.items.insert(item));
  }
}
