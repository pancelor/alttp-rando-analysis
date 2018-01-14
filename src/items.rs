use multiset::HashMultiSet;

#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Item {
  Nothing,
  L1Sword,
  L1SwordAndShield,
  L2Sword,
  MasterSword,
  L3Sword,
  L4Sword,
  BlueShield,
  RedShield,
  MirrorShield,
  FireRod,
  IceRod,
  Hammer,
  Hookshot,
  Bow,
  Boomerang,
  Powder,
  Bee,
  Bombos,
  Ether,
  Quake,
  Lamp,
  Shovel,
  OcarinaInactive,
  CaneOfSomaria,
  Bottle,
  PieceOfHeart,
  CaneOfByrna,
  Cape,
  MagicMirror,
  PowerGlove,
  TitansMitt,
  BookOfMudora,
  Flippers,
  MoonPearl,
  BugCatchingNet,
  BlueMail,
  RedMail,
  Key,
  Compass,
  HeartContainerNoAnimation,
  Bomb,
  ThreeBombs,
  Mushroom,
  RedBoomerang,
  BottleWithRedPotion,
  BottleWithGreenPotion,
  BottleWithBluePotion,
  RedPotion,
  GreenPotion,
  BluePotion,
  TenBombs,
  BigKey,
  Map,
  OneRupee,
  FiveRupees,
  TwentyRupees,
  PendantOfCourage,
  PendantOfWisdom,
  PendantOfPower,
  BowAndArrows,
  BowAndSilverArrows,
  BottleWithBee,
  BottleWithFairy,
  BossHeartContainer,
  HeartContainer,
  OneHundredRupees,
  FiftyRupees,
  Heart,
  Arrow,
  TenArrows,
  SmallMagic,
  ThreeHundredRupees,
  TwentyRupees2,
  BottleWithGoldBee,
  OcarinaActive,
  PegasusBoots,
  BombUpgrade5,
  BombUpgrade10,
  BombUpgrade50,
  ArrowUpgrade5,
  ArrowUpgrade10,
  ArrowUpgrade70,
  HalfMagic,
  QuarterMagic,
  Programmable1,
  Programmable2,
  Programmable3,
  SilverArrowUpgrade,
  Rupoor,
  RedClock,
  BlueClock,
  GreenClock,
  ProgressiveSword,
  ProgressiveShield,
  ProgressiveArmor,
  ProgressiveGlove,
  singleRNG,
  multiRNG,
  Triforce,
  PowerStar,
  TriforcePiece,
  MapLW,
  MapDW,
  MapA2,
  MapD7,
  MapD4,
  MapP3,
  MapD5,
  MapD3,
  MapD6,
  MapD1,
  MapD2,
  MapA1,
  MapP2,
  MapP1,
  MapH1,
  MapH2,
  CompassA2,
  CompassD7,
  CompassD4,
  CompassP3,
  CompassD5,
  CompassD3,
  CompassD6,
  CompassD1,
  CompassD2,
  CompassA1,
  CompassP2,
  CompassP1,
  CompassH1,
  CompassH2,
  BigKeyA2,
  BigKeyD7,
  BigKeyD4,
  BigKeyP3,
  BigKeyD5,
  BigKeyD3,
  BigKeyD6,
  BigKeyD1,
  BigKeyD2,
  BigKeyA1,
  BigKeyP2,
  BigKeyP1,
  BigKeyH1,
  BigKeyH2,
  KeyH2,
  KeyH1,
  KeyP1,
  KeyP2,
  KeyA1,
  KeyD2,
  KeyD1,
  KeyD6,
  KeyD3,
  KeyD5,
  KeyP3,
  KeyD4,
  KeyD7,
  KeyA2,
  Crystal1,
  Crystal2,
  Crystal3,
  Crystal4,
  Crystal5,
  Crystal6,
  Crystal7,
  DefeatAgahnim,
  DefeatAgahnim2,
  DefeatGanon,
}

