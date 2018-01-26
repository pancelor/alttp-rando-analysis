#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashMap;
use super::{locations2, items};
use items::*;

pub fn count(item: &Item, my_items: &Vec<Item>) -> usize {
  my_items.iter()
    .filter(|&&it| it == *item)
    .count()
}

pub fn can_lift_rocks(my_items: &Vec<Item>) -> bool {
  my_items.contains(&PowerGlove)
  || my_items.contains(&ProgressiveGlove)
  || my_items.contains(&TitansMitt)
}

pub fn can_lift_dark_rocks(my_items: &Vec<Item>) -> bool {
  my_items.contains(&TitansMitt)
  || count(&ProgressiveGlove, &my_items) >= 2
}

pub fn can_light_torches(my_items: &Vec<Item>) -> bool {
  my_items.contains(&FireRod)
  || my_items.contains(&Lamp)
}

pub fn can_melt_things(my_items: &Vec<Item>) -> bool {
  my_items.contains(&FireRod)
  || (my_items.contains(&Bombos) && has_sword(&my_items))
}

pub fn can_fly(my_items: &Vec<Item>) -> bool {
  my_items.contains(&OcarinaActive)
  || my_items.contains(&OcarinaInactive)
}

pub fn can_spin_speed(my_items: &Vec<Item>) -> bool {
  my_items.contains(&PegasusBoots) && (
    has_sword(&my_items)
    || my_items.contains(&Hookshot)
  )
}

pub fn can_shoot_arrows(my_items: &Vec<Item>) -> bool {
  my_items.contains(&Bow)
  || my_items.contains(&BowAndArrows)
  || my_items.contains(&BowAndSilverArrows)
}

pub fn can_block_lasers(my_items: &Vec<Item>) -> bool {
  my_items.contains(&MirrorShield)
  || count(&ProgressiveShield, &my_items) >= 3
}

pub fn can_extend_magic(my_items: &Vec<Item>) -> bool {
  my_items.contains(&HalfMagic)
  || my_items.contains(&QuarterMagic)
  || has_a_bottle(&my_items)
}

pub fn glitched_link_in_dark_world(my_items: &Vec<Item>) -> bool {
  my_items.contains(&MoonPearl)
  || has_a_bottle(&my_items)
}

pub fn can_kill_most_things(my_items: &Vec<Item>) -> bool {
  has_sword(&my_items)
  || my_items.contains(&CaneOfSomaria)
  || my_items.contains(&CaneOfByrna)
  || can_shoot_arrows(&my_items)
  || my_items.contains(&Hammer)
  || my_items.contains(&FireRod)
}

pub fn can_get_good_bee(_my_items: &Vec<Item>) -> bool {
  unimplemented!() // TODO impl
}

pub fn has_sword(my_items: &Vec<Item>) -> bool {
  my_items.contains(&L1Sword)
  || my_items.contains(&L1SwordAndShield)
  || my_items.contains(&ProgressiveSword)
  || has_upgraded_sword(&my_items)
}

pub fn has_upgraded_sword(my_items: &Vec<Item>) -> bool {
  my_items.contains(&L2Sword)
  || my_items.contains(&MasterSword)
  || my_items.contains(&L3Sword)
  || my_items.contains(&L4Sword)
  || count(&ProgressiveSword, &my_items) >= 2
}

pub fn has_a_bottle(my_items: &Vec<Item>) -> bool {
  my_items.contains(&BottleWithBee)
  || my_items.contains(&BottleWithFairy)
  || my_items.contains(&BottleWithRedPotion)
  || my_items.contains(&BottleWithGreenPotion)
  || my_items.contains(&BottleWithBluePotion)
  || my_items.contains(&Bottle)
  || my_items.contains(&BottleWithGoldBee)
}
