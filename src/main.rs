extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;

use std::collections::HashMap;

mod items {
  #[allow(dead_code)]
  #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
  pub enum Item {
    L1Sword,
    MasterSword,
    ProgressiveSword,
    Bottle,
    BottleWithRedPotion,
    BottleWithGreenPotion,
    BottleWithBluePotion,
    BottleWithBee,
    BottleWithGoldBee,
    BottleWithFairy,
    Bombos,
    BookOfMudora,
    Bow,
    CaneOfSomaria,
    Cape,
    Ether,
    FireRod,
    Flippers,
    Hammer,
    Hookshot,
    IceRod,
    Lamp,
    MagicMirror,
    MoonPearl,
    Mushroom,
    OcarinaInactive,
    OcarinaActive,
    PegasusBoots,
    Powder,
    PowerGlove,
    Quake,
    Shovel,
    TitansMitt,
    BowAndSilverArrows,
    SilverArrowUpgrade,
    ProgressiveGlove,
    TriforcePiece,
    PowerStar,
    BugCatchingNet,
    MirrorShield,
    ProgressiveShield,
    CaneOfByrna,
    HalfMagic,
    QuarterMagic,
    L3Sword,
    L4Sword,
    HeartContainer,
    BossHeartContainer,
    BlueShield,
    ProgressiveArmor,
    BlueMail,
    Boomerang,
    RedBoomerang,
    RedShield,
    RedMail,
    BlueClock,
    RedClock,
    GreenClock,
    PieceOfHeart,
    BombUpgrade5,
    BombUpgrade10,
    BombUpgrade50,
    ArrowUpgrade5,
    ArrowUpgrade10,
    ArrowUpgrade70,
    Arrow,
    TenArrows,
    Bomb,
    ThreeBombs,
    OneRupee,
    FiveRupees,
    TwentyRupees,
    FiftyRupees,
    OneHundredRupees,
    ThreeHundredRupees,
    Heart,
    Rupoor,
    BigKeyA2,
    BigKeyD1,
    BigKeyD2,
    BigKeyD3,
    BigKeyD4,
    BigKeyD5,
    BigKeyD6,
    BigKeyD7,
    BigKeyP1,
    BigKeyP2,
    BigKeyP3,
    KeyA2,
    KeyD1,
    KeyD2,
    KeyD3,
    KeyD4,
    KeyD5,
    KeyD6,
    KeyD7,
    KeyA1,
    KeyH2,
    KeyP2,
    KeyP3,
    MapA2,
    MapD1,
    MapD2,
    MapD3,
    MapD4,
    MapD5,
    MapD6,
    MapD7,
    MapH2,
    MapP1,
    MapP2,
    MapP3,
    CompassA2,
    CompassD1,
    CompassD2,
    CompassD3,
    CompassD4,
    CompassD5,
    CompassD6,
    CompassD7,
    CompassP1,
    CompassP2,
    CompassP3,
    Crystal1,
    Crystal2,
    Crystal3,
    Crystal4,
    Crystal5,
    Crystal6,
    Crystal7,
    PendantOfCourage,
    PendantOfPower,
    PendantOfWisdom,
  }

  pub fn get_advancement_items() -> Vec<Item> {
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
}

mod medallions {
  #[allow(dead_code)]
  #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
  pub enum Medallion {
    Bombos,
    Ether,
    Quake,
  }

  pub fn get_all_medallions() -> Vec<Medallion> {
    vec![
      Medallion::Bombos,
      Medallion::Ether,
      Medallion::Quake,
    ]
  }

  #[allow(dead_code)]
  #[derive(Eq, PartialEq, Copy, Clone, Debug)]
  pub struct Entrance {
    pub turtle_rock: Medallion,
    pub misery_mire: Medallion,
  }
}

mod locations {
  use super::{medallions, regions, items};

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
    MireShedLeft,
    MireShedRight,
    Catfish,
    Pyramid,
    PyramidFairySword,
    PyramidFairyBow,
    PyramidFairyLeft,
    PyramidFairyRight,
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
    Mushroom,
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
    vec![
      Location::DesertPalaceBigChest,
      Location::DesertPalaceMapChest,
      Location::DesertPalaceTorch,
      Location::DesertPalaceBigKeyChest,
      Location::DesertPalaceCompassChest,
      Location::DesertPalaceLanmolas,
      Location::DesertPalacePrize,
      Location::EasternPalaceCompassChest,
      Location::EasternPalaceBigChest,
      Location::EasternPalaceCannonballChest,
      Location::EasternPalaceBigKeyChest,
      Location::EasternPalaceMapChest,
      Location::EasternPalaceArmosKnights,
      Location::EasternPalacePrize,
      Location::TowerOfHeraBigKeyChest,
      Location::TowerOfHeraBasementCage,
      Location::TowerOfHeraMapChest,
      Location::TowerOfHeraCompassChest,
      Location::TowerOfHeraBigChest,
      Location::TowerOfHeraMoldorm,
      Location::TowerOfHeraPrize,
      Location::IcePalaceBigKeyChest,
      Location::IcePalaceCompassChest,
      Location::IcePalaceMapChest,
      Location::IcePalaceSpikeRoom,
      Location::IcePalaceFreezorChest,
      Location::IcePalaceIcedTRoom,
      Location::IcePalaceBigChest,
      Location::IcePalaceKholdstare,
      Location::IcePalacePrize,
      Location::MiseryMireBigChest,
      Location::MiseryMireMainLobby,
      Location::MiseryMireBigKeyChest,
      Location::MiseryMireCompassChest,
      Location::MiseryMireBridgeChest,
      Location::MiseryMireMapChest,
      Location::MiseryMireSpikeChest,
      Location::MiseryMireVitreous,
      Location::MiseryMirePrize,
      Location::PalaceOfDarknessBigKeyChest,
      Location::PalaceOfDarknessTheArenaLedge,
      Location::PalaceOfDarknessTheArenaBridge,
      Location::PalaceOfDarknessBigChest,
      Location::PalaceOfDarknessCompassChest,
      Location::PalaceOfDarknessHarmlessHellway,
      Location::PalaceOfDarknessStalfosBasement,
      Location::PalaceOfDarknessDarkBasementLeft,
      Location::PalaceOfDarknessDarkBasementRight,
      Location::PalaceOfDarknessMapChest,
      Location::PalaceOfDarknessDarkMazeTop,
      Location::PalaceOfDarknessDarkMazeBottom,
      Location::PalaceOfDarknessShooterRoom,
      Location::PalaceOfDarknessHelmasaurKing,
      Location::PalaceOfDarknessPrize,
      Location::SkullWoodsBigChest,
      Location::SkullWoodsBigKeyChest,
      Location::SkullWoodsCompassChest,
      Location::SkullWoodsMapChest,
      Location::SkullWoodsBridgeRoom,
      Location::SkullWoodsPotPrison,
      Location::SkullWoodsPinballRoom,
      Location::SkullWoodsMothula,
      Location::SkullWoodsPrize,
      Location::SwampPalaceEntrance,
      Location::SwampPalaceBigChest,
      Location::SwampPalaceBigKeyChest,
      Location::SwampPalaceMapChest,
      Location::SwampPalaceWestChest,
      Location::SwampPalaceCompassChest,
      Location::SwampPalaceFloodedRoomLeft,
      Location::SwampPalaceFloodedRoomRight,
      Location::SwampPalaceWaterfallRoom,
      Location::SwampPalaceArrghus,
      Location::SwampPalacePrize,
      Location::ThievesTownAttic,
      Location::ThievesTownBigKeyChest,
      Location::ThievesTownMapChest,
      Location::ThievesTownCompassChest,
      Location::ThievesTownAmbushChest,
      Location::ThievesTownBigChest,
      Location::ThievesTownBlindSCell,
      Location::ThievesTownBlind,
      Location::ThievesTownPrize,
      Location::TurtleRockChainChomps,
      Location::TurtleRockCompassChest,
      Location::TurtleRockRollerRoomLeft,
      Location::TurtleRockRollerRoomRight,
      Location::TurtleRockBigChest,
      Location::TurtleRockBigKeyChest,
      Location::TurtleRockCrystarollerRoom,
      Location::TurtleRockEyeBridgeBottomLeft,
      Location::TurtleRockEyeBridgeBottomRight,
      Location::TurtleRockEyeBridgeTopLeft,
      Location::TurtleRockEyeBridgeTopRight,
      Location::TurtleRockTrinexx,
      Location::TurtleRockPrize,
      Location::GanonSTowerBobSTorch,
      Location::GanonSTowerDMsRoomTopLeft,
      Location::GanonSTowerDMsRoomTopRight,
      Location::GanonSTowerDMsRoomBottomLeft,
      Location::GanonSTowerDMsRoomBottomRight,
      Location::GanonSTowerRandomizerRoomTopLeft,
      Location::GanonSTowerRandomizerRoomTopRight,
      Location::GanonSTowerRandomizerRoomBottomLeft,
      Location::GanonSTowerRandomizerRoomBottomRight,
      Location::GanonSTowerFiresnakeRoom,
      Location::GanonSTowerMapChest,
      Location::GanonSTowerBigChest,
      Location::GanonSTowerHopeRoomLeft,
      Location::GanonSTowerHopeRoomRight,
      Location::GanonSTowerBobSChest,
      Location::GanonSTowerTileRoom,
      Location::GanonSTowerCompassRoomTopLeft,
      Location::GanonSTowerCompassRoomTopRight,
      Location::GanonSTowerCompassRoomBottomLeft,
      Location::GanonSTowerCompassRoomBottomRight,
      Location::GanonSTowerBigKeyChest,
      Location::GanonSTowerBigKeyRoomLeft,
      Location::GanonSTowerBigKeyRoomRight,
      Location::GanonSTowerMiniHelmasaurRoomLeft,
      Location::GanonSTowerMiniHelmasaurRoomRight,
      Location::GanonSTowerPreMoldormChest,
      Location::GanonSTowerMoldormChest,
      Location::WaterfallBottle,
      Location::PyramidBottle,
      Location::Sanctuary,
      Location::SewersSecretRoomLeft,
      Location::SewersSecretRoomMiddle,
      Location::SewersSecretRoomRight,
      Location::SewersDarkCross,
      Location::HyruleCastleBoomerangChest,
      Location::HyruleCastleMapChest,
      Location::HyruleCastleZeldaSCell,
      Location::CastleTowerRoom03,
      Location::CastleTowerDarkMaze,
      Location::MireShedLeft,
      Location::MireShedRight,
      Location::Catfish,
      Location::Pyramid,
      Location::PyramidFairySword,
      Location::PyramidFairyBow,
      Location::PyramidFairyLeft,
      Location::PyramidFairyRight,
      Location::Brewery,
      Location::CShapedHouse,
      Location::ChestGame,
      Location::HammerPegs,
      Location::BumperCave,
      Location::Blacksmith,
      Location::PurpleChest,
      Location::HypeCaveTop,
      Location::HypeCaveMiddleRight,
      Location::HypeCaveMiddleLeft,
      Location::HypeCaveBottom,
      Location::Stumpy,
      Location::HypeCaveNPC,
      Location::DiggingGame,
      Location::SuperbunnyCaveTop,
      Location::SuperbunnyCaveBottom,
      Location::HookshotCaveTopRight,
      Location::HookshotCaveTopLeft,
      Location::HookshotCaveBottomLeft,
      Location::HookshotCaveBottomRight,
      Location::SpikeCave,
      Location::SpiralCave,
      Location::MimicCave,
      Location::ParadoxCaveLowerFarLeft,
      Location::ParadoxCaveLowerLeft,
      Location::ParadoxCaveLowerRight,
      Location::ParadoxCaveLowerFarRight,
      Location::ParadoxCaveLowerMiddle,
      Location::ParadoxCaveUpperLeft,
      Location::ParadoxCaveUpperRight,
      Location::FloatingIsland,
      Location::OldMan,
      Location::SpectacleRockCave,
      Location::EtherTablet,
      Location::SpectacleRock,
      Location::MasterSwordPedestal,
      Location::LinkSUncle,
      Location::SecretPassage,
      Location::KingSTomb,
      Location::FloodgateChest,
      Location::LinkSHouse,
      Location::KakarikoTavern,
      Location::ChickenHouse,
      Location::AginahSCave,
      Location::SahasrahlaSHutLeft,
      Location::SahasrahlaSHutMiddle,
      Location::SahasrahlaSHutRight,
      Location::KakrikoWellTop,
      Location::KakrikoWellLeft,
      Location::KakrikoWellMiddle,
      Location::KakrikoWellRight,
      Location::KakrikoWellBottom,
      Location::BlindSHideoutTop,
      Location::BlindSHideoutLeft,
      Location::BlindSHideoutRight,
      Location::BlindSHideoutFarLeft,
      Location::BlindSHideoutFarRight,
      Location::PegasusRocks,
      Location::MiniMoldormCaveFarLeft,
      Location::MiniMoldormCaveLeft,
      Location::MiniMoldormCaveRight,
      Location::MiniMoldormCaveFarRight,
      Location::IceRodCave,
      Location::BottleMerchant,
      Location::Sahasrahla,
      Location::MagicBat,
      Location::SickKid,
      Location::Hobo,
      Location::BombosTablet,
      Location::KingZora,
      Location::LostWoodsHideout,
      Location::LumberjackTree,
      Location::Cave45,
      Location::GraveyardLedge,
      Location::CheckerboardCave,
      Location::MiniMoldormCaveNPC,
      Location::Library,
      Location::Mushroom,
      Location::PotionShop,
      Location::MazeRace,
      Location::DesertLedge,
      Location::LakeHyliaIsland,
      Location::SunkenTreasure,
      Location::ZoraSLedge,
      Location::FluteSpot,
      Location::WaterfallFairyLeft,
      Location::WaterfallFairyRight,
    ]
  }

