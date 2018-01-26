#![allow(unused_imports)]

use std::collections::{HashMap, BTreeSet};
use super::locations2::*;
use super::world::World;
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
      .or_insert(Vec::new())
      .push(idoor);
    let idoor = ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: true,
      can_pass_callback: $cb,
    };
    $gr.itemfrontier_from_zone.entry($z2)
      .or_insert(Vec::new())
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
      .or_insert(BTreeSet::new())
      .insert(kdoor);
    $gr.keyfrontier_from_zone.entry($z2)
      .or_insert(BTreeSet::new())
      .insert(kdoor);
  });
}

lazy_static! {
  pub static ref WG: WorldGraph = {
    let mut gr = WorldGraph::new();

    // EasternPalace
    cxn!(gr, TempEastLightWorld <=> EP1: Box::new(|ref items| { items.contains(&CanEnterEP) }));
    cxn!(gr, EP1 <=> EP2: Box::new(|ref items| { items.contains(&Lamp) }));
    cxn!(gr, EP2 <k> EP3);
    cxn!(gr, EP1 <=> EP4: Box::new(|ref items| { items.contains(&BigKeyP1) }));
    cxn!(gr, EP1 <=> EP5: Box::new(|ref items| { items.contains(&Lamp) && items.contains(&BigKeyP1) }));
    cxn!(gr, EP5 <k> EP56);
    cxn!(gr, EP56 <=> EP6: Box::new(|ref items| { can_shoot_arrows(&items) }));

    // DesertPalace
    // TODO: merge w/ mire / ledge etc when those are added
    // TODO: TempEastLightWorld <=> DP1 should be ==>, but I haven't thought through s+q yet
    cxn!(gr, TempEastLightWorld <=> DP1: Box::new(|ref items| { items.contains(&BookOfMudora) && items.contains(&CanEnterDP)}));
    cxn!(gr, TempEastLightWorld <=> DP1: Box::new(|ref items| { can_fly(&items) && can_lift_dark_rocks(&items) && items.contains(&MagicMirror) && items.contains(&CanEnterDP)}));
    cxn!(gr, DP1 <=> DP2: Box::new(|ref items| { items.contains(&PegasusBoots) }));
    cxn!(gr, DP1 <k> DP3);
    cxn!(gr, DP1 <=> DP4: Box::new(|ref items| { items.contains(&BigKeyP2) }));
    cxn!(gr, DP1 <=> DP15A: Box::new(|ref items| { can_lift_rocks(&items) }));
    cxn!(gr, DP15A <k> DP15D);
    // cxn!(gr, DP15B <k> DP15C);
    // cxn!(gr, DP15C <k> DP15D);
    cxn!(gr, DP15D <=> DP5: Box::new(|ref items| { items.contains(&BigKeyP2) && can_light_torches(&items) && (can_kill_most_things(&items) || items.contains(&IceRod)) }));

    // TowerOfHera
    cxn!(gr, TempEastLightWorld <=> TH1: Box::new(|ref items| { items.contains(&CanEnterTH) }));
    cxn!(gr, TH1 <k> TH12);
    cxn!(gr, TH12 <=> TH2: Box::new(|ref items| { can_light_torches(&items) }));
    cxn!(gr, TH1 <=> TH3: Box::new(|ref items| { items.contains(&BigKeyP3) }));
    cxn!(gr, TH3 <=> TH4: Box::new(|ref items| { has_sword(&items) || items.contains(&Hammer) }));

    // PalaceOfDarkness
    cxn!(gr, TempEastLightWorld <=> POD1: Box::new(|ref items| { items.contains(&CanEnterPOD) }));
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
      TempOverworld6,
      TempOverworld7,
      TempOverworld8,
      TempOverworld9,
      TempOverworld10,
    ]);

    // Eastern Palace
    gr.register_zone(Some(EasternPalace), EP1, btreeset!{
      EasternPalaceCompassChest,
      EasternPalaceCannonballChest,
      EasternPalaceMapChest,
    });
    gr.register_zone(Some(EasternPalace), EP2, btreeset!{
      EasternPalaceKeyPot,
    });
    gr.register_zone(Some(EasternPalace), EP3, btreeset!{
      EasternPalaceBigKeyChest,
    });
    gr.register_zone(Some(EasternPalace), EP4, btreeset!{
      EasternPalaceBigChest,
    });
    gr.register_zone(Some(EasternPalace), EP5, btreeset!{
      EasternPalaceKeyEyegore,
    });
    gr.register_zone(Some(EasternPalace), EP6, btreeset!{
      EasternPalaceArmosKnights,
      EasternPalacePrize,
    });
    gr.register_zone(Some(EasternPalace), EP56, btreeset!{});

    gr.preset_item(EasternPalaceKeyPot, KeyP1);
    gr.preset_item(EasternPalaceKeyEyegore, KeyP1);

    // DesertPalace
    gr.register_zone(Some(DesertPalace), DP1, btreeset!{
      DesertPalaceMapChest,
    });
    gr.register_zone(Some(DesertPalace), DP2, btreeset!{
      DesertPalaceTorch,
    });
    gr.register_zone(Some(DesertPalace), DP3, btreeset!{
      DesertPalaceBigKeyChest,
      DesertPalaceCompassChest,
    });
    gr.register_zone(Some(DesertPalace), DP4, btreeset!{
      DesertPalaceBigChest,
    });
    gr.register_zone(Some(DesertPalace), DP5, btreeset!{
      DesertPalaceLanmolas,
      DesertPalacePrize,
    });
    gr.register_zone(Some(DesertPalace), DP15A, btreeset!{
      DesertPalaceKeyPotA,
    });
    // gr.register_zone(Some(DesertPalace), DP15B, btreeset!{
    //   DesertPalaceKeyPotB,
    // });
    // gr.register_zone(Some(DesertPalace), DP15C, btreeset!{
    //   DesertPalaceKeyPotC,
    // });
    gr.register_zone(Some(DesertPalace), DP15D, btreeset!{});

    gr.preset_item(DesertPalaceKeyPotA, KeyP2);
    // gr.preset_item(DesertPalaceKeyPotB, KeyP2);
    // gr.preset_item(DesertPalaceKeyPotC, KeyP2);

    // TowerOfHera
    gr.register_zone(Some(TowerOfHera), TH1, btreeset!{
      TowerOfHeraMapChest,
      TowerOfHeraBasementCage,
    });
    gr.register_zone(Some(TowerOfHera), TH2, btreeset!{
      TowerOfHeraBigKeyChest,
    });
    gr.register_zone(Some(TowerOfHera), TH3, btreeset!{
      TowerOfHeraCompassChest,
      TowerOfHeraBigChest,
    });
    gr.register_zone(Some(TowerOfHera), TH4, btreeset!{
      TowerOfHeraMoldorm,
      TowerOfHeraPrize,
    });
    gr.register_zone(Some(TowerOfHera), TH12, btreeset!{});

    // PalaceOfDarkness
    gr.register_zone(Some(PalaceOfDarkness), POD1, btreeset!{
      PalaceOfDarknessShooterRoom
    });
    gr.register_zone(Some(PalaceOfDarkness), POD2, btreeset!{
      PalaceOfDarknessStalfosBasement,
      PalaceOfDarknessTheArenaBridge,
    });
    gr.register_zone(Some(PalaceOfDarkness), POD3, btreeset!{
      PalaceOfDarknessBigKeyChest
    });
    gr.register_zone(Some(PalaceOfDarkness), POD4, btreeset!{
      PalaceOfDarknessCompassChest
    });
    gr.register_zone(Some(PalaceOfDarkness), POD5, btreeset!{
      PalaceOfDarknessHarmlessHellway
    });
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
    gr.register_zone(Some(PalaceOfDarkness), POD10, btreeset!{
      PalaceOfDarknessBigChest
    });
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

  preset_items: HashMap<Location2, Item>,
}

