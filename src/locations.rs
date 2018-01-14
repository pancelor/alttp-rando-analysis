#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use multiset::HashMultiSet;
use super::{regions};

// CamelCase versions of the php names for all locations
#[allow(dead_code)]
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Location {
  DesertPalaceBigChest,
  DesertPalaceMapChest,
  DesertPalaceTorch,
  DesertPalaceBigKeyChest,
  DesertPalaceCompassChest,
  DesertPalaceLanmolas,
  DesertPalacePrize,
  EasternPalaceCompassChest,
  EasternPalaceBigChest,
  EasternPalaceCannonballChest,
  EasternPalaceBigKeyChest,
  EasternPalaceMapChest,
  EasternPalaceArmosKnights,
  EasternPalacePrize,
  TowerOfHeraBigKeyChest,
  TowerOfHeraBasementCage,
  TowerOfHeraMapChest,
  TowerOfHeraCompassChest,
  TowerOfHeraBigChest,
  TowerOfHeraMoldorm,
  TowerOfHeraPrize,
  IcePalaceBigKeyChest,
  IcePalaceCompassChest,
  IcePalaceMapChest,
  IcePalaceSpikeRoom,
  IcePalaceFreezorChest,
  IcePalaceIcedTRoom,
  IcePalaceBigChest,
  IcePalaceKholdstare,
  IcePalacePrize,
  MiseryMireBigChest,
  MiseryMireMainLobby,
  MiseryMireBigKeyChest,
  MiseryMireCompassChest,
  MiseryMireBridgeChest,
  MiseryMireMapChest,
  MiseryMireSpikeChest,
  MiseryMireVitreous,
  MiseryMirePrize,
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
  SkullWoodsBigChest,
  SkullWoodsBigKeyChest,
  SkullWoodsCompassChest,
  SkullWoodsMapChest,
  SkullWoodsBridgeRoom,
  SkullWoodsPotPrison,
  SkullWoodsPinballRoom,
  SkullWoodsMothula,
  SkullWoodsPrize,
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
  ThievesTownAttic,
  ThievesTownBigKeyChest,
  ThievesTownMapChest,
  ThievesTownCompassChest,
  ThievesTownAmbushChest,
  ThievesTownBigChest,
  ThievesTownBlindSCell,
  ThievesTownBlind,
  ThievesTownPrize,
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
  WaterfallBottle,
  PyramidBottle,
  Sanctuary,
  SewersSecretRoomLeft,
  SewersSecretRoomMiddle,
  SewersSecretRoomRight,
  SewersDarkCross,
  HyruleCastleBoomerangChest,
  HyruleCastleMapChest,
  HyruleCastleZeldaSCell,
  CastleTowerRoom03,
  CastleTowerDarkMaze,
  Agahnim,
  MireShedLeft,
  MireShedRight,
  Catfish,
  Pyramid,
  PyramidFairySword,
  PyramidFairyBow,
  PyramidFairyLeft,
  PyramidFairyRight,
  Ganon,
  Brewery,
  CShapedHouse,
  ChestGame,
  HammerPegs,
  BumperCave,
  Blacksmith,
  PurpleChest,
  HypeCaveTop,
  HypeCaveMiddleRight,
  HypeCaveMiddleLeft,
  HypeCaveBottom,
  Stumpy,
  HypeCaveNPC,
  DiggingGame,
  SuperbunnyCaveTop,
  SuperbunnyCaveBottom,
  HookshotCaveTopRight,
  HookshotCaveTopLeft,
  HookshotCaveBottomLeft,
  HookshotCaveBottomRight,
  SpikeCave,
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
  OldMan,
  SpectacleRockCave,
  EtherTablet,
  SpectacleRock,
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
  MushroomPatch, // @hack: changed the name here to avoid collision with the item
  PotionShop,
  MazeRace,
  DesertLedge,
  LakeHyliaIsland,
  SunkenTreasure,
  ZoraSLedge,
  FluteSpot,
  WaterfallFairyLeft,
  WaterfallFairyRight,
}

