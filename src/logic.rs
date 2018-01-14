#![allow(dead_code)]

use std::collections::HashMap;
use super::{locations, regions, items};
use locations::Location::*;
use regions::Region::*;
use items::Item::*;

fn count(
  item: &items::Item,
  my_items: &Vec<items::Item>,
) -> usize {
  my_items.iter()
    .filter(|&&it| it == *item)
    .cloned()
    .collect::<Vec<items::Item>>()
    .len()
}

fn can_lift_rocks(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&PowerGlove)
  || my_items.contains(&ProgressiveGlove)
  || my_items.contains(&TitansMitt)
}

fn can_lift_dark_rocks(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&TitansMitt)
  || count(&ProgressiveGlove, &my_items) >= 2
}

fn can_light_torches(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&FireRod)
  || my_items.contains(&Lamp)
}

fn can_melt_things(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&FireRod)
  || (my_items.contains(&Bombos) && has_sword(&my_items))
}

fn can_fly(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&OcarinaActive)
  || my_items.contains(&OcarinaInactive)
}

fn can_spin_speed(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&PegasusBoots) && (
    has_sword(&my_items)
    || my_items.contains(&Hookshot)
  )
}

fn can_shoot_arrows(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&Bow)
  || my_items.contains(&BowAndArrows)
  || my_items.contains(&BowAndSilverArrows)
}

fn can_block_lasers(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&MirrorShield)
  || count(&ProgressiveShield, &my_items) >= 3
}

fn can_extend_magic(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&HalfMagic)
  || my_items.contains(&QuarterMagic)
  || has_a_bottle(&my_items)
}

fn glitched_link_in_dark_world(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&MoonPearl)
  || has_a_bottle(&my_items)
}

fn has_sword(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&L1Sword)
  || my_items.contains(&L1SwordAndShield)
  || my_items.contains(&ProgressiveSword)
  || has_upgraded_sword(&my_items)
}

fn has_upgraded_sword(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&L2Sword)
  || my_items.contains(&MasterSword)
  || my_items.contains(&L3Sword)
  || my_items.contains(&L4Sword)
  || count(&ProgressiveSword, &my_items) >= 2
}

fn has_a_bottle(
  my_items: &Vec<items::Item>,
) -> bool {
  my_items.contains(&BottleWithBee)
  || my_items.contains(&BottleWithFairy)
  || my_items.contains(&BottleWithRedPotion)
  || my_items.contains(&BottleWithGreenPotion)
  || my_items.contains(&BottleWithBluePotion)
  || my_items.contains(&Bottle)
  || my_items.contains(&BottleWithGoldBee)
}

pub fn can_complete(
  reg: regions::Region,
  my_items: &Vec<items::Item>,
  assignments: &HashMap<locations::Location, items::Item>,
) -> bool {
  let prize_loc = match reg {
    LightWorld                 => None,
    Escape                     => None,
    EasternPalace              => Some(EasternPalacePrize),
    DesertPalace               => Some(DesertPalacePrize),
    WestDeathMountain          => None,
    EastDeathMountain          => None,
    TowerofHera                => Some(TowerOfHeraPrize),
    HyruleCastleTower          => Some(Agahnim),
    EastDarkWorldDeathMountain => None,
    WestDarkWorldDeathMountain => None,
    NorthEastDarkWorld         => None,
    NorthWestDarkWorld         => None,
    SouthDarkWorld             => None,
    Mire                       => None,
    PalaceofDarkness           => Some(PalaceOfDarknessPrize),
    SwampPalace                => Some(SwampPalacePrize),
    SkullWoods                 => Some(SkullWoodsPrize),
    ThievesTown                => Some(ThievesTownPrize),
    IcePalace                  => Some(IcePalacePrize),
    MiseryMire                 => Some(MiseryMirePrize),
    TurtleRock                 => Some(TurtleRockPrize),
    GanonsTower                => Some(Agahnim2),
    Fountains                  => None,
  };
  match prize_loc {
    Some(loc) => can_access(loc, &my_items, &assignments),
    None => true,
  }
}

