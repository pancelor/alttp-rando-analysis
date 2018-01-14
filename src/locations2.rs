#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use multiset::HashMultiSet;
use super::zones;
use super::zones::*;
use super::items::Item;

type CanAccessClosure = Fn(&HashMultiSet<Item>) -> bool + Sync;

pub struct Location2 {
  name: &'static str,
  zone: Zone,
  can_access_callback: &'static CanAccessClosure,
}

use std::fmt;
impl fmt::Debug for Location2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Location2 {
  pub fn can_access(&self, items: &HashMultiSet<Item>) -> bool {
    (self.can_access_callback)(&items)
  }
}

const TODO : bool = true;
fn temp_allow_unused(_: &HashMultiSet<Item>) {}

macro_rules! loc {
  // Creates a Location2 in a compact form
  ($loc_name:ident, $zone:ident, $cb:expr) => {
    pub static $loc_name : Location2 = Location2 {
      name: stringify!($loc_name),
      zone: $zone,
      can_access_callback: $cb,
    };
  }
}

loc!(TempOverworld1, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld2, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld3, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld4, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld5, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld6, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld7, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld8, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld9, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld10, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld11, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld12, TempOverworld, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(PalaceOfDarknessBigKeyChest, POD3, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessTheArenaLedge, POD8, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessTheArenaBridge, POD2, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessBigChest, POD7, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessCompassChest, POD4, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessHarmlessHellway, POD5, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessStalfosBasement, POD2, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkBasementLeft, POD6, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkBasementRight, POD6, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessMapChest, POD8, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkMazeTop, POD7, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkMazeBottom, POD7, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessShooterRoom, POD1, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessHelmasaurKing, POD9, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessPrize, POD9, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});