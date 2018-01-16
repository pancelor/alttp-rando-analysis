
#![allow(unused_imports)]

extern crate env_logger;
#[macro_use]
extern crate group_by;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rand;

mod multiset;
use multiset::HashMultiSet;

mod items;
mod medallions;
mod locations;
mod locations2;
mod regions;
mod zones;
mod world;
mod world2;
mod glue;
mod logic;
mod generator;
mod generator2;

fn main() {
  println!("{:?}", zones::KEYDOORS);
  println!("{:?}", zones::ITEMDOORS);
  // real_main();
}

#[allow(dead_code)]
fn real_main() {
  env_logger::init().unwrap();

  let advancement_items = items::get_advancement_items();
  trace!("advancement_items: {:?}", advancement_items);

  let nice_items = items::get_nice_items();
  trace!("nice_items: {:?}", nice_items);

  let junk_items = items::get_item_pool();
  trace!("item_pool: {:?}", junk_items);

  let dungeon_items = items::get_dungeon_pool();
  trace!("dungeon_items: {:?}", dungeon_items);

  let mut rng = rand::thread_rng();

  let sim_count = 1;
  for _ in 0..sim_count {
    let world = generator::generate_world(&advancement_items, &nice_items, &junk_items, &dungeon_items, &mut rng);
    info!("{:?}", world);

    // { // @TODO: debug code; rm
    //   for loc in vec![
    //     locations::Location::DesertPalaceBigChest,
    //     locations::Location::DesertPalaceMapChest,
    //     locations::Location::DesertPalaceTorch,
    //     locations::Location::DesertPalaceBigKeyChest,
    //     locations::Location::DesertPalaceCompassChest,
    //     locations::Location::DesertPalaceLanmolas,
    //     locations::Location::DesertPalacePrize,
    //   ] {
    //     error!("{:?}: {:?}", loc, world.assignments.get(&loc));
    //   }
    // }
  }
}