pub fn can_fill(
  item: items::Item,
  loc: locations::Location,
  _my_items: &Vec<items::Item>,
  _assignments: &HashMap<locations::Location, items::Item>,
) -> bool {

  // @TODO: some sort of logic in Region.php#canFill
  //   I assume it keeps keys where they belong but idk
  //   and I'm too tired right now to get this right.
  match regions::get_dungeon_items_for(locations::get_region_for(loc)) {
    Some(_dungeon_items) => {

    },
    None => {

    }
  };

  let todo = true;
  match loc { // @TODO
    DesertPalaceBigChest => item != items::Item::BigKeyP2,
    DesertPalaceMapChest => true,
    DesertPalaceTorch => true,
    DesertPalaceBigKeyChest => item != items::Item::KeyP2,
    DesertPalaceCompassChest => item != items::Item::KeyP2,
    DesertPalaceLanmolas => item != items::Item::KeyP2 && item != items::Item::BigKeyP2,
    DesertPalacePrize => true,
    EasternPalaceCompassChest => todo,
    EasternPalaceBigChest => todo,
    EasternPalaceCannonballChest => todo,
    EasternPalaceBigKeyChest => todo,
    EasternPalaceMapChest => todo,
    EasternPalaceArmosKnights => todo,
    EasternPalacePrize => todo,
    TowerOfHeraBigKeyChest => todo,
    TowerOfHeraBasementCage => todo,
    TowerOfHeraMapChest => todo,
    TowerOfHeraCompassChest => todo,
    TowerOfHeraBigChest => todo,
    TowerOfHeraMoldorm => todo,
    TowerOfHeraPrize => todo,
    IcePalaceBigKeyChest => todo,
    IcePalaceCompassChest => todo,
    IcePalaceMapChest => todo,
    IcePalaceSpikeRoom => todo,
    IcePalaceFreezorChest => todo,
    IcePalaceIcedTRoom => todo,
    IcePalaceBigChest => todo,
    IcePalaceKholdstare => todo,
    IcePalacePrize => todo,
    MiseryMireBigChest => todo,
    MiseryMireMainLobby => todo,
    MiseryMireBigKeyChest => todo,
    MiseryMireCompassChest => todo,
    MiseryMireBridgeChest => todo,
    MiseryMireMapChest => todo,
    MiseryMireSpikeChest => todo,
    MiseryMireVitreous => todo,
    MiseryMirePrize => todo,
    PalaceOfDarknessBigKeyChest => todo,
    PalaceOfDarknessTheArenaLedge => todo,
    PalaceOfDarknessTheArenaBridge => todo,
    PalaceOfDarknessBigChest => todo,
    PalaceOfDarknessCompassChest => todo,
    PalaceOfDarknessHarmlessHellway => todo,
    PalaceOfDarknessStalfosBasement => todo,
    PalaceOfDarknessDarkBasementLeft => todo,
    PalaceOfDarknessDarkBasementRight => todo,
    PalaceOfDarknessMapChest => todo,
    PalaceOfDarknessDarkMazeTop => todo,
    PalaceOfDarknessDarkMazeBottom => todo,
    PalaceOfDarknessShooterRoom => todo,
    PalaceOfDarknessHelmasaurKing => todo,
    PalaceOfDarknessPrize => todo,
    SkullWoodsBigChest => todo,
    SkullWoodsBigKeyChest => todo,
    SkullWoodsCompassChest => todo,
    SkullWoodsMapChest => todo,
    SkullWoodsBridgeRoom => todo,
    SkullWoodsPotPrison => todo,
    SkullWoodsPinballRoom => todo,
    SkullWoodsMothula => todo,
    SkullWoodsPrize => todo,
    SwampPalaceEntrance => todo,
    SwampPalaceBigChest => todo,
    SwampPalaceBigKeyChest => todo,
    SwampPalaceMapChest => todo,
    SwampPalaceWestChest => todo,
    SwampPalaceCompassChest => todo,
    SwampPalaceFloodedRoomLeft => todo,
    SwampPalaceFloodedRoomRight => todo,
    SwampPalaceWaterfallRoom => todo,
    SwampPalaceArrghus => todo,
    SwampPalacePrize => todo,
    ThievesTownAttic => todo,
    ThievesTownBigKeyChest => todo,
    ThievesTownMapChest => todo,
    ThievesTownCompassChest => todo,
    ThievesTownAmbushChest => todo,
    ThievesTownBigChest => todo,
    ThievesTownBlindSCell => todo,
    ThievesTownBlind => todo,
    ThievesTownPrize => todo,
    TurtleRockChainChomps => todo,
    TurtleRockCompassChest => todo,
    TurtleRockRollerRoomLeft => todo,
    TurtleRockRollerRoomRight => todo,
    TurtleRockBigChest => todo,
    TurtleRockBigKeyChest => todo,
    TurtleRockCrystarollerRoom => todo,
    TurtleRockEyeBridgeBottomLeft => todo,
    TurtleRockEyeBridgeBottomRight => todo,
    TurtleRockEyeBridgeTopLeft => todo,
    TurtleRockEyeBridgeTopRight => todo,
    TurtleRockTrinexx => todo,
    TurtleRockPrize => todo,
    GanonSTowerBobSTorch => todo,
    GanonSTowerDMsRoomTopLeft => todo,
    GanonSTowerDMsRoomTopRight => todo,
    GanonSTowerDMsRoomBottomLeft => todo,
    GanonSTowerDMsRoomBottomRight => todo,
    GanonSTowerRandomizerRoomTopLeft => todo,
    GanonSTowerRandomizerRoomTopRight => todo,
    GanonSTowerRandomizerRoomBottomLeft => todo,
    GanonSTowerRandomizerRoomBottomRight => todo,
    GanonSTowerFiresnakeRoom => todo,
    GanonSTowerMapChest => todo,
    GanonSTowerBigChest => todo,
    GanonSTowerHopeRoomLeft => todo,
    GanonSTowerHopeRoomRight => todo,
    GanonSTowerBobSChest => todo,
    GanonSTowerTileRoom => todo,
    GanonSTowerCompassRoomTopLeft => todo,
    GanonSTowerCompassRoomTopRight => todo,
    GanonSTowerCompassRoomBottomLeft => todo,
    GanonSTowerCompassRoomBottomRight => todo,
    GanonSTowerBigKeyChest => todo,
    GanonSTowerBigKeyRoomLeft => todo,
    GanonSTowerBigKeyRoomRight => todo,
    GanonSTowerMiniHelmasaurRoomLeft => todo,
    GanonSTowerMiniHelmasaurRoomRight => todo,
    GanonSTowerPreMoldormChest => todo,
    GanonSTowerMoldormChest => todo,
    WaterfallBottle => todo,
    PyramidBottle => todo,
    Sanctuary => todo,
    SewersSecretRoomLeft => todo,
    SewersSecretRoomMiddle => todo,
    SewersSecretRoomRight => todo,
    SewersDarkCross => todo,
    HyruleCastleBoomerangChest => todo,
    HyruleCastleMapChest => todo,
    HyruleCastleZeldaSCell => todo,
    CastleTowerRoom03 => todo,
    CastleTowerDarkMaze => todo,
    MireShedLeft => todo,
    MireShedRight => todo,
    Catfish => todo,
    Pyramid => todo,
    PyramidFairySword => todo,
    PyramidFairyBow => todo,
    PyramidFairyLeft => todo,
    PyramidFairyRight => todo,
    Brewery => todo,
    CShapedHouse => todo,
    ChestGame => todo,
    HammerPegs => todo,
    BumperCave => todo,
    Blacksmith => todo,
    PurpleChest => todo,
    HypeCaveTop => todo,
    HypeCaveMiddleRight => todo,
    HypeCaveMiddleLeft => todo,
    HypeCaveBottom => todo,
    Stumpy => todo,
    HypeCaveNPC => todo,
    DiggingGame => todo,
    SuperbunnyCaveTop => todo,
    SuperbunnyCaveBottom => todo,
    HookshotCaveTopRight => todo,
    HookshotCaveTopLeft => todo,
    HookshotCaveBottomLeft => todo,
    HookshotCaveBottomRight => todo,
    SpikeCave => todo,
    SpiralCave => todo,
    MimicCave => todo,
    ParadoxCaveLowerFarLeft => todo,
    ParadoxCaveLowerLeft => todo,
    ParadoxCaveLowerRight => todo,
    ParadoxCaveLowerFarRight => todo,
    ParadoxCaveLowerMiddle => todo,
    ParadoxCaveUpperLeft => todo,
    ParadoxCaveUpperRight => todo,
    FloatingIsland => todo,
    OldMan => todo,
    SpectacleRockCave => todo,
    EtherTablet => todo,
    SpectacleRock => todo,
    MasterSwordPedestal => todo,
    LinkSUncle => todo,
    SecretPassage => todo,
    KingSTomb => todo,
    FloodgateChest => todo,
    LinkSHouse => todo,
    KakarikoTavern => todo,
    ChickenHouse => todo,
    AginahSCave => todo,
    SahasrahlaSHutLeft => todo,
    SahasrahlaSHutMiddle => todo,
    SahasrahlaSHutRight => todo,
    KakrikoWellTop => todo,
    KakrikoWellLeft => todo,
    KakrikoWellMiddle => todo,
    KakrikoWellRight => todo,
    KakrikoWellBottom => todo,
    BlindSHideoutTop => todo,
    BlindSHideoutLeft => todo,
    BlindSHideoutRight => todo,
    BlindSHideoutFarLeft => todo,
    BlindSHideoutFarRight => todo,
    PegasusRocks => todo,
    MiniMoldormCaveFarLeft => todo,
    MiniMoldormCaveLeft => todo,
    MiniMoldormCaveRight => todo,
    MiniMoldormCaveFarRight => todo,
    IceRodCave => todo,
    BottleMerchant => todo,
    Sahasrahla => todo,
    MagicBat => todo,
    SickKid => todo,
    Hobo => todo,
    BombosTablet => todo,
    KingZora => todo,
    LostWoodsHideout => todo,
    LumberjackTree => todo,
    Cave45 => todo,
    GraveyardLedge => todo,
    CheckerboardCave => todo,
    MiniMoldormCaveNPC => todo,
    Library => todo,
    MushroomPatch => todo,
    PotionShop => todo,
    MazeRace => todo,
    DesertLedge => todo,
    LakeHyliaIsland => todo,
    SunkenTreasure => todo,
    ZoraSLedge => todo,
    FluteSpot => todo,
    WaterfallFairyLeft => todo,
    WaterfallFairyRight => todo,
    Agahnim => todo,
    Agahnim2 => todo,
  }
}

