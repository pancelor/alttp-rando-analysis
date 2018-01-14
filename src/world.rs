use std::collections::HashMap;
use super::{medallions, locations, items};

#[derive(Eq, PartialEq, Debug)]
pub struct World {
  pub medallions: medallions::EntranceConfig,
  pub assignments: HashMap<locations::Location, items::Item>,
}
