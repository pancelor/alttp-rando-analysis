extern crate env_logger;
extern crate group_by;
// #[macro_use]
// extern crate lazy_static;
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
  use connections::KeyDoor;
  use dive::Dive;

  let d1 = Dive{
    zones: btreeset!{POD47, TempEastLightWorld, POD1},
    items: vec![Lamp, KeyD1, KeyD1, Hammer, Bow],
    open_doors: btreeset!{},
  };
  let d2 = Dive{
    zones: btreeset!{TempEastLightWorld, POD1, POD47},
    items: vec![KeyD1, Bow, Lamp, Hammer, KeyD1],
    open_doors: btreeset!{},
  };

  println!("eq={} h1={} h2={}", (d1==d2), d1.hash_value(), d2.hash_value());
}

#[allow(dead_code)]
fn real_main() {
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

  let sim_count = match env::var("NSIM") {
    Ok(val) => val.parse().expect("bad NSIM format"),
    Err(_) => 1,
  };
  for ii in 0..sim_count {
    info!("sim #{:?}", ii);
    let world = generator::generate_world(&advancement_items, &junk_items, &mut rng);
    info!("worldgen finished: {:?}", world);
    if !generator::can_win(&world) {
      println!("{:?}", world);
      panic!("uh oh, this world isn't beatable");
    }
  }
}

// TODO rm
#[allow(dead_code)]
fn key_in_dark_maze(world: &world::World) -> bool{
  use locations2::*;
  use items::*;
  (
    world.assignments.get(&PalaceOfDarknessBigChest) == Some(&KeyD1)
    || world.assignments.get(&PalaceOfDarknessDarkMazeTop) == Some(&KeyD1)
    || world.assignments.get(&PalaceOfDarknessDarkMazeBottom) == Some(&KeyD1)
  )
}