  pub fn get_region_for(loc: Location) -> regions::Region {
    match loc {
      Location::DesertPalaceBigChest                 => regions::Region::DesertPalace,
      Location::DesertPalaceMapChest                 => regions::Region::DesertPalace,
      Location::DesertPalaceTorch                    => regions::Region::DesertPalace,
      Location::DesertPalaceBigKeyChest              => regions::Region::DesertPalace,
      Location::DesertPalaceCompassChest             => regions::Region::DesertPalace,
      Location::DesertPalaceLanmolas                 => regions::Region::DesertPalace,
      Location::DesertPalacePrize                    => regions::Region::DesertPalace,

      Location::EasternPalaceCompassChest            => regions::Region::EasternPalace,
      Location::EasternPalaceBigChest                => regions::Region::EasternPalace,
      Location::EasternPalaceCannonballChest         => regions::Region::EasternPalace,
      Location::EasternPalaceBigKeyChest             => regions::Region::EasternPalace,
      Location::EasternPalaceMapChest                => regions::Region::EasternPalace,
      Location::EasternPalaceArmosKnights            => regions::Region::EasternPalace,
      Location::EasternPalacePrize                   => regions::Region::EasternPalace,

      Location::TowerOfHeraBigKeyChest               => regions::Region::TowerofHera,
      Location::TowerOfHeraBasementCage              => regions::Region::TowerofHera,
      Location::TowerOfHeraMapChest                  => regions::Region::TowerofHera,
      Location::TowerOfHeraCompassChest              => regions::Region::TowerofHera,
      Location::TowerOfHeraBigChest                  => regions::Region::TowerofHera,
      Location::TowerOfHeraMoldorm                   => regions::Region::TowerofHera,
      Location::TowerOfHeraPrize                     => regions::Region::TowerofHera,

      Location::IcePalaceBigKeyChest                 => regions::Region::IcePalace,
      Location::IcePalaceCompassChest                => regions::Region::IcePalace,
      Location::IcePalaceMapChest                    => regions::Region::IcePalace,
      Location::IcePalaceSpikeRoom                   => regions::Region::IcePalace,
      Location::IcePalaceFreezorChest                => regions::Region::IcePalace,
      Location::IcePalaceIcedTRoom                   => regions::Region::IcePalace,
      Location::IcePalaceBigChest                    => regions::Region::IcePalace,
      Location::IcePalaceKholdstare                  => regions::Region::IcePalace,
      Location::IcePalacePrize                       => regions::Region::IcePalace,

      Location::MiseryMireBigChest                   => regions::Region::MiseryMire,
      Location::MiseryMireMainLobby                  => regions::Region::MiseryMire,
      Location::MiseryMireBigKeyChest                => regions::Region::MiseryMire,
      Location::MiseryMireCompassChest               => regions::Region::MiseryMire,
      Location::MiseryMireBridgeChest                => regions::Region::MiseryMire,
      Location::MiseryMireMapChest                   => regions::Region::MiseryMire,
      Location::MiseryMireSpikeChest                 => regions::Region::MiseryMire,
      Location::MiseryMireVitreous                   => regions::Region::MiseryMire,
      Location::MiseryMirePrize                      => regions::Region::MiseryMire,

      Location::PalaceOfDarknessBigKeyChest          => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessTheArenaLedge        => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessTheArenaBridge       => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessBigChest             => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessCompassChest         => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessHarmlessHellway      => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessStalfosBasement      => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessDarkBasementLeft     => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessDarkBasementRight    => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessMapChest             => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessDarkMazeTop          => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessDarkMazeBottom       => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessShooterRoom          => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessHelmasaurKing        => regions::Region::PalaceofDarkness,
      Location::PalaceOfDarknessPrize                => regions::Region::PalaceofDarkness,

      Location::SkullWoodsBigChest                   => regions::Region::SkullWoods,
      Location::SkullWoodsBigKeyChest                => regions::Region::SkullWoods,
      Location::SkullWoodsCompassChest               => regions::Region::SkullWoods,
      Location::SkullWoodsMapChest                   => regions::Region::SkullWoods,
      Location::SkullWoodsBridgeRoom                 => regions::Region::SkullWoods,
      Location::SkullWoodsPotPrison                  => regions::Region::SkullWoods,
      Location::SkullWoodsPinballRoom                => regions::Region::SkullWoods,
      Location::SkullWoodsMothula                    => regions::Region::SkullWoods,
      Location::SkullWoodsPrize                      => regions::Region::SkullWoods,

      Location::SwampPalaceEntrance                  => regions::Region::SwampPalace,
      Location::SwampPalaceBigChest                  => regions::Region::SwampPalace,
      Location::SwampPalaceBigKeyChest               => regions::Region::SwampPalace,
      Location::SwampPalaceMapChest                  => regions::Region::SwampPalace,
      Location::SwampPalaceWestChest                 => regions::Region::SwampPalace,
      Location::SwampPalaceCompassChest              => regions::Region::SwampPalace,
      Location::SwampPalaceFloodedRoomLeft           => regions::Region::SwampPalace,
      Location::SwampPalaceFloodedRoomRight          => regions::Region::SwampPalace,
      Location::SwampPalaceWaterfallRoom             => regions::Region::SwampPalace,
      Location::SwampPalaceArrghus                   => regions::Region::SwampPalace,
      Location::SwampPalacePrize                     => regions::Region::SwampPalace,

      Location::ThievesTownAttic                     => regions::Region::ThievesTown,
      Location::ThievesTownBigKeyChest               => regions::Region::ThievesTown,
      Location::ThievesTownMapChest                  => regions::Region::ThievesTown,
      Location::ThievesTownCompassChest              => regions::Region::ThievesTown,
      Location::ThievesTownAmbushChest               => regions::Region::ThievesTown,
      Location::ThievesTownBigChest                  => regions::Region::ThievesTown,
      Location::ThievesTownBlindSCell                => regions::Region::ThievesTown,
      Location::ThievesTownBlind                     => regions::Region::ThievesTown,
      Location::ThievesTownPrize                     => regions::Region::ThievesTown,

      Location::TurtleRockChainChomps                => regions::Region::TurtleRock,
      Location::TurtleRockCompassChest               => regions::Region::TurtleRock,
      Location::TurtleRockRollerRoomLeft             => regions::Region::TurtleRock,
      Location::TurtleRockRollerRoomRight            => regions::Region::TurtleRock,
      Location::TurtleRockBigChest                   => regions::Region::TurtleRock,
      Location::TurtleRockBigKeyChest                => regions::Region::TurtleRock,
      Location::TurtleRockCrystarollerRoom           => regions::Region::TurtleRock,
      Location::TurtleRockEyeBridgeBottomLeft        => regions::Region::TurtleRock,
      Location::TurtleRockEyeBridgeBottomRight       => regions::Region::TurtleRock,
      Location::TurtleRockEyeBridgeTopLeft           => regions::Region::TurtleRock,
      Location::TurtleRockEyeBridgeTopRight          => regions::Region::TurtleRock,
      Location::TurtleRockTrinexx                    => regions::Region::TurtleRock,
      Location::TurtleRockPrize                      => regions::Region::TurtleRock,

      Location::GanonSTowerBobSTorch                 => regions::Region::GanonsTower,
      Location::GanonSTowerDMsRoomTopLeft            => regions::Region::GanonsTower,
      Location::GanonSTowerDMsRoomTopRight           => regions::Region::GanonsTower,
      Location::GanonSTowerDMsRoomBottomLeft         => regions::Region::GanonsTower,
      Location::GanonSTowerDMsRoomBottomRight        => regions::Region::GanonsTower,
      Location::GanonSTowerRandomizerRoomTopLeft     => regions::Region::GanonsTower,
      Location::GanonSTowerRandomizerRoomTopRight    => regions::Region::GanonsTower,
      Location::GanonSTowerRandomizerRoomBottomLeft  => regions::Region::GanonsTower,
      Location::GanonSTowerRandomizerRoomBottomRight => regions::Region::GanonsTower,
      Location::GanonSTowerFiresnakeRoom             => regions::Region::GanonsTower,
      Location::GanonSTowerMapChest                  => regions::Region::GanonsTower,
      Location::GanonSTowerBigChest                  => regions::Region::GanonsTower,
      Location::GanonSTowerHopeRoomLeft              => regions::Region::GanonsTower,
      Location::GanonSTowerHopeRoomRight             => regions::Region::GanonsTower,
      Location::GanonSTowerBobSChest                 => regions::Region::GanonsTower,
      Location::GanonSTowerTileRoom                  => regions::Region::GanonsTower,
      Location::GanonSTowerCompassRoomTopLeft        => regions::Region::GanonsTower,
      Location::GanonSTowerCompassRoomTopRight       => regions::Region::GanonsTower,
      Location::GanonSTowerCompassRoomBottomLeft     => regions::Region::GanonsTower,
      Location::GanonSTowerCompassRoomBottomRight    => regions::Region::GanonsTower,
      Location::GanonSTowerBigKeyChest               => regions::Region::GanonsTower,
      Location::GanonSTowerBigKeyRoomLeft            => regions::Region::GanonsTower,
      Location::GanonSTowerBigKeyRoomRight           => regions::Region::GanonsTower,
      Location::GanonSTowerMiniHelmasaurRoomLeft     => regions::Region::GanonsTower,
      Location::GanonSTowerMiniHelmasaurRoomRight    => regions::Region::GanonsTower,
      Location::GanonSTowerPreMoldormChest           => regions::Region::GanonsTower,
      Location::GanonSTowerMoldormChest              => regions::Region::GanonsTower,

      Location::WaterfallBottle                      => regions::Region::Fountains,
      Location::PyramidBottle                        => regions::Region::Fountains,

      Location::Sanctuary                            => regions::Region::Escape,
      Location::SewersSecretRoomLeft                 => regions::Region::Escape,
      Location::SewersSecretRoomMiddle               => regions::Region::Escape,
      Location::SewersSecretRoomRight                => regions::Region::Escape,
      Location::SewersDarkCross                      => regions::Region::Escape,
      Location::HyruleCastleBoomerangChest           => regions::Region::Escape,
      Location::HyruleCastleMapChest                 => regions::Region::Escape,
      Location::HyruleCastleZeldaSCell               => regions::Region::Escape,

      Location::CastleTowerRoom03                    => regions::Region::HyruleCastleTower,
      Location::CastleTowerDarkMaze                  => regions::Region::HyruleCastleTower,

      Location::MireShedLeft                         => regions::Region::Mire,
      Location::MireShedRight                        => regions::Region::Mire,

      Location::Catfish                              => regions::Region::NorthEastDarkWorld,
      Location::Pyramid                              => regions::Region::NorthEastDarkWorld,
      Location::PyramidFairySword                    => regions::Region::NorthEastDarkWorld,
      Location::PyramidFairyBow                      => regions::Region::NorthEastDarkWorld,
      Location::PyramidFairyLeft                     => regions::Region::NorthEastDarkWorld,
      Location::PyramidFairyRight                    => regions::Region::NorthEastDarkWorld,

      Location::Brewery                              => regions::Region::NorthWestDarkWorld,
      Location::CShapedHouse                         => regions::Region::NorthWestDarkWorld,
      Location::ChestGame                            => regions::Region::NorthWestDarkWorld,
      Location::HammerPegs                           => regions::Region::NorthWestDarkWorld,
      Location::BumperCave                           => regions::Region::NorthWestDarkWorld,
      Location::Blacksmith                           => regions::Region::NorthWestDarkWorld,
      Location::PurpleChest                          => regions::Region::NorthWestDarkWorld,

      Location::HypeCaveTop                          => regions::Region::SouthDarkWorld,
      Location::HypeCaveMiddleRight                  => regions::Region::SouthDarkWorld,
      Location::HypeCaveMiddleLeft                   => regions::Region::SouthDarkWorld,
      Location::HypeCaveBottom                       => regions::Region::SouthDarkWorld,
      Location::Stumpy                               => regions::Region::SouthDarkWorld,
      Location::HypeCaveNPC                          => regions::Region::SouthDarkWorld,
      Location::DiggingGame                          => regions::Region::SouthDarkWorld,

      Location::SuperbunnyCaveTop                    => regions::Region::EastDarkWorldDeathMountain,
      Location::SuperbunnyCaveBottom                 => regions::Region::EastDarkWorldDeathMountain,
      Location::HookshotCaveTopRight                 => regions::Region::EastDarkWorldDeathMountain,
      Location::HookshotCaveTopLeft                  => regions::Region::EastDarkWorldDeathMountain,
      Location::HookshotCaveBottomLeft               => regions::Region::EastDarkWorldDeathMountain,
      Location::HookshotCaveBottomRight              => regions::Region::EastDarkWorldDeathMountain,

      Location::SpikeCave                            => regions::Region::WestDarkWorldDeathMountain,

      Location::SpiralCave                           => regions::Region::EastDeathMountain,
      Location::MimicCave                            => regions::Region::EastDeathMountain,
      Location::ParadoxCaveLowerFarLeft              => regions::Region::EastDeathMountain,
      Location::ParadoxCaveLowerLeft                 => regions::Region::EastDeathMountain,
      Location::ParadoxCaveLowerRight                => regions::Region::EastDeathMountain,
      Location::ParadoxCaveLowerFarRight             => regions::Region::EastDeathMountain,
      Location::ParadoxCaveLowerMiddle               => regions::Region::EastDeathMountain,
      Location::ParadoxCaveUpperLeft                 => regions::Region::EastDeathMountain,
      Location::ParadoxCaveUpperRight                => regions::Region::EastDeathMountain,
      Location::FloatingIsland                       => regions::Region::EastDeathMountain,

      Location::OldMan                               => regions::Region::WestDeathMountain,
      Location::SpectacleRockCave                    => regions::Region::WestDeathMountain,
      Location::EtherTablet                          => regions::Region::WestDeathMountain,
      Location::SpectacleRock                        => regions::Region::WestDeathMountain,

      Location::MasterSwordPedestal                  => regions::Region::LightWorld,
      Location::LinkSUncle                           => regions::Region::LightWorld,
      Location::SecretPassage                        => regions::Region::LightWorld,
      Location::KingSTomb                            => regions::Region::LightWorld,
      Location::FloodgateChest                       => regions::Region::LightWorld,
      Location::LinkSHouse                           => regions::Region::LightWorld,
      Location::KakarikoTavern                       => regions::Region::LightWorld,
      Location::ChickenHouse                         => regions::Region::LightWorld,
      Location::AginahSCave                          => regions::Region::LightWorld,
      Location::SahasrahlaSHutLeft                   => regions::Region::LightWorld,
      Location::SahasrahlaSHutMiddle                 => regions::Region::LightWorld,
      Location::SahasrahlaSHutRight                  => regions::Region::LightWorld,
      Location::KakrikoWellTop                       => regions::Region::LightWorld,
      Location::KakrikoWellLeft                      => regions::Region::LightWorld,
      Location::KakrikoWellMiddle                    => regions::Region::LightWorld,
      Location::KakrikoWellRight                     => regions::Region::LightWorld,
      Location::KakrikoWellBottom                    => regions::Region::LightWorld,
      Location::BlindSHideoutTop                     => regions::Region::LightWorld,
      Location::BlindSHideoutLeft                    => regions::Region::LightWorld,
      Location::BlindSHideoutRight                   => regions::Region::LightWorld,
      Location::BlindSHideoutFarLeft                 => regions::Region::LightWorld,
      Location::BlindSHideoutFarRight                => regions::Region::LightWorld,
      Location::PegasusRocks                         => regions::Region::LightWorld,
      Location::MiniMoldormCaveFarLeft               => regions::Region::LightWorld,
      Location::MiniMoldormCaveLeft                  => regions::Region::LightWorld,
      Location::MiniMoldormCaveRight                 => regions::Region::LightWorld,
      Location::MiniMoldormCaveFarRight              => regions::Region::LightWorld,
      Location::IceRodCave                           => regions::Region::LightWorld,
      Location::BottleMerchant                       => regions::Region::LightWorld,
      Location::Sahasrahla                           => regions::Region::LightWorld,
      Location::MagicBat                             => regions::Region::LightWorld,
      Location::SickKid                              => regions::Region::LightWorld,
      Location::Hobo                                 => regions::Region::LightWorld,
      Location::BombosTablet                         => regions::Region::LightWorld,
      Location::KingZora                             => regions::Region::LightWorld,
      Location::LostWoodsHideout                     => regions::Region::LightWorld,
      Location::LumberjackTree                       => regions::Region::LightWorld,
      Location::Cave45                               => regions::Region::LightWorld,
      Location::GraveyardLedge                       => regions::Region::LightWorld,
      Location::CheckerboardCave                     => regions::Region::LightWorld,
      Location::MiniMoldormCaveNPC                   => regions::Region::LightWorld,
      Location::Library                              => regions::Region::LightWorld,
      Location::Mushroom                             => regions::Region::LightWorld,
      Location::PotionShop                           => regions::Region::LightWorld,
      Location::MazeRace                             => regions::Region::LightWorld,
      Location::DesertLedge                          => regions::Region::LightWorld,
      Location::LakeHyliaIsland                      => regions::Region::LightWorld,
      Location::SunkenTreasure                       => regions::Region::LightWorld,
      Location::ZoraSLedge                           => regions::Region::LightWorld,
      Location::FluteSpot                            => regions::Region::LightWorld,
      Location::WaterfallFairyLeft                   => regions::Region::LightWorld,
      Location::WaterfallFairyRight                  => regions::Region::LightWorld,
    }
  }
}

