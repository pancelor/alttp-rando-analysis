#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::collections::VecDeque;
use rand::{Rng, ThreadRng};
use std::hash::{Hash, Hasher};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons};
use super::glue::*;
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::items::Item;
use super::dungeons::Dungeon;
use super::world2::Assignments;
use super::locations2::Location2;
use group_by;

// All `Dive`s are greedy wrt items
// `Dive.explore` expands over itemdoor boundaries, but not keylock boundaries (besides the one door passed as an argument)

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Dive {
  pub zones: HashSet<Zone>,
  pub items: Vec<Item>, // includes big/small keys
  pub open_doors: HashSet<KeyDoor>, // all open keydoors on the entire map, not just the immediately accessible ones
}

// TODO: is this a horrible idea? why aren't HashSets Hash by default??
impl Hash for Dive {
  fn hash<H>(&self, state: &mut H)
    where H: Hasher,
  {
    for entry in self.zones.iter() { entry.hash(state); }
    self.items.hash(state);
    for entry in self.open_doors.iter() { entry.hash(state); }
  }
}

impl Dive {
  /// all reachable keydoors
  pub fn key_frontier(&self) -> HashSet<KeyDoor> {
    self.zones.iter()
      .flat_map(|&zone| keyfrontier_from_zone(zone))
      .collect()
  }

  /// all reachable keydoors, filtered to ones that we have keys for
  pub fn actual_key_frontier(&self) -> HashSet<KeyDoor> {
    let mut dungeons_i_own_keys_for : HashSet<Dungeon> = HashSet::new();
    let all_keys: HashMap<dungeons::Dungeon, Vec<items::Item>> =
      group_by::group_by(
        self.items.clone().into_iter()
          .filter(|&item| items::is_key(item)),
        |&item| dungeon_from_key(item)
      );

    let all_doors: HashMap<dungeons::Dungeon, Vec<zones::KeyDoor>> =
      group_by::group_by(
        self.key_frontier().into_iter(),
        |&keydoor| dungeon_from_keydoor(keydoor)
      );

    for (&dungeon, keydoors) in all_doors.iter() {
      match all_keys.get(&dungeon) {
        Some(keys) => {
          if keydoors.len() < keys.len() {
            dungeons_i_own_keys_for.insert(dungeon);
          }
        },
        None => {},
      }
    }

    all_doors.into_iter()
      .filter(|&(dung, ref _keys)| dungeons_i_own_keys_for.contains(&dung))
      .flat_map(|(_dung, keys)| keys)
      .collect()

    // TODO: use something built on this instead? slower, but idk if the current algo even works lol
    // fn currently_has_key(key: Item, dive: Dive) -> bool {
    //   // TODO: assert key is one of the keys; add a fxn to Item prolly
    //   let num_keys = dive.items.count_of(&key);
    //   let open_dungeon_doors: HashSet<KeyDoor> = keyfrontier_from_dungeon(dungeon_from_key(key)) & dive.open_doors;
    //   num_keys > open_dungeon_doors.len()
    // }
  }

  /// all reachable itemdoors
  pub fn item_frontier(&self) -> VecDeque<ItemDoor> {
    self.zones.iter()
      .flat_map(|&zone| itemfrontier_from_zone(zone))
      .collect()
  }

  pub fn explore_keydoor(&mut self, door: KeyDoor, assignments: &Assignments) {
    self.open_keydoor(door, &assignments);
    self.explore(&assignments);
  }

  pub fn explore(&mut self, assignments: &Assignments) {
    // assumes self is already greedy (i.e. wont re-explore self.zones)
    // to calculate `frontier: HashSet<KeyDoor | ItemDoor> = self.zones.flat_map(.frontier)`

    let mut item_frontier: VecDeque<ItemDoor> = self.item_frontier();

    while item_frontier.len() > 0 {
      let current_edge: ItemDoor = item_frontier.pop_front().expect("not sure what went wrong");
      if !current_edge.can_pass(&self.items) { continue; }
      let zone: Zone = if self.zones.insert(current_edge.zone2) {
        current_edge.zone2
      } else if current_edge.reversible && self.zones.insert(current_edge.zone1) {
        current_edge.zone1
      } else {
        continue;
      };
      for &idoor in itemfrontier_from_zone(zone).iter() {
        // if idoor != current_edge { // not necessary actually; the continue; above will filter it out when we hit it
          item_frontier.push_back(idoor);
        // }
      }
      self.loot_zone(zone, &assignments);
    }
  }

  fn open_keydoor(&mut self, door: KeyDoor, assignments: &Assignments) {
    // sanity check door is in frontier
    let key_frontier: HashSet<KeyDoor> = self.zones.iter()
      .flat_map(|&zone| keyfrontier_from_zone(zone))
      .collect();
    if !key_frontier.contains(&door) { panic!("trying to cross through a door not in the frontier"); }

    // sanity check we have a key to open the door
    // TODO
    // let keys_used : usize = self.open_doors.intersect(door.dungeon.keydoors_for()).len();
    // if count()

    let is_new: bool = self.open_doors.insert(door);
    if !is_new {
      panic!("pretty sure this shouldn't happen but idk i guess");
    }

    // We only want to `insert` the items from unexplored zones (b/c `self.items` is a HashMultiSet)
    let new_zone: Zone;
    if self.zones.insert(door.zone1) {
      new_zone = door.zone1;
    } else if self.zones.insert(door.zone1) {
      new_zone = door.zone2;
    } else {
      panic!("umm something is wrong with the frontier");
    }
    self.loot_zone(new_zone, &assignments);
  }

  fn loot_zone(&mut self, zone: Zone, assignments: &Assignments) {
    // TODO: make self.items a method, and calc it on the fly from zones? makes for easier debugging... yeah lets do it
    locations_from_zone(zone).iter()
      .filter_map(|loc| assignments.get(&loc))
      .for_each(|&item| self.items.push(item));
  }
}
