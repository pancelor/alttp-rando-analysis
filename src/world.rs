#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use super::locations2::Location2;
use super::items::Item;

pub type Assignments = HashMap<Location2, Item>;

#[derive(Eq, PartialEq, Debug)]
pub struct World {
  pub assignments: Assignments,
}
