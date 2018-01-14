#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashMap;
use rand::{Rng, ThreadRng};
use super::{medallions, logic, world2, locations2, regions, items, zones};

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
  assignments: &mut HashMap<locations2::Location2, items::Item>,
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
  assignments: &mut HashMap<locations2::Location2, items::Item>,
) {
  let mut remaining_fill_items: Vec<items::Item> = fill_items.collect();
  for _ in 0..remaining_fill_items.len() {
    let item = remaining_fill_items.pop().expect("bad for loop sync");
    let mut assumed_items = base_assumed_items.clone();
    assumed_items.append(&mut (remaining_fill_items.clone()));

    // TODO: new algo instead here
    // assumed_items = collect_items(&assumed_items, &assignments);

    // let loc = locations.iter()
    //   .filter(|&&loc| assignments.get(&loc) == None)
    //   .filter(|&&loc| logic::can_fill(item, loc, &assumed_items, &assignments))
    //   .next();
    // match loc {
    //   Some(loc) => {
    //     debug!("Filling {:?} with {:?}", loc, item);
    //     assignments.insert(*loc, item);
    //   },
    //   None => {
    //     panic!("No available locations for {:?}", item); // TODO: ~s/panic/Result/
    //   }
    // }
  }
}
