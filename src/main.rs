
#![allow(unused_imports)]

extern crate env_logger;
#[macro_use]
extern crate group_by;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rand;

mod items;
mod medallions;
mod locations;
mod regions;
mod world;
mod logic;
mod generator;

mod world2;
mod locations2;
mod generator2;
mod zones;
mod glue;
mod dungeons;
mod dive;
mod connections;

fn main() {
  real_main();
}

#[allow(dead_code)]
fn real_main() {
  env_logger::init().unwrap();

  let mut advancement_items = items::get_advancement_items();
  trace!("advancement_items: {:?}", advancement_items);

  let mut dungeon_items = items::get_dungeon_pool();
  trace!("dungeon_items: {:?}", dungeon_items);

  advancement_items.append(&mut dungeon_items);

  let junk_items = items::get_item_pool();
  trace!("item_pool: {:?}", junk_items);

  let mut rng = rand::thread_rng();

  let sim_count = 1;
  for _ in 0..sim_count {
    let world = generator2::generate_world(&advancement_items, &junk_items, &mut rng);
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
