#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};
use rand::{Rng, ThreadRng};
use super::{medallions, logic, world2, locations2, regions, items, zones, dungeons};
use super::glue::*;
use super::locations2::*;
use super::world2::Assignments;
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::dive::Dive;

pub fn generate_world(
  advancement_items: &Vec<items::Item>,
  junk_items: &Vec<items::Item>,
  rng: &mut ThreadRng,
) -> world2::World2 {
  let mut assignments;
  { // Set up assignments
    assignments = HashMap::new();

    let advancement_items_iter;
    let mut junk_items_iter;
    { // init item iterators
      let mut advancement_items_clone = advancement_items.clone();
      rng.shuffle(&mut advancement_items_clone);
      advancement_items_iter = advancement_items_clone.into_iter();

      let mut junk_items_clone = junk_items.clone();
      rng.shuffle(&mut junk_items_clone);
      junk_items_iter = junk_items_clone.into_iter();
    }

    // The code from here to the end is based on RandomAssumed.php

    let mut randomized_order_locations = locations2::get_all_locations();
    rng.shuffle(&mut randomized_order_locations);
    trace!("randomized_order_locations: {:?}", randomized_order_locations);

    fill_items_in_locations(advancement_items_iter, &randomized_order_locations, &vec![], &mut assignments);

    // rng.shuffle(&mut randomized_order_locations); // TODO: does this even do anything?

    fast_fill_items_in_locations(&mut junk_items_iter, &randomized_order_locations, &mut assignments);
    assert_eq!(junk_items_iter.next(), None);
  }

  let world = world2::World2 {
    assignments,
  };
  world
}

fn fast_fill_items_in_locations(
  fill_items: &mut IntoIter<items::Item>,
  locations: &Vec<locations2::Location2>,
  assignments: &mut Assignments,
) {
  for &loc in locations.iter() {
    if assignments.contains_key(&loc) { continue };
    match fill_items.next() {
      Some(item) => {
        debug!("Filling {:?} with {:?}", loc, item);
        assignments.insert(loc, item)
      },
      None => break,
    };
  }
}

use std::vec::IntoIter;
#[allow(unused_variables)]
fn fill_items_in_locations(
  fill_items: IntoIter<items::Item>,
  locations: &Vec<locations2::Location2>,
  base_assumed_items: &Vec<items::Item>,
  assignments: &mut Assignments,
) {
  let mut remaining_fill_items: Vec<items::Item> = fill_items.collect();
  for _ in 0..remaining_fill_items.len() {
    let item = remaining_fill_items.pop().expect("bad for loop sync");
    let mut assumed_items = base_assumed_items.clone();
    assumed_items.append(&mut (remaining_fill_items.clone()));

    place_item(item, assumed_items, &locations, &mut assignments);
  }
}

fn place_item(
  item: items::Item,
  assumed: Vec<items::Item>,
  locations: &Vec<locations2::Location2>,
  assignments: &mut Assignments,
) {
  let mut firstDive: Dive = Dive{
    zones: hashset!{Zone::Overworld},
    items: assumed,
    openDoors: HashSet::new(),
  };
  firstDive.explore(&assignments);
  let mut queue: VecDeque<Dive> = VecDeque::new();
  queue.push_back(firstDive);
  let mut maximal_dives: HashSet<Dive> = HashSet::new();

  while queue.len() > 0 {
    let v: Dive = queue.pop_front().expect("idk man");
    let mut f: HashSet<KeyDoor> = v.actual_key_frontier();
    if f.len() == 0 {
      maximal_dives.insert(v);
      continue;
    }

    let dungeon : &dungeons::Dungeon = dungeons::all().iter()
      .filter(|&&dgn| !(&keyfrontier_from_dungeon(dgn) & &f).is_empty())
      .next()
      .expect("no dungeons or something");
    let temp_g: HashSet<KeyDoor> = keyfrontier_from_dungeon(*dungeon);
    let doors_to_explore: HashSet<KeyDoor> = &temp_g & &f;
    for door in doors_to_explore {
      let mut new_dive: Dive = v.clone();
      new_dive.explore_keydoor(door, &assignments);
      queue.push_back(new_dive);
    }
  }

  // glbZone := intersection(maximal_dives)
  let mut glbZone: Option<HashSet<Location2>> = None;
  for dive in maximal_dives.iter() {
    let new_locs: HashSet<Location2> = dive.zones.iter()
      .flat_map(|&zone| locations_from_zone(zone))
      .collect();
    match glbZone {
      None => { glbZone = Some(new_locs); }
      Some(glb) => { glbZone = Some(&glb & &new_locs); }
    }
  }
  let glbZone = glbZone.expect("no available locations");

  let loc: &Location2 = locations.iter()
    .filter(|&&loc| glbZone.contains(&loc))
    .next()
    .expect("No locations left");
  assignments.insert(*loc, item);
}
