
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

  use items::*;
  let advancement_items = vec![
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    KeyD1,
    Bow,
    Hammer,
    Lamp,
    BigKeyD1,
    MapD1,
    CompassD1,
  ];

  let junk_items = vec![
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
    BottleWithBee,
  ];

  // need 14 + prize + 5 overworld = 20 items total

  let mut rng = rand::thread_rng();

  let sim_count = 50;
  for ii in 0..sim_count {
    info!("ii: {:?}", ii);
    let world = generator2::generate_world(&advancement_items, &junk_items, &mut rng);
    if key_in_dark_maze(&world) {
      info!("{:?}", world);
      info!("Winnable? {:?}", generator2::can_win(&world));
    }
  }
}

fn key_in_dark_maze(world: &world2::World2) -> bool{
  use locations2::*;
  use items::*;
  (
    world.assignments.get(&PalaceOfDarknessBigChest) == Some(&KeyD1)
    || world.assignments.get(&PalaceOfDarknessDarkMazeTop) == Some(&KeyD1)
    || world.assignments.get(&PalaceOfDarknessDarkMazeBottom) == Some(&KeyD1)
  )
}