pub fn can_enter(
  reg: regions::Region,
  my_items: &Vec<items::Item>,
  _assignments: &HashMap<locations::Location, items::Item>,
) -> bool {
  use items::Item::*;
  use regions::Region::*;

  let todo = true;
  match reg { // @TODO
    LightWorld => todo,
    Escape => todo,
    EasternPalace => todo,
    DesertPalace => {
      my_items.contains(&BookOfMudora)
      || (
        my_items.contains(&MagicMirror)
        && can_lift_dark_rocks(&my_items)
        && can_fly(&my_items)
      )
    },
    WestDeathMountain => todo,
    EastDeathMountain => todo,
    TowerofHera => todo,
    HyruleCastleTower => todo,
    EastDarkWorldDeathMountain => todo,
    WestDarkWorldDeathMountain => todo,
    NorthEastDarkWorld => todo,
    NorthWestDarkWorld => todo,
    SouthDarkWorld => todo,
    Mire => todo,
    PalaceofDarkness => todo,
    SwampPalace => todo,
    SkullWoods => todo,
    ThievesTown => todo,
    IcePalace => todo,
    MiseryMire => todo,
    TurtleRock => todo,
    GanonsTower => todo,
    Fountains => todo,
  }
}

pub fn can_access(
  loc: locations::Location,
  my_items: &Vec<items::Item>,
  assignments: &HashMap<locations::Location, items::Item>,
) -> bool {
  let reg = locations::get_region_for(loc);
  if !can_enter(reg, &my_items, &assignments) {
    return false;
  }

  let todo = true;
  match loc { // @TODO
    DesertPalaceBigChest => {
      my_items.contains(&BigKeyP2)
    },
    DesertPalaceMapChest => true,
    DesertPalaceTorch => {
      my_items.contains(&PegasusBoots)
    },
    DesertPalaceBigKeyChest => {
      my_items.contains(&KeyP2)
    },
    DesertPalaceCompassChest => {
      my_items.contains(&KeyP2)
    },
    DesertPalaceLanmolas => {
      if !(
        has_sword(&my_items)
        || my_items.contains(&Hammer)
        || can_shoot_arrows(&my_items)
        || my_items.contains(&FireRod)
        || my_items.contains(&IceRod)
        || my_items.contains(&CaneOfByrna)
        || my_items.contains(&CaneOfSomaria)
      ) {
        return false;
      }

      (
        can_enter(DesertPalace, &my_items, &assignments)
        && can_lift_rocks(&my_items)
        && can_light_torches(&my_items)
        && my_items.contains(&BigKeyP2)
        && my_items.contains(&KeyP2)
      )
    },
    DesertPalacePrize => can_access(DesertPalaceLanmolas, &my_items, &assignments),
    EasternPalaceCompassChest => todo,
    EasternPalaceBigChest => todo,
    EasternPalaceCannonballChest => todo,
    EasternPalaceBigKeyChest => todo,
    EasternPalaceMapChest => todo,
    EasternPalaceArmosKnights => todo,
    EasternPalacePrize => todo,
    TowerOfHeraBigKeyChest => todo,
    TowerOfHeraBasementCage => todo,
    TowerOfHeraMapChest => todo,
    TowerOfHeraCompassChest => todo,
    TowerOfHeraBigChest => todo,
    TowerOfHeraMoldorm => todo,
    TowerOfHeraPrize => todo,
    IcePalaceBigKeyChest => todo,
    IcePalaceCompassChest => todo,
    IcePalaceMapChest => todo,
    IcePalaceSpikeRoom => todo,
    IcePalaceFreezorChest => todo,
    IcePalaceIcedTRoom => todo,
    IcePalaceBigChest => todo,
    IcePalaceKholdstare => todo,
    IcePalacePrize => todo,
    MiseryMireBigChest => todo,
    MiseryMireMainLobby => todo,
    MiseryMireBigKeyChest => todo,
    MiseryMireCompassChest => todo,
    MiseryMireBridgeChest => todo,
    MiseryMireMapChest => todo,
    MiseryMireSpikeChest => todo,
    MiseryMireVitreous => todo,
    MiseryMirePrize => todo,
    PalaceOfDarknessBigKeyChest => todo,
    PalaceOfDarknessTheArenaLedge => todo,
    PalaceOfDarknessTheArenaBridge => todo,
    PalaceOfDarknessBigChest => todo,
    PalaceOfDarknessCompassChest => todo,
    PalaceOfDarknessHarmlessHellway => todo,
    PalaceOfDarknessStalfosBasement => todo,
    PalaceOfDarknessDarkBasementLeft => todo,
    PalaceOfDarknessDarkBasementRight => todo,
    PalaceOfDarknessMapChest => todo,
    PalaceOfDarknessDarkMazeTop => todo,
    PalaceOfDarknessDarkMazeBottom => todo,
    PalaceOfDarknessShooterRoom => todo,
    PalaceOfDarknessHelmasaurKing => todo,
    PalaceOfDarknessPrize => todo,
    SkullWoodsBigChest => todo,
    SkullWoodsBigKeyChest => todo,
    SkullWoodsCompassChest => todo,
    SkullWoodsMapChest => todo,
    SkullWoodsBridgeRoom => todo,
    SkullWoodsPotPrison => todo,
    SkullWoodsPinballRoom => todo,
    SkullWoodsMothula => todo,
    SkullWoodsPrize => todo,
    SwampPalaceEntrance => todo,
    SwampPalaceBigChest => todo,
    SwampPalaceBigKeyChest => todo,
    SwampPalaceMapChest => todo,
    SwampPalaceWestChest => todo,
    SwampPalaceCompassChest => todo,
    SwampPalaceFloodedRoomLeft => todo,
    SwampPalaceFloodedRoomRight => todo,
    SwampPalaceWaterfallRoom => todo,
    SwampPalaceArrghus => todo,
    SwampPalacePrize => todo,
    ThievesTownAttic => todo,
    ThievesTownBigKeyChest => todo,
    ThievesTownMapChest => todo,
    ThievesTownCompassChest => todo,
    ThievesTownAmbushChest => todo,
    ThievesTownBigChest => todo,
    ThievesTownBlindSCell => todo,
    ThievesTownBlind => todo,
    ThievesTownPrize => todo,
    TurtleRockChainChomps => todo,
    TurtleRockCompassChest => todo,
    TurtleRockRollerRoomLeft => todo,
    TurtleRockRollerRoomRight => todo,
    TurtleRockBigChest => todo,
    TurtleRockBigKeyChest => todo,
    TurtleRockCrystarollerRoom => todo,
    TurtleRockEyeBridgeBottomLeft => todo,
    TurtleRockEyeBridgeBottomRight => todo,
    TurtleRockEyeBridgeTopLeft => todo,
    TurtleRockEyeBridgeTopRight => todo,
    TurtleRockTrinexx => todo,
    TurtleRockPrize => todo,
    GanonSTowerBobSTorch => todo,
    GanonSTowerDMsRoomTopLeft => todo,
    GanonSTowerDMsRoomTopRight => todo,
    GanonSTowerDMsRoomBottomLeft => todo,
    GanonSTowerDMsRoomBottomRight => todo,
    GanonSTowerRandomizerRoomTopLeft => todo,
    GanonSTowerRandomizerRoomTopRight => todo,
    GanonSTowerRandomizerRoomBottomLeft => todo,
    GanonSTowerRandomizerRoomBottomRight => todo,
    GanonSTowerFiresnakeRoom => todo,
    GanonSTowerMapChest => todo,
    GanonSTowerBigChest => todo,
    GanonSTowerHopeRoomLeft => todo,
    GanonSTowerHopeRoomRight => todo,
    GanonSTowerBobSChest => todo,
    GanonSTowerTileRoom => todo,
    GanonSTowerCompassRoomTopLeft => todo,
    GanonSTowerCompassRoomTopRight => todo,
    GanonSTowerCompassRoomBottomLeft => todo,
    GanonSTowerCompassRoomBottomRight => todo,
    GanonSTowerBigKeyChest => todo,
    GanonSTowerBigKeyRoomLeft => todo,
    GanonSTowerBigKeyRoomRight => todo,
    GanonSTowerMiniHelmasaurRoomLeft => todo,
    GanonSTowerMiniHelmasaurRoomRight => todo,
    GanonSTowerPreMoldormChest => todo,
    GanonSTowerMoldormChest => todo,
    WaterfallBottle => todo,
    PyramidBottle => todo,
    Sanctuary => todo,
    SewersSecretRoomLeft => todo,
    SewersSecretRoomMiddle => todo,
    SewersSecretRoomRight => todo,
    SewersDarkCross => todo,
    HyruleCastleBoomerangChest => todo,
    HyruleCastleMapChest => todo,
    HyruleCastleZeldaSCell => todo,
    CastleTowerRoom03 => todo,
    CastleTowerDarkMaze => todo,
    MireShedLeft => todo,
    MireShedRight => todo,
    Catfish => todo,
    Pyramid => todo,
    PyramidFairySword => todo,
    PyramidFairyBow => todo,
    PyramidFairyLeft => todo,
    PyramidFairyRight => todo,
    Brewery => todo,
    CShapedHouse => todo,
    ChestGame => todo,
    HammerPegs => todo,
    BumperCave => todo,
    Blacksmith => todo,
    PurpleChest => todo,
    HypeCaveTop => todo,
    HypeCaveMiddleRight => todo,
    HypeCaveMiddleLeft => todo,
    HypeCaveBottom => todo,
    Stumpy => todo,
    HypeCaveNPC => todo,
    DiggingGame => todo,
    SuperbunnyCaveTop => todo,
    SuperbunnyCaveBottom => todo,
    HookshotCaveTopRight => todo,
    HookshotCaveTopLeft => todo,
    HookshotCaveBottomLeft => todo,
    HookshotCaveBottomRight => todo,
    SpikeCave => todo,
    SpiralCave => todo,
    MimicCave => todo,
    ParadoxCaveLowerFarLeft => todo,
    ParadoxCaveLowerLeft => todo,
    ParadoxCaveLowerRight => todo,
    ParadoxCaveLowerFarRight => todo,
    ParadoxCaveLowerMiddle => todo,
    ParadoxCaveUpperLeft => todo,
    ParadoxCaveUpperRight => todo,
    FloatingIsland => todo,
    OldMan => todo,
    SpectacleRockCave => todo,
    EtherTablet => todo,
    SpectacleRock => todo,
    MasterSwordPedestal => todo,
    LinkSUncle => todo,
    SecretPassage => todo,
    KingSTomb => todo,
    FloodgateChest => todo,
    LinkSHouse => todo,
    KakarikoTavern => todo,
    ChickenHouse => todo,
    AginahSCave => todo,
    SahasrahlaSHutLeft => todo,
    SahasrahlaSHutMiddle => todo,
    SahasrahlaSHutRight => todo,
    KakrikoWellTop => todo,
    KakrikoWellLeft => todo,
    KakrikoWellMiddle => todo,
    KakrikoWellRight => todo,
    KakrikoWellBottom => todo,
    BlindSHideoutTop => todo,
    BlindSHideoutLeft => todo,
    BlindSHideoutRight => todo,
    BlindSHideoutFarLeft => todo,
    BlindSHideoutFarRight => todo,
    PegasusRocks => todo,
    MiniMoldormCaveFarLeft => todo,
    MiniMoldormCaveLeft => todo,
    MiniMoldormCaveRight => todo,
    MiniMoldormCaveFarRight => todo,
    IceRodCave => todo,
    BottleMerchant => todo,
    Sahasrahla => todo,
    MagicBat => todo,
    SickKid => todo,
    Hobo => todo,
    BombosTablet => todo,
    KingZora => todo,
    LostWoodsHideout => todo,
    LumberjackTree => todo,
    Cave45 => todo,
    GraveyardLedge => todo,
    CheckerboardCave => todo,
    MiniMoldormCaveNPC => todo,
    Library => todo,
    MushroomPatch => todo,
    PotionShop => todo,
    MazeRace => todo,
    DesertLedge => todo,
    LakeHyliaIsland => todo,
    SunkenTreasure => todo,
    ZoraSLedge => todo,
    FluteSpot => todo,
    WaterfallFairyLeft => todo,
    WaterfallFairyRight => todo,
    Agahnim => todo,
    Agahnim2 => todo,
  }
}
