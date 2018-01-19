extern crate env_logger;
extern crate group_by;
// #[macro_use]
// extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate maplit;
extern crate rand;

mod items;
mod locations;
mod locations2;
mod zones;
mod dungeons;

mod medallions;
mod world;
mod generator;
mod glue;
mod dive;
mod connections;
mod logic;


fn main() {
  env_logger::init().unwrap();
  temp_main();
}

#[allow(dead_code)]
fn temp_main() {
  use locations2::*;
  use items::*;
  let assignments = hashmap!{TempOverworld4 => KeyD1, PalaceOfDarknessHelmasaurKing => BottleWithBee, TempOverworld3 => BottleWithBee, PalaceOfDarknessDarkMazeTop => BottleWithBee, PalaceOfDarknessBigKeyChest => BottleWithBee, PalaceOfDarknessTheArenaBridge => BottleWithBee, PalaceOfDarknessHarmlessHellway => Bow, TempOverworld1 => KeyD1, PalaceOfDarknessDarkBasementLeft => BottleWithBee, PalaceOfDarknessMapChest => KeyD1, PalaceOfDarknessDarkMazeBottom => BigKeyD1, PalaceOfDarknessStalfosBasement => KeyD1, TempOverworld5 => Lamp, PalaceOfDarknessCompassChest => CompassD1, PalaceOfDarknessDarkBasementRight => KeyD1, PalaceOfDarknessPrize => BottleWithBee, PalaceOfDarknessBigChest => MapD1, TempOverworld2 => KeyD1, PalaceOfDarknessTheArenaLedge => Hammer, PalaceOfDarknessShooterRoom => BottleWithBee};
  let wrd = world::World{assignments};
  println!("can win {:?}", generator::can_win(&wrd));
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

  let sim_count = 100;
  for ii in 0..sim_count {
    println!("ii: {:?}", ii);
    let world = generator::generate_world(&advancement_items, &junk_items, &mut rng);
    if key_in_dark_maze(&world) {
      println!("{:?}", world);
    }
    if !generator::can_win(&world) {
      println!("{:?}", world);
      panic!("uh oh, this world isn't beatable");
    }
  }
}

fn key_in_dark_maze(world: &world::World) -> bool{
  use locations2::*;
  use items::*;
  (
    world.assignments.get(&PalaceOfDarknessBigChest) == Some(&KeyD1)
    || world.assignments.get(&PalaceOfDarknessDarkMazeTop) == Some(&KeyD1)
    || world.assignments.get(&PalaceOfDarknessDarkMazeBottom) == Some(&KeyD1)
  )
}
