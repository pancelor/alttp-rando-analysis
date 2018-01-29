#![allow(unused_imports)]

use std::fmt;
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

// This can't be `impl PartialEq for KeyDoor` b/c
//   that would break all `BTreeSet<KeyDoor>`s;
//   see https://doc.rust-lang.org/std/cmp/trait.Ord.html
impl KeyDoor {
  #[allow(dead_code)]
  fn equals(&self, other: &Self) -> bool {
    (
         self.zone1 == other.zone1
      && self.zone2 == other.zone2
    ) || (
         self.zone1 == other.zone2
      && self.zone2 == other.zone1
    )
  }
}

impl ItemDoor {
  // crappy name; idk. Only used in debugging
  #[allow(dead_code)]
  fn is_collocated_with(&self, other: &Self) -> bool {
    (
         self.zone1 == other.zone1
      && self.zone2 == other.zone2
    ) || (
         self.zone1 == other.zone2
      && self.zone2 == other.zone1
    )
  }
}

impl ItemDoor {
  pub fn can_pass(&self, items: &Vec<Item>) -> bool {
    (self.can_pass_callback)(&items)
  }
}


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

    // TODO other dungeons; overworld

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
      TempOverworld11,
      TempOverworld12,
      TempOverworld13,
      TempOverworld14,
      TempOverworld15,
      TempOverworld16,
      TempOverworld17,
      TempOverworld18,
      TempOverworld19,
      TempOverworld20,
    ]);


    // EasternPalace
    cxn!(gr, TempEastLightWorld <=> EP1: Box::new(|ref items| { items.contains(&CanEnterEP) }));
    cxn!(gr, EP1 <=> EP2: Box::new(|ref items| { items.contains(&Lamp) }));
    cxn!(gr, EP2 <k> EP3);
    cxn!(gr, EP1 <=> EP4: Box::new(|ref items| { items.contains(&BigKeyP1) }));
    cxn!(gr, EP1 <=> EP5: Box::new(|ref items| { items.contains(&Lamp) && items.contains(&BigKeyP1) }));
    cxn!(gr, EP5 <k> EP56);
    cxn!(gr, EP56 <=> EP6: Box::new(|ref items| { can_shoot_arrows(&items) }));

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
    // TODO: merge w/ mire / ledge etc when those are added
    // TODO: TempEastLightWorld <=> DP1 should be ==>, but I haven't thought through s+q yet
    cxn!(gr, TempEastLightWorld <=> DP1: Box::new(|ref items| { items.contains(&BookOfMudora) && items.contains(&CanEnterDP)}));
    cxn!(gr, TempEastLightWorld <=> DP1: Box::new(|ref items| { can_fly(&items) && can_lift_dark_rocks(&items) && items.contains(&MagicMirror) && items.contains(&CanEnterDP)}));
    cxn!(gr, DP1 <=> DP2: Box::new(|ref items| { items.contains(&PegasusBoots) }));
    cxn!(gr, DP1 <k> DP3);
    cxn!(gr, DP1 <=> DP4: Box::new(|ref items| { items.contains(&BigKeyP2) }));
    cxn!(gr, DP1 <=> DP15A: Box::new(|ref items| { can_lift_rocks(&items) }));
    cxn!(gr, DP15A <k> DP15B);
    cxn!(gr, DP15B <k> DP15C);
    cxn!(gr, DP15C <k> DP15D);
    cxn!(gr, DP15D <=> DP5: Box::new(|ref items| { items.contains(&BigKeyP2) && can_light_torches(&items) && (can_kill_most_things(&items) || items.contains(&IceRod)) }));

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
    gr.register_zone(Some(DesertPalace), DP15B, btreeset!{
      DesertPalaceKeyPotB,
    });
    gr.register_zone(Some(DesertPalace), DP15C, btreeset!{
      DesertPalaceKeyPotC,
    });
    gr.register_zone(Some(DesertPalace), DP15D, btreeset!{});

    gr.preset_item(DesertPalaceKeyPotA, KeyP2);
    gr.preset_item(DesertPalaceKeyPotB, KeyP2);
    gr.preset_item(DesertPalaceKeyPotC, KeyP2);


    // TowerOfHera
    cxn!(gr, TempEastLightWorld <=> TH1: Box::new(|ref items| { items.contains(&CanEnterTH) }));
    cxn!(gr, TH1 <k> TH12);
    cxn!(gr, TH12 <=> TH2: Box::new(|ref items| { can_light_torches(&items) }));
    cxn!(gr, TH1 <=> TH3: Box::new(|ref items| { items.contains(&BigKeyP3) }));
    cxn!(gr, TH3 <=> TH4: Box::new(|ref items| { has_sword(&items) || items.contains(&Hammer) }));

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


    // Swamp Palace
    // TODO: careful when ripping out TempPreSP
    cxn!(gr, TempEastLightWorld <=> TempPreSP: Box::new(|ref items| { items.contains(&MoonPearl) && items.contains(&CanEnterSP) }));
    cxn!(gr, TempPreSP <=> SP0: Box::new(|ref items| { items.contains(&Flippers) && items.contains(&MagicMirror) }));
    cxn!(gr, SP0 <k> SP1);
    cxn!(gr, SP1 <k> SP12);
    cxn!(gr, SP12 <=> SP2: Box::new(|ref items| { items.contains(&Hammer) }));
    cxn!(gr, SP2 <=> SP3: Box::new(|ref items| { items.contains(&BigKeyD2) }));
    cxn!(gr, SP2 <k> SP4);
    cxn!(gr, SP2 <=> SP25: Box::new(|ref items| { items.contains(&Hookshot) }));
    cxn!(gr, SP25 <k> SP5);
    cxn!(gr, SP5 <k> SP56);
    cxn!(gr, SP56 <=> SP6: Box::new(|ref items| { items.contains(&Hookshot) }));

    gr.register_zone(Some(SwampPalace), SP0, btreeset!{
      SwampPalaceEntrance,
    });
    gr.register_zone(Some(SwampPalace), SP1, btreeset!{
      SwampPalaceMapChest,
      SwampPalaceKeySkullA,
    });
    gr.register_zone(Some(SwampPalace), SP12, btreeset!{
      SwampPalaceKeySkullB,
    });
    gr.register_zone(Some(SwampPalace), SP2, btreeset!{
      SwampPalaceCompassChest,
      SwampPalaceKeySkullC,
    });
    gr.register_zone(Some(SwampPalace), SP3, btreeset!{
      SwampPalaceBigChest,
    });
    gr.register_zone(Some(SwampPalace), SP4, btreeset!{
      SwampPalaceBigKeyChest,
      SwampPalaceWestChest,
    });
    gr.register_zone(Some(SwampPalace), SP25, btreeset!{
      SwampPalaceKeySkullD,
    });
    gr.register_zone(Some(SwampPalace), SP5, btreeset!{
      SwampPalaceFloodedRoomLeft,
      SwampPalaceFloodedRoomRight,
      SwampPalaceWaterfallRoom,
      SwampPalaceKeySkullE,
    });
    gr.register_zone(Some(SwampPalace), SP56, btreeset!{});
    gr.register_zone(Some(SwampPalace), SP6, btreeset!{
      SwampPalaceArrghus,
      SwampPalacePrize,
    });

    gr.preset_item(SwampPalaceKeySkullA, KeyD2);
    gr.preset_item(SwampPalaceKeySkullB, KeyD2);
    gr.preset_item(SwampPalaceKeySkullC, KeyD2);
    gr.preset_item(SwampPalaceKeySkullD, KeyD2);
    gr.preset_item(SwampPalaceKeySkullE, KeyD2);


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

  fn dungeon_from_zone(&self, zone: Zone) -> Option<&Dungeon> {
    self.dungeon_from_zone.get(&zone)
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

  pub fn get_presets(&self) -> &HashMap<Location2, Item> {
    &self.preset_items
  }
}

