#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use super::{medallions, locations2, items};

#[derive(Eq, PartialEq, Debug)]
pub struct World2 {
  pub assignments: HashMap<locations2::Location2, items::Item>,
}