mod regions {
  use super::{medallions, locations, items};

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
    match reg {
      Region::DesertPalace => vec![
        locations::Location::DesertPalaceBigChest,
        locations::Location::DesertPalaceMapChest,
        locations::Location::DesertPalaceTorch,
        locations::Location::DesertPalaceBigKeyChest,
        locations::Location::DesertPalaceCompassChest,
        locations::Location::DesertPalaceLanmolas,
        locations::Location::DesertPalacePrize,
      ],
      Region::EasternPalace => vec![
        locations::Location::EasternPalaceCompassChest,
        locations::Location::EasternPalaceBigChest,
        locations::Location::EasternPalaceCannonballChest,
        locations::Location::EasternPalaceBigKeyChest,
        locations::Location::EasternPalaceMapChest,
        locations::Location::EasternPalaceArmosKnights,
        locations::Location::EasternPalacePrize,
      ],
      Region::TowerofHera => vec![
        locations::Location::TowerOfHeraBigKeyChest,
        locations::Location::TowerOfHeraBasementCage,
        locations::Location::TowerOfHeraMapChest,
        locations::Location::TowerOfHeraCompassChest,
        locations::Location::TowerOfHeraBigChest,
        locations::Location::TowerOfHeraMoldorm,
        locations::Location::TowerOfHeraPrize,
      ],
      Region::IcePalace => vec![
        locations::Location::IcePalaceBigKeyChest,
        locations::Location::IcePalaceCompassChest,
        locations::Location::IcePalaceMapChest,
        locations::Location::IcePalaceSpikeRoom,
        locations::Location::IcePalaceFreezorChest,
        locations::Location::IcePalaceIcedTRoom,
        locations::Location::IcePalaceBigChest,
        locations::Location::IcePalaceKholdstare,
        locations::Location::IcePalacePrize,
      ],
      Region::MiseryMire => vec![
        locations::Location::MiseryMireBigChest,
        locations::Location::MiseryMireMainLobby,
        locations::Location::MiseryMireBigKeyChest,
        locations::Location::MiseryMireCompassChest,
        locations::Location::MiseryMireBridgeChest,
        locations::Location::MiseryMireMapChest,
        locations::Location::MiseryMireSpikeChest,
        locations::Location::MiseryMireVitreous,
        locations::Location::MiseryMirePrize,
      ],
      Region::PalaceofDarkness => vec![
        locations::Location::PalaceOfDarknessBigKeyChest,
        locations::Location::PalaceOfDarknessTheArenaLedge,
        locations::Location::PalaceOfDarknessTheArenaBridge,
        locations::Location::PalaceOfDarknessBigChest,
        locations::Location::PalaceOfDarknessCompassChest,
        locations::Location::PalaceOfDarknessHarmlessHellway,
        locations::Location::PalaceOfDarknessStalfosBasement,
        locations::Location::PalaceOfDarknessDarkBasementLeft,
        locations::Location::PalaceOfDarknessDarkBasementRight,
        locations::Location::PalaceOfDarknessMapChest,
        locations::Location::PalaceOfDarknessDarkMazeTop,
        locations::Location::PalaceOfDarknessDarkMazeBottom,
        locations::Location::PalaceOfDarknessShooterRoom,
        locations::Location::PalaceOfDarknessHelmasaurKing,
        locations::Location::PalaceOfDarknessPrize,
      ],
      Region::SkullWoods => vec![
        locations::Location::SkullWoodsBigChest,
        locations::Location::SkullWoodsBigKeyChest,
        locations::Location::SkullWoodsCompassChest,
        locations::Location::SkullWoodsMapChest,
        locations::Location::SkullWoodsBridgeRoom,
        locations::Location::SkullWoodsPotPrison,
        locations::Location::SkullWoodsPinballRoom,
        locations::Location::SkullWoodsMothula,
        locations::Location::SkullWoodsPrize,
      ],
      Region::SwampPalace => vec![
        locations::Location::SwampPalaceEntrance,
        locations::Location::SwampPalaceBigChest,
        locations::Location::SwampPalaceBigKeyChest,
        locations::Location::SwampPalaceMapChest,
        locations::Location::SwampPalaceWestChest,
        locations::Location::SwampPalaceCompassChest,
        locations::Location::SwampPalaceFloodedRoomLeft,
        locations::Location::SwampPalaceFloodedRoomRight,
        locations::Location::SwampPalaceWaterfallRoom,
        locations::Location::SwampPalaceArrghus,
        locations::Location::SwampPalacePrize,
      ],
      Region::ThievesTown => vec![
        locations::Location::ThievesTownAttic,
        locations::Location::ThievesTownBigKeyChest,
        locations::Location::ThievesTownMapChest,
        locations::Location::ThievesTownCompassChest,
        locations::Location::ThievesTownAmbushChest,
        locations::Location::ThievesTownBigChest,
        locations::Location::ThievesTownBlindSCell,
        locations::Location::ThievesTownBlind,
        locations::Location::ThievesTownPrize,
      ],
      Region::TurtleRock => vec![
        locations::Location::TurtleRockChainChomps,
        locations::Location::TurtleRockCompassChest,
        locations::Location::TurtleRockRollerRoomLeft,
        locations::Location::TurtleRockRollerRoomRight,
        locations::Location::TurtleRockBigChest,
        locations::Location::TurtleRockBigKeyChest,
        locations::Location::TurtleRockCrystarollerRoom,
        locations::Location::TurtleRockEyeBridgeBottomLeft,
        locations::Location::TurtleRockEyeBridgeBottomRight,
        locations::Location::TurtleRockEyeBridgeTopLeft,
        locations::Location::TurtleRockEyeBridgeTopRight,
        locations::Location::TurtleRockTrinexx,
        locations::Location::TurtleRockPrize,
      ],
      Region::GanonsTower => vec![
        locations::Location::GanonSTowerBobSTorch,
        locations::Location::GanonSTowerDMsRoomTopLeft,
        locations::Location::GanonSTowerDMsRoomTopRight,
        locations::Location::GanonSTowerDMsRoomBottomLeft,
        locations::Location::GanonSTowerDMsRoomBottomRight,
        locations::Location::GanonSTowerRandomizerRoomTopLeft,
        locations::Location::GanonSTowerRandomizerRoomTopRight,
        locations::Location::GanonSTowerRandomizerRoomBottomLeft,
        locations::Location::GanonSTowerRandomizerRoomBottomRight,
        locations::Location::GanonSTowerFiresnakeRoom,
        locations::Location::GanonSTowerMapChest,
        locations::Location::GanonSTowerBigChest,
        locations::Location::GanonSTowerHopeRoomLeft,
        locations::Location::GanonSTowerHopeRoomRight,
        locations::Location::GanonSTowerBobSChest,
        locations::Location::GanonSTowerTileRoom,
        locations::Location::GanonSTowerCompassRoomTopLeft,
        locations::Location::GanonSTowerCompassRoomTopRight,
        locations::Location::GanonSTowerCompassRoomBottomLeft,
        locations::Location::GanonSTowerCompassRoomBottomRight,
        locations::Location::GanonSTowerBigKeyChest,
        locations::Location::GanonSTowerBigKeyRoomLeft,
        locations::Location::GanonSTowerBigKeyRoomRight,
        locations::Location::GanonSTowerMiniHelmasaurRoomLeft,
        locations::Location::GanonSTowerMiniHelmasaurRoomRight,
        locations::Location::GanonSTowerPreMoldormChest,
        locations::Location::GanonSTowerMoldormChest,
      ],
      Region::Fountains => vec![
        locations::Location::WaterfallBottle,
        locations::Location::PyramidBottle,
      ],
      Region::Escape => vec![
        locations::Location::Sanctuary,
        locations::Location::SewersSecretRoomLeft,
        locations::Location::SewersSecretRoomMiddle,
        locations::Location::SewersSecretRoomRight,
        locations::Location::SewersDarkCross,
        locations::Location::HyruleCastleBoomerangChest,
        locations::Location::HyruleCastleMapChest,
        locations::Location::HyruleCastleZeldaSCell,
      ],
      Region::HyruleCastleTower => vec![
        locations::Location::CastleTowerRoom03,
        locations::Location::CastleTowerDarkMaze,
      ],
      Region::Mire => vec![
        locations::Location::MireShedLeft,
        locations::Location::MireShedRight,
      ],
      Region::NorthEastDarkWorld => vec![
        locations::Location::Catfish,
        locations::Location::Pyramid,
        locations::Location::PyramidFairySword,
        locations::Location::PyramidFairyBow,
        locations::Location::PyramidFairyLeft ,
        locations::Location::PyramidFairyRight,
      ],
      Region::NorthWestDarkWorld => vec![
        locations::Location::Brewery,
        locations::Location::CShapedHouse,
        locations::Location::ChestGame,
        locations::Location::HammerPegs,
        locations::Location::BumperCave,
        locations::Location::Blacksmith,
        locations::Location::PurpleChest,
      ],
      Region::SouthDarkWorld => vec![
        locations::Location::HypeCaveTop,
        locations::Location::HypeCaveMiddleRight,
        locations::Location::HypeCaveMiddleLeft,
        locations::Location::HypeCaveBottom,
        locations::Location::Stumpy,
        locations::Location::HypeCaveNPC,
        locations::Location::DiggingGame,
      ],
      Region::EastDarkWorldDeathMountain => vec![
        locations::Location::SuperbunnyCaveTop,
        locations::Location::SuperbunnyCaveBottom,
        locations::Location::HookshotCaveTopRight,
        locations::Location::HookshotCaveTopLeft,
        locations::Location::HookshotCaveBottomLeft,
        locations::Location::HookshotCaveBottomRight,
      ],
      Region::WestDarkWorldDeathMountain => vec![
        locations::Location::SpikeCave,
      ],
      Region::EastDeathMountain => vec![
        locations::Location::SpiralCave,
        locations::Location::MimicCave,
        locations::Location::ParadoxCaveLowerFarLeft,
        locations::Location::ParadoxCaveLowerLeft,
        locations::Location::ParadoxCaveLowerRight,
        locations::Location::ParadoxCaveLowerFarRight,
        locations::Location::ParadoxCaveLowerMiddle,
        locations::Location::ParadoxCaveUpperLeft,
        locations::Location::ParadoxCaveUpperRight,
        locations::Location::FloatingIsland,
      ],
      Region::WestDeathMountain => vec![
        locations::Location::OldMan,
        locations::Location::SpectacleRockCave,
        locations::Location::EtherTablet,
        locations::Location::SpectacleRock,
      ],
      Region::LightWorld => vec![
        locations::Location::MasterSwordPedestal,
        locations::Location::LinkSUncle,
        locations::Location::SecretPassage,
        locations::Location::KingSTomb,
        locations::Location::FloodgateChest,
        locations::Location::LinkSHouse,
        locations::Location::KakarikoTavern,
        locations::Location::ChickenHouse,
        locations::Location::AginahSCave,
        locations::Location::SahasrahlaSHutLeft,
        locations::Location::SahasrahlaSHutMiddle,
        locations::Location::SahasrahlaSHutRight,
        locations::Location::KakrikoWellTop,
        locations::Location::KakrikoWellLeft,
        locations::Location::KakrikoWellMiddle,
        locations::Location::KakrikoWellRight,
        locations::Location::KakrikoWellBottom,
        locations::Location::BlindSHideoutTop,
        locations::Location::BlindSHideoutLeft,
        locations::Location::BlindSHideoutRight,
        locations::Location::BlindSHideoutFarLeft,
        locations::Location::BlindSHideoutFarRight,
        locations::Location::PegasusRocks,
        locations::Location::MiniMoldormCaveFarLeft,
        locations::Location::MiniMoldormCaveLeft,
        locations::Location::MiniMoldormCaveRight,
        locations::Location::MiniMoldormCaveFarRight,
        locations::Location::IceRodCave,
        locations::Location::BottleMerchant,
        locations::Location::Sahasrahla,
        locations::Location::MagicBat,
        locations::Location::SickKid,
        locations::Location::Hobo,
        locations::Location::BombosTablet,
        locations::Location::KingZora,
        locations::Location::LostWoodsHideout,
        locations::Location::LumberjackTree,
        locations::Location::Cave45,
        locations::Location::GraveyardLedge,
        locations::Location::CheckerboardCave,
        locations::Location::MiniMoldormCaveNPC,
        locations::Location::Library,
        locations::Location::Mushroom,
        locations::Location::PotionShop,
        locations::Location::MazeRace,
        locations::Location::DesertLedge,
        locations::Location::LakeHyliaIsland,
        locations::Location::SunkenTreasure,
        locations::Location::ZoraSLedge,
        locations::Location::FluteSpot,
        locations::Location::WaterfallFairyLeft,
        locations::Location::WaterfallFairyRight,
      ],
    }
  }
}

