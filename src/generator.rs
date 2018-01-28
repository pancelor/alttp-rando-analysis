#![allow(unused_imports)]

use std::collections::{HashMap, HashSet, BTreeSet};
use rand::{Rng, ThreadRng};
use super::{medallions, logic, locations2, items, zones, dungeons};
use super::locations2::*;
use super::stats::*;
use super::items::Item;
use super::world::World;
use super::zones::Zone;
use super::connections::*;
use super::dungeons::*;
use super::dive::Dive;

pub fn generate_world(
  advancement_items: Vec<Item>,
  dungeon_items: Vec<Item>,
  mut junk_items: Vec<Item>,
  // TODO: add back in nice_items and do the ganon tower pre-fill thing
  rng: &mut ThreadRng,
) -> World {
  trace!("fn generate_world(\nadvancement_items={:?},\n\tjunk_items={:?},\n\trng\n)", advancement_items, junk_items);
  let medallions;
  { // Set up medallions
    let all_meds = medallions::get_all_medallions();
    medallions = medallions::EntranceConfig {
      turtle_rock: *rng.choose(&all_meds).expect("empty medallion array"),
      misery_mire: *rng.choose(&all_meds).expect("empty medallion array"),
    };
  }

  let mut world = World::new(medallions);

  world.fill_presets(WG.get_presets());

  { // Set up assignments
    { // Place prizes
      // TODO: bring back old code here
      world.assign(EasternPalacePrize, items::BeatEP);
      world.assign(DesertPalacePrize, items::BeatDP);
      world.assign(TowerOfHeraPrize, items::BeatTH);
      // world.assign(SkullWoodsPrize, items::BeatSW);
      // world.assign(ThievesTownPrize, items::BeatTT);
      // world.assign(MiseryMirePrize, items::BeatMM);
      // world.assign(SwampPalacePrize, items::BeatSP);
      // world.assign(IcePalacePrize, items::BeatIP);
      world.assign(PalaceOfDarknessPrize, items::BeatPOD);
      // world.assign(TurtleRockPrize, items::BeatTR);
    }

    rng.shuffle(&mut junk_items);
    let mut junk_items_iter = junk_items.into_iter();

    // The code from here to the end is based on RandomAssumed.php

    let mut randomized_order_locations = locations2::get_all_locations();
    rng.shuffle(&mut randomized_order_locations);

    fill_items_in_locations(dungeon_items, &randomized_order_locations, &advancement_items, &mut world);

    // TODO: very old code; re-implement at some point
    // // { // put some junk in ganon
    // //   let num_junk_items = rng.next_u32() % 16;
    // //   let ganon_locs: Vec<locations::Location> = regions::get_locations_for(regions::Region::GanonsTower).into_iter()
    // //     .filter(|loc| world.assignments.get(loc) == None)
    // //     .take(num_junk_items as usize)
    // //     .collect();
    // //   fast_fill_items_in_locations(&mut junk_items_iter, &ganon_locs, &mut world.assignments);
    // // }

    randomized_order_locations.reverse();

    fill_items_in_locations(advancement_items, &randomized_order_locations, &vec![], &mut world);

    // rng.shuffle(&mut randomized_order_locations); // TODO: does this even do anything?

    fast_fill_items_in_locations(&mut junk_items_iter, &randomized_order_locations, &mut world);

    assert_eq!(world.num_assignments(), randomized_order_locations.len(), "Item/Loc count mismatch");
  }

  world
}

use std::vec::IntoIter;
fn fast_fill_items_in_locations(
  fill_items: &mut IntoIter<Item>,
  locations: &Vec<Location2>,
  world: &mut World,
) {
  trace!("fn fast_fill_items_in_locations(\nfill_items={:?},\n\tlocations={:?},\n\tworld{:?}\n)", fill_items, locations, world);
  for &loc in locations.iter() {
    if world.contains_key(&loc) { continue };
    match fill_items.next() {
      Some(item) => world.assign(loc, item),
      None => break,
    };
  }
}

fn fill_items_in_locations(
  mut remaining_fill_items: Vec<Item>,
  locations: &Vec<Location2>,
  base_assumed_items: &Vec<Item>,
  world: &mut World,
) {
  trace!("fn fill_items_in_locations(\n\tfill_items={:?},\n\tlocations={:?},\n\tbase_assumed_items={:?},\n\tworld{:?}\n)", remaining_fill_items, locations, base_assumed_items, world);
  for _ in 0..remaining_fill_items.len() {
    let item = remaining_fill_items.pop().expect("bad for loop sync");
    let mut assumed_items = base_assumed_items.clone();
    assumed_items.append(&mut (remaining_fill_items.clone()));

    let assumed_items_str = format!("{:?}", assumed_items); // avoid move-checker memes
    let allowed_locations = get_allowed_locations_to_place_next_item(assumed_items, &world);
    debug!("Found locations:\n\titem={:?}\n\tassumed_items={:?}\n\tallowed_locations={:?}\n\tallowed_locations.len()={}", item, assumed_items_str, allowed_locations, allowed_locations.len());

    let loc: &Location2 = locations.iter()
      .filter(|&&loc| !world.contains_key(&loc))
      .filter(|&&loc| allowed_locations.contains(&loc))
      .filter(|&&loc| WG.item_can_be_placed_at(item, loc)) // TODO: do this earlier to save work?
      .next()
      .expect("No locations left");
    world.assign(*loc, item);
  }
}

