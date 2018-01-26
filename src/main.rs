extern crate env_logger;
extern crate group_by;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rand;

use std::env;

mod items;
mod locations;
mod locations2;
mod zones;
mod dungeons;

mod medallions;
mod world;
mod generator;
mod dive;
mod connections;
mod logic;

mod stats;


fn main() {
  env_logger::init().unwrap();
  real_main();
}

#[allow(dead_code)]
#[allow(unused_imports)]
fn temp_main() {
  use locations2::*;
  use items::*;
  use zones::*;
  use dungeons::*;
  use connections::{WG, KeyDoor};
  use dive::Dive;

  let locs1 = WG.locations_from_dungeon(EasternPalace);
  let locs2 = WG.locations_from_dungeon(PalaceOfDarkness);
  println!("EP={:?} PD={:?}", locs1, locs2);
}

struct ItemPools {
  advancement_items: Vec<items::Item>,
  dungeon_items: Vec<items::Item>,
  junk_items: Vec<items::Item>,
  required_items_to_win: Vec<items::Item>,
}

#[allow(dead_code)]
fn real_main() {
  use stats::*;

  let pools = get_items();
  let mut rng = rand::thread_rng();

  let sim_count = match env::var("NSIM") {
    Ok(val) => val.parse().expect("bad NSIM format"),
    Err(_) => 1,
  };
  for ii in 0..sim_count {
    info!("sim #{:?}", ii);
    let world = generator::generate_world(pools.advancement_items.clone(), pools.dungeon_items.clone(), pools.junk_items.clone(), &mut rng);

    info!("worldgen finished: {:?}", world);
    info!("Final dive stats: {}", DIVES.lock().unwrap().format());
    if !generator::can_collect(&world, &pools.required_items_to_win) {
      println!("{:?}", world);
      panic!("uh oh, this world isn't beatable");
    }
  }
}

#[allow(unused_imports)]
fn get_items() -> ItemPools {
  use connections::WG;
  use items::*;

  let mut advancement_items = vec![
    FireRod,
    IceRod,
    Bow,
    Hammer,
    Lamp,
    BookOfMudora,
    PegasusBoots,
    ProgressiveGlove,
    ProgressiveGlove,
    MagicMirror,
    OcarinaActive, // TODO: inactive
  ];

  let mut dungeon_items;
  let required_items_to_win: Vec<Item>;
  { // init dungeon_items
    #![allow(non_snake_case)]

    let mut EP_dungeon_items = vec![
      CanEnterEP,
      MapP1,
      CompassP1,
      BigKeyP1,
    ];
    let mut DP_dungeon_items = vec![
      CanEnterDP,
      MapP2,
      CompassP2,
      KeyP2,
      BigKeyP2,
    ];
    let mut TH_dungeon_items = vec![
      CanEnterTH,
      MapP3,
      CompassP3,
      KeyP3,
      BigKeyP3,
    ];
    let mut POD_dungeon_items = vec![
      CanEnterPOD,
      MapD1,
      CompassD1,
      KeyD1,
      KeyD1,
      KeyD1,
      KeyD1,
      KeyD1,
      KeyD1,
      BigKeyD1,
    ];

    dungeon_items = vec![];
    if !env_is_set("EP") { // pass EP=<any> to turn OFF item placement in EP
      dungeon_items.append(&mut EP_dungeon_items);
    }
    if !env_is_set("DP") {
      dungeon_items.append(&mut DP_dungeon_items);
    }
    if !env_is_set("TH") {
      dungeon_items.append(&mut TH_dungeon_items);
    }
    if !env_is_set("POD") {
      dungeon_items.append(&mut POD_dungeon_items);
    }
    dungeon_items.sort();

    required_items_to_win = dungeon_items.iter()
      .filter_map(|&item| canenter_to_beat(item))
      .collect();
  }

  let keysanity = env_is_set("KEYSANITY");
  if keysanity {
    advancement_items.append(&mut dungeon_items);
  }
  debug!("keysanity={}, dungeon_items={:?}", keysanity, dungeon_items);

  let junk_items: Vec<Item> = (0..).take(100).map(|_| items::TEMP_JUNK).collect();

  ItemPools{advancement_items, dungeon_items, junk_items, required_items_to_win}
}

fn env_is_set(name: &str) -> bool {
  env::var(name).is_ok()
}

// TODO rm
#[allow(dead_code)]
fn key_in_dark_maze(world: &world::World) -> bool{
  use locations2::*;
  use items::*;
  (
    world.get(&PalaceOfDarknessBigChest) == Some(&KeyD1)
    || world.get(&PalaceOfDarknessDarkMazeTop) == Some(&KeyD1)
    || world.get(&PalaceOfDarknessDarkMazeBottom) == Some(&KeyD1)
  )
}