mod world {
  use std::collections::HashMap;
  use super::{medallions, locations, items};

  #[derive(Eq, PartialEq, Debug)]
  pub struct World {
    pub medallions: medallions::Entrance,
    pub assignments: HashMap<locations::Location, items::Item>,
  }
}

mod logic {
  use std::collections::HashMap;
  use super::{medallions, locations, regions, items};

  pub fn can_fill(
    item: items::Item,
    loc: locations::Location,
    assumed_items: &Vec<items::Item>,
    assignments: &HashMap<locations::Location, items::Item>,
  ) -> bool {
    match loc { // @TODO
      locations::Location::DesertPalaceBigChest => true,
      locations::Location::DesertPalaceMapChest => true,
      locations::Location::DesertPalaceTorch => true,
      locations::Location::DesertPalaceBigKeyChest => true,
      locations::Location::DesertPalaceCompassChest => true,
      locations::Location::DesertPalaceLanmolas => true,
      locations::Location::DesertPalacePrize => true,
      locations::Location::EasternPalaceCompassChest => true,
      locations::Location::EasternPalaceBigChest => true,
      locations::Location::EasternPalaceCannonballChest => true,
      locations::Location::EasternPalaceBigKeyChest => true,
      locations::Location::EasternPalaceMapChest => true,
      locations::Location::EasternPalaceArmosKnights => true,
      locations::Location::EasternPalacePrize => true,
      locations::Location::TowerOfHeraBigKeyChest => true,
      locations::Location::TowerOfHeraBasementCage => true,
      locations::Location::TowerOfHeraMapChest => true,
      locations::Location::TowerOfHeraCompassChest => true,
      locations::Location::TowerOfHeraBigChest => true,
      locations::Location::TowerOfHeraMoldorm => true,
      locations::Location::TowerOfHeraPrize => true,
      locations::Location::IcePalaceBigKeyChest => true,
      locations::Location::IcePalaceCompassChest => true,
      locations::Location::IcePalaceMapChest => true,
      locations::Location::IcePalaceSpikeRoom => true,
      locations::Location::IcePalaceFreezorChest => true,
      locations::Location::IcePalaceIcedTRoom => true,
      locations::Location::IcePalaceBigChest => true,
      locations::Location::IcePalaceKholdstare => true,
      locations::Location::IcePalacePrize => true,
      locations::Location::MiseryMireBigChest => true,
      locations::Location::MiseryMireMainLobby => true,
      locations::Location::MiseryMireBigKeyChest => true,
      locations::Location::MiseryMireCompassChest => true,
      locations::Location::MiseryMireBridgeChest => true,
      locations::Location::MiseryMireMapChest => true,
      locations::Location::MiseryMireSpikeChest => true,
      locations::Location::MiseryMireVitreous => true,
      locations::Location::MiseryMirePrize => true,
      locations::Location::PalaceOfDarknessBigKeyChest => true,
      locations::Location::PalaceOfDarknessTheArenaLedge => true,
      locations::Location::PalaceOfDarknessTheArenaBridge => true,
      locations::Location::PalaceOfDarknessBigChest => true,
      locations::Location::PalaceOfDarknessCompassChest => true,
      locations::Location::PalaceOfDarknessHarmlessHellway => true,
      locations::Location::PalaceOfDarknessStalfosBasement => true,
      locations::Location::PalaceOfDarknessDarkBasementLeft => true,
      locations::Location::PalaceOfDarknessDarkBasementRight => true,
      locations::Location::PalaceOfDarknessMapChest => true,
      locations::Location::PalaceOfDarknessDarkMazeTop => true,
      locations::Location::PalaceOfDarknessDarkMazeBottom => true,
      locations::Location::PalaceOfDarknessShooterRoom => true,
      locations::Location::PalaceOfDarknessHelmasaurKing => true,
      locations::Location::PalaceOfDarknessPrize => true,
      locations::Location::SkullWoodsBigChest => true,
      locations::Location::SkullWoodsBigKeyChest => true,
      locations::Location::SkullWoodsCompassChest => true,
      locations::Location::SkullWoodsMapChest => true,
      locations::Location::SkullWoodsBridgeRoom => true,
      locations::Location::SkullWoodsPotPrison => true,
      locations::Location::SkullWoodsPinballRoom => true,
      locations::Location::SkullWoodsMothula => true,
      locations::Location::SkullWoodsPrize => true,
      locations::Location::SwampPalaceEntrance => true,
      locations::Location::SwampPalaceBigChest => true,
      locations::Location::SwampPalaceBigKeyChest => true,
      locations::Location::SwampPalaceMapChest => true,
      locations::Location::SwampPalaceWestChest => true,
      locations::Location::SwampPalaceCompassChest => true,
      locations::Location::SwampPalaceFloodedRoomLeft => true,
      locations::Location::SwampPalaceFloodedRoomRight => true,
      locations::Location::SwampPalaceWaterfallRoom => true,
      locations::Location::SwampPalaceArrghus => true,
      locations::Location::SwampPalacePrize => true,
      locations::Location::ThievesTownAttic => true,
      locations::Location::ThievesTownBigKeyChest => true,
      locations::Location::ThievesTownMapChest => true,
      locations::Location::ThievesTownCompassChest => true,
      locations::Location::ThievesTownAmbushChest => true,
      locations::Location::ThievesTownBigChest => true,
      locations::Location::ThievesTownBlindSCell => true,
      locations::Location::ThievesTownBlind => true,
      locations::Location::ThievesTownPrize => true,
      locations::Location::TurtleRockChainChomps => true,
      locations::Location::TurtleRockCompassChest => true,
      locations::Location::TurtleRockRollerRoomLeft => true,
      locations::Location::TurtleRockRollerRoomRight => true,
      locations::Location::TurtleRockBigChest => true,
      locations::Location::TurtleRockBigKeyChest => true,
      locations::Location::TurtleRockCrystarollerRoom => true,
      locations::Location::TurtleRockEyeBridgeBottomLeft => true,
      locations::Location::TurtleRockEyeBridgeBottomRight => true,
      locations::Location::TurtleRockEyeBridgeTopLeft => true,
      locations::Location::TurtleRockEyeBridgeTopRight => true,
      locations::Location::TurtleRockTrinexx => true,
      locations::Location::TurtleRockPrize => true,
      locations::Location::GanonSTowerBobSTorch => true,
      locations::Location::GanonSTowerDMsRoomTopLeft => true,
      locations::Location::GanonSTowerDMsRoomTopRight => true,
      locations::Location::GanonSTowerDMsRoomBottomLeft => true,
      locations::Location::GanonSTowerDMsRoomBottomRight => true,
      locations::Location::GanonSTowerRandomizerRoomTopLeft => true,
      locations::Location::GanonSTowerRandomizerRoomTopRight => true,
      locations::Location::GanonSTowerRandomizerRoomBottomLeft => true,
      locations::Location::GanonSTowerRandomizerRoomBottomRight => true,
      locations::Location::GanonSTowerFiresnakeRoom => true,
      locations::Location::GanonSTowerMapChest => true,
      locations::Location::GanonSTowerBigChest => true,
      locations::Location::GanonSTowerHopeRoomLeft => true,
      locations::Location::GanonSTowerHopeRoomRight => true,
      locations::Location::GanonSTowerBobSChest => true,
      locations::Location::GanonSTowerTileRoom => true,
      locations::Location::GanonSTowerCompassRoomTopLeft => true,
      locations::Location::GanonSTowerCompassRoomTopRight => true,
      locations::Location::GanonSTowerCompassRoomBottomLeft => true,
      locations::Location::GanonSTowerCompassRoomBottomRight => true,
      locations::Location::GanonSTowerBigKeyChest => true,
      locations::Location::GanonSTowerBigKeyRoomLeft => true,
      locations::Location::GanonSTowerBigKeyRoomRight => true,
      locations::Location::GanonSTowerMiniHelmasaurRoomLeft => true,
      locations::Location::GanonSTowerMiniHelmasaurRoomRight => true,
      locations::Location::GanonSTowerPreMoldormChest => true,
      locations::Location::GanonSTowerMoldormChest => true,
      locations::Location::WaterfallBottle => true,
      locations::Location::PyramidBottle => true,
      locations::Location::Sanctuary => true,
      locations::Location::SewersSecretRoomLeft => true,
      locations::Location::SewersSecretRoomMiddle => true,
      locations::Location::SewersSecretRoomRight => true,
      locations::Location::SewersDarkCross => true,
      locations::Location::HyruleCastleBoomerangChest => true,
      locations::Location::HyruleCastleMapChest => true,
      locations::Location::HyruleCastleZeldaSCell => true,
      locations::Location::CastleTowerRoom03 => true,
      locations::Location::CastleTowerDarkMaze => true,
      locations::Location::MireShedLeft => true,
      locations::Location::MireShedRight => true,
      locations::Location::Catfish => true,
      locations::Location::Pyramid => true,
      locations::Location::PyramidFairySword => true,
      locations::Location::PyramidFairyBow => true,
      locations::Location::PyramidFairyLeft => true,
      locations::Location::PyramidFairyRight => true,
      locations::Location::Brewery => true,
      locations::Location::CShapedHouse => true,
      locations::Location::ChestGame => true,
      locations::Location::HammerPegs => true,
      locations::Location::BumperCave => true,
      locations::Location::Blacksmith => true,
      locations::Location::PurpleChest => true,
      locations::Location::HypeCaveTop => true,
      locations::Location::HypeCaveMiddleRight => true,
      locations::Location::HypeCaveMiddleLeft => true,
      locations::Location::HypeCaveBottom => true,
      locations::Location::Stumpy => true,
      locations::Location::HypeCaveNPC => true,
      locations::Location::DiggingGame => true,
      locations::Location::SuperbunnyCaveTop => true,
      locations::Location::SuperbunnyCaveBottom => true,
      locations::Location::HookshotCaveTopRight => true,
      locations::Location::HookshotCaveTopLeft => true,
      locations::Location::HookshotCaveBottomLeft => true,
      locations::Location::HookshotCaveBottomRight => true,
      locations::Location::SpikeCave => true,
      locations::Location::SpiralCave => true,
      locations::Location::MimicCave => true,
      locations::Location::ParadoxCaveLowerFarLeft => true,
      locations::Location::ParadoxCaveLowerLeft => true,
      locations::Location::ParadoxCaveLowerRight => true,
      locations::Location::ParadoxCaveLowerFarRight => true,
      locations::Location::ParadoxCaveLowerMiddle => true,
      locations::Location::ParadoxCaveUpperLeft => true,
      locations::Location::ParadoxCaveUpperRight => true,
      locations::Location::FloatingIsland => true,
      locations::Location::OldMan => true,
      locations::Location::SpectacleRockCave => true,
      locations::Location::EtherTablet => true,
      locations::Location::SpectacleRock => true,
      locations::Location::MasterSwordPedestal => true,
      locations::Location::LinkSUncle => true,
      locations::Location::SecretPassage => true,
      locations::Location::KingSTomb => true,
      locations::Location::FloodgateChest => true,
      locations::Location::LinkSHouse => true,
      locations::Location::KakarikoTavern => true,
      locations::Location::ChickenHouse => true,
      locations::Location::AginahSCave => true,
      locations::Location::SahasrahlaSHutLeft => true,
      locations::Location::SahasrahlaSHutMiddle => true,
      locations::Location::SahasrahlaSHutRight => true,
      locations::Location::KakrikoWellTop => true,
      locations::Location::KakrikoWellLeft => true,
      locations::Location::KakrikoWellMiddle => true,
      locations::Location::KakrikoWellRight => true,
      locations::Location::KakrikoWellBottom => true,
      locations::Location::BlindSHideoutTop => true,
      locations::Location::BlindSHideoutLeft => true,
      locations::Location::BlindSHideoutRight => true,
      locations::Location::BlindSHideoutFarLeft => true,
      locations::Location::BlindSHideoutFarRight => true,
      locations::Location::PegasusRocks => true,
      locations::Location::MiniMoldormCaveFarLeft => true,
      locations::Location::MiniMoldormCaveLeft => true,
      locations::Location::MiniMoldormCaveRight => true,
      locations::Location::MiniMoldormCaveFarRight => true,
      locations::Location::IceRodCave => true,
      locations::Location::BottleMerchant => true,
      locations::Location::Sahasrahla => true,
      locations::Location::MagicBat => true,
      locations::Location::SickKid => true,
      locations::Location::Hobo => true,
      locations::Location::BombosTablet => true,
      locations::Location::KingZora => true,
      locations::Location::LostWoodsHideout => true,
      locations::Location::LumberjackTree => true,
      locations::Location::Cave45 => true,
      locations::Location::GraveyardLedge => true,
      locations::Location::CheckerboardCave => true,
      locations::Location::MiniMoldormCaveNPC => true,
      locations::Location::Library => true,
      locations::Location::Mushroom => true,
      locations::Location::PotionShop => true,
      locations::Location::MazeRace => true,
      locations::Location::DesertLedge => true,
      locations::Location::LakeHyliaIsland => true,
      locations::Location::SunkenTreasure => true,
      locations::Location::ZoraSLedge => true,
      locations::Location::FluteSpot => true,
      locations::Location::WaterfallFairyLeft => true,
      locations::Location::WaterfallFairyRight => true,
    }
  }

