#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use std::collections::{HashMap, BTreeSet};
use super::locations2::*;
use super::zones::*;
use super::dungeons::*;
use super::logic::*;
use super::items::*;


type CanPassClosure = Fn(&Vec<Item>) -> bool + Sync;

#[derive(Copy, Clone)]
pub struct ItemDoor {
  pub zone1: Zone,
  pub zone2: Zone,
  pub reversible: bool,
  pub can_pass_callback: &'static CanPassClosure,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct KeyDoor {
  pub zone1: Zone,
  pub zone2: Zone,
}

// TODO: does this breaks any invariants? e.g. will Ord be broken? which breaks all BTreeSet s?
// impl PartialEq for KeyDoor {
//   fn eq(&self, other: &Self) -> bool {
//     (
//          self.zone1 == other.zone1
//       && self.zone2 == other.zone2
//     ) || (
//          self.zone1 == other.zone2
//       && self.zone2 == other.zone1
//     )
//   }
// }

impl ItemDoor {
  pub fn can_pass(&self, items: &Vec<Item>) -> bool {
    (self.can_pass_callback)(&items)
  }
}


use std::fmt;
impl fmt::Debug for KeyDoor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} <k> {:?}", self.zone1, self.zone2)
  }
}

impl fmt::Debug for ItemDoor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let op : &str =
      if self.reversible {
        "<=>"
      } else {
        "==>"
      };
    write!(f, "{:?} {} {:?}", self.zone1, op, self.zone2)
  }
}

macro_rules! cxn {
  ($gr:ident, $z1:ident ==> $z2:ident: $cb:expr) => ({
    let idoor = ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: false,
      can_pass_callback: $cb,
    };
    gr.itemfrontier_from_zone.entry($z1)
      .or_insert(Vec::new())
      .insert(idoor);
    gr.itemfrontier_from_zone.entry($z2)
      .or_insert(Vec::new())
      .insert(idoor);
  });
  ($gr:ident, $z1:ident <=> $z2:ident: $cb:expr) => ({
    let idoor = ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: true,
      can_pass_callback: $cb,
    };
    gr.itemfrontier_from_zone.entry($z1)
      .or_insert(Vec::new())
      .insert(idoor);
    gr.itemfrontier_from_zone.entry($z2)
      .or_insert(Vec::new())
      .insert(idoor);
  });
  ($gr:ident, $z1:ident ==> $z2:ident) => (
    cxn!($gr, $z1 ==> $z2: &|ref _items| true);
  );
  ($gr:ident, $z1:ident <=> $z2:ident) => (
    cxn!($gr, $z1 <=> $z2: &|ref _items| true);
  );
  ($gr:ident, $z1:ident <k> $z2:ident) => ({
    let kdoor = KeyDoor {
      zone1: $z1,
      zone2: $z2,
    };
    gr.keyfrontier_from_zone.entry($z1)
      .or_insert(BTreeSet::new())
      .insert(kdoor);
    gr.keyfrontier_from_zone.entry($z2)
      .or_insert(BTreeSet::new())
      .insert(kdoor);
  });
}


/// A master record of connections in the world
pub struct WorldGraph {
  // add_zone fields
  zones_from_dungeon: HashMap<Dungeon, BTreeSet<Zone>>,
  dungeon_from_zone: HashMap<Zone, Dungeon>,
  locations_from_zone: HashMap<Zone, BTreeSet<Location2>>,

  // connection fields
  keyfrontier_from_zone: HashMap<Zone, BTreeSet<KeyDoor>>,
  itemfrontier_from_zone: HashMap<Zone, Vec<ItemDoor>>,
}

impl WorldGraph {
  fn get() -> Self {
    // TODO cache cache cache pleeease. using lazy_static?
    Self::new()
  }

  fn new() -> Self {
    let gr = Self {
      zones_from_dungeon: HashMap::new(),
      dungeon_from_zone: HashMap::new(),
      locations_from_zone: HashMap::new(),
      keyfrontier_from_zone: HashMap::new(),
      itemfrontier_from_zone: HashMap::new(),
    };

    cxn!(gr, TempEastLightWorld <=> POD1);
    cxn!(gr, POD1   <=> POD8:   &|ref items| { can_shoot_arrows(&items) });
    cxn!(gr, POD8   ==> POD2:   &|ref items| { items.contains(&Hammer) });
    cxn!(gr, POD47  <=> POD7:   &|ref items| { items.contains(&Lamp) });
    cxn!(gr, POD7   <=> POD10:  &|ref items| { items.contains(&BigKeyD1) });
    cxn!(gr, POD4   <=> POD6:   &|ref items| { items.contains(&Lamp) });
    cxn!(gr, POD2   <=> POD29A: &|ref items| { can_shoot_arrows(&items) && items.contains(&Lamp) && items.contains(&Hammer) });
    cxn!(gr, POD29B <=> POD9:   &|ref items| { items.contains(&BigKeyD1) });

    cxn!(gr, POD1   <k> POD2);
    cxn!(gr, POD2   <k> POD3);
    cxn!(gr, POD2   <k> POD4);
    cxn!(gr, POD4   <k> POD47);
    cxn!(gr, POD4   <k> POD5);
    cxn!(gr, POD29A <k> POD29B);

    gr.set_foo(
      Overworld => {
        TempEastLightWorld => [
          TempOverworld1,
          TempOverworld2,
          TempOverworld3,
          TempOverworld4,
          TempOverworld5,
        ],
      },
      PalaceOfDarkness => {
        POD1 => [PalaceOfDarknessShooterRoom],
        POD2 => [PalaceOfDarknessStalfosBasement, PalaceOfDarknessTheArenaBridge],
        // etc
      },
    );
  }
}


gr.add_zone(
  zone=POD8,
  locations=[
    PalaceOfDarknessTheArenaLedge,
    PalaceOfDarknessMapChest,
  ],
  dungeon=PalaceOfDarkness,
)

gr.add_keydoor(
  z1, z2
)

gr.add_itemdoor(
  z1, z2, reversible=true, cb=()=>true
)

/*
Stuff to replicate:

index also has methods that replace glue.rs:
  // only deals with add_zone info
  zones_from_dungeon
  dungeon_from_zone
  locations_from_zone

  // deals with connections
  keyfrontier_from_zone
  itemfrontier_from_zone

  // easy
  keyfrontier_from_dungeon
  dungeon_from_keydoor
  // maybe shouldnt live here
    dungeon_from_key
    key_from_dungeon
*/

// fn todo(_: &Vec<Item>) -> bool { true } // for warning suppression