pub fn get_all_locations() -> Vec<Location> {
  use self::Location::*;
  vec![
    DesertPalaceBigChest,
    DesertPalaceMapChest,
    DesertPalaceTorch,
    DesertPalaceBigKeyChest,
    DesertPalaceCompassChest,
    DesertPalaceLanmolas,
    DesertPalacePrize,
    EasternPalaceCompassChest,
    EasternPalaceBigChest,
    EasternPalaceCannonballChest,
    EasternPalaceBigKeyChest,
    EasternPalaceMapChest,
    EasternPalaceArmosKnights,
    EasternPalacePrize,
    TowerOfHeraBigKeyChest,
    TowerOfHeraBasementCage,
    TowerOfHeraMapChest,
    TowerOfHeraCompassChest,
    TowerOfHeraBigChest,
    TowerOfHeraMoldorm,
    TowerOfHeraPrize,
    IcePalaceBigKeyChest,
    IcePalaceCompassChest,
    IcePalaceMapChest,
    IcePalaceSpikeRoom,
    IcePalaceFreezorChest,
    IcePalaceIcedTRoom,
    IcePalaceBigChest,
    IcePalaceKholdstare,
    IcePalacePrize,
    MiseryMireBigChest,
    MiseryMireMainLobby,
    MiseryMireBigKeyChest,
    MiseryMireCompassChest,
    MiseryMireBridgeChest,
    MiseryMireMapChest,
    MiseryMireSpikeChest,
    MiseryMireVitreous,
    MiseryMirePrize,
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
    SkullWoodsBigChest,
    SkullWoodsBigKeyChest,
    SkullWoodsCompassChest,
    SkullWoodsMapChest,
    SkullWoodsBridgeRoom,
    SkullWoodsPotPrison,
    SkullWoodsPinballRoom,
    SkullWoodsMothula,
    SkullWoodsPrize,
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
    ThievesTownAttic,
    ThievesTownBigKeyChest,
    ThievesTownMapChest,
    ThievesTownCompassChest,
    ThievesTownAmbushChest,
    ThievesTownBigChest,
    ThievesTownBlindSCell,
    ThievesTownBlind,
    ThievesTownPrize,
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
    WaterfallBottle,
    PyramidBottle,
    Sanctuary,
    SewersSecretRoomLeft,
    SewersSecretRoomMiddle,
    SewersSecretRoomRight,
    SewersDarkCross,
    HyruleCastleBoomerangChest,
    HyruleCastleMapChest,
    HyruleCastleZeldaSCell,
    CastleTowerRoom03,
    CastleTowerDarkMaze,
    Agahnim,
    MireShedLeft,
    MireShedRight,
    Catfish,
    Pyramid,
    PyramidFairySword,
    PyramidFairyBow,
    PyramidFairyLeft,
    PyramidFairyRight,
    Ganon,
    Brewery,
    CShapedHouse,
    ChestGame,
    HammerPegs,
    BumperCave,
    Blacksmith,
    PurpleChest,
    HypeCaveTop,
    HypeCaveMiddleRight,
    HypeCaveMiddleLeft,
    HypeCaveBottom,
    Stumpy,
    HypeCaveNPC,
    DiggingGame,
    SuperbunnyCaveTop,
    SuperbunnyCaveBottom,
    HookshotCaveTopRight,
    HookshotCaveTopLeft,
    HookshotCaveBottomLeft,
    HookshotCaveBottomRight,
    SpikeCave,
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
    OldMan,
    SpectacleRockCave,
    EtherTablet,
    SpectacleRock,
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
  ]
}

