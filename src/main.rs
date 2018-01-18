
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

  let sim_count = 1;
  for _ in 0..sim_count {
    let world = generator2::generate_world(&advancement_items, &junk_items, &mut rng);
    info!("{:?}", world);
    info!("Winnable? {:?}", generator2::can_win(&world));
  }
}