  pub fn can_enter(
    reg: regions::Region,
    my_items: &Vec<items::Item>,
    assignments: &HashMap<locations::Location, items::Item>,
  ) -> bool {
    match reg { // @TODO
      regions::Region::LightWorld => true,
      regions::Region::Escape => true,
      regions::Region::EasternPalace => true,
      regions::Region::DesertPalace => true,
      regions::Region::WestDeathMountain => true,
      regions::Region::EastDeathMountain => true,
      regions::Region::TowerofHera => true,
      regions::Region::HyruleCastleTower => true,
      regions::Region::EastDarkWorldDeathMountain => true,
      regions::Region::WestDarkWorldDeathMountain => true,
      regions::Region::NorthEastDarkWorld => true,
      regions::Region::NorthWestDarkWorld => true,
      regions::Region::SouthDarkWorld => true,
      regions::Region::Mire => true,
      regions::Region::PalaceofDarkness => true,
      regions::Region::SwampPalace => true,
      regions::Region::SkullWoods => true,
      regions::Region::ThievesTown => true,
      regions::Region::IcePalace => true,
      regions::Region::MiseryMire => true,
      regions::Region::TurtleRock => true,
      regions::Region::GanonsTower => true,
      regions::Region::Fountains => true,
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

    match loc { // @TODO
      locations::Location::DesertPalaceBigChest => true,
      locations::Location::DesertPalaceMapChest => true,
      locations::Location::DesertPalaceTorch => true,
      locations::Location::DesertPalaceBigKeyChest => true,
      locations::Location::DesertPalaceCompassChest => true,
      locations::Location::DesertPalaceLanmolas => true,
      locations::Location::DesertPalacePrize => true,
      locations::Location::EasternPalaceCompassChest => true,
      locations::Location::EasternPalaceBigChest => true,
      locations::Location::EasternPalaceCannonballChest => true,
      locations::Location::EasternPalaceBigKeyChest => true,
      locations::Location::EasternPalaceMapChest => true,
      locations::Location::EasternPalaceArmosKnights => true,
      locations::Location::EasternPalacePrize => true,
      locations::Location::TowerOfHeraBigKeyChest => true,
      locations::Location::TowerOfHeraBasementCage => true,
      locations::Location::TowerOfHeraMapChest => true,
      locations::Location::TowerOfHeraCompassChest => true,
      locations::Location::TowerOfHeraBigChest => true,
      locations::Location::TowerOfHeraMoldorm => true,
      locations::Location::TowerOfHeraPrize => true,
      locations::Location::IcePalaceBigKeyChest => true,
      locations::Location::IcePalaceCompassChest => true,
      locations::Location::IcePalaceMapChest => true,
      locations::Location::IcePalaceSpikeRoom => true,
      locations::Location::IcePalaceFreezorChest => true,
      locations::Location::IcePalaceIcedTRoom => true,
      locations::Location::IcePalaceBigChest => true,
      locations::Location::IcePalaceKholdstare => true,
      locations::Location::IcePalacePrize => true,
      locations::Location::MiseryMireBigChest => true,
      locations::Location::MiseryMireMainLobby => true,
      locations::Location::MiseryMireBigKeyChest => true,
      locations::Location::MiseryMireCompassChest => true,
      locations::Location::MiseryMireBridgeChest => true,
      locations::Location::MiseryMireMapChest => true,
      locations::Location::MiseryMireSpikeChest => true,
      locations::Location::MiseryMireVitreous => true,
      locations::Location::MiseryMirePrize => true,
      locations::Location::PalaceOfDarknessBigKeyChest => true,
      locations::Location::PalaceOfDarknessTheArenaLedge => true,
      locations::Location::PalaceOfDarknessTheArenaBridge => true,
      locations::Location::PalaceOfDarknessBigChest => true,
      locations::Location::PalaceOfDarknessCompassChest => true,
      locations::Location::PalaceOfDarknessHarmlessHellway => true,
      locations::Location::PalaceOfDarknessStalfosBasement => true,
      locations::Location::PalaceOfDarknessDarkBasementLeft => true,
      locations::Location::PalaceOfDarknessDarkBasementRight => true,
      locations::Location::PalaceOfDarknessMapChest => true,
      locations::Location::PalaceOfDarknessDarkMazeTop => true,
      locations::Location::PalaceOfDarknessDarkMazeBottom => true,
      locations::Location::PalaceOfDarknessShooterRoom => true,
      locations::Location::PalaceOfDarknessHelmasaurKing => true,
      locations::Location::PalaceOfDarknessPrize => true,
      locations::Location::SkullWoodsBigChest => true,
      locations::Location::SkullWoodsBigKeyChest => true,
      locations::Location::SkullWoodsCompassChest => true,
      locations::Location::SkullWoodsMapChest => true,
      locations::Location::SkullWoodsBridgeRoom => true,
      locations::Location::SkullWoodsPotPrison => true,
      locations::Location::SkullWoodsPinballRoom => true,
      locations::Location::SkullWoodsMothula => true,
      locations::Location::SkullWoodsPrize => true,
      locations::Location::SwampPalaceEntrance => true,
      locations::Location::SwampPalaceBigChest => true,
      locations::Location::SwampPalaceBigKeyChest => true,
      locations::Location::SwampPalaceMapChest => true,
      locations::Location::SwampPalaceWestChest => true,
      locations::Location::SwampPalaceCompassChest => true,
      locations::Location::SwampPalaceFloodedRoomLeft => true,
      locations::Location::SwampPalaceFloodedRoomRight => true,
      locations::Location::SwampPalaceWaterfallRoom => true,
      locations::Location::SwampPalaceArrghus => true,
      locations::Location::SwampPalacePrize => true,
      locations::Location::ThievesTownAttic => true,
      locations::Location::ThievesTownBigKeyChest => true,
      locations::Location::ThievesTownMapChest => true,
      locations::Location::ThievesTownCompassChest => true,
      locations::Location::ThievesTownAmbushChest => true,
      locations::Location::ThievesTownBigChest => true,
      locations::Location::ThievesTownBlindSCell => true,
      locations::Location::ThievesTownBlind => true,
      locations::Location::ThievesTownPrize => true,
      locations::Location::TurtleRockChainChomps => true,
      locations::Location::TurtleRockCompassChest => true,
      locations::Location::TurtleRockRollerRoomLeft => true,
      locations::Location::TurtleRockRollerRoomRight => true,
      locations::Location::TurtleRockBigChest => true,
      locations::Location::TurtleRockBigKeyChest => true,
      locations::Location::TurtleRockCrystarollerRoom => true,
      locations::Location::TurtleRockEyeBridgeBottomLeft => true,
      locations::Location::TurtleRockEyeBridgeBottomRight => true,
      locations::Location::TurtleRockEyeBridgeTopLeft => true,
      locations::Location::TurtleRockEyeBridgeTopRight => true,
      locations::Location::TurtleRockTrinexx => true,
      locations::Location::TurtleRockPrize => true,
      locations::Location::GanonSTowerBobSTorch => true,
      locations::Location::GanonSTowerDMsRoomTopLeft => true,
      locations::Location::GanonSTowerDMsRoomTopRight => true,
      locations::Location::GanonSTowerDMsRoomBottomLeft => true,
      locations::Location::GanonSTowerDMsRoomBottomRight => true,
      locations::Location::GanonSTowerRandomizerRoomTopLeft => true,
      locations::Location::GanonSTowerRandomizerRoomTopRight => true,
      locations::Location::GanonSTowerRandomizerRoomBottomLeft => true,
      locations::Location::GanonSTowerRandomizerRoomBottomRight => true,
      locations::Location::GanonSTowerFiresnakeRoom => true,
      locations::Location::GanonSTowerMapChest => true,
      locations::Location::GanonSTowerBigChest => true,
      locations::Location::GanonSTowerHopeRoomLeft => true,
      locations::Location::GanonSTowerHopeRoomRight => true,
      locations::Location::GanonSTowerBobSChest => true,
      locations::Location::GanonSTowerTileRoom => true,
      locations::Location::GanonSTowerCompassRoomTopLeft => true,
      locations::Location::GanonSTowerCompassRoomTopRight => true,
      locations::Location::GanonSTowerCompassRoomBottomLeft => true,
      locations::Location::GanonSTowerCompassRoomBottomRight => true,
      locations::Location::GanonSTowerBigKeyChest => true,
      locations::Location::GanonSTowerBigKeyRoomLeft => true,
      locations::Location::GanonSTowerBigKeyRoomRight => true,
      locations::Location::GanonSTowerMiniHelmasaurRoomLeft => true,
      locations::Location::GanonSTowerMiniHelmasaurRoomRight => true,
      locations::Location::GanonSTowerPreMoldormChest => true,
      locations::Location::GanonSTowerMoldormChest => true,
      locations::Location::WaterfallBottle => true,
      locations::Location::PyramidBottle => true,
      locations::Location::Sanctuary => true,
      locations::Location::SewersSecretRoomLeft => true,
      locations::Location::SewersSecretRoomMiddle => true,
      locations::Location::SewersSecretRoomRight => true,
      locations::Location::SewersDarkCross => true,
      locations::Location::HyruleCastleBoomerangChest => true,
      locations::Location::HyruleCastleMapChest => true,
      locations::Location::HyruleCastleZeldaSCell => true,
      locations::Location::CastleTowerRoom03 => true,
      locations::Location::CastleTowerDarkMaze => true,
      locations::Location::MireShedLeft => true,
      locations::Location::MireShedRight => true,
      locations::Location::Catfish => true,
      locations::Location::Pyramid => true,
      locations::Location::PyramidFairySword => true,
      locations::Location::PyramidFairyBow => true,
      locations::Location::PyramidFairyLeft => true,
      locations::Location::PyramidFairyRight => true,
      locations::Location::Brewery => true,
      locations::Location::CShapedHouse => true,
      locations::Location::ChestGame => true,
      locations::Location::HammerPegs => true,
      locations::Location::BumperCave => true,
      locations::Location::Blacksmith => true,
      locations::Location::PurpleChest => true,
      locations::Location::HypeCaveTop => true,
      locations::Location::HypeCaveMiddleRight => true,
      locations::Location::HypeCaveMiddleLeft => true,
      locations::Location::HypeCaveBottom => true,
      locations::Location::Stumpy => true,
      locations::Location::HypeCaveNPC => true,
      locations::Location::DiggingGame => true,
      locations::Location::SuperbunnyCaveTop => true,
      locations::Location::SuperbunnyCaveBottom => true,
      locations::Location::HookshotCaveTopRight => true,
      locations::Location::HookshotCaveTopLeft => true,
      locations::Location::HookshotCaveBottomLeft => true,
      locations::Location::HookshotCaveBottomRight => true,
      locations::Location::SpikeCave => true,
      locations::Location::SpiralCave => true,
      locations::Location::MimicCave => true,
      locations::Location::ParadoxCaveLowerFarLeft => true,
      locations::Location::ParadoxCaveLowerLeft => true,
      locations::Location::ParadoxCaveLowerRight => true,
      locations::Location::ParadoxCaveLowerFarRight => true,
      locations::Location::ParadoxCaveLowerMiddle => true,
      locations::Location::ParadoxCaveUpperLeft => true,
      locations::Location::ParadoxCaveUpperRight => true,
      locations::Location::FloatingIsland => true,
      locations::Location::OldMan => true,
      locations::Location::SpectacleRockCave => true,
      locations::Location::EtherTablet => true,
      locations::Location::SpectacleRock => true,
      locations::Location::MasterSwordPedestal => true,
      locations::Location::LinkSUncle => true,
      locations::Location::SecretPassage => true,
      locations::Location::KingSTomb => true,
      locations::Location::FloodgateChest => true,
      locations::Location::LinkSHouse => true,
      locations::Location::KakarikoTavern => true,
      locations::Location::ChickenHouse => true,
      locations::Location::AginahSCave => true,
      locations::Location::SahasrahlaSHutLeft => true,
      locations::Location::SahasrahlaSHutMiddle => true,
      locations::Location::SahasrahlaSHutRight => true,
      locations::Location::KakrikoWellTop => true,
      locations::Location::KakrikoWellLeft => true,
      locations::Location::KakrikoWellMiddle => true,
      locations::Location::KakrikoWellRight => true,
      locations::Location::KakrikoWellBottom => true,
      locations::Location::BlindSHideoutTop => true,
      locations::Location::BlindSHideoutLeft => true,
      locations::Location::BlindSHideoutRight => true,
      locations::Location::BlindSHideoutFarLeft => true,
      locations::Location::BlindSHideoutFarRight => true,
      locations::Location::PegasusRocks => true,
      locations::Location::MiniMoldormCaveFarLeft => true,
      locations::Location::MiniMoldormCaveLeft => true,
      locations::Location::MiniMoldormCaveRight => true,
      locations::Location::MiniMoldormCaveFarRight => true,
      locations::Location::IceRodCave => true,
      locations::Location::BottleMerchant => true,
      locations::Location::Sahasrahla => true,
      locations::Location::MagicBat => true,
      locations::Location::SickKid => true,
      locations::Location::Hobo => true,
      locations::Location::BombosTablet => true,
      locations::Location::KingZora => true,
      locations::Location::LostWoodsHideout => true,
      locations::Location::LumberjackTree => true,
      locations::Location::Cave45 => true,
      locations::Location::GraveyardLedge => true,
      locations::Location::CheckerboardCave => true,
      locations::Location::MiniMoldormCaveNPC => true,
      locations::Location::Library => true,
      locations::Location::Mushroom => true,
      locations::Location::PotionShop => true,
      locations::Location::MazeRace => true,
      locations::Location::DesertLedge => true,
      locations::Location::LakeHyliaIsland => true,
      locations::Location::SunkenTreasure => true,
      locations::Location::ZoraSLedge => true,
      locations::Location::FluteSpot => true,
      locations::Location::WaterfallFairyLeft => true,
      locations::Location::WaterfallFairyRight => true,
    }
  }
}

mod generator {
  use std::collections::HashMap;
  use rand::{Rng, ThreadRng};
  use super::{medallions, logic, world, locations, regions, items};

