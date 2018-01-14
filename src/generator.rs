use std::collections::HashMap;
use rand::{Rng, ThreadRng};
use super::{medallions, logic, world, locations, regions, items};

pub fn generate_world(
  advancement_items: &Vec<items::Item>,
  nice_items: &Vec<items::Item>,
  junk_items: &Vec<items::Item>,
  dungeon_items: &Vec<items::Item>,
  rng: &mut ThreadRng,
) -> world::World {
  let medallions;
  { // Set up medallions
    let all_meds = medallions::get_all_medallions();
    medallions = medallions::EntranceConfig {
      turtle_rock: *rng.choose(&all_meds).expect("empty medallion array"),
      misery_mire: *rng.choose(&all_meds).expect("empty medallion array"),
    };
  }

  let mut assignments;
  { // Set up assignments
    assignments = HashMap::new();
    { // Place prizes
      let mut prizes = vec![
        items::Item::Crystal1,
        items::Item::Crystal2,
        items::Item::Crystal3,
        items::Item::Crystal4,
        items::Item::Crystal5,
        items::Item::Crystal6,
        items::Item::Crystal7,
        items::Item::PendantOfCourage,
        items::Item::PendantOfPower,
        items::Item::PendantOfWisdom,
      ];
      rng.shuffle(&mut prizes);
      let mut iter = prizes.into_iter();
      assignments.insert(locations::Location::TowerOfHeraPrize, iter.next().unwrap());
      assignments.insert(locations::Location::EasternPalacePrize, iter.next().unwrap());
      assignments.insert(locations::Location::DesertPalacePrize, iter.next().unwrap());
      assignments.insert(locations::Location::SkullWoodsPrize, iter.next().unwrap());
      assignments.insert(locations::Location::ThievesTownPrize, iter.next().unwrap());
      assignments.insert(locations::Location::MiseryMirePrize, iter.next().unwrap());
      assignments.insert(locations::Location::SwampPalacePrize, iter.next().unwrap());
      assignments.insert(locations::Location::IcePalacePrize, iter.next().unwrap());
      assignments.insert(locations::Location::PalaceOfDarknessPrize, iter.next().unwrap());
      assignments.insert(locations::Location::TurtleRockPrize, iter.next().unwrap());
      assert!(iter.next() == None);
      // TODO: php has a weird conditional-and-`throw`; is this relevant?
      //   There aren't any restrictions, are there?
      //   e.g. something like "Turtle Rock can't have the red pendant"?
    }

    // TODO: not sure what's up in the php here;
    //   it sets waterfall fairy stuff and bow fairy stuff for some reason.
    //   It seems to set what the fairy fills your bottles with? idk

    let advancement_items_iter;
    let mut nice_items_iter;
    let mut junk_items_iter;
    let dungeon_items_iter;
    { // init item iterators
      let mut advancement_items_clone = advancement_items.clone();
      rng.shuffle(&mut advancement_items_clone);
      advancement_items_iter = advancement_items_clone.into_iter();

      let mut nice_items_clone = nice_items.clone();
      rng.shuffle(&mut nice_items_clone);
      nice_items_iter = nice_items_clone.into_iter();

      let mut junk_items_clone = junk_items.clone();
      rng.shuffle(&mut junk_items_clone);
      junk_items_iter = junk_items_clone.into_iter();

      let mut dungeon_items_clone = dungeon_items.clone();
      rng.shuffle(&mut dungeon_items_clone);
      dungeon_items_iter = dungeon_items_clone.into_iter();
    }

    // The code from here to the end is based on RandomAssumed.php

    let mut randomized_order_locations = locations::get_all_locations();
    rng.shuffle(&mut randomized_order_locations);
    trace!("randomized_order_locations: {:?}", randomized_order_locations);

    fill_items_in_locations(dungeon_items_iter, &randomized_order_locations, &advancement_items, &mut assignments);

    { // put some junk in ganon
      let num_junk_items = rng.next_u32() % 16;
      let ganon_locs: Vec<locations::Location> = regions::get_locations_for(regions::Region::GanonsTower).into_iter()
        .filter(|loc| assignments.get(loc) == None)
        .take(num_junk_items as usize)
        .collect();
      fast_fill_items_in_locations(&mut junk_items_iter, &ganon_locs, &mut assignments);
    }

    randomized_order_locations.reverse();

    fill_items_in_locations(advancement_items_iter, &randomized_order_locations, &vec![], &mut assignments);

    rng.shuffle(&mut randomized_order_locations);

    fast_fill_items_in_locations(&mut nice_items_iter, &randomized_order_locations, &mut assignments);
    assert_eq!(nice_items_iter.next(), None);
    // @hack: the php randomizes junk_items _again_ here;
    //   I'm skipping that useless step (or maybe I'm dumb?)
    fast_fill_items_in_locations(&mut junk_items_iter, &randomized_order_locations, &mut assignments);
    assert_eq!(junk_items_iter.next(), None);
  }

  let world = world::World {
    assignments,
    medallions,
  };
  world
}

fn fast_fill_items_in_locations(
  fill_items: &mut IntoIter<items::Item>,
  locations: &Vec<locations::Location>,
  assignments: &mut HashMap<locations::Location, items::Item>,
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
fn fill_items_in_locations(
  fill_items: IntoIter<items::Item>,
  locations: &Vec<locations::Location>,
  base_assumed_items: &Vec<items::Item>,
  assignments: &mut HashMap<locations::Location, items::Item>,
) {
  let mut remaining_fill_items: Vec<items::Item> = fill_items.collect();
  for _ in 0..remaining_fill_items.len() {
    let item = remaining_fill_items.pop().expect("bad for loop sync");
    let mut assumed_items = base_assumed_items.clone();
    assumed_items.append(&mut (remaining_fill_items.clone()));
    assumed_items = collect_items(&assumed_items, &assignments);

    let loc = locations.iter()
      .filter(|&&loc| assignments.get(&loc) == None)
      .filter(|&&loc| logic::can_fill(item, loc, &assumed_items, &assignments))
      .next();
    match loc {
      Some(loc) => {
        debug!("Filling {:?} with {:?}", loc, item);
        assignments.insert(*loc, item);
      },
      None => {
        panic!("No available locations for {:?}", item); // TODO: ~s/panic/Result/
      }
    }
  }
}

use std::collections::HashSet;
fn collect_items(
  assumed_items: &Vec<items::Item>,
  assignments: &HashMap<locations::Location, items::Item>,
) -> Vec<items::Item> {
  let mut my_items = assumed_items.clone();
  let mut available_locations: HashSet<locations::Location> = locations::get_all_locations()
    .into_iter()
    .filter(|&loc| assignments.get(&loc).is_some())
    .collect();
  loop {
    let search_locations: Vec<locations::Location> = available_locations.iter()
      .filter(|&&loc| logic::can_access(loc, &my_items, &assignments))
      .cloned()
      .collect();

    // available_locations -= search_locations
    available_locations = available_locations.into_iter()
      .filter(|&loc| !search_locations.contains(&loc))
      .collect();
    let mut found_items: Vec<items::Item> = search_locations.into_iter()
      .filter_map(|loc| assignments.get(&loc))
      .map(|&item| item)
      .collect();
    if found_items.len() == 0 {
      break;
    }
    my_items.append(&mut found_items);
  }

  return my_items;
}
