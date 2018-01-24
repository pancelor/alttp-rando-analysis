#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use std::collections::{HashMap, BTreeSet};
use super::locations2::*;
use super::zones::*;
use super::dungeons::*;
use super::logic::*;
use super::items::*;


type CanPassClosure = Fn(&Vec<Item>) -> bool + Sync;

pub struct ItemDoor {
  pub zone1: Zone,
  pub zone2: Zone,
  pub reversible: bool,
  pub can_pass_callback: Box<CanPassClosure>,
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
    $gr.itemfrontier_from_zone.entry($z1)
      .or_insert(Vec::new())
      .push(idoor);
    let idoor = ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: false,
      can_pass_callback: $cb,
    };
    $gr.itemfrontier_from_zone.entry($z2)
      .or_insert(Vec::new())
      .push(idoor);
  });
  ($gr:ident, $z1:ident <=> $z2:ident: $cb:expr) => ({
    let idoor = ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: true,
      can_pass_callback: $cb,
    };
    $gr.itemfrontier_from_zone.entry($z1)
      .or_insert(Vec::new()) // TODO: same (below)
      .push(idoor);
    let idoor = ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: true,
      can_pass_callback: $cb,
    };
    $gr.itemfrontier_from_zone.entry($z2)
      .or_insert(Vec::new()) // TODO: same (below)
      .push(idoor);
  });
  ($gr:ident, $z1:ident ==> $z2:ident) => (
    cxn!($gr, $z1 ==> $z2: Box::new(|ref _items| true));
  );
  ($gr:ident, $z1:ident <=> $z2:ident) => (
    cxn!($gr, $z1 <=> $z2: Box::new(|ref _items| true));
  );
  ($gr:ident, $z1:ident <k> $z2:ident) => ({
    let kdoor = KeyDoor {
      zone1: $z1,
      zone2: $z2,
    };
    $gr.keyfrontier_from_zone.entry($z1)
      .or_insert(BTreeSet::new()) // TODO: throw errors if we need to insert; WorldGraph::new() sets these up for us
      .insert(kdoor);
    $gr.keyfrontier_from_zone.entry($z2)
      .or_insert(BTreeSet::new()) // TODO: same
      .insert(kdoor);
  });
}

