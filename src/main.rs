extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;

mod items {
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
  use super::{items};

  #[allow(dead_code)]
  #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
  pub enum MedallionLock {
    BombosLock,
    EtherLock,
    QuakeLock,
  }

  pub fn get_all_medallions() -> Vec<MedallionLock> {
    use self::MedallionLock::*;
    vec![
      BombosLock,
      EtherLock,
      QuakeLock,
    ]
  }

  pub fn as_item(med: MedallionLock) -> items::Item {
    use self::MedallionLock::*;
    use items::Item::*;
    match med {
      BombosLock => Bombos,
      EtherLock  => Ether,
      QuakeLock  => Quake,
    }
  }

  #[allow(dead_code)]
  #[derive(Eq, PartialEq, Copy, Clone, Debug)]
  pub struct EntranceConfig {
    pub turtle_rock: MedallionLock,
    pub misery_mire: MedallionLock,
  }
}

mod locations {
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
}

mod regions {
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
}

mod world {
  use std::collections::HashMap;
  use super::{medallions, locations, items};

  #[derive(Eq, PartialEq, Debug)]
  pub struct World {
    pub medallions: medallions::EntranceConfig,
    pub assignments: HashMap<locations::Location, items::Item>,
  }
}

mod logic {
  use std::collections::HashMap;
  use super::{locations, regions, items, world, medallions};
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

  fn item_in_locations(
    item: &items::Item,
    locs: Vec<locations::Location>,
    world: &world::World,
  ) -> bool {
    locs.iter()
      .filter_map(|&loc| world.assignments.get(&loc))
      .any(|&it| it == *item)
  }

