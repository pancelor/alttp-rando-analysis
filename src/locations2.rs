#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use multiset::HashMultiSet;
use super::{regions};
use super::items::Item;

type CanAccessClosure = Fn(&HashMultiSet<Item>) -> bool + Sync;

pub struct Location2 {
  name: &'static str,
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
  ($loc_name:ident, $cb:expr) => {
    pub static $loc_name : Location2 = Location2 {
      name: stringify!($loc_name),
      can_access_callback: $cb,
    };
  }
}

loc!(TempOverworld1, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld2, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld3, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld4, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld5, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld6, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld7, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld8, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld9, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld10, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld11, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(TempOverworld12, &|ref items| -> bool {
  temp_allow_unused(&items);
  true
});
loc!(PalaceOfDarknessBigKeyChest, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessTheArenaLedge, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessTheArenaBridge, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessBigChest, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessCompassChest, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessHarmlessHellway, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessStalfosBasement, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkBasementLeft, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkBasementRight, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessMapChest, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkMazeTop, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessDarkMazeBottom, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessShooterRoom, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessHelmasaurKing, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
loc!(PalaceOfDarknessPrize, &|ref items| -> bool {
  temp_allow_unused(&items); TODO
});