lazy_static! {
  pub static ref WG: WorldGraph = {
    let mut gr = WorldGraph::new();

    // PalaceOfDarkness
    cxn!(gr, TempEastLightWorld <=> POD1);
    cxn!(gr, POD1   <=> POD8:   Box::new(|ref items| { can_shoot_arrows(&items) }));
    cxn!(gr, POD8   ==> POD2:   Box::new(|ref items| { items.contains(&Hammer) }));
    cxn!(gr, POD47  <=> POD7:   Box::new(|ref items| { items.contains(&Lamp) }));
    cxn!(gr, POD7   <=> POD10:  Box::new(|ref items| { items.contains(&BigKeyD1) }));
    cxn!(gr, POD4   <=> POD6:   Box::new(|ref items| { items.contains(&Lamp) }));
    cxn!(gr, POD2   <=> POD29A: Box::new(|ref items| { can_shoot_arrows(&items) && items.contains(&Lamp) && items.contains(&Hammer) }));
    cxn!(gr, POD29B <=> POD9:   Box::new(|ref items| { items.contains(&BigKeyD1) }));
    cxn!(gr, POD1   <k> POD2);
    cxn!(gr, POD2   <k> POD3);
    cxn!(gr, POD2   <k> POD4);
    cxn!(gr, POD4   <k> POD47);
    cxn!(gr, POD4   <k> POD5);
    cxn!(gr, POD29A <k> POD29B);

    // Overworld
    gr.register_zone(None, TempEastLightWorld, btreeset![
      TempOverworld1,
      TempOverworld2,
      TempOverworld3,
      TempOverworld4,
      TempOverworld5,
    ]);

    // PalaceOfDarkness
    gr.register_zone(Some(PalaceOfDarkness), POD1, btreeset!{PalaceOfDarknessShooterRoom});
    gr.register_zone(Some(PalaceOfDarkness), POD2, btreeset!{
      PalaceOfDarknessStalfosBasement,
      PalaceOfDarknessTheArenaBridge,
    });
    gr.register_zone(Some(PalaceOfDarkness), POD3, btreeset!{PalaceOfDarknessBigKeyChest});
    gr.register_zone(Some(PalaceOfDarkness), POD4, btreeset!{PalaceOfDarknessCompassChest});
    gr.register_zone(Some(PalaceOfDarkness), POD5, btreeset!{PalaceOfDarknessHarmlessHellway});
    gr.register_zone(Some(PalaceOfDarkness), POD6, btreeset!{
      PalaceOfDarknessDarkBasementLeft,
      PalaceOfDarknessDarkBasementRight,
    });
    gr.register_zone(Some(PalaceOfDarkness), POD7, btreeset!{
      PalaceOfDarknessDarkMazeTop,
      PalaceOfDarknessDarkMazeBottom,
    });
    gr.register_zone(Some(PalaceOfDarkness), POD8, btreeset!{
      PalaceOfDarknessMapChest,
      PalaceOfDarknessTheArenaLedge,
    });
    gr.register_zone(Some(PalaceOfDarkness), POD9, btreeset!{
      PalaceOfDarknessHelmasaurKing,
      PalaceOfDarknessPrize,
    });
    gr.register_zone(Some(PalaceOfDarkness), POD10, btreeset!{PalaceOfDarknessBigChest});
    gr.register_zone(Some(PalaceOfDarkness), POD47, btreeset!{});
    gr.register_zone(Some(PalaceOfDarkness), POD29A, btreeset!{});
    gr.register_zone(Some(PalaceOfDarkness), POD29B, btreeset!{});

    // TODO other dungeons; overworld
    gr
  };
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
  fn new() -> Self {
    let mut myself = Self {
      zones_from_dungeon: HashMap::new(),
      dungeon_from_zone: HashMap::new(),
      locations_from_zone: HashMap::new(),
      keyfrontier_from_zone: HashMap::new(),
      itemfrontier_from_zone: HashMap::new(),
    };
    for &dungeon in ALL_DUNGEONS.iter() { // TODO: remove eventually?
      myself.zones_from_dungeon.insert(dungeon, btreeset!{});
    }
    for &zone in ALL_ZONES.iter() { // TODO: remove eventually?
      myself.locations_from_zone.insert(zone, btreeset!{});
      myself.keyfrontier_from_zone.insert(zone, btreeset!{});
      myself.itemfrontier_from_zone.insert(zone, vec![]);
    }
    myself
  }

  fn register_zone(&mut self, dungeon: Option<Dungeon>, zone: Zone, locs: BTreeSet<Location2>) {
    if let Some(dung) = dungeon {
      self.zones_from_dungeon.entry(dung)
        .or_insert(BTreeSet::new())
        .insert(zone);
      self.dungeon_from_zone.insert(zone, dung);
    }
    self.locations_from_zone.insert(zone, locs);
  }


  pub fn zones_from_dungeon(&self, dungeon: Dungeon) -> &BTreeSet<Zone> {
    self.zones_from_dungeon.get(&dungeon).expect("worldindex is borked 1")
  }

  pub fn dungeon_from_zone(&self, zone: Zone) -> Option<Dungeon> {
    self.dungeon_from_zone.get(&zone)
      .and_then(|x| Some(x.clone()))
  }

  pub fn locations_from_zone(&self, zone: Zone) -> &BTreeSet<Location2> {
    self.locations_from_zone.get(&zone).expect("worldindex is borked 2")
  }

  pub fn keyfrontier_from_zone(&self, zone: Zone) -> Option<&BTreeSet<KeyDoor>> {
    self.keyfrontier_from_zone.get(&zone)
  }

  pub fn itemfrontier_from_zone(&self, zone: Zone) -> &Vec<ItemDoor> {
    self.itemfrontier_from_zone.get(&zone).expect("worldindex is borked 4")
  }

  pub fn keyfrontier_from_dungeon(&self, dungeon: Dungeon) -> BTreeSet<KeyDoor> {
    let zones = self.zones_from_dungeon(dungeon);
    zones.iter() // TODO what happens if we into_iter here?
      .filter_map(|&zone| self.keyfrontier_from_zone(zone))
      .flat_map(|&ref kdoorset| kdoorset)
      .cloned()
      .collect()
  }

  pub fn dungeon_from_keydoor(&self, keydoor: KeyDoor) -> Dungeon {
    self.dungeon_from_zone(keydoor.zone1).expect("your keydoor is outside").clone()
  }

  // TODO: maybe shouldnt live here
  #[allow(dead_code)] // TODO: rm?
  pub fn dungeon_from_key(&self, key: Item) -> Option<Dungeon> {
    use super::items::*;
    use super::dungeons::*;
    match key {
      KeyH1 => None, // TODO: change later
      KeyH2 => None, // TODO: change later
      KeyP1 => Some(EasternPalace),
      KeyP2 => Some(DesertPalace),
      KeyP3 => Some(TowerOfHera),
      KeyD1 => Some(PalaceOfDarkness),
      KeyD2 => Some(SwampPalace),
      KeyD3 => Some(SkullWoods),
      KeyD4 => Some(ThievesTown),
      KeyD5 => Some(IcePalace),
      KeyD6 => Some(MiseryMire),
      KeyD7 => Some(TurtleRock),
      KeyA1 => None, // TODO: change later
      KeyA2 => None, // TODO: change later
      _     => panic!("bad arg"), // TODO: rm? return None?
    }
  }

  // TODO: maybe shouldnt live here
  pub fn key_from_dungeon(&self, dungeon: Dungeon) -> Item {
    use super::items::*;
    use super::dungeons::*;
    match dungeon {
      EasternPalace => KeyP1,
      DesertPalace => KeyP2,
      TowerOfHera => KeyP3,
      PalaceOfDarkness => KeyD1,
      SwampPalace => KeyD2,
      SkullWoods => KeyD3,
      ThievesTown => KeyD4,
      IcePalace => KeyD5,
      MiseryMire => KeyD6,
      TurtleRock => KeyD7,
    }
  }
}

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