impl WorldGraph {
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

  pub fn dungeon_from_keydoor(&self, keydoor: KeyDoor) -> &Dungeon {
    self.dungeon_from_zone(keydoor.zone1).expect("your keydoor is outside")
  }
}


// misfit functions


impl WorldGraph {
  pub fn item_can_be_placed_at(&self, item: Item, loc: Location2) -> bool {
    use std::env;

    let keysanity = env::var("KEYSANITY").is_ok();

    keysanity || match item {
      // TODO: more (A1/A2/H1/H2)
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
        let ret = locs.contains(&loc);
        trace!("item_can_be_placed_at\n\titem={:?}\n\tloc={:?}\n\tlocs={:?}\n\tcan={}", item, loc, locs, ret);
        ret
      },
      _ => true,
    }
  }

  fn dungeon_from_item(&self, key: Item) -> Option<Dungeon> {
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
      _     => None,
    }
  }

  pub fn dungeon_info(&self, dungeon: Dungeon) -> &DungeonInfo {
    DUNGEON_INFO.get(&dungeon).expect("not a dungeon somehow")
  }
}

pub struct DungeonInfo {
  pub key: Item,
  pub prize_loc: Location2,
  pub can_enter_token: Item,
  pub beat_token: Item,
}

lazy_static! {
  static ref DUNGEON_INFO: HashMap<Dungeon, DungeonInfo> = hashmap!{
    EasternPalace => DungeonInfo{
      key: KeyP1,
      prize_loc: EasternPalacePrize,
      can_enter_token: CanEnterEP,
      beat_token: BeatEP,
    },
    DesertPalace => DungeonInfo{
      key: KeyP2,
      prize_loc: DesertPalacePrize,
      can_enter_token: CanEnterDP,
      beat_token: BeatDP,
    },
    TowerOfHera => DungeonInfo{
      key: KeyP3,
      prize_loc: TowerOfHeraPrize,
      can_enter_token: CanEnterTH,
      beat_token: BeatTH,
    },
    PalaceOfDarkness => DungeonInfo{
      key: KeyD1,
      prize_loc: PalaceOfDarknessPrize,
      can_enter_token: CanEnterPOD,
      beat_token: BeatPOD,
    },
    SwampPalace => DungeonInfo{
      key: KeyD2,
      prize_loc: SwampPalacePrize,
      can_enter_token: CanEnterSP,
      beat_token: BeatSP,
    },
    SkullWoods => DungeonInfo{
      key: KeyD3,
      prize_loc: SkullWoodsPrize,
      can_enter_token: CanEnterSW,
      beat_token: BeatSW,
    },
    ThievesTown => DungeonInfo{
      key: KeyD4,
      prize_loc: ThievesTownPrize,
      can_enter_token: CanEnterTT,
      beat_token: BeatTT,
    },
    IcePalace => DungeonInfo{
      key: KeyD5,
      prize_loc: IcePalacePrize,
      can_enter_token: CanEnterIP,
      beat_token: BeatIP,
    },
    MiseryMire => DungeonInfo{
      key: KeyD6,
      prize_loc: MiseryMirePrize,
      can_enter_token: CanEnterMM,
      beat_token: BeatMM,
    },
    TurtleRock => DungeonInfo{
      key: KeyD7,
      prize_loc: TurtleRockPrize,
      can_enter_token: CanEnterTR,
      beat_token: BeatTR,
    },
  };
}
