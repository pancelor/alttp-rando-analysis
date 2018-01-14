#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::{locations, items};

#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Region {
  LightWorld,
  Escape,
  EasternPalace,
  DesertPalace,
  WestDeathMountain,
  EastDeathMountain,
  TowerofHera,
  HyruleCastleTower,
  EastDarkWorldDeathMountain,
  WestDarkWorldDeathMountain,
  NorthEastDarkWorld,
  NorthWestDarkWorld,
  SouthDarkWorld,
  Mire,
  PalaceofDarkness,
  SwampPalace,
  SkullWoods,
  ThievesTown,
  IcePalace,
  MiseryMire,
  TurtleRock,
  GanonsTower,
  Fountains,
}

pub fn get_locations_for(reg: Region) -> Vec<locations::Location> {
  use self::Region::*;
  use locations::Location::*;
  match reg {
    DesertPalace => vec![
      DesertPalaceBigChest,
      DesertPalaceMapChest,
      DesertPalaceTorch,
      DesertPalaceBigKeyChest,
      DesertPalaceCompassChest,
      DesertPalaceLanmolas,
      DesertPalacePrize,
    ],
    EasternPalace => vec![
      EasternPalaceCompassChest,
      EasternPalaceBigChest,
      EasternPalaceCannonballChest,
      EasternPalaceBigKeyChest,
      EasternPalaceMapChest,
      EasternPalaceArmosKnights,
      EasternPalacePrize,
    ],
    TowerofHera => vec![
      TowerOfHeraBigKeyChest,
      TowerOfHeraBasementCage,
      TowerOfHeraMapChest,
      TowerOfHeraCompassChest,
      TowerOfHeraBigChest,
      TowerOfHeraMoldorm,
      TowerOfHeraPrize,
    ],
    IcePalace => vec![
      IcePalaceBigKeyChest,
      IcePalaceCompassChest,
      IcePalaceMapChest,
      IcePalaceSpikeRoom,
      IcePalaceFreezorChest,
      IcePalaceIcedTRoom,
      IcePalaceBigChest,
      IcePalaceKholdstare,
      IcePalacePrize,
    ],
    MiseryMire => vec![
      MiseryMireBigChest,
      MiseryMireMainLobby,
      MiseryMireBigKeyChest,
      MiseryMireCompassChest,
      MiseryMireBridgeChest,
      MiseryMireMapChest,
      MiseryMireSpikeChest,
      MiseryMireVitreous,
      MiseryMirePrize,
    ],
    PalaceofDarkness => vec![
      PalaceOfDarknessBigKeyChest,
      PalaceOfDarknessTheArenaLedge,
      PalaceOfDarknessTheArenaBridge,
      PalaceOfDarknessBigChest,
      PalaceOfDarknessCompassChest,
      PalaceOfDarknessHarmlessHellway,
      PalaceOfDarknessStalfosBasement,
      PalaceOfDarknessDarkBasementLeft,
      PalaceOfDarknessDarkBasementRight,
      PalaceOfDarknessMapChest,
      PalaceOfDarknessDarkMazeTop,
      PalaceOfDarknessDarkMazeBottom,
      PalaceOfDarknessShooterRoom,
      PalaceOfDarknessHelmasaurKing,
      PalaceOfDarknessPrize,
    ],
    SkullWoods => vec![
      SkullWoodsBigChest,
      SkullWoodsBigKeyChest,
      SkullWoodsCompassChest,
      SkullWoodsMapChest,
      SkullWoodsBridgeRoom,
      SkullWoodsPotPrison,
      SkullWoodsPinballRoom,
      SkullWoodsMothula,
      SkullWoodsPrize,
    ],
    SwampPalace => vec![
      SwampPalaceEntrance,
      SwampPalaceBigChest,
      SwampPalaceBigKeyChest,
      SwampPalaceMapChest,
      SwampPalaceWestChest,
      SwampPalaceCompassChest,
      SwampPalaceFloodedRoomLeft,
      SwampPalaceFloodedRoomRight,
      SwampPalaceWaterfallRoom,
      SwampPalaceArrghus,
      SwampPalacePrize,
    ],
    ThievesTown => vec![
      ThievesTownAttic,
      ThievesTownBigKeyChest,
      ThievesTownMapChest,
      ThievesTownCompassChest,
      ThievesTownAmbushChest,
      ThievesTownBigChest,
      ThievesTownBlindSCell,
      ThievesTownBlind,
      ThievesTownPrize,
    ],
    TurtleRock => vec![
      TurtleRockChainChomps,
      TurtleRockCompassChest,
      TurtleRockRollerRoomLeft,
      TurtleRockRollerRoomRight,
      TurtleRockBigChest,
      TurtleRockBigKeyChest,
      TurtleRockCrystarollerRoom,
      TurtleRockEyeBridgeBottomLeft,
      TurtleRockEyeBridgeBottomRight,
      TurtleRockEyeBridgeTopLeft,
      TurtleRockEyeBridgeTopRight,
      TurtleRockTrinexx,
      TurtleRockPrize,
    ],
    GanonsTower => vec![
      GanonSTowerBobSTorch,
      GanonSTowerDMsRoomTopLeft,
      GanonSTowerDMsRoomTopRight,
      GanonSTowerDMsRoomBottomLeft,
      GanonSTowerDMsRoomBottomRight,
      GanonSTowerRandomizerRoomTopLeft,
      GanonSTowerRandomizerRoomTopRight,
      GanonSTowerRandomizerRoomBottomLeft,
      GanonSTowerRandomizerRoomBottomRight,
      GanonSTowerFiresnakeRoom,
      GanonSTowerMapChest,
      GanonSTowerBigChest,
      GanonSTowerHopeRoomLeft,
      GanonSTowerHopeRoomRight,
      GanonSTowerBobSChest,
      GanonSTowerTileRoom,
      GanonSTowerCompassRoomTopLeft,
      GanonSTowerCompassRoomTopRight,
      GanonSTowerCompassRoomBottomLeft,
      GanonSTowerCompassRoomBottomRight,
      GanonSTowerBigKeyChest,
      GanonSTowerBigKeyRoomLeft,
      GanonSTowerBigKeyRoomRight,
      GanonSTowerMiniHelmasaurRoomLeft,
      GanonSTowerMiniHelmasaurRoomRight,
      GanonSTowerPreMoldormChest,
      GanonSTowerMoldormChest,
      Agahnim2,
    ],
    Fountains => vec![
      WaterfallBottle,
      PyramidBottle,
    ],
    Escape => vec![
      Sanctuary,
      SewersSecretRoomLeft,
      SewersSecretRoomMiddle,
      SewersSecretRoomRight,
      SewersDarkCross,
      HyruleCastleBoomerangChest,
      HyruleCastleMapChest,
      HyruleCastleZeldaSCell,
    ],
    HyruleCastleTower => vec![
      CastleTowerRoom03,
      CastleTowerDarkMaze,
      Agahnim,
    ],
    Mire => vec![
      MireShedLeft,
      MireShedRight,
    ],
    NorthEastDarkWorld => vec![
      Catfish,
      Pyramid,
      PyramidFairySword,
      PyramidFairyBow,
      PyramidFairyLeft ,
      PyramidFairyRight,
      Ganon,
    ],
    NorthWestDarkWorld => vec![
      Brewery,
      CShapedHouse,
      ChestGame,
      HammerPegs,
      BumperCave,
      Blacksmith,
      PurpleChest,
    ],
    SouthDarkWorld => vec![
      HypeCaveTop,
      HypeCaveMiddleRight,
      HypeCaveMiddleLeft,
      HypeCaveBottom,
      Stumpy,
      HypeCaveNPC,
      DiggingGame,
    ],
    EastDarkWorldDeathMountain => vec![
      SuperbunnyCaveTop,
      SuperbunnyCaveBottom,
      HookshotCaveTopRight,
      HookshotCaveTopLeft,
      HookshotCaveBottomLeft,
      HookshotCaveBottomRight,
    ],
    WestDarkWorldDeathMountain => vec![
      SpikeCave,
    ],
    EastDeathMountain => vec![
      SpiralCave,
      MimicCave,
      ParadoxCaveLowerFarLeft,
      ParadoxCaveLowerLeft,
      ParadoxCaveLowerRight,
      ParadoxCaveLowerFarRight,
      ParadoxCaveLowerMiddle,
      ParadoxCaveUpperLeft,
      ParadoxCaveUpperRight,
      FloatingIsland,
    ],
    WestDeathMountain => vec![
      OldMan,
      SpectacleRockCave,
      EtherTablet,
      SpectacleRock,
    ],
    LightWorld => vec![
      MasterSwordPedestal,
      LinkSUncle,
      SecretPassage,
      KingSTomb,
      FloodgateChest,
      LinkSHouse,
      KakarikoTavern,
      ChickenHouse,
      AginahSCave,
      SahasrahlaSHutLeft,
      SahasrahlaSHutMiddle,
      SahasrahlaSHutRight,
      KakrikoWellTop,
      KakrikoWellLeft,
      KakrikoWellMiddle,
      KakrikoWellRight,
      KakrikoWellBottom,
      BlindSHideoutTop,
      BlindSHideoutLeft,
      BlindSHideoutRight,
      BlindSHideoutFarLeft,
      BlindSHideoutFarRight,
      PegasusRocks,
      MiniMoldormCaveFarLeft,
      MiniMoldormCaveLeft,
      MiniMoldormCaveRight,
      MiniMoldormCaveFarRight,
      IceRodCave,
      BottleMerchant,
      Sahasrahla,
      MagicBat,
      SickKid,
      Hobo,
      BombosTablet,
      KingZora,
      LostWoodsHideout,
      LumberjackTree,
      Cave45,
      GraveyardLedge,
      CheckerboardCave,
      MiniMoldormCaveNPC,
      Library,
      MushroomPatch,
      PotionShop,
      MazeRace,
      DesertLedge,
      LakeHyliaIsland,
      SunkenTreasure,
      ZoraSLedge,
      FluteSpot,
      WaterfallFairyLeft,
      WaterfallFairyRight,
    ],
  }
}