pub fn get_advancement_items() -> Vec<Item> { // TODO: HashMultiSet<Item> instead
  let mut res = Vec::new();
  for _ in 0..4 { res.push(Item::ProgressiveSword); }
  // @hack: the web code randomly chooses a specific bottle each seed
  //   since we're reusing this list between seeds, we'll just use the base bottle
  for _ in 0..1 { res.push(Item::Bottle); }
  for _ in 0..1 { res.push(Item::Bombos); }
  for _ in 0..1 { res.push(Item::BookOfMudora); }
  for _ in 0..1 { res.push(Item::Bow); }
  for _ in 0..1 { res.push(Item::CaneOfSomaria); }
  for _ in 0..1 { res.push(Item::Cape); }
  for _ in 0..1 { res.push(Item::Ether); }
  for _ in 0..1 { res.push(Item::FireRod); }
  for _ in 0..1 { res.push(Item::Flippers); }
  for _ in 0..1 { res.push(Item::Hammer); }
  for _ in 0..1 { res.push(Item::Hookshot); }
  for _ in 0..1 { res.push(Item::IceRod); }
  for _ in 0..1 { res.push(Item::Lamp); }
  for _ in 0..1 { res.push(Item::MagicMirror); }
  for _ in 0..1 { res.push(Item::MoonPearl); }
  for _ in 0..1 { res.push(Item::Mushroom); }
  for _ in 0..1 { res.push(Item::OcarinaInactive); }
  for _ in 0..1 { res.push(Item::PegasusBoots); }
  for _ in 0..1 { res.push(Item::Powder); }
  for _ in 0..1 { res.push(Item::Quake); }
  for _ in 0..1 { res.push(Item::Shovel); }
  for _ in 0..1 { res.push(Item::SilverArrowUpgrade); }
  for _ in 0..2 { res.push(Item::ProgressiveGlove); }
  for _ in 0..1 { res.push(Item::BugCatchingNet); }
  for _ in 0..3 { res.push(Item::ProgressiveShield); }
  for _ in 0..1 { res.push(Item::CaneOfByrna); }
  for _ in 0..1 { res.push(Item::HalfMagic); }
  return res;
}

pub fn get_nice_items() -> Vec<Item> {
  let mut res = Vec::new();
  for _ in 0..1 { res.push(Item::HeartContainer); }
  for _ in 0..10 { res.push(Item::BossHeartContainer); }
  // @hack: the web code randomly chooses a specific bottle each seed
  //   since we're reusing this list between seeds, we'll just use the base bottle
  for _ in 0..3 { res.push(Item::Bottle); }
  for _ in 0..2 { res.push(Item::ProgressiveArmor); }
  for _ in 0..1 { res.push(Item::Boomerang); }
  for _ in 0..1 { res.push(Item::RedBoomerang); }
  return res;
}

pub fn get_item_pool() -> Vec<Item> {
  let mut res = Vec::new();
  for _ in 0..24 { res.push(Item::PieceOfHeart); }
  for _ in 0..6 { res.push(Item::BombUpgrade5); }
  for _ in 0..1 { res.push(Item::BombUpgrade10); }
  for _ in 0..6 { res.push(Item::ArrowUpgrade5); }
  for _ in 0..1 { res.push(Item::ArrowUpgrade10); }
  for _ in 0..1 { res.push(Item::Arrow); }
  for _ in 0..5 { res.push(Item::TenArrows); }
  for _ in 0..10 { res.push(Item::ThreeBombs); }
  for _ in 0..2 { res.push(Item::OneRupee); }
  for _ in 0..4 { res.push(Item::FiveRupees); }
  for _ in 0..28 { res.push(Item::TwentyRupees); }
  for _ in 0..7 { res.push(Item::FiftyRupees); }
  for _ in 0..1 { res.push(Item::OneHundredRupees); }
  for _ in 0..5 { res.push(Item::ThreeHundredRupees); }
  return res;
}

