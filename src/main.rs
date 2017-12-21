extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;

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
    OcarinaInactive, // @TODO: merge these two? php seems indifferent between them; probably just a ROM thing
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
    DefeatAgahnim,
    DefeatAgahnim2,
    DefeatGanon,
    BowAndArrows,
    L1SwordAndShield,
    L2Sword, // no idea why this is different than MasterSword
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
    Agahnim,
    Agahnim2,
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
  use super::{locations};

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

  fn has_bottle(
    my_items: &Vec<items::Item>,
  ) -> bool {
    unimplemented!() // @TODO? only used in tests
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
    my_items: &Vec<items::Item>,
    assignments: &HashMap<locations::Location, items::Item>,
  ) -> bool {
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
    assignments: &HashMap<locations::Location, items::Item>,
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
      DesertPalaceBigChest => todo,
      DesertPalaceMapChest => todo,
      DesertPalaceTorch => todo,
      DesertPalaceBigKeyChest => todo,
      DesertPalaceCompassChest => todo,
      DesertPalaceLanmolas => todo,
      DesertPalacePrize => todo,
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
        .cloned()
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