  pub fn generate_world(
    advancement_items: &Vec<items::Item>,
    nice_items: &Vec<items::Item>,
    junk_items: &Vec<items::Item>,
    dungeon_items: &Vec<items::Item>,
    rng: &mut ThreadRng,
  ) -> world::World {
    let medallions;
    { // Set up medallions
      let all_meds = medallions::get_all_medallions();
      medallions = medallions::Entrance {
        turtle_rock: *rng.choose(&all_meds).expect("empty medallion array"),
        misery_mire: *rng.choose(&all_meds).expect("empty medallion array"),
      };
    }

    let mut assignments;
    { // Set up assignments
      assignments = HashMap::new();
      { // Place prizes
        let mut prizes = vec![
          items::Item::Crystal1,
          items::Item::Crystal2,
          items::Item::Crystal3,
          items::Item::Crystal4,
          items::Item::Crystal5,
          items::Item::Crystal6,
          items::Item::Crystal7,
          items::Item::PendantOfCourage,
          items::Item::PendantOfPower,
          items::Item::PendantOfWisdom,
        ];
        rng.shuffle(&mut prizes);
        let mut iter = prizes.into_iter();
        assignments.insert(locations::Location::TowerOfHeraPrize, iter.next().unwrap());
        assignments.insert(locations::Location::EasternPalacePrize, iter.next().unwrap());
        assignments.insert(locations::Location::DesertPalacePrize, iter.next().unwrap());
        assignments.insert(locations::Location::SkullWoodsPrize, iter.next().unwrap());
        assignments.insert(locations::Location::ThievesTownPrize, iter.next().unwrap());
        assignments.insert(locations::Location::MiseryMirePrize, iter.next().unwrap());
        assignments.insert(locations::Location::SwampPalacePrize, iter.next().unwrap());
        assignments.insert(locations::Location::IcePalacePrize, iter.next().unwrap());
        assignments.insert(locations::Location::PalaceOfDarknessPrize, iter.next().unwrap());
        assignments.insert(locations::Location::TurtleRockPrize, iter.next().unwrap());
        assert!(iter.next() == None);
        // TODO: php has a weird conditional-and-`throw`; is this relevant?
        //   There aren't any restrictions, are there?
        //   e.g. something like "Turtle Rock can't have the red pendant"?
      }

      // TODO: not sure what's up in the php here;
      //   it sets waterfall fairy stuff and bow fairy stuff for some reason.
      //   It seems to set what the fairy fills your bottles with? idk

      let mut advancement_items_iter;
      let mut nice_items_iter;
      let mut junk_items_iter;
      let mut dungeon_items_iter;
      { // init item iterators
        let mut advancement_items_clone = advancement_items.clone();
        rng.shuffle(&mut advancement_items_clone);
        advancement_items_iter = advancement_items_clone.into_iter();

        let mut nice_items_clone = nice_items.clone();
        rng.shuffle(&mut nice_items_clone);
        nice_items_iter = nice_items_clone.into_iter();

        let mut junk_items_clone = junk_items.clone();
        rng.shuffle(&mut junk_items_clone);
        junk_items_iter = junk_items_clone.into_iter();

        let mut dungeon_items_clone = dungeon_items.clone();
        rng.shuffle(&mut dungeon_items_clone);
        dungeon_items_iter = dungeon_items_clone.into_iter();
      }

      // The code from here to the end is based on RandomAssumed.php

      let mut randomized_order_locations = locations::get_all_locations();
      rng.shuffle(&mut randomized_order_locations);
      trace!("randomized_order_locations: {:?}", randomized_order_locations);

      fill_items_in_locations(dungeon_items_iter, &randomized_order_locations, &advancement_items, &mut assignments);

      { // put some junk in ganon
        let num_junk_items = rng.next_u32() % 16;
        let ganon_locs: Vec<locations::Location> = regions::get_locations_for(regions::Region::GanonsTower).into_iter()
          .filter(|loc| assignments.get(loc) == None)
          .take(num_junk_items as usize)
          .collect();
        fast_fill_items_in_locations(&mut junk_items_iter, &ganon_locs, &mut assignments);
      }

      randomized_order_locations.reverse();

      fill_items_in_locations(advancement_items_iter, &randomized_order_locations, &vec![], &mut assignments);

      rng.shuffle(&mut randomized_order_locations);

      fast_fill_items_in_locations(&mut nice_items_iter, &randomized_order_locations, &mut assignments);
      assert_eq!(nice_items_iter.next(), None);
      // @hack: the php randomizes junk_items _again_ here;
      //   I'm skipping that useless step (or maybe I'm dumb?)
      fast_fill_items_in_locations(&mut junk_items_iter, &randomized_order_locations, &mut assignments);
      assert_eq!(junk_items_iter.next(), None);
    }

    let world = world::World {
      assignments,
      medallions,
    };
    world
  }