pub fn get_region_for(loc: Location) -> regions::Region {
  use self::Location::*;
  use regions::Region::*;

  match loc {
    DesertPalaceBigChest                 => DesertPalace,
    DesertPalaceMapChest                 => DesertPalace,
    DesertPalaceTorch                    => DesertPalace,
    DesertPalaceBigKeyChest              => DesertPalace,
    DesertPalaceCompassChest             => DesertPalace,
    DesertPalaceLanmolas                 => DesertPalace,
    DesertPalacePrize                    => DesertPalace,

    EasternPalaceCompassChest            => EasternPalace,
    EasternPalaceBigChest                => EasternPalace,
    EasternPalaceCannonballChest         => EasternPalace,
    EasternPalaceBigKeyChest             => EasternPalace,
    EasternPalaceMapChest                => EasternPalace,
    EasternPalaceArmosKnights            => EasternPalace,
    EasternPalacePrize                   => EasternPalace,

    TowerOfHeraBigKeyChest               => TowerofHera,
    TowerOfHeraBasementCage              => TowerofHera,
    TowerOfHeraMapChest                  => TowerofHera,
    TowerOfHeraCompassChest              => TowerofHera,
    TowerOfHeraBigChest                  => TowerofHera,
    TowerOfHeraMoldorm                   => TowerofHera,
    TowerOfHeraPrize                     => TowerofHera,

    IcePalaceBigKeyChest                 => IcePalace,
    IcePalaceCompassChest                => IcePalace,
    IcePalaceMapChest                    => IcePalace,
    IcePalaceSpikeRoom                   => IcePalace,
    IcePalaceFreezorChest                => IcePalace,
    IcePalaceIcedTRoom                   => IcePalace,
    IcePalaceBigChest                    => IcePalace,
    IcePalaceKholdstare                  => IcePalace,
    IcePalacePrize                       => IcePalace,

    MiseryMireBigChest                   => MiseryMire,
    MiseryMireMainLobby                  => MiseryMire,
    MiseryMireBigKeyChest                => MiseryMire,
    MiseryMireCompassChest               => MiseryMire,
    MiseryMireBridgeChest                => MiseryMire,
    MiseryMireMapChest                   => MiseryMire,
    MiseryMireSpikeChest                 => MiseryMire,
    MiseryMireVitreous                   => MiseryMire,
    MiseryMirePrize                      => MiseryMire,

    PalaceOfDarknessBigKeyChest          => PalaceofDarkness,
    PalaceOfDarknessTheArenaLedge        => PalaceofDarkness,
    PalaceOfDarknessTheArenaBridge       => PalaceofDarkness,
    PalaceOfDarknessBigChest             => PalaceofDarkness,
    PalaceOfDarknessCompassChest         => PalaceofDarkness,
    PalaceOfDarknessHarmlessHellway      => PalaceofDarkness,
    PalaceOfDarknessStalfosBasement      => PalaceofDarkness,
    PalaceOfDarknessDarkBasementLeft     => PalaceofDarkness,
    PalaceOfDarknessDarkBasementRight    => PalaceofDarkness,
    PalaceOfDarknessMapChest             => PalaceofDarkness,
    PalaceOfDarknessDarkMazeTop          => PalaceofDarkness,
    PalaceOfDarknessDarkMazeBottom       => PalaceofDarkness,
    PalaceOfDarknessShooterRoom          => PalaceofDarkness,
    PalaceOfDarknessHelmasaurKing        => PalaceofDarkness,
    PalaceOfDarknessPrize                => PalaceofDarkness,

    SkullWoodsBigChest                   => SkullWoods,
    SkullWoodsBigKeyChest                => SkullWoods,
    SkullWoodsCompassChest               => SkullWoods,
    SkullWoodsMapChest                   => SkullWoods,
    SkullWoodsBridgeRoom                 => SkullWoods,
    SkullWoodsPotPrison                  => SkullWoods,
    SkullWoodsPinballRoom                => SkullWoods,
    SkullWoodsMothula                    => SkullWoods,
    SkullWoodsPrize                      => SkullWoods,

    SwampPalaceEntrance                  => SwampPalace,
    SwampPalaceBigChest                  => SwampPalace,
    SwampPalaceBigKeyChest               => SwampPalace,
    SwampPalaceMapChest                  => SwampPalace,
    SwampPalaceWestChest                 => SwampPalace,
    SwampPalaceCompassChest              => SwampPalace,
    SwampPalaceFloodedRoomLeft           => SwampPalace,
    SwampPalaceFloodedRoomRight          => SwampPalace,
    SwampPalaceWaterfallRoom             => SwampPalace,
    SwampPalaceArrghus                   => SwampPalace,
    SwampPalacePrize                     => SwampPalace,

    ThievesTownAttic                     => ThievesTown,
    ThievesTownBigKeyChest               => ThievesTown,
    ThievesTownMapChest                  => ThievesTown,
    ThievesTownCompassChest              => ThievesTown,
    ThievesTownAmbushChest               => ThievesTown,
    ThievesTownBigChest                  => ThievesTown,
    ThievesTownBlindSCell                => ThievesTown,
    ThievesTownBlind                     => ThievesTown,
    ThievesTownPrize                     => ThievesTown,

    TurtleRockChainChomps                => TurtleRock,
    TurtleRockCompassChest               => TurtleRock,
    TurtleRockRollerRoomLeft             => TurtleRock,
    TurtleRockRollerRoomRight            => TurtleRock,
    TurtleRockBigChest                   => TurtleRock,
    TurtleRockBigKeyChest                => TurtleRock,
    TurtleRockCrystarollerRoom           => TurtleRock,
    TurtleRockEyeBridgeBottomLeft        => TurtleRock,
    TurtleRockEyeBridgeBottomRight       => TurtleRock,
    TurtleRockEyeBridgeTopLeft           => TurtleRock,
    TurtleRockEyeBridgeTopRight          => TurtleRock,
    TurtleRockTrinexx                    => TurtleRock,
    TurtleRockPrize                      => TurtleRock,

    GanonSTowerBobSTorch                 => GanonsTower,
    GanonSTowerDMsRoomTopLeft            => GanonsTower,
    GanonSTowerDMsRoomTopRight           => GanonsTower,
    GanonSTowerDMsRoomBottomLeft         => GanonsTower,
    GanonSTowerDMsRoomBottomRight        => GanonsTower,
    GanonSTowerRandomizerRoomTopLeft     => GanonsTower,
    GanonSTowerRandomizerRoomTopRight    => GanonsTower,
    GanonSTowerRandomizerRoomBottomLeft  => GanonsTower,
    GanonSTowerRandomizerRoomBottomRight => GanonsTower,
    GanonSTowerFiresnakeRoom             => GanonsTower,
    GanonSTowerMapChest                  => GanonsTower,
    GanonSTowerBigChest                  => GanonsTower,
    GanonSTowerHopeRoomLeft              => GanonsTower,
    GanonSTowerHopeRoomRight             => GanonsTower,
    GanonSTowerBobSChest                 => GanonsTower,
    GanonSTowerTileRoom                  => GanonsTower,
    GanonSTowerCompassRoomTopLeft        => GanonsTower,
    GanonSTowerCompassRoomTopRight       => GanonsTower,
    GanonSTowerCompassRoomBottomLeft     => GanonsTower,
    GanonSTowerCompassRoomBottomRight    => GanonsTower,
    GanonSTowerBigKeyChest               => GanonsTower,
    GanonSTowerBigKeyRoomLeft            => GanonsTower,
    GanonSTowerBigKeyRoomRight           => GanonsTower,
    GanonSTowerMiniHelmasaurRoomLeft     => GanonsTower,
    GanonSTowerMiniHelmasaurRoomRight    => GanonsTower,
    GanonSTowerPreMoldormChest           => GanonsTower,
    GanonSTowerMoldormChest              => GanonsTower,
    Agahnim2                             => GanonsTower,

    WaterfallBottle                      => Fountains,
    PyramidBottle                        => Fountains,

    Sanctuary                            => Escape,
    SewersSecretRoomLeft                 => Escape,
    SewersSecretRoomMiddle               => Escape,
    SewersSecretRoomRight                => Escape,
    SewersDarkCross                      => Escape,
    HyruleCastleBoomerangChest           => Escape,
    HyruleCastleMapChest                 => Escape,
    HyruleCastleZeldaSCell               => Escape,

    CastleTowerRoom03                    => HyruleCastleTower,
    CastleTowerDarkMaze                  => HyruleCastleTower,
    Agahnim                              => HyruleCastleTower,

    MireShedLeft                         => Mire,
    MireShedRight                        => Mire,

    Catfish                              => NorthEastDarkWorld,
    Pyramid                              => NorthEastDarkWorld,
    PyramidFairySword                    => NorthEastDarkWorld,
    PyramidFairyBow                      => NorthEastDarkWorld,
    PyramidFairyLeft                     => NorthEastDarkWorld,
    PyramidFairyRight                    => NorthEastDarkWorld,
    Ganon                                => NorthEastDarkWorld,

    Brewery                              => NorthWestDarkWorld,
    CShapedHouse                         => NorthWestDarkWorld,
    ChestGame                            => NorthWestDarkWorld,
    HammerPegs                           => NorthWestDarkWorld,
    BumperCave                           => NorthWestDarkWorld,
    Blacksmith                           => NorthWestDarkWorld,
    PurpleChest                          => NorthWestDarkWorld,

    HypeCaveTop                          => SouthDarkWorld,
    HypeCaveMiddleRight                  => SouthDarkWorld,
    HypeCaveMiddleLeft                   => SouthDarkWorld,
    HypeCaveBottom                       => SouthDarkWorld,
    Stumpy                               => SouthDarkWorld,
    HypeCaveNPC                          => SouthDarkWorld,
    DiggingGame                          => SouthDarkWorld,

    SuperbunnyCaveTop                    => EastDarkWorldDeathMountain,
    SuperbunnyCaveBottom                 => EastDarkWorldDeathMountain,
    HookshotCaveTopRight                 => EastDarkWorldDeathMountain,
    HookshotCaveTopLeft                  => EastDarkWorldDeathMountain,
    HookshotCaveBottomLeft               => EastDarkWorldDeathMountain,
    HookshotCaveBottomRight              => EastDarkWorldDeathMountain,

    SpikeCave                            => WestDarkWorldDeathMountain,

    SpiralCave                           => EastDeathMountain,
    MimicCave                            => EastDeathMountain,
    ParadoxCaveLowerFarLeft              => EastDeathMountain,
    ParadoxCaveLowerLeft                 => EastDeathMountain,
    ParadoxCaveLowerRight                => EastDeathMountain,
    ParadoxCaveLowerFarRight             => EastDeathMountain,
    ParadoxCaveLowerMiddle               => EastDeathMountain,
    ParadoxCaveUpperLeft                 => EastDeathMountain,
    ParadoxCaveUpperRight                => EastDeathMountain,
    FloatingIsland                       => EastDeathMountain,

    OldMan                               => WestDeathMountain,
    SpectacleRockCave                    => WestDeathMountain,
    EtherTablet                          => WestDeathMountain,
    SpectacleRock                        => WestDeathMountain,

    MasterSwordPedestal                  => LightWorld,
    LinkSUncle                           => LightWorld,
    SecretPassage                        => LightWorld,
    KingSTomb                            => LightWorld,
    FloodgateChest                       => LightWorld,
    LinkSHouse                           => LightWorld,
    KakarikoTavern                       => LightWorld,
    ChickenHouse                         => LightWorld,
    AginahSCave                          => LightWorld,
    SahasrahlaSHutLeft                   => LightWorld,
    SahasrahlaSHutMiddle                 => LightWorld,
    SahasrahlaSHutRight                  => LightWorld,
    KakrikoWellTop                       => LightWorld,
    KakrikoWellLeft                      => LightWorld,
    KakrikoWellMiddle                    => LightWorld,
    KakrikoWellRight                     => LightWorld,
    KakrikoWellBottom                    => LightWorld,
    BlindSHideoutTop                     => LightWorld,
    BlindSHideoutLeft                    => LightWorld,
    BlindSHideoutRight                   => LightWorld,
    BlindSHideoutFarLeft                 => LightWorld,
    BlindSHideoutFarRight                => LightWorld,
    PegasusRocks                         => LightWorld,
    MiniMoldormCaveFarLeft               => LightWorld,
    MiniMoldormCaveLeft                  => LightWorld,
    MiniMoldormCaveRight                 => LightWorld,
    MiniMoldormCaveFarRight              => LightWorld,
    IceRodCave                           => LightWorld,
    BottleMerchant                       => LightWorld,
    Sahasrahla                           => LightWorld,
    MagicBat                             => LightWorld,
    SickKid                              => LightWorld,
    Hobo                                 => LightWorld,
    BombosTablet                         => LightWorld,
    KingZora                             => LightWorld,
    LostWoodsHideout                     => LightWorld,
    LumberjackTree                       => LightWorld,
    Cave45                               => LightWorld,
    GraveyardLedge                       => LightWorld,
    CheckerboardCave                     => LightWorld,
    MiniMoldormCaveNPC                   => LightWorld,
    Library                              => LightWorld,
    MushroomPatch                        => LightWorld,
    PotionShop                           => LightWorld,
    MazeRace                             => LightWorld,
    DesertLedge                          => LightWorld,
    LakeHyliaIsland                      => LightWorld,
    SunkenTreasure                       => LightWorld,
    ZoraSLedge                           => LightWorld,
    FluteSpot                            => LightWorld,
    WaterfallFairyLeft                   => LightWorld,
    WaterfallFairyRight                  => LightWorld,
  }
}



type Closure = Fn(&HashMultiSet<Location2>) -> bool + Sync;

pub struct Location2 {
  name: &'static str,
  can_access_callback: &'static Closure,
}

use std::fmt;
impl fmt::Debug for Location2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Location2 {
  pub fn can_access(&self, items: &HashMultiSet<Location2>) -> bool {
    (self.can_access_callback)(&items)
  }
}

pub static DesertPalaceBigChest : Location2 = Location2 {
  name: "DesertPalaceBigChest",
  can_access_callback: &|_items: &HashMultiSet<Location2>| -> bool {
    true
  },
};