impl WorldGraph {
  fn new() -> Self {
    Self {
      zones_from_dungeon: HashMap::new(),
      dungeon_from_zone: HashMap::new(),
      locations_from_zone: HashMap::new(),
      keyfrontier_from_zone: HashMap::new(),
      itemfrontier_from_zone: HashMap::new(),
      preset_items: HashMap::new(),
    }
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


  fn zones_from_dungeon(&self, dungeon: Dungeon) -> Option<&BTreeSet<Zone>> {
    self.zones_from_dungeon.get(&dungeon)
  }

  fn dungeon_from_zone(&self, zone: Zone) -> Option<Dungeon> {
    self.dungeon_from_zone.get(&zone)
      .and_then(|x| Some(x.clone()))
  }

  pub fn locations_from_zone(&self, zone: Zone) -> Option<&BTreeSet<Location2>> {
    self.locations_from_zone.get(&zone)
  }

  pub fn keyfrontier_from_zone(&self, zone: Zone) -> Option<&BTreeSet<KeyDoor>> {
    self.keyfrontier_from_zone.get(&zone)
  }

  pub fn itemfrontier_from_zone(&self, zone: Zone) -> Option<&Vec<ItemDoor>> {
    self.itemfrontier_from_zone.get(&zone)
  }

  pub fn preset_item(&mut self, loc: Location2, item: Item) {
    self.preset_items.insert(loc, item);
  }

  // TODO: rm
  #[allow(dead_code)]
  pub fn num_presets(&self) -> usize {
    self.preset_items.len()
  }

  pub fn prefill_pots_etc(&self, world: &mut World) {
    for (&loc, &item) in self.preset_items.iter() {
      world.assign(loc, item);
    }
  }

  pub fn keyfrontier_from_dungeon(&self, dungeon: Dungeon) -> BTreeSet<KeyDoor> {
    match self.zones_from_dungeon(dungeon) {
      Some(zones) => {
        zones.iter()
          .filter_map(|&zone| self.keyfrontier_from_zone(zone))
          .flat_map(|&ref kdoorset| kdoorset)
          .cloned()
          .collect()
      },
      None => {
        BTreeSet::new()
      }
    }
  }

  // TODO: temp pub
  pub fn locations_from_dungeon(&self, dungeon: Dungeon) -> Option<BTreeSet<Location2>> {
    self.zones_from_dungeon(dungeon)
      .and_then(|zones| {
        let ret = zones.iter()
          .filter_map(|&zone| self.locations_from_zone(zone))
          .flat_map(|&ref locset| locset)
          .cloned()
          .collect();
        Some(ret)
      })
  }

  pub fn dungeon_from_keydoor(&self, keydoor: KeyDoor) -> Dungeon {
    self.dungeon_from_zone(keydoor.zone1).expect("your keydoor is outside").clone()
  }

  // TODO: maybe shouldn't live here
  pub fn item_can_be_placed_at(&self, item: Item, loc: Location2) -> bool {
    use std::env;

    let keysanity = env::var("KEYSANITY").is_ok();

    keysanity || match item {
      // TODO: more
        KeyP1 | BigKeyP1 | CompassP1 | MapP1
      | KeyP2 | BigKeyP2 | CompassP2 | MapP2
      | KeyP3 | BigKeyP3 | CompassP3 | MapP3
      | KeyD1 | BigKeyD1 | CompassD1 | MapD1
      | KeyD2 | BigKeyD2 | CompassD2 | MapD2
      | KeyD3 | BigKeyD3 | CompassD3 | MapD3
      | KeyD4 | BigKeyD4 | CompassD4 | MapD4
      | KeyD5 | BigKeyD5 | CompassD5 | MapD5
      | KeyD6 | BigKeyD6 | CompassD6 | MapD6
      | KeyD7 | BigKeyD7 | CompassD7 | MapD7
      => {
        let dungeon = WG.dungeon_from_item(item).expect("bad key enum");
        let locs = WG.locations_from_dungeon(dungeon).expect("not a dungeon somehow");
        let x = locs.contains(&loc);
        // TODO: s/debug/trace
        // debug!("item_can_be_placed_at\n\titem={:?}\n\tloc={:?}\n\tlocs={:?}\n\tcan={}", item, loc, locs, x);
        x
      },
      _ => true,
    }
  }

  // TODO: maybe shouldn't live here
  fn dungeon_from_item(&self, key: Item) -> Option<Dungeon> {
    use super::items::*;
    use super::dungeons::*;
    match key {
      KeyH1 | BigKeyH1 | CompassH1 | MapH1 => None, // TODO: set this for real
      KeyH2 | BigKeyH2 | CompassH2 | MapH2 => None, // TODO: set this for real
      KeyP1 | BigKeyP1 | CompassP1 | MapP1 => Some(EasternPalace),
      KeyP2 | BigKeyP2 | CompassP2 | MapP2 => Some(DesertPalace),
      KeyP3 | BigKeyP3 | CompassP3 | MapP3 => Some(TowerOfHera),
      KeyD1 | BigKeyD1 | CompassD1 | MapD1 => Some(PalaceOfDarkness),
      KeyD2 | BigKeyD2 | CompassD2 | MapD2 => Some(SwampPalace),
      KeyD3 | BigKeyD3 | CompassD3 | MapD3 => Some(SkullWoods),
      KeyD4 | BigKeyD4 | CompassD4 | MapD4 => Some(ThievesTown),
      KeyD5 | BigKeyD5 | CompassD5 | MapD5 => Some(IcePalace),
      KeyD6 | BigKeyD6 | CompassD6 | MapD6 => Some(MiseryMire),
      KeyD7 | BigKeyD7 | CompassD7 | MapD7 => Some(TurtleRock),
      KeyA1 | BigKeyA1 | CompassA1 | MapA1 => None, // TODO: set this for real
      KeyA2 | BigKeyA2 | CompassA2 | MapA2 => None, // TODO: set this for real
      _     => panic!("bad arg"), // TODO: rm? return None?
    }
  }

  // TODO: maybe shouldn't live here
  #[allow(dead_code)]
  pub fn prize_loc_from_dungeon(&self, dungeon: Dungeon) -> Option<Location2> {
    use super::locations2::*;
    use super::dungeons::*;
    match dungeon {
      EasternPalace => Some(EasternPalacePrize),
      DesertPalace => Some(DesertPalacePrize),
      TowerOfHera => Some(TowerOfHeraPrize),
      PalaceOfDarkness => Some(PalaceOfDarknessPrize),
      // SwampPalace => Some(SwampPalacePrize),
      // SkullWoods => Some(SkullWoodsPrize),
      // ThievesTown => Some(ThievesTownPrize),
      // IcePalace => Some(IcePalacePrize),
      // MiseryMire => Some(MiseryMirePrize),
      // TurtleRock => Some(TurtleRockPrize),
      _ => None,
    }
  }

  // TODO: maybe shouldn't live here
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

// fn todo(_: &Vec<Item>) -> bool { true } // for warning suppression