pub fn get_dungeon_pool() -> Vec<Item> {
  let mut res = Vec::new();
  for _ in 0..1 { res.push(Item::BigKeyA2); }
  for _ in 0..1 { res.push(Item::BigKeyD1); }
  for _ in 0..1 { res.push(Item::BigKeyD2); }
  for _ in 0..1 { res.push(Item::BigKeyD3); }
  for _ in 0..1 { res.push(Item::BigKeyD4); }
  for _ in 0..1 { res.push(Item::BigKeyD5); }
  for _ in 0..1 { res.push(Item::BigKeyD6); }
  for _ in 0..1 { res.push(Item::BigKeyD7); }
  for _ in 0..1 { res.push(Item::BigKeyP1); }
  for _ in 0..1 { res.push(Item::BigKeyP2); }
  for _ in 0..1 { res.push(Item::BigKeyP3); }
  for _ in 0..4 { res.push(Item::KeyA2); }
  for _ in 0..6 { res.push(Item::KeyD1); }
  for _ in 0..1 { res.push(Item::KeyD2); }
  for _ in 0..2 { res.push(Item::KeyD3); }
  for _ in 0..1 { res.push(Item::KeyD4); }
  for _ in 0..2 { res.push(Item::KeyD5); }
  for _ in 0..3 { res.push(Item::KeyD6); }
  for _ in 0..4 { res.push(Item::KeyD7); }
  for _ in 0..2 { res.push(Item::KeyA1); }
  for _ in 0..1 { res.push(Item::KeyH2); }
  for _ in 0..1 { res.push(Item::KeyP2); }
  for _ in 0..1 { res.push(Item::KeyP3); }
  for _ in 0..1 { res.push(Item::MapA2); }
  for _ in 0..1 { res.push(Item::MapD1); }
  for _ in 0..1 { res.push(Item::MapD2); }
  for _ in 0..1 { res.push(Item::MapD3); }
  for _ in 0..1 { res.push(Item::MapD4); }
  for _ in 0..1 { res.push(Item::MapD5); }
  for _ in 0..1 { res.push(Item::MapD6); }
  for _ in 0..1 { res.push(Item::MapD7); }
  for _ in 0..1 { res.push(Item::MapH2); }
  for _ in 0..1 { res.push(Item::MapP1); }
  for _ in 0..1 { res.push(Item::MapP2); }
  for _ in 0..1 { res.push(Item::MapP3); }
  for _ in 0..1 { res.push(Item::CompassA2); }
  for _ in 0..1 { res.push(Item::CompassD1); }
  for _ in 0..1 { res.push(Item::CompassD2); }
  for _ in 0..1 { res.push(Item::CompassD3); }
  for _ in 0..1 { res.push(Item::CompassD4); }
  for _ in 0..1 { res.push(Item::CompassD5); }
  for _ in 0..1 { res.push(Item::CompassD6); }
  for _ in 0..1 { res.push(Item::CompassD7); }
  for _ in 0..1 { res.push(Item::CompassP1); }
  for _ in 0..1 { res.push(Item::CompassP2); }
  for _ in 0..1 { res.push(Item::CompassP3); }
  return res;
}



use std::fmt;

type Closure = Fn(&HashMultiSet<Item2>) -> bool + Sync;

pub struct Item2 {
  name: &'static str,
  can_access_callback: &'static Closure,
}

impl fmt::Debug for Item2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Item2 {
  pub fn can_access(&self, items: &HashMultiSet<Item2>) -> bool {
    (self.can_access_callback)(&items)
  }
}

// pub const LANGUAGE: &'static [&'static str] = &["firefox", "chrome"];

pub static Nothing : Item2 = Item2 {
  name: "Nothing",
  can_access_callback: &|items: &HashMultiSet<Item2>| -> bool {
    true
  },
};