pub fn get_dungeon_items_for(reg: Region) -> Option<Vec<items::Item>> {
  use self::Region::*;
  use items::Item::*;
  match reg {
    DesertPalace => Some(vec![
      BigKeyP2,
      CompassP2,
      KeyP2,
      MapP2,
    ]),
    EasternPalace => Some(vec![
      BigKeyP1,
      CompassP1,
      KeyP1,
      MapP1,
    ]),
    TowerofHera => Some(vec![
      BigKeyP3,
      CompassP3,
      KeyP3,
      MapP3,
    ]),
    IcePalace => Some(vec![
      BigKeyD5,
      CompassD5,
      KeyD5,
      MapD5,
    ]),
    MiseryMire => Some(vec![
      BigKeyD6,
      CompassD6,
      KeyD6,
      MapD6,
    ]),
    PalaceofDarkness => Some(vec![
      BigKeyD1,
      CompassD1,
      KeyD1,
      MapD1,
    ]),
    SkullWoods => Some(vec![
      BigKeyD3,
      CompassD3,
      KeyD3,
      MapD3,
    ]),
    SwampPalace => Some(vec![
      BigKeyD2,
      CompassD2,
      KeyD2,
      MapD2,
    ]),
    ThievesTown => Some(vec![
      BigKeyD4,
      CompassD4,
      KeyD4,
      MapD4,
    ]),
    TurtleRock => Some(vec![
      BigKeyD7,
      CompassD7,
      KeyD7,
      MapD7,
    ]),
    GanonsTower => Some(vec![
      BigKeyA2,
      CompassA2,
      KeyA2,
      MapA2,
    ]),
    Fountains => None,
    Escape => Some(vec![
      BigKeyH2,
      CompassH2,
      KeyH2,
      MapH2,
    ]),
    HyruleCastleTower => Some(vec![
      BigKeyA1,
      CompassA1,
      KeyA1,
      MapA1,
    ]),
    Mire => None,
    NorthEastDarkWorld => None,
    NorthWestDarkWorld => None,
    SouthDarkWorld => None,
    EastDarkWorldDeathMountain => None,
    WestDarkWorldDeathMountain => None,
    EastDeathMountain => None,
    WestDeathMountain => None,
    LightWorld => None,
  }
}