  pub fn can_complete(
    reg: regions::Region,
    my_items: &Vec<items::Item>,
    world: &world::World,
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
      Some(loc) => can_access(loc, &my_items, &world),
      None => true,
    }
  }

  pub fn can_fill(
    item: items::Item,
    loc: locations::Location,
    my_items: &Vec<items::Item>,
  ) -> bool {

    // @TODO: some sort of logic in Region.php#canFill
    //   I assume it keeps keys where they belong but idk
    //   and I'm too tired right now to get this right.
    match regions::get_dungeon_items_for(locations::get_region_for(loc)) {
      Some(dungeon_items) => {

      },
      None => {

      }
    };

    let todo = true;
    match loc { // @TODO
      DesertPalaceBigChest => item != BigKeyP2,
      DesertPalaceMapChest => true,
      DesertPalaceTorch => true,
      DesertPalaceBigKeyChest => item != KeyP2,
      DesertPalaceCompassChest => item != KeyP2,
      DesertPalaceLanmolas => item != KeyP2 && item != BigKeyP2,
      DesertPalacePrize => true,
      EasternPalaceCompassChest => true,
      EasternPalaceBigChest => item != BigKeyP1,
      EasternPalaceCannonballChest => true,
      EasternPalaceBigKeyChest => true,
      EasternPalaceMapChest => true,
      EasternPalaceArmosKnights => item != BigKeyP1,
      EasternPalacePrize => true,
      TowerOfHeraBigKeyChest => item != KeyP3,
      TowerOfHeraBasementCage => true,
      TowerOfHeraMapChest => true,
      TowerOfHeraCompassChest => item != BigKeyP3,
      TowerOfHeraBigChest => item != BigKeyP3,
      TowerOfHeraMoldorm => item != BigKeyP3,
      TowerOfHeraPrize => true,
      IcePalaceBigKeyChest => true,
      IcePalaceCompassChest => true,
      IcePalaceMapChest => true,
      IcePalaceSpikeRoom => true,
      IcePalaceFreezorChest => true,
      IcePalaceIcedTRoom => true,
      IcePalaceBigChest => item != BigKeyD5,
      IcePalaceKholdstare => item != BigKeyD5,
      IcePalacePrize => true,
      MiseryMireBigChest => item != BigKeyD6,
      MiseryMireMainLobby => true,
      MiseryMireBigKeyChest => true,
      MiseryMireCompassChest => true,
      MiseryMireBridgeChest => true,
      MiseryMireMapChest => true,
      MiseryMireSpikeChest => true,
      MiseryMireVitreous => item != BigKeyD6,
      MiseryMirePrize => true,
      PalaceOfDarknessBigKeyChest => true,
      PalaceOfDarknessTheArenaLedge => true,
      PalaceOfDarknessTheArenaBridge => true,
      PalaceOfDarknessBigChest => item != KeyD1 && item != BigKeyD1,
      PalaceOfDarknessCompassChest => true,
      PalaceOfDarknessHarmlessHellway => true,
      PalaceOfDarknessStalfosBasement => true,
      PalaceOfDarknessDarkBasementLeft => true,
      PalaceOfDarknessDarkBasementRight => true,
      PalaceOfDarknessMapChest => true,
      PalaceOfDarknessDarkMazeTop => item != KeyD1,
      PalaceOfDarknessDarkMazeBottom => item != KeyD1,
      PalaceOfDarknessShooterRoom => true,
      PalaceOfDarknessHelmasaurKing => item != KeyD1 && item != BigKeyD1,
      PalaceOfDarknessPrize => true,
      SkullWoodsBigChest => item != BigKeyD3,
      SkullWoodsBigKeyChest => true,
      SkullWoodsCompassChest => true,
      SkullWoodsMapChest => true,
      SkullWoodsBridgeRoom => true,
      SkullWoodsPotPrison => true,
      SkullWoodsPinballRoom => item == KeyD3,
      SkullWoodsMothula => item != KeyD3,
      SkullWoodsPrize => true,
      SwampPalaceEntrance => item == KeyD2,
      SwampPalaceBigChest => item != BigKeyD2,
      SwampPalaceBigKeyChest => true,
      SwampPalaceMapChest => true,
      SwampPalaceWestChest => true,
      SwampPalaceCompassChest => true,
      SwampPalaceFloodedRoomLeft => true,
      SwampPalaceFloodedRoomRight => true,
      SwampPalaceWaterfallRoom => true,
      SwampPalaceArrghus => true,
      SwampPalacePrize => true,
      ThievesTownAttic => item != KeyD4 && item != BigKeyD4,
      ThievesTownBigKeyChest => item != KeyD4 && item != BigKeyD4,
      ThievesTownMapChest => true,
      ThievesTownCompassChest => true,
      ThievesTownAmbushChest => true,
      ThievesTownBigChest => true,
      ThievesTownBlindSCell => item != BigKeyD4,
      ThievesTownBlind => item != KeyD4 && item != BigKeyD4,
      ThievesTownPrize => true,
      TurtleRockChainChomps => true,
      TurtleRockCompassChest => true,
      TurtleRockRollerRoomLeft => true,
      TurtleRockRollerRoomRight => true,
      TurtleRockBigChest => item != BigKeyD7,
      TurtleRockBigKeyChest => true,
      TurtleRockCrystarollerRoom => item != BigKeyD7,
      TurtleRockEyeBridgeBottomLeft => item != BigKeyD7,
      TurtleRockEyeBridgeBottomRight => item != BigKeyD7,
      TurtleRockEyeBridgeTopLeft => item != BigKeyD7,
      TurtleRockEyeBridgeTopRight => item != BigKeyD7,
      TurtleRockTrinexx => item != KeyD7 && item != BigKeyD7,
      TurtleRockPrize => true,
      GanonSTowerBobSTorch => true,
      GanonSTowerDMsRoomTopLeft => true,
      GanonSTowerDMsRoomTopRight => true,
      GanonSTowerDMsRoomBottomLeft => true,
      GanonSTowerDMsRoomBottomRight => true,
      GanonSTowerRandomizerRoomTopLeft => true,
      GanonSTowerRandomizerRoomTopRight => true,
      GanonSTowerRandomizerRoomBottomLeft => true,
      GanonSTowerRandomizerRoomBottomRight => true,
      GanonSTowerFiresnakeRoom => true,
      GanonSTowerMapChest => true,
      GanonSTowerBigChest => item != BigKeyA2,
      GanonSTowerHopeRoomLeft => true,
      GanonSTowerHopeRoomRight => true,
      GanonSTowerBobSChest => true,
      GanonSTowerTileRoom => true,
      GanonSTowerCompassRoomTopLeft => true,
      GanonSTowerCompassRoomTopRight => true,
      GanonSTowerCompassRoomBottomLeft => true,
      GanonSTowerCompassRoomBottomRight => true,
      GanonSTowerBigKeyChest => true,
      GanonSTowerBigKeyRoomLeft => true,
      GanonSTowerBigKeyRoomRight => true,
      GanonSTowerMiniHelmasaurRoomLeft => item != BigKeyA2,
      GanonSTowerMiniHelmasaurRoomRight => item != BigKeyA2,
      GanonSTowerPreMoldormChest => item != BigKeyA2,
      GanonSTowerMoldormChest => item != KeyA2 && item != BigKeyA2,
      Agahnim2 => true,
      WaterfallBottle => true,
      PyramidBottle => true,
      Sanctuary => true,
      SewersSecretRoomLeft => true,
      SewersSecretRoomMiddle => true,
      SewersSecretRoomRight => true,
      SewersDarkCross => true,
      HyruleCastleBoomerangChest => item != KeyH2,
      HyruleCastleMapChest => true,
      HyruleCastleZeldaSCell => item != KeyH2,
      CastleTowerRoom03 => true,
      CastleTowerDarkMaze => true,
      Agahnim => true,
      MireShedLeft => true,
      MireShedRight => true,
      Catfish => true,
      Pyramid => true,
      PyramidFairySword => true,
      PyramidFairyBow => true,
      PyramidFairyLeft => true,
      PyramidFairyRight => true,
      Ganon => true,
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
    }
  }

  pub fn can_enter(
    reg: regions::Region,
    my_items: &Vec<items::Item>,
    world: &world::World,
  ) -> bool {
    use items::Item::*;
    use regions::Region::*;

    let todo = true;
    match reg { // @TODO
      LightWorld => todo,
      Escape => true,
      EasternPalace => true,
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
      TowerofHera => {
        (
          my_items.contains(&MagicMirror)
          || (
            my_items.contains(&Hookshot)
            && my_items.contains(&Hammer)
          )
        ) && can_enter(WestDeathMountain, &my_items, &world)
      },
      HyruleCastleTower => {
        my_items.contains(&Cape)
        || has_upgraded_sword(&my_items)
      },
      EastDarkWorldDeathMountain => todo,
      WestDarkWorldDeathMountain => todo,
      NorthEastDarkWorld => {
        my_items.contains(&DefeatAgahnim)
        || (
          my_items.contains(&Hammer)
          && can_lift_rocks(&my_items)
          && my_items.contains(&MoonPearl)
        ) || (
          can_lift_dark_rocks(&my_items)
          && my_items.contains(&Flippers)
          && my_items.contains(&MoonPearl)
          )
      },
      NorthWestDarkWorld => todo,
      SouthDarkWorld => todo,
      Mire => {
        can_fly(&my_items)
        && can_lift_dark_rocks(&my_items)
      },
      PalaceofDarkness => {
        my_items.contains(&MoonPearl)
        && can_enter(NorthEastDarkWorld, &my_items, &world)
      },
      SwampPalace => {
        my_items.contains(&MoonPearl)
        && my_items.contains(&MagicMirror)
        && my_items.contains(&Flippers)
        && can_enter(SouthDarkWorld, &my_items, &world)
      },
      SkullWoods => {
        my_items.contains(&MoonPearl)
        && can_enter(NorthWestDarkWorld, &my_items, &world)
      },
      ThievesTown => {
        my_items.contains(&MoonPearl)
        && can_enter(NorthWestDarkWorld, &my_items, &world)
      },
      IcePalace => {
        my_items.contains(&MoonPearl)
        && my_items.contains(&Flippers)
        && can_lift_dark_rocks(&my_items)
        && can_melt_things(&my_items)
      },
      MiseryMire => {
        my_items.contains(&medallions::as_item(world.medallions.misery_mire))
        && has_sword(&my_items)
        && my_items.contains(&MoonPearl)
        && (
          my_items.contains(&PegasusBoots)
          || my_items.contains(&Hookshot)
        ) && can_enter(Mire, &my_items, &world)
      },
      TurtleRock => {
        my_items.contains(&medallions::as_item(world.medallions.turtle_rock))
        && has_sword(&my_items)
        && my_items.contains(&MoonPearl)
        && my_items.contains(&CaneOfSomaria)
        && my_items.contains(&Hammer)
        && can_enter(EastDeathMountain, &my_items, &world)
      },
      GanonsTower => {
        my_items.contains(&MoonPearl)
        && my_items.contains(&Crystal1)
        && my_items.contains(&Crystal2)
        && my_items.contains(&Crystal3)
        && my_items.contains(&Crystal4)
        && my_items.contains(&Crystal5)
        && my_items.contains(&Crystal6)
        && my_items.contains(&Crystal7)
        && can_enter(EastDarkWorldDeathMountain, &my_items, &world)
      },
      Fountains => true,
    }
  }

  pub fn can_access(
    loc: locations::Location,
    my_items: &Vec<items::Item>,
    world: &world::World,
  ) -> bool {
    let reg = locations::get_region_for(loc);
    if !can_enter(reg, &my_items, &world) {
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
          can_enter(DesertPalace, &my_items, &world)
          && can_lift_rocks(&my_items)
          && can_light_torches(&my_items)
          && my_items.contains(&BigKeyP2)
          && my_items.contains(&KeyP2)
        )
      },
      DesertPalacePrize => can_access(DesertPalaceLanmolas, &my_items, &world),
      EasternPalaceCompassChest => true,
      EasternPalaceBigChest => {
        my_items.contains(&BigKeyP1)
      },
      EasternPalaceCannonballChest => true,
      EasternPalaceBigKeyChest => {
        my_items.contains(&Lamp)
      },
      EasternPalaceMapChest => true,
      EasternPalaceArmosKnights => {
        can_shoot_arrows(&my_items)
        && my_items.contains(&Lamp)
        && my_items.contains(&BigKeyP1)
      },
      EasternPalacePrize => can_access(EasternPalaceArmosKnights, &my_items, &world),
      TowerOfHeraBigKeyChest => {
        can_light_torches(&my_items)
        && my_items.contains(&KeyP3)
      },
      TowerOfHeraBasementCage => true,
      TowerOfHeraMapChest => true,
      TowerOfHeraCompassChest => {
        my_items.contains(&BigKeyP3)
      },
      TowerOfHeraBigChest => {
        my_items.contains(&BigKeyP3)
      },
      TowerOfHeraMoldorm => {
        can_enter(TowerofHera, &my_items, &world)
        && (
          has_sword(&my_items)
          || my_items.contains(&Hammer)
        ) && my_items.contains(&BigKeyP3)
      },
      TowerOfHeraPrize => can_access(TowerOfHeraMoldorm, &my_items, &world),
      IcePalaceBigKeyChest => {
        my_items.contains(&Hammer)
        && can_lift_rocks(&my_items)
        && (
          my_items.contains(&Hookshot)
          || (
            item_in_locations(&BigKeyD5,
              vec![IcePalaceMapChest, IcePalaceSpikeRoom], &world)
            && my_items.contains(&KeyD5)
          ) || count(&KeyD5, &my_items) >= 2
        ) && (
          my_items.contains(&Hookshot)
          || my_items.contains(&CaneOfByrna)
          || my_items.contains(&Cape)
        )
      },
      IcePalaceCompassChest => true,
      IcePalaceMapChest => {
        my_items.contains(&Hammer)
        && can_lift_rocks(&my_items)
        && (
          my_items.contains(&Hookshot)
          || (
            item_in_locations(&BigKeyD5,
              vec![IcePalaceSpikeRoom, IcePalaceBigKeyChest], &world)
            && my_items.contains(&KeyD5)
          ) || count(&KeyD5, &my_items) >= 2
        ) && (
          my_items.contains(&Hookshot)
          || my_items.contains(&CaneOfByrna)
          || my_items.contains(&Cape)
        )
      },
      IcePalaceSpikeRoom => {
        (
          my_items.contains(&Hookshot)
          || (
            item_in_locations(&BigKeyD5,
              vec![IcePalaceMapChest, IcePalaceBigKeyChest], &world)
            && my_items.contains(&KeyD5)
          ) || count(&KeyD5, &my_items) >= 2
        ) && (
          my_items.contains(&Hookshot)
          || my_items.contains(&CaneOfByrna)
          || my_items.contains(&Cape)
        )
      },
      IcePalaceFreezorChest => can_melt_things(&my_items),
      IcePalaceIcedTRoom => true,
      IcePalaceBigChest => my_items.contains(&BigKeyD5),
      IcePalaceKholdstare => {
        my_items.contains(&Hammer)
        && can_melt_things(&my_items)
        && can_lift_rocks(&my_items)
        && my_items.contains(&BigKeyD5)
        && (
          (
            my_items.contains(&CaneOfSomaria)
            && my_items.contains(&KeyD5)
          ) || count(&KeyD5, &my_items) >= 2
        )
      },
      IcePalacePrize => can_access(IcePalaceKholdstare, &my_items, &world),
      MiseryMireBigChest => my_items.contains(&BigKeyD6),
      MiseryMireMainLobby => {
        my_items.contains(&KeyD6)
        || my_items.contains(&BigKeyD6)
      },
      MiseryMireBigKeyChest => {
        can_light_torches(&my_items)
        && (
          (
            item_in_locations(&BigKeyD6, vec![MiseryMireCompassChest], &world)
            && count(&KeyD6, &my_items) >= 2
          ) || count(&KeyD6, &my_items) >= 3
        )
      },
      MiseryMireCompassChest => {
        can_light_torches(&my_items)
        && (
          (
            item_in_locations(&BigKeyD6, vec![MiseryMireBigKeyChest], &world)
            && count(&KeyD6, &my_items) >= 2
          ) || count(&KeyD6, &my_items) >= 3
        )
      },
      MiseryMireBridgeChest => true,
      MiseryMireMapChest => {
        my_items.contains(&KeyD6)
        || my_items.contains(&BigKeyD6)
      },
      MiseryMireSpikeChest => {
        my_items.contains(&Cape)
        || my_items.contains(&CaneOfByrna)
      },
      MiseryMireVitreous => {
        my_items.contains(&CaneOfSomaria)
        && my_items.contains(&Lamp)
        && my_items.contains(&BigKeyD6)
        && (
          has_sword(&my_items)
          || my_items.contains(&Hammer)
          || can_shoot_arrows(&my_items)
        )
      },
      MiseryMirePrize => can_access(MiseryMireVitreous, &my_items, &world),
      PalaceOfDarknessBigKeyChest => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {5} else {4};

        count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessTheArenaLedge => can_shoot_arrows(&my_items),
      PalaceOfDarknessTheArenaBridge => {
        my_items.contains(&KeyD1)
        || (
          can_shoot_arrows(&my_items)
          && my_items.contains(&Hammer)
        )
      },
      PalaceOfDarknessBigChest => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {5} else {4};

        my_items.contains(&Lamp)
        && my_items.contains(&BigKeyD1)
        && count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessCompassChest => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {4} else {3};

        count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessHarmlessHellway => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {5} else {4};

        count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessStalfosBasement => {
        my_items.contains(&KeyD1)
        || (
          can_shoot_arrows(&my_items)
          && my_items.contains(&Hammer)
        )
      },
      PalaceOfDarknessDarkBasementLeft => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {4} else {3};

        my_items.contains(&Lamp)
        && count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessDarkBasementRight => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {4} else {3};

        my_items.contains(&Lamp)
        && count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessMapChest => can_shoot_arrows(&my_items),
      PalaceOfDarknessDarkMazeTop => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {5} else {4};

        my_items.contains(&Lamp)
        && count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessDarkMazeBottom => {
        let key_count = if my_items.contains(&Hammer) && can_shoot_arrows(&my_items)
          {5} else {4};

        my_items.contains(&Lamp)
        && count(&KeyD1, &my_items) >= key_count
      },
      PalaceOfDarknessShooterRoom => true,
      PalaceOfDarknessHelmasaurKing => {
        my_items.contains(&Hammer)
        && my_items.contains(&Lamp)
        && can_shoot_arrows(&my_items)
        && my_items.contains(&BigKeyD1)
        && count(&KeyD1, &my_items) >= 6
      },
      PalaceOfDarknessPrize => can_access(PalaceOfDarknessHelmasaurKing, &my_items, &world),
      SkullWoodsBigChest => my_items.contains(&BigKeyD3),
      SkullWoodsBigKeyChest => true,
      SkullWoodsCompassChest => true,
      SkullWoodsMapChest => true,
      SkullWoodsBridgeRoom => {
        my_items.contains(&MoonPearl)
        && my_items.contains(&FireRod)
      },
      SkullWoodsPotPrison => true,
      SkullWoodsPinballRoom => true,
      SkullWoodsMothula => {
        my_items.contains(&MoonPearl)
        && my_items.contains(&FireRod)
        && has_sword(&my_items)
        && count(&KeyD3, &my_items) >= 3
      },
      SkullWoodsPrize => {
        // TODO: not sure why this isn't just:
        //   `can_access(SkullWoodsMothula, &my_items, &world)`
        // probably something to do with `initOverworldGlitches`?
        my_items.contains(&FireRod)
        && has_sword(&my_items)
        && count(&KeyD3, &my_items) >= 3
      }
      SwampPalaceEntrance => true,
      SwampPalaceBigChest => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
        && my_items.contains(&BigKeyD2)
      },
      SwampPalaceBigKeyChest => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
      },
      SwampPalaceMapChest => my_items.contains(&KeyD2),
      SwampPalaceWestChest => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
      },
      SwampPalaceCompassChest => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
      },
      SwampPalaceFloodedRoomLeft => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
      },
      SwampPalaceFloodedRoomRight => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
      },
      SwampPalaceWaterfallRoom => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
      },
      SwampPalaceArrghus => {
        my_items.contains(&KeyD2)
        && my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
      },
      SwampPalacePrize => can_access(SwampPalaceArrghus, &my_items, &world),
      ThievesTownAttic => {
        my_items.contains(&KeyD4)
        && my_items.contains(&BigKeyD4)
      },
      ThievesTownBigKeyChest => true,
      ThievesTownMapChest => true,
      ThievesTownCompassChest => true,
      ThievesTownAmbushChest => true,
      ThievesTownBigChest => {
        my_items.contains(&Hammer)
        && my_items.contains(&KeyD4)
        && my_items.contains(&BigKeyD4)
      },
      ThievesTownBlindSCell => my_items.contains(&BigKeyD4),
      ThievesTownBlind => {
        my_items.contains(&KeyD4)
        && my_items.contains(&BigKeyD4)
        && (
          has_sword(&my_items)
          || my_items.contains(&Hammer)
          || my_items.contains(&CaneOfSomaria)
          || my_items.contains(&CaneOfByrna)
        )
      },
      ThievesTownPrize => can_access(ThievesTownBlind, &my_items, &world),
      TurtleRockChainChomps => my_items.contains(&KeyD7),
      TurtleRockCompassChest => my_items.contains(&CaneOfSomaria),
      TurtleRockRollerRoomLeft => {
        my_items.contains(&FireRod)
        && my_items.contains(&CaneOfSomaria)
      },
      TurtleRockRollerRoomRight => {
        my_items.contains(&FireRod)
        && my_items.contains(&CaneOfSomaria)
      },
      TurtleRockBigChest => {
        my_items.contains(&BigKeyD7)
        && count(&KeyD7, &my_items) >= 2
      },
      TurtleRockBigKeyChest => count(&KeyD7, &my_items) >= 2,
      TurtleRockCrystarollerRoom => {
        my_items.contains(&BigKeyD7)
        && count(&KeyD7, &my_items) >= 2
      },
      TurtleRockEyeBridgeBottomLeft
      | TurtleRockEyeBridgeBottomRight
      | TurtleRockEyeBridgeTopLeft
      | TurtleRockEyeBridgeTopRight => {
        my_items.contains(&Lamp)
        && my_items.contains(&CaneOfSomaria)
        && my_items.contains(&BigKeyD7)
        && count(&KeyD7, &my_items) >= 3
        && (
          my_items.contains(&Cape)
          || my_items.contains(&CaneOfByrna)
          || can_block_lasers(&my_items)
        )
      },
      TurtleRockTrinexx => {
        count(&KeyD7, &my_items) >= 4
        && my_items.contains(&FireRod)
        && my_items.contains(&IceRod)
        && my_items.contains(&Lamp)
        && my_items.contains(&BigKeyD7)
        && my_items.contains(&CaneOfSomaria)
        && (
          my_items.contains(&Hammer)
          || has_upgraded_sword(&my_items)
        )
      },
      TurtleRockPrize => can_access(TurtleRockTrinexx, &my_items, &world),
      GanonSTowerBobSTorch => my_items.contains(&PegasusBoots),
      GanonSTowerDMsRoomTopLeft
      | GanonSTowerDMsRoomTopRight
      | GanonSTowerDMsRoomBottomLeft
      | GanonSTowerDMsRoomBottomRight => {
        my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
      },
      GanonSTowerRandomizerRoomTopLeft
      | GanonSTowerRandomizerRoomTopRight
      | GanonSTowerRandomizerRoomBottomLeft
      | GanonSTowerRandomizerRoomBottomRight => {
        my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
        && (
          (
            item_in_locations(&BigKeyD5, vec![  // TODO: wtf why D5?? typo?
              GanonSTowerRandomizerRoomTopLeft,
              GanonSTowerRandomizerRoomTopRight,
              GanonSTowerRandomizerRoomBottomLeft,
              GanonSTowerRandomizerRoomBottomRight,
            ], &world)
            && count(&KeyA2, &my_items) >= 3
          ) || count(&KeyA2, &my_items) >= 4
        )
      },
      GanonSTowerFiresnakeRoom => {
        my_items.contains(&Hammer)
        && my_items.contains(&Hookshot)
        && (
          (
            (
              item_in_locations(&BigKeyD5, vec![  // TODO: wtf why D5?? typo?
                GanonSTowerRandomizerRoomTopLeft,
                GanonSTowerRandomizerRoomTopRight,
                GanonSTowerRandomizerRoomBottomLeft,
                GanonSTowerRandomizerRoomBottomRight,
              ], &world)
              || item_in_locations(&KeyA2, vec![GanonSTowerFiresnakeRoom], &world) // TODO: umm isn't this impossible since we haven't filled this location yet?
            ) && count(&KeyA2, &my_items) >= 2
          ) || count(&KeyA2, &my_items) >= 3
        )
      },
      GanonSTowerMapChest => {
        my_items.contains(&Hammer)
        && (
          my_items.contains(&PegasusBoots)
          || my_items.contains(&Hookshot)
        ) && count(&KeyA2, &my_items) >= 3
      },
      GanonSTowerBigChest => {
        my_items.contains(&BigKeyA2)
        && count(&KeyA2, &my_items) >= 3
        && (
          (
            my_items.contains(&Hammer)
            && my_items.contains(&Hookshot)
          ) || (
            my_items.contains(&FireRod)
            && my_items.contains(&CaneOfSomaria)
          )
        )
      },
      GanonSTowerHopeRoomLeft => true,
      GanonSTowerHopeRoomRight => true,
      GanonSTowerBobSChest => {
        (
          (
            my_items.contains(&Hammer)
            && my_items.contains(&Hookshot)
          ) || (
            my_items.contains(&FireRod)
            && my_items.contains(&CaneOfSomaria)
          )
        )
        && count(&KeyA2, &my_items) >= 3
      },
      GanonSTowerTileRoom => my_items.contains(&CaneOfSomaria),
      GanonSTowerCompassRoomTopLeft
      | GanonSTowerCompassRoomTopRight
      | GanonSTowerCompassRoomBottomLeft
      | GanonSTowerCompassRoomBottomRight => {
        my_items.contains(&FireRod)
        && my_items.contains(&CaneOfSomaria)
        && (
          (
            item_in_locations(&BigKeyD5, vec![  // TODO: wtf why D5?? typo?
              GanonSTowerCompassRoomTopLeft,
              GanonSTowerCompassRoomTopRight,
              GanonSTowerCompassRoomBottomLeft,
              GanonSTowerCompassRoomBottomRight,
            ], &world)
            && count(&KeyA2, &my_items) >= 3
          ) || count(&KeyA2, &my_items) >= 4
        )
      },
      GanonSTowerBigKeyChest => {
        (
          (
            my_items.contains(&Hammer)
            && my_items.contains(&Hookshot)
          ) || (
            my_items.contains(&FireRod)
            && my_items.contains(&CaneOfSomaria)
          )
        )
        && count(&KeyA2, &my_items) >= 3
      },
      GanonSTowerBigKeyRoomLeft
      | GanonSTowerBigKeyRoomRight => {
        (
          (
            my_items.contains(&Hammer)
            && my_items.contains(&Hookshot)
          ) || (
            my_items.contains(&FireRod)
            && my_items.contains(&CaneOfSomaria)
          )
        )
        && count(&KeyA2, &my_items) >= 3
      },
      GanonSTowerMiniHelmasaurRoomLeft
      | GanonSTowerMiniHelmasaurRoomRight => {
        can_shoot_arrows(&my_items)
        && can_light_torches(&my_items)
        && my_items.contains(&BigKeyA2)
        && count(&KeyA2, &my_items) >= 3
      },
      GanonSTowerPreMoldormChest => {
        can_shoot_arrows(&my_items)
        && can_light_torches(&my_items)
        && my_items.contains(&BigKeyA2)
        && count(&KeyA2, &my_items) >= 3
      },
      GanonSTowerMoldormChest => {
        my_items.contains(&Hookshot)
        && can_shoot_arrows(&my_items)
        && can_light_torches(&my_items)
        && my_items.contains(&BigKeyA2)
        && count(&KeyA2, &my_items) >= 4
      },
      Agahnim2 => {
        my_items.contains(&Hookshot)
        && can_shoot_arrows(&my_items)
        && can_light_torches(&my_items)
        && my_items.contains(&BigKeyA2)
        && count(&KeyA2, &my_items) >= 4
        && (
          has_sword(&my_items)
          || my_items.contains(&BugCatchingNet)
          || my_items.contains(&Hammer)
        )
      },
      WaterfallBottle => true,
      PyramidBottle => true,
      Sanctuary => true,
      SewersSecretRoomLeft
      | SewersSecretRoomMiddle
      | SewersSecretRoomRight => {
        can_lift_rocks(&my_items)
        || (
          my_items.contains(&Lamp)
          && my_items.contains(&KeyH2)
        )
      },
      SewersDarkCross => my_items.contains(&Lamp),
      HyruleCastleBoomerangChest => true,
      HyruleCastleMapChest => true,
      HyruleCastleZeldaSCell => true,
      CastleTowerRoom03 => true,
      CastleTowerDarkMaze => {
        my_items.contains(&Lamp)
        && my_items.contains(&KeyA1)
      },
      Agahnim => {
        count(&KeyA1, &my_items) >= 2
        && my_items.contains(&Lamp)
        && has_sword(&my_items)
      },
      MireShedLeft
      | MireShedRight => my_items.contains(&MoonPearl),
      Catfish => {
        my_items.contains(&MoonPearl)
        && can_lift_rocks(&my_items)
      },
      Pyramid => true,
      PyramidFairySword
      | PyramidFairyBow
      | PyramidFairyLeft
      | PyramidFairyRight => {
        has_sword(&my_items)
        && my_items.contains(&Crystal5)
        && my_items.contains(&Crystal6)
        && my_items.contains(&MoonPearl)
        && can_enter(SouthDarkWorld, &my_items, &world)
        && (
          my_items.contains(&Hammer)
          || (
            my_items.contains(&MagicMirror)
            && my_items.contains(&DefeatAgahnim)
          )
        )
      },
      Ganon => {
        my_items.contains(&MoonPearl)
        && my_items.contains(&DefeatAgahnim2)
        && can_light_torches(&my_items)
        && (
          my_items.contains(&BowAndSilverArrows)
          || (
            my_items.contains(&SilverArrowUpgrade)
            && (
              my_items.contains(&Bow)
              || my_items.contains(&BowAndArrows)
            )
          )
        ) && (
          my_items.contains(&L3Sword)
          || my_items.contains(&L4Sword)
          || count(&ProgressiveSword, &my_items) >= 3
        )
      }
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
      medallions = medallions::EntranceConfig {
        turtle_rock: *rng.choose(&all_meds).expect("empty medallion array"),
        misery_mire: *rng.choose(&all_meds).expect("empty medallion array"),
      };
    }

    let mut world;
    { // Set up assignments
      world = world::World {
        assignments: HashMap::new(),
        medallions,
      };
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
        world.assignments.insert(locations::Location::TowerOfHeraPrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::EasternPalacePrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::DesertPalacePrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::SkullWoodsPrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::ThievesTownPrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::MiseryMirePrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::SwampPalacePrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::IcePalacePrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::PalaceOfDarknessPrize, iter.next().unwrap());
        world.assignments.insert(locations::Location::TurtleRockPrize, iter.next().unwrap());
        assert!(iter.next() == None);
        // TODO: php has a weird conditional-and-`throw`; is this relevant?
        //   There aren't any restrictions, are there?
        //   e.g. something like "Turtle Rock can't have the red pendant"?
      }

      // TODO: not sure what's up in the php here;
      //   it sets waterfall fairy stuff and bow fairy stuff for some reason.
      //   It seems to set what the fairy fills your bottles with? idk

      let advancement_items_iter;
      let mut nice_items_iter;
      let mut junk_items_iter;
      let dungeon_items_iter;
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

      fill_items_in_locations(dungeon_items_iter, &randomized_order_locations, &advancement_items, &mut world);

      { // put some junk in ganon
        let num_junk_items = rng.next_u32() % 16;
        let ganon_locs: Vec<locations::Location> = regions::get_locations_for(regions::Region::GanonsTower).into_iter()
          .filter(|loc| world.assignments.get(loc) == None)
          .take(num_junk_items as usize)
          .collect();
        fast_fill_items_in_locations(&mut junk_items_iter, &ganon_locs, &mut world.assignments);
      }

      randomized_order_locations.reverse();

      fill_items_in_locations(advancement_items_iter, &randomized_order_locations, &vec![], &mut world);

      rng.shuffle(&mut randomized_order_locations);

      fast_fill_items_in_locations(&mut nice_items_iter, &randomized_order_locations, &mut world.assignments);
      assert_eq!(nice_items_iter.next(), None);
      // @hack: the php randomizes junk_items _again_ here;
      //   I'm skipping that useless step (or maybe I'm dumb?)
      fast_fill_items_in_locations(&mut junk_items_iter, &randomized_order_locations, &mut world.assignments);
      assert_eq!(junk_items_iter.next(), None);
    }

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
    world: &mut world::World,
  ) {
    let mut remaining_fill_items: Vec<items::Item> = fill_items.collect();
    for _ in 0..remaining_fill_items.len() {
      let item = remaining_fill_items.pop().expect("bad for loop sync");
      let mut assumed_items = base_assumed_items.clone();
      assumed_items.append(&mut (remaining_fill_items.clone()));
      assumed_items = collect_items(&assumed_items, &world);

      let loc = locations.iter()
        .filter(|&&loc| world.assignments.get(&loc) == None)
        .filter(|&&loc| logic::can_fill(item, loc, &assumed_items))
        .next();
      match loc {
        Some(loc) => {
          debug!("Filling {:?} with {:?}", loc, item);
          world.assignments.insert(*loc, item);
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
    world: &world::World,
  ) -> Vec<items::Item> {
    let mut my_items = assumed_items.clone();
    let mut available_locations: HashSet<locations::Location> = locations::get_all_locations()
      .into_iter()
      .filter(|&loc| world.assignments.get(&loc).is_some())
      .collect();
    loop {
      let search_locations: Vec<locations::Location> = available_locations.iter()
        .filter(|&&loc| logic::can_access(loc, &my_items, &world))
        .cloned()
        .collect();

      // available_locations -= search_locations
      available_locations = available_locations.into_iter()
        .filter(|&loc| !search_locations.contains(&loc))
        .collect();
      let mut found_items: Vec<items::Item> = search_locations.into_iter()
        .filter_map(|loc| world.assignments.get(&loc))
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

    // { // @TODO: debug code; rm
    //   for loc in vec![
    //     locations::Location::DesertPalaceBigChest,
    //     locations::Location::DesertPalaceMapChest,
    //     locations::Location::DesertPalaceTorch,
    //     locations::Location::DesertPalaceBigKeyChest,
    //     locations::Location::DesertPalaceCompassChest,
    //     locations::Location::DesertPalaceLanmolas,
    //     locations::Location::DesertPalacePrize,
    //   ] {
    //     error!("{:?}: {:?}", loc, world.assignments.get(&loc));
    //   }
    // }
  }
}
