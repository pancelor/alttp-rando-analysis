mod Items {
  #[allow(dead_code)]
  #[derive(Copy, Clone, Debug)]
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
  }

  pub fn getAdvancementItems() -> Vec<Item> {
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

  pub fn getNiceItems() -> Vec<Item> {
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

  pub fn getItemPool() -> Vec<Item> {
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

  pub fn getDungeonPool() -> Vec<Item> {
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

mod Medallions {
  #[allow(dead_code)]
  pub enum MedallionGoal {
    Bombos,
    Ether,
    Quake,
  }

  pub fn getAllMedallionGoals() -> Vec<MedallionGoal> {
    vec![
      MedallionGoal::Bombos,
      MedallionGoal::Ether,
      MedallionGoal::Quake,
    ]
  }

  pub fn setMedallions() {
    // take in a world
    unimplemented!();
  }
}

mod Location {
  #[allow(dead_code)]
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
    TurtleRockMedallion,
    MiseryMireMedallion,
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
}

fn main() {
  let advancement_items = Items::getAdvancementItems();
  println!("advancement_items: {:?}", advancement_items);

  let nice_items = Items::getNiceItems();
  println!("nice_items: {:?}", nice_items);

  let junk_items = Items::getItemPool();
  println!("item_pool: {:?}", junk_items);

  let dungeon_items = Items::getDungeonPool();
  println!("dungeon_items: {:?}", dungeon_items);
}