  fn fast_fill_items_in_locations(
    fill_items: &mut IntoIter<items::Item>,
    locations: &Vec<locations::Location>,
    assignments: &mut HashMap<locations::Location, items::Item>,
  ) {
    for &loc in locations.iter() {
      if assignments.contains_key(&loc) { continue };
      match fill_items.next() {
        Some(item) => {
          debug!("Filling {:?} with {:?}", loc, item);
          assignments.insert(loc, item)
        },
        None => break,
      };
    }
  }

  use std::vec::IntoIter;
  fn fill_items_in_locations(
    fill_items: IntoIter<items::Item>,
    locations: &Vec<locations::Location>,
    base_assumed_items: &Vec<items::Item>,
    assignments: &mut HashMap<locations::Location, items::Item>,
  ) {
    let mut remaining_fill_items: Vec<items::Item> = fill_items.collect();
    for _ in 0..remaining_fill_items.len() {
      let item = remaining_fill_items.pop().expect("bad for loop sync");
      let mut assumed_items = base_assumed_items.clone();
      assumed_items.append(&mut (remaining_fill_items.clone()));
      assumed_items = collect_items(&assumed_items, &assignments);

      let loc = locations.iter()
        .filter(|&&loc| assignments.get(&loc) == None)
        .filter(|&&loc| logic::can_fill(item, loc, &assumed_items, &assignments))
        .next();
      match loc {
        Some(loc) => {
          debug!("Filling {:?} with {:?}", loc, item);
          assignments.insert(*loc, item);
        },
        None => {
          panic!("No available locations for {:?}", item); // TODO: ~s/panic/Result/
        }
      }
    }
  }

