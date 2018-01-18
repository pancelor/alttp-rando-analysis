#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::{HashMap, HashSet, BTreeSet};
use rand::{Rng, ThreadRng};
use super::{medallions, logic, locations2, regions, items, zones, dungeons};
use super::glue::*;
use super::locations2::*;
use super::world::{Assignments, World};
use super::zones::{Zone, KeyDoor, ItemDoor};
use super::dive::Dive;

pub fn generate_world(
  advancement_items: &Vec<items::Item>,
  junk_items: &Vec<items::Item>,
  // TODO: add back in nice_items and do the ganon tower pre-fill thing
  rng: &mut ThreadRng,
) -> World {
  debug!("fn generate_world(\nadvancement_items={:?},\n\tjunk_items={:?},\n\trng\n)", advancement_items, junk_items);
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

    fill_items_in_locations(advancement_items_iter, &randomized_order_locations, &vec![], &mut assignments);

    // rng.shuffle(&mut randomized_order_locations); // TODO: does this even do anything?

    fast_fill_items_in_locations(&mut junk_items_iter, &randomized_order_locations, &mut assignments);
    // assert_eq!(junk_items_iter.next(), None); // TODO uncomment?
  }

  let world = World {
    assignments,
  };
  world
}

// TODO: temp fxn
pub fn can_win(world: &World) -> bool {
  let locs = get_allowed_locations_to_place_next_item(vec![], &world.assignments);
  locs.contains(&Location2::PalaceOfDarknessPrize) // TODO: s/PalaceOfDarknessPrize/DefeatGanon/
}

fn fast_fill_items_in_locations(
  fill_items: &mut IntoIter<items::Item>,
  locations: &Vec<locations2::Location2>,
  assignments: &mut Assignments,
) {
  debug!("fn fast_fill_items_in_locations(\nfill_items={:?},\n\tlocations={:?},\n\tassignments={:?}\n)", fill_items, locations, assignments);
  for &loc in locations.iter() {
    if assignments.contains_key(&loc) { continue };
    match fill_items.next() {
      Some(item) => {
        info!("Fast filling {:?} with {:?}", loc, item);
        assignments.insert(loc, item)
      },
      None => break,
    };
  }
}

use std::vec::IntoIter;
fn fill_items_in_locations(
  fill_items: IntoIter<items::Item>,
  locations: &Vec<locations2::Location2>,
  base_assumed_items: &Vec<items::Item>,
  mut assignments: &mut Assignments, // TODO WTF why do we need 2 `mut`s here?? and only here???
) {
  debug!("fn fill_items_in_locations(\n\tfill_items={:?},\n\tlocations={:?},\n\tbase_assumed_items={:?},\n\tassignments={:?}\n)", fill_items, locations, base_assumed_items, assignments);
  let mut remaining_fill_items: Vec<items::Item> = fill_items.collect();
  for _ in 0..remaining_fill_items.len() {
    let item = remaining_fill_items.pop().expect("bad for loop sync");
    let mut assumed_items = base_assumed_items.clone();
    assumed_items.append(&mut (remaining_fill_items.clone()));

    debug!("About to place {:?}", item);
    let allowed_locations = get_allowed_locations_to_place_next_item(assumed_items, &mut assignments);
    let loc: &Location2 = locations.iter()
      .filter(|&&loc| !assignments.contains_key(&loc))
      .filter(|&&loc| allowed_locations.contains(&loc))
      .next()
      .expect("No locations left");
    info!("Filling {:?} with {:?}", loc, item);
    assignments.insert(*loc, item);
  }
}

fn get_allowed_locations_to_place_next_item(
  assumed: Vec<items::Item>,
  assignments: &Assignments,
) -> BTreeSet<Location2> {
  debug!("fn get_allowed_locations_to_place_next_item(\nassumed={:?},\n\tassignments={:?}\n)", assumed, assignments);
  let first_dive: Dive = Dive::new(assumed, &assignments);
  let mut stack: Vec<Dive> = Vec::new();
  stack.push(first_dive);
  // The set of Locations that are common to every maximal dive
  let mut common_locs: Option<BTreeSet<Location2>> = None;

  while stack.len() > 0 {
    debug!("while stack (\n\tstack={:?},\n\tcommon_locs={:?}\n)", stack, common_locs);

    let v: Dive = stack.pop().expect("idk man");
    let f: BTreeSet<KeyDoor> = v.actual_key_frontier();
    if f.len() == 0 {
      // This is a maximal dive; restrict common_locs accordingly
      let locs: BTreeSet<Location2> = v.zones.iter()
        .flat_map(|&zone| locations_from_zone(zone))
        .collect();
      match common_locs {
        None => { common_locs = Some(locs); }
        Some(glb) => { common_locs = Some(&glb & &locs); }
      }
      continue;
    }

    // Don't fan out into _every_ keydoor available;
    //   only fan out into the ones in the _earliest available dungeon_.
    //   E.g. if we have two POD keys and two EP keys; only fan out into the two
    //   EP keydoors, because that dungeon comes earlier (in vanilla).
    //   We'll end up going to POD later after there are no more EP keys
    //   available.
    let dungeon : &dungeons::Dungeon = dungeons::ALL.iter()
      .filter(|&&dgn| !(&keyfrontier_from_dungeon(dgn) & &f).is_empty())
      .next()
      .expect("this dive has no keys");
    debug!("First dungeon w/ openable doors: {:?}", dungeon);

    let doors_to_explore: BTreeSet<KeyDoor> = &keyfrontier_from_dungeon(*dungeon) & &f;
    for door in doors_to_explore {
      let mut new_dive: Dive = v.clone();
      new_dive.explore_keydoor(door, &assignments);
      stack.push(new_dive);
    }
  }

  common_locs.expect("there are no locations common to every maximal dive")
}
