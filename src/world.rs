#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::collections::HashMap;
use super::{medallions, locations, items};

#[derive(Eq, PartialEq, Debug)]
pub struct World {
  pub medallions: medallions::EntranceConfig,
  pub assignments: HashMap<locations::Location, items::Item>,
}