  use std::collections::HashSet;
  fn collect_items(
    assumed_items: &Vec<items::Item>,
    assignments: &HashMap<locations::Location, items::Item>,
  ) -> Vec<items::Item> {
    let mut my_items = assumed_items.clone();
    let mut available_locations: HashSet<locations::Location> = locations::get_all_locations()
      .into_iter()
      .filter(|&loc| assignments.get(&loc).is_some())
      .collect();
    loop {
      let search_locations: Vec<locations::Location> = available_locations.iter()
        .filter(|&&loc| logic::can_access(loc, &my_items, &assignments))
        .map(|&loc| loc)
        .collect();

      // available_locations -= search_locations
      available_locations = available_locations.into_iter()
        .filter(|&loc| !search_locations.contains(&loc))
        .collect();
      let mut found_items: Vec<items::Item> = search_locations.into_iter()
        .filter_map(|loc| assignments.get(&loc))
        .map(|&item| item)
        .collect();
      if found_items.len() == 0 {
        break;
      }
      my_items.append(&mut found_items);
    }

    return my_items;
  }
}

fn main() {
  env_logger::init().unwrap();

  let advancement_items = items::get_advancement_items();
  trace!("advancement_items: {:?}", advancement_items);

  let nice_items = items::get_nice_items();
  trace!("nice_items: {:?}", nice_items);

  let junk_items = items::get_item_pool();
  trace!("item_pool: {:?}", junk_items);

  let dungeon_items = items::get_dungeon_pool();
  trace!("dungeon_items: {:?}", dungeon_items);

  let mut rng = rand::thread_rng();

  let sim_count = 1;
  for _ in 0..sim_count {
    let world = generator::generate_world(&advancement_items, &nice_items, &junk_items, &dungeon_items, &mut rng);
    info!("{:?}", world);
  }
}