fn get_allowed_locations_to_place_next_item(
  assumed: Vec<Item>,
  world: &World,
) -> BTreeSet<Location2> {
  trace!("fn get_allowed_locations_to_place_next_item(\n\tassumed={:?},\n\tworld{:?}\n)", assumed, world);
  let first_dive: Dive = Dive::new(assumed, &world);

  let mut num_dives_seen: usize = 0;
  let mut num_duplicate_dives_seen: usize = 0;
  let mut dive_hashes_seen: HashSet<u64> = HashSet::new();

  // The set of Locations that are common to every maximal dive
  let mut common_locs: Option<BTreeSet<Location2>> = None;

  let mut stack: Vec<Dive> = Vec::new();
  stack.push(first_dive);
  while stack.len() > 0 {
    trace!("while stack (\n\tstack={:?},\n\tcommon_locs={:?}\n)", stack, common_locs);
    let current_dive: Dive = stack.pop().expect("umm this is impossible");
    num_dives_seen += 1;

    let never_seen_before = dive_hashes_seen.insert(current_dive.hash_value());
    if !never_seen_before {
      // this is a duplicate
      num_duplicate_dives_seen += 1;
      continue;
    }

    let keyfrontier: BTreeSet<KeyDoor> = current_dive.key_frontier();

    debug!("Popping dive stack (size {}):\n\tdive.zones={:?}\n\tkeyfrontier={:?}\n\tkeycounts={:?}", stack.len()+1, current_dive.zones, keyfrontier, current_dive.keycounts());
    if keyfrontier.len() == 0 {
      // This is a maximal dive; restrict common_locs accordingly
      let current_locs: BTreeSet<Location2> = current_dive.zones.iter()
        .filter_map(|&zone| WG.locations_from_zone(zone))
        .flat_map(|&ref locset| locset)
        .cloned()
        .collect();
      match common_locs {
        None => { common_locs = Some(current_locs); }
        Some(common) => { common_locs = Some(&common & &current_locs); }
      }
      debug!("Restricting common_locs:\n\tcommon_locs={:?}", common_locs);
      continue;
    }

    // Don't fan out into _every_ keydoor available;
    //   only fan out into the ones in the _earliest available dungeon_.
    //   E.g. if we have two POD keys and two EP keys; only fan out into the two
    //   EP keydoors, because that dungeon comes earlier (in vanilla).
    //   We'll end up going to POD later after there are no more EP keys
    //   available.
    let dungeon: &Dungeon = ALL_DUNGEONS.iter()
      .filter(|&&dgn| !WG.keyfrontier_from_dungeon(dgn).is_disjoint(&keyfrontier)) // only keep dungeons with common keydoors
      .next()
      .expect("this dive has no keys and yet is somehow not maximal?");
    let dungeon_keyfrontier = WG.keyfrontier_from_dungeon(*dungeon);
    let doors_to_explore: BTreeSet<KeyDoor> = &dungeon_keyfrontier & &keyfrontier;

    let num_keys: usize = current_dive.keycounts().get(&WG.dungeon_info(*dungeon).key).expect("bad key").clone();
    let have_all_keys = dungeon_keyfrontier.len() == num_keys;

    debug!("key stats: num_keys={}/{}", num_keys, dungeon_keyfrontier.len());
    // If we have every key for the given dungeon; don't fan out at all;
    //   just explore every available keydoor. This is an optimization;
    //   setting `have_all_keys=false` will make the algorithm slower but
    //   still correct
    if have_all_keys {
      let mut current_dive = current_dive;
      for (ii, &door) in doors_to_explore.iter().enumerate() {
        debug!("Have all keys; exploring door {}/{}: {:?}", ii+1, doors_to_explore.len(), door);
        current_dive.explore_keydoor(door, &world);
      }
      debug!("Pushing dive stack (special: have all keys)");
      stack.push(current_dive);
    } else {
      for (ii, &door) in doors_to_explore.iter().enumerate() {
        debug!("Pushing dive stack ({}/{}): {:?}", ii+1, doors_to_explore.len(), door);
        let mut new_dive: Dive = current_dive.clone();
        new_dive.explore_keydoor(door, &world);
        stack.push(new_dive);
      }
    }
  }

  DIVES.lock().unwrap().record(num_dives_seen, num_duplicate_dives_seen);
  common_locs.expect("there are no locations common to every maximal dive")
}


// misfit functions


// note: doesn't handle multiples; e.g. it will say you can collect 2 bottles even if you can only collect 1
pub fn can_collect(world: &World, items: &HashSet<Item>) -> bool {
  let reachable_locs = get_allowed_locations_to_place_next_item(vec![], &world);
  let reachable_items: HashSet<Item> = reachable_locs.iter()
    .filter_map(|&loc| world.get(&loc))
    .cloned()
    .collect();
  &(&reachable_items & items) == items
}
