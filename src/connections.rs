#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use super::locations;
use super::dungeons;
use super::items::Item;
use super::zones::{Zone, ItemDoor, KeyDoor};

macro_rules! cxn {
  // Creates a ItemDoor in a compact form
  ($z1:ident ==> $z2:ident: $cb:expr) => (
    ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: false,
      can_pass_callback: $cb,
    }
  );
  ($z1:ident <=> $z2:ident: $cb:expr) => (
    ItemDoor {
      zone1: $z1,
      zone2: $z2,
      reversible: true,
      can_pass_callback: $cb,
    }
  );
  ($z1:ident ==> $z2:ident) => (
    cxn!($z1 ==> $z2: &|ref _items| true)
  );
  ($z1:ident <=> $z2:ident) => (
    cxn!($z1 <=> $z2: &|ref _items| true)
  );
  ($z1:ident <k> $z2:ident) => (
    KeyDoor {
      zone1: $z1,
      zone2: $z2,
      dungeon: dungeons::EasternPalace, // TODO
    }
  );
}



fn todo(_: &Vec<Item>) -> bool { true } // for warning suppression

pub static ITEMDOORS : &[ItemDoor] = &[
  cxn!(Overworld <=> POD1),
  cxn!(POD1   <=> POD8:   &|ref items| {todo(items) /*Bow!*/}),
  cxn!(POD8   ==> POD2:   &|ref items| {todo(items) /*Hammer*/}),
  cxn!(POD47  <=> POD7:   &|ref items| {todo(items) /*Lamp*/}),
  cxn!(POD7   <=> POD10:  &|ref items| {todo(items) /*BigKeyPOD*/}),
  cxn!(POD4   <=> POD6:   &|ref items| {todo(items) /*Lamp*/}),
  cxn!(POD2   <=> POD29A: &|ref items| {todo(items) /*Bow!, Lamp, Hammer*/}),
  cxn!(POD29B <=> POD9:   &|ref items| {todo(items) /*BigKeyPOD*/}),
];

pub static KEYDOORS : &[KeyDoor] = &[
  cxn!(POD1   <k> POD2),
  cxn!(POD2   <k> POD3),
  cxn!(POD2   <k> POD4),
  cxn!(POD4   <k> POD47),
  cxn!(POD4   <k> POD5),
  cxn!(POD29A <k> POD29B),
];
