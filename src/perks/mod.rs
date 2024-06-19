pub mod buff_perks;
pub mod exotic_armor;
pub mod exotic_perks;
pub mod lib;
pub mod meta_perks;
pub mod origin_perks;
pub mod other_perks;
pub mod perk_options_handler;
pub mod year_1_perks;
pub mod year_2_perks;
pub mod year_3_perks;
pub mod year_4_perks;
pub mod year_5_perks;
pub mod year_6_perks;
pub mod year_7_perks;

use std::borrow::BorrowMut;
use std::collections::HashMap;

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::d2_enums::{BungieHash, StatBump, StatHashes, WeaponType};
use crate::database;

use self::{
    buff_perks::*,
    exotic_armor::*,
    exotic_perks::*,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, FlinchModifierResponse, HandlingModifierResponse,
        InventoryModifierResponse, MagazineModifierResponse, ModifierResponseSummary,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
        VelocityModifierResponse,
    },
    meta_perks::*,
    origin_perks::*,
    other_perks::*,
    year_1_perks::*,
    year_2_perks::*,
    year_3_perks::*,
    year_4_perks::*,
    year_5_perks::*,
    year_6_perks::*,
    year_7_perks::*,
};

use crate::weapons::Weapon;

pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Perk {
    pub stat_buffs: HashMap<u32, i32>,
    pub enhanced: bool,
    pub value: u32, //used for toggle and stacks
    pub hash: u32,
    pub raw_hash: u32,
}

pub fn enhanced_check(_hash: u32) -> (u32, bool) {
    let mut result = _hash;
    let mut found = false;
    for (_, (h, r)) in database::ENHANCE_PERK_MAPPING.iter().enumerate() {
        if _hash == *h {
            result = *r;
            found = true;
            break;
        }
    }
    (result, found)
}

// all armor pekrs are for the future but wanted to started to compile them now

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Perks {
    //Meta perks
    BuiltIn = 0,
    RallyBarricade = 444,
    EmpRift = 555,

    Radiant = 1380009033,
    Weaken = 1464159054,
    Sever = 2519102437,
    WellOfRadiance = 2274196887,
    WardOfDawn = 4260353953,
    BannerShield = 4260353952,
    DeadFall = 2722573683,
    MoebiusQuiver = 2722573681,

    //intrinsics
    RapidFireFrame = 902,
    PrecisionFrame = 906,
    SupportFrame = 912, // 2622129339

    //armor
    DexterityMod = 1001,
    TargetingMod = 1002,
    ReserveMod = 1003,
    LoaderMod = 1004,
    UnflinchingMod = 1005,
    SurgeMod = 1006,
    LucentBlades = 531057500,
    DragonShadow = 593361144,
    OphidianAspect = 1147638875,
    LunaFaction = 3347978672,
    TomeOfDawn = 926349160,
    PathOfTheBurningSteps = 2500502982,
    KnuckleheadRadar = 2663272111,
    Foetracers = 2663272109,
    MechaneersTricksleeves = 481860151,
    Oathkeeper = 1449897496,
    SealedAhamkaraGrasps = 2805134531,
    LuckyPants = 1694242448,
    NoBackupPlans = 569260333,
    ActiumWarRig = 1667892711,
    HallowfireHeart = 1667892708,
    LionRampart = 3241194940,
    Peacekeepers = 3241194941,
    PeregrineGreaves = 235075862,
    EyeOfAnotherWorld = 3927963100,
    AstrocyteVerse = 3295796664,
    NecroticGrips = 3824622015,
    BootsOfTheAssembler = 902934539,
    RainOfFire = 4222205045,
    SpeedloaderSlacks = 858592012,
    MantleOfBattleHarmony = 2618534366,
    MaskOfBakris = 692285813,
    BallindorseWrathweavers = 2894608781,
    Gyrfalcon = 3809192347,
    AeonInsight = 3651607301,
    Felwinters = 622433369,
    SanguineAlchemy = 1420473289,
    EternalWarrior = 926976844,
    TritonVice = 187957397,
    GlacialGuard = 185514250,

    //parts
    ImpactCasing = 3796465595,
    SwapMag = 3721627275,
    FullChoke = 1047830412,
    SpikeGrenades = 3301904089,
    AlloyMag = 1431678320,
    LiquidCoils = 1687452232,
    AcceleratedCoils = 689005463,
    ChargetimeMW = 3128594062,
    DisorientingGrenades = 3032599245,
    AssaultMag = 791862061,
    PhaseMag = 830282363,
    //bow strings
    SlowerStringT1 = 3371775011,
    FasterStringT2 = 2801223209,
    #[num_enum(alternatives = [
        1639384016,
        4067834857,
        852209214,
    ])]
    FasterStringT1 = 1885045197,

    //mods
    QuickAccessSling = 1334978104,
    BossSpec = 2788909693,
    MajorSpec = 984527513,
    MinorSpec = 4091000557,
    BigOnesSpec = 3018373291,
    TakenSpec = 1513326571,
    AdeptChargeTime = 744770875,
    FreehandGrip = 736000386,
    UmbralSharpening = 2804214704,
    EnhancedScannerAugment = 1578165808,

    //origin | year 5+
    VeistStinger = 3988215619,
    HakkeBreach = 1607056502,
    Alacrity = 2988596335,
    FluidDynamics = 2839173408,
    QuietMoment = 4091460919,
    SurosSynergy = 4008116374,
    BitterSpite = 4154828211,
    RunnethOver = 120721526,
    HotSwap = 1260401931,
    RightHook = 3907865655,
    Ambush = 192157151,
    TexBalancedStock = 2437618208,
    SearchParty = 2250679103,
    FieldTested = 2120661319,
    HarmonicResonance = 3186078099,
    NobleDeeds = 1823011807,

    //season 1 | year 1
    KillClip = 1015611457,
    #[num_enum(alternatives = [
        1528281896, //rose
        3124871000, //redrix
        1266037487, //R0
        1931066670, //necrochasm
    ])]
    Outlaw = 1168162263,
    BackupPlan = 1600092898,
    FieldPrep = 2869569095,
    #[num_enum(alternatives = [3551326236, ])] //huckleberry
    Rampage = 3425386926,
    OpeningShot = 47981717,
    MovingTarget = 588594999,
    AmbitiousAssassin = 2010801679,
    ClusterBomb = 1275731761,
    #[num_enum(alternatives = [2907129557, ])] //sunshot
    ExplosivePayload = 3038247973,
    FirmlyPlanted = 280464955,
    FullAutoTrigger = 2117683199,
    HeadSeeker = 460017080,
    HighImpactReserves = 2213355989,
    HipFireGrip = 1866048759,
    Snapshot = 957782887,
    TapTheTrigger = 1890422124,
    SlideWays = 2039302152,
    QuickDraw = 706527188,
    TimedPayload = 1954620775,
    ThreatDetector = 4071163871,
    SlideShot = 3161816588,
    #[num_enum(alternatives = [1409312565, ])] //cloudstrike
    TripleTap = 3400784728,
    UnderPressure = 1645158859,
    PulseMonitor = 972757866,

    //season 2 | year 1
    //lmao bozo

    //season 3 | year 1
    RangeFinder = 2846385770,
    #[num_enum(alternatives = [1683379515, ])] //Arbalest
    DisruptionBreak = 3871884143,
    #[num_enum(alternatives = [
        806159697,  // s3 barrel
        2079505485, // acrius old cat
        4233905577, // acrius dummy cat
        456628588,  // acrius cat
    ])]
    TrenchBarrel = 2360754333,
    Desperado = 3047969693,
    BoxBreathing = 2551157718,

    //season 4 | year 2
    #[num_enum(alternatives = [2362217257, ])] //levi cat
    ArchersTempo = 201365942,
    ExplosiveHead = 3365897133,
    #[num_enum(alternatives = [1266037485, ])] //R0
    FeedingFrenzy = 2779035018,
    #[num_enum(alternatives = [1266037486, ])] //R0
    FourthTimesTheCharm = 1354429876,
    #[num_enum(alternatives = [1557274655, 2578928835, ])]
    //touch of malice cat and crafted outbreak
    RapidHit = 247725512,

    //season 5 | year 2
    ResevoirBurst = 1427256713,
    Surrounded = 3708227201,
    AirAssault = 3722653512,
    OverFlow = 3643424744,

    //season 6 | year 2
    FiringLine = 1771339417,
    FullCourt = 2888557110,
    #[num_enum(alternatives = [557221067, ])] // delirium
    KillingTally = 2782457288,
    Demolitionist = 3523296417,
    MultikillClip = 2458213969,
    Swashbuckler = 4082225868,

    //season 7 | year 2
    UnderDog = 205890336,
    ExplosiveLight = 3194351027,
    EyeOfTheStorm = 699525795,
    #[num_enum(alternatives = [
        2162261876, // mida old cat
        800074992,  // mida dummy cat
        3466057365, // mida cat
    ])]
    NoDistractions = 2866798147,
    ArchersGambit = 3414324643,

    //season 8 | year 3
    //TODO

    //season 9 | year 3
    ClownCartridge = 2284787283,
    ElementalCapacitor = 3511092054,
    #[num_enum(alternatives = [
        3547298847, // grav lance old cat, to be removed after UI update
        1620506139, // grav lance cat
        1234111636, // malf cat
    ])]
    Vorpal = 1546637391,

    //season 10 | year 3
    //bad season lmao

    //season 11 | year 3
    KillingWind = 2450788523,

    //season 12 | year 4
    DualLoader = 25606670,
    OneForAll = 4049631843,
    Recombination = 469285294,
    Reconstruction = 1523832109,
    #[num_enum(alternatives = [3967134106, ])]
    Surplus = 3436462433,

    //season 13 | year 4
    ImpulseAmplifier = 951095735,
    Frenzy = 4104185692,
    LastingImpression = 3927722942,
    KickStart = 1754714824,

    //season 14 | year 4
    Cornered = 1799762209,
    AdrenalineJunkie = 11612903,
    RewindRounds = 3418782618,
    HeatingUp = 1570042021,
    FireFly = 3824105627,
    DangerZone = 960810156,
    TunnelVision = 2946784966,

    //season 15 | year 4
    Encore = 1195158366,
    #[num_enum(alternatives = [
        615063267, //V-wing
    ])]
    Ensemble = 2621346526,
    GoldenTricorn = 2610012052,
    Harmony = 438098033,
    PerpetualMotion = 1428297954,
    Adagio = 3673922083,
    BluntExecutionRounds = 454085387,

    //season 16 | year 5
    BaitAndSwitch = 3078487919,
    CompulsiveReloader = 671806388,
    FocusedFury = 2896038713,
    ChillClip = 2978966579,
    SleightOfHand = 2172504645,
    StatsForAll = 1583705720,
    SteadyHands = 509074078,
    SuccesfulWarmup = 2652708987,
    UnstoppableForce = 2224838837,

    //season 17 | year 5
    FragileFocus = 2451262963,
    WellRounded = 744594675,

    //season 18 | year 5
    GutShot = 1365187766,
    Pugilist = 691659142,
    Slickdraw = 1821614984,
    UnderOver = 1870851715,

    //season 19 | year 5
    CascadePoint = 3751912585,
    CloseToMelee = 1782407750,
    OffhandStrike = 2416023159,
    PerfectFloat = 2272927194,
    ShotSwap = 2586829431,
    TargetLock = 365154968,

    //season 20 | year 6
    KeepAway = 3619207468,
    ParacausalAffinity = 3215448563,
    EnviousAssassin = 968510818,

    //season 21 | year 6
    CollectiveAction = 3324494224,
    Bipod = 1439600632,
    ControlledBurst = 2594592626,
    InvisibleHand = 3018146897,
    UnsatedHunger = 2053642371,
    Discord = 3978468247,
    #[num_enum(alternatives = [4218954970, ])]
    EddyCurrent = 684456054,

    //seaqson 22 | year 6
    PrecisionInstrument = 555281244,
    LooseChange = 1119449540,
    HighGround = 2319119708,
    HeadRush = 2565067140,
    EnlightendAction = 3828510309,
    SwordLogic = 31345821,

    //season 23 | year 6
    Onslaught = 95528736,
    DesperateMeasures = 2109543898,
    MasterOfArms = 3081867624,

    //episode 1 | year 7
    ChaosReshaped = 3640170453,
    CircleOfLife = 2298656195,

    //subclass
    OnYourMark = 3066103999,
    Hedrons = 3469412970,
    FlowState = 4194622036,
    Frequency = 1727069361,
    HeatRises = 83039194,
    Tempering = 362132290,
    ThreadOfAscent = 4208512216,
    Amplified = 880704824,

    //kinetic exotic
    CranialSpike = 1319823571,
    #[num_enum(alternatives = [499501121, ])] //s22 variant
    DarkForgedTrigger = 1301843770,
    AgersCall = 3164944314,
    RatPack = 2121086290,
    StringofCurses = 4004944400,
    RocketTracers = 3602718766,
    ParacausalShot = 213689231,
    TimeSlip = 3556949035,
    ToM = 2724693746,
    CorruptionSpreads = 4208418110,
    HonedEdge = 1070100196,
    HakkeHeavyBurst = 2206869417,
    FateOfAllFools = 3668782036,
    ExplosiveShadow = 1791592647,
    TakenPredator = 2130042297,
    SurosLegacy = 944506345,
    SpinningUp = 1378047685,
    DualSpeedReceiver = 4012962526,
    FullStop = 2984682260,
    RideTheBull = 630329983,
    HuntersTrance = 383825919,
    NobleRounds = 2144092201,
    StormAndStress = 2238035098,
    Roadborn = 1658733671,
    MarkovChain = 2814973067,
    MementoMori = 647617635,
    AgersScepterCatalyst = 970163821,
    Broadhead = 2287699930,
    HuntersTrace = 891750160,
    Desperation = 525593296,

    //energy exotic
    LagragianSight = 2881100038,
    IgnitionTrigger = 961505134,
    GuidanceRing = 2226793914,
    FirstGlance = 3174300811,
    ConserveMomentum = 656200654,
    Broadside = 407549716,
    FourthHorsemanCatalyst = 2826187530,
    Impetus = 2333607307,
    Stormbringer = 3117514172,
    PerfectFith = 1000724343,
    PrismaticInferno = 571267712,
    CalculatedBalance = 838219733,
    Chimera = 924149234,
    ThinTheHerd = 2003108620,
    RavenousBeast = 2540536653,
    LordOfWolvesCatalyst = 431220296,
    ReleaseTheWolves = 299272945,
    SwoopingTalons = 2656694271,
    #[num_enum(alternatives = [
        3081173348, //borealis
    ])]
    Fundamentals = 2620589274,
    HarmonicLaser = 459441288,
    ColdFusion = 1036269296,
    BlackHole = 3905543891,
    TemporalUnlimiter = 806917387,
    IonicReturn = 501717180,
    Unrepentant = 2641107734,
    ArcConductor = 2516532331,
    VoidLeech = 3441203855,
    InverseRelationship = 1833111001,
    Spindle = 1180907940,
    TheRightChoice = 34498892,

    //heavy exotic
    ReignHavoc = 4148158229,
    WormsHunger = 1174163613,
    WormByproduct = 1679262379,
    WhisperCatalyst = 1340292993,
    DarkDescent = 3333994164,
    TargetAquired = 939227542,
    TractorCannon = 1210807262,
    MarksmanSights = 1408087975,

    #[num_enum(default)]
    Ignore = 69420,
}

#[derive(Debug)]
pub struct ModifierResponseInput<'a> {
    calc_data: &'a CalculationInput<'a>,
    value: u32,
    is_enhanced: bool,
    pvp: bool,
    cached_data: &'a mut HashMap<String, f64>,
}
type ModifierFunction<T> = Box<dyn Fn(ModifierResponseInput) -> T>;
type StatMap = HashMap<BungieHash, StatBump>;
type ModifierMap<T> = HashMap<Perks, ModifierFunction<T>>;

#[derive(Default)]
pub struct PersistentModifierResponses {
    pub sbr: ModifierMap<StatMap>,
    pub dmr: ModifierMap<DamageModifierResponse>,
    pub hmr: ModifierMap<HandlingModifierResponse>,
    pub rmr: ModifierMap<RangeModifierResponse>,
    pub rsmr: ModifierMap<ReloadModifierResponse>,
    pub fmr: ModifierMap<FiringModifierResponse>,
    pub flmr: ModifierMap<FlinchModifierResponse>,
    pub edr: ModifierMap<ExtraDamageResponse>,
    pub rr: ModifierMap<RefundResponse>,
    pub vmr: ModifierMap<VelocityModifierResponse>,
    pub epr: ModifierMap<ExplosivePercentResponse>,
    pub mmr: ModifierMap<MagazineModifierResponse>,
    pub imr: ModifierMap<InventoryModifierResponse>,
}
impl PersistentModifierResponses {
    fn is_empty(&self) -> bool {
        self.sbr.is_empty()
    }
}

thread_local! {
    static PERK_FUNC_MAP: std::cell::RefCell<PersistentModifierResponses>  = std::cell::RefCell::new(PersistentModifierResponses::default());
}

pub fn map_perks() {
    let is_empty = PERK_FUNC_MAP.with(|p| p.borrow().is_empty());
    if is_empty {
        year_1_perks();
        year_2_perks();
        year_3_perks();
        year_4_perks();
        year_5_perks();
        year_6_perks();
        year_7_perks();
        meta_perks();
        exotic_perks();
        exotic_armor();
        buff_perks();
        other_perks();
        origin_perks();
    }
}

impl PersistentModifierResponses {
    fn get_sbr(&self, perk: Perks, input: ModifierResponseInput) -> HashMap<BungieHash, StatBump> {
        if let Some(func) = self.sbr.get(&perk) {
            func(input)
        } else {
            HashMap::new()
        }
    }
    fn get_dmr(&self, perk: Perks, input: ModifierResponseInput) -> DamageModifierResponse {
        if let Some(func) = self.dmr.get(&perk) {
            func(input)
        } else {
            DamageModifierResponse::default()
        }
    }
    fn get_hmr(&self, perk: Perks, input: ModifierResponseInput) -> HandlingModifierResponse {
        if let Some(func) = self.hmr.get(&perk) {
            func(input)
        } else {
            HandlingModifierResponse::default()
        }
    }
    fn get_rmr(&self, perk: Perks, input: ModifierResponseInput) -> RangeModifierResponse {
        if let Some(func) = self.rmr.get(&perk) {
            func(input)
        } else {
            RangeModifierResponse::default()
        }
    }
    fn get_rsmr(&self, perk: Perks, input: ModifierResponseInput) -> ReloadModifierResponse {
        if let Some(func) = self.rsmr.get(&perk) {
            func(input)
        } else {
            ReloadModifierResponse::default()
        }
    }
    fn get_fmr(&self, perk: Perks, input: ModifierResponseInput) -> FiringModifierResponse {
        if let Some(func) = self.fmr.get(&perk) {
            func(input)
        } else {
            FiringModifierResponse::default()
        }
    }
    fn get_flmr(&self, perk: Perks, input: ModifierResponseInput) -> FlinchModifierResponse {
        if let Some(func) = self.flmr.get(&perk) {
            func(input)
        } else {
            FlinchModifierResponse::default()
        }
    }
    fn get_edr(&self, perk: Perks, input: ModifierResponseInput) -> ExtraDamageResponse {
        if let Some(func) = self.edr.get(&perk) {
            func(input)
        } else {
            ExtraDamageResponse::default()
        }
    }
    fn get_rr(&self, perk: Perks, input: ModifierResponseInput) -> RefundResponse {
        if let Some(func) = self.rr.get(&perk) {
            func(input)
        } else {
            RefundResponse::default()
        }
    }
    fn get_vmr(&self, perk: Perks, input: ModifierResponseInput) -> VelocityModifierResponse {
        if let Some(func) = self.vmr.get(&perk) {
            func(input)
        } else {
            VelocityModifierResponse::default()
        }
    }
    fn get_epr(&self, perk: Perks, input: ModifierResponseInput) -> ExplosivePercentResponse {
        if let Some(func) = self.epr.get(&perk) {
            func(input)
        } else {
            ExplosivePercentResponse::default()
        }
    }
    fn get_mmr(&self, perk: Perks, input: ModifierResponseInput) -> MagazineModifierResponse {
        if let Some(func) = self.mmr.get(&perk) {
            func(input)
        } else {
            MagazineModifierResponse::default()
        }
    }
    fn get_imr(&self, perk: Perks, input: ModifierResponseInput) -> InventoryModifierResponse {
        if let Some(func) = self.imr.get(&perk) {
            func(input)
        } else {
            InventoryModifierResponse::default()
        }
    }
}

fn add_sbr(perk: Perks, func: ModifierFunction<StatMap>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().sbr.insert(perk, func);
    });
}
fn add_dmr(perk: Perks, func: ModifierFunction<DamageModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().dmr.insert(perk, func);
    });
}
fn add_hmr(perk: Perks, func: ModifierFunction<HandlingModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().hmr.insert(perk, func);
    });
}
fn add_rmr(perk: Perks, func: ModifierFunction<RangeModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().rmr.insert(perk, func);
    });
}
fn add_rsmr(perk: Perks, func: ModifierFunction<ReloadModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().rsmr.insert(perk, func);
    });
}
fn add_fmr(perk: Perks, func: ModifierFunction<FiringModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().fmr.insert(perk, func);
    });
}
fn add_flmr(perk: Perks, func: ModifierFunction<FlinchModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().flmr.insert(perk, func);
    });
}
fn add_edr(perk: Perks, func: ModifierFunction<ExtraDamageResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().edr.insert(perk, func);
    });
}
fn add_rr(perk: Perks, func: ModifierFunction<RefundResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().rr.insert(perk, func);
    });
}
fn add_vmr(perk: Perks, func: ModifierFunction<VelocityModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().vmr.insert(perk, func);
    });
}
fn add_epr(perk: Perks, func: ModifierFunction<ExplosivePercentResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().epr.insert(perk, func);
    });
}
fn add_mmr(perk: Perks, func: ModifierFunction<MagazineModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().mmr.insert(perk, func);
    });
}
fn add_imr(perk: Perks, func: ModifierFunction<InventoryModifierResponse>) {
    PERK_FUNC_MAP.with(|map| {
        map.borrow_mut().imr.insert(perk, func);
    });
}

pub fn get_stat_bumps(
    _perks: Vec<Perk>,
    _input_data: CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> [HashMap<u32, i32>; 2] {
    let mut dynamic_stats: HashMap<u32, i32> = HashMap::new();
    let mut static_stats: HashMap<u32, i32> = HashMap::new();
    for perk in _perks {
        let perk_stats = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: &_input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_sbr(perk.hash.into(), inp)
        });
        for (key, value) in perk_stats {
            let entry = dynamic_stats.entry(key).or_insert(0);
            *entry += value;
        }
        for (key, value) in perk.stat_buffs {
            let entry = static_stats.entry(key).or_insert(0);
            *entry += value;
        }
    }
    [dynamic_stats, static_stats]
}

pub fn get_dmg_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut dmg_modifier = DamageModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_dmr(perk.hash.into(), inp)
        });
        dmg_modifier.impact_dmg_scale *= tmp.impact_dmg_scale;
        dmg_modifier.explosive_dmg_scale *= tmp.explosive_dmg_scale;
        dmg_modifier.crit_scale *= tmp.crit_scale;
    }
    dmg_modifier
}

pub fn get_reload_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let mut reload_modifier = ReloadModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_rsmr(perk.hash.into(), inp)
        });
        reload_modifier.reload_stat_add += tmp.reload_stat_add;
        reload_modifier.reload_time_scale *= tmp.reload_time_scale;
    }
    reload_modifier
}

pub fn get_firing_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    let mut firing_modifier = FiringModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_fmr(perk.hash.into(), inp)
        });
        firing_modifier.burst_delay_scale *= tmp.burst_delay_scale;
        firing_modifier.burst_delay_add += tmp.burst_delay_add;
        firing_modifier.inner_burst_scale *= tmp.inner_burst_scale;
        firing_modifier.burst_size_add += tmp.burst_size_add;
    }
    firing_modifier
}

pub fn get_handling_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HandlingModifierResponse {
    let mut handling_modifier = HandlingModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_hmr(perk.hash.into(), inp)
        });
        handling_modifier.stat_add += tmp.stat_add;
        handling_modifier.stow_add += tmp.stow_add;
        handling_modifier.draw_add += tmp.draw_add;
        handling_modifier.ads_add += tmp.ads_add;
        handling_modifier.stow_scale *= tmp.stow_scale;
        handling_modifier.draw_scale *= tmp.draw_scale;
        handling_modifier.ads_scale *= tmp.ads_scale;
    }
    handling_modifier
}

pub fn get_magazine_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut magazine_modifier = MagazineModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_mmr(perk.hash.into(), inp)
        });
        magazine_modifier.magazine_stat_add += tmp.magazine_stat_add;
        magazine_modifier.magazine_add += tmp.magazine_add;
        magazine_modifier.magazine_scale *= tmp.magazine_scale;
    }
    magazine_modifier
}

pub fn get_reserve_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> InventoryModifierResponse {
    let mut reserve_modifier = InventoryModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_imr(perk.hash.into(), inp)
        });
        reserve_modifier.inv_stat_add += tmp.inv_stat_add;
        reserve_modifier.inv_add += tmp.inv_add;
        reserve_modifier.inv_scale *= tmp.inv_scale;
    }
    reserve_modifier
}

pub fn get_range_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RangeModifierResponse {
    let mut range_modifier = RangeModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_rmr(perk.hash.into(), inp)
        });
        range_modifier.range_stat_add += tmp.range_stat_add;
        range_modifier.range_all_scale *= tmp.range_all_scale;
        range_modifier.range_hip_scale *= tmp.range_hip_scale;
        range_modifier.range_zoom_scale *= tmp.range_zoom_scale;
    }
    range_modifier
}

pub fn get_refund_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<RefundResponse> {
    let mut refund_modifier = vec![];
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_rr(perk.hash.into(), inp)
        });
        if tmp.requirement > 0 {
            refund_modifier.push(tmp);
        }
    }
    refund_modifier
}

pub fn get_extra_damage(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> Vec<ExtraDamageResponse> {
    let mut extra_damage = vec![];
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_edr(perk.hash.into(), inp)
        });
        if tmp.additive_damage > 0.0 {
            extra_damage.push(tmp);
        }
    }
    extra_damage
}

// pub fn get_reload_overrides(
//     _perks: Vec<Perk>,
//     _input_data: &CalculationInput,
//     _pvp: bool,
//     _cached_data: &mut HashMap<String, f64>,
// ) -> Vec<ReloadOverrideResponse> {
//     let mut reload_overrides = vec![];
//     for perk in _perks {
//         let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
//             let inp = ModifierResponsInput {
//                 is_enhanced: perk.enhanced,
//                 value: perk.value,
//                 calc_data: &_input_data,
//                 pvp: _pvp,
//                 cached_data: _cached_data,
//             };
//             pers_modifier.borrow().get_ror(perk.hash.into(), inp)
//         });
//         if tmp.valid {
//             reload_overrides.push(tmp);
//         }
//     }
//     reload_overrides
// }

pub fn get_explosion_data(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
) -> ExplosivePercentResponse {
    let mut highest_so_far = ExplosivePercentResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: &mut HashMap::new(),
            };
            pers_modifier.borrow().get_epr(perk.hash.into(), inp)
        });
        if tmp.percent > highest_so_far.percent {
            highest_so_far = tmp;
        }
    }
    highest_so_far
}

pub fn get_flinch_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FlinchModifierResponse {
    let mut flinch = FlinchModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_flmr(perk.hash.into(), inp)
        });
        flinch.flinch_scale *= tmp.flinch_scale;
    }
    flinch
}

pub fn get_velocity_modifier(
    _perks: Vec<Perk>,
    _input_data: &CalculationInput,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> VelocityModifierResponse {
    let mut velocity = VelocityModifierResponse::default();
    for perk in _perks {
        let tmp = PERK_FUNC_MAP.with(|pers_modifier| {
            let inp = ModifierResponseInput {
                is_enhanced: perk.enhanced,
                value: perk.value,
                calc_data: _input_data,
                pvp: _pvp,
                cached_data: _cached_data,
            };
            pers_modifier.borrow().get_vmr(perk.hash.into(), inp)
        });
        velocity.velocity_scaler *= tmp.velocity_scaler;
    }
    velocity
}

impl Weapon {
    pub fn get_modifier_summary(
        &self,
        _calc_input: Option<CalculationInput>,
        _pvp: bool,
        _cached_data: Option<&mut HashMap<String, f64>>,
    ) -> HashMap<BungieHash, ModifierResponseSummary> {
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);
        let mut buffer: HashMap<u32, ModifierResponseSummary> = HashMap::new();
        if _calc_input.is_none() {
            return buffer;
        }

        let calc_input = _calc_input.unwrap();

        for perk in self.list_perks() {
            let mod_buffer = PERK_FUNC_MAP.with(|pers_modifier| {
                let perk_modifiers = pers_modifier.borrow();
                let mut mod_response = ModifierResponseSummary::default();

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_rmr(perk.hash.into(), inp);
                if modifier != RangeModifierResponse::default() {
                    mod_response.rmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_dmr(perk.hash.into(), inp);
                if modifier != DamageModifierResponse::default() {
                    mod_response.dmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_hmr(perk.hash.into(), inp);
                if modifier != HandlingModifierResponse::default() {
                    mod_response.hmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_fmr(perk.hash.into(), inp);
                if modifier != FiringModifierResponse::default() {
                    mod_response.fmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_flmr(perk.hash.into(), inp);
                if modifier != FlinchModifierResponse::default() {
                    mod_response.flmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_rsmr(perk.hash.into(), inp);
                if modifier != ReloadModifierResponse::default() {
                    mod_response.rsmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_mmr(perk.hash.into(), inp);
                if modifier != MagazineModifierResponse::default() {
                    mod_response.mmr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let modifier = perk_modifiers.get_imr(perk.hash.into(), inp);
                if modifier != InventoryModifierResponse::default() {
                    mod_response.imr = Some(modifier);
                }

                let inp = ModifierResponseInput {
                    is_enhanced: perk.enhanced,
                    value: perk.value,
                    calc_data: &calc_input,
                    pvp: _pvp,
                    cached_data,
                };
                let stat_mod = perk_modifiers.get_sbr(perk.hash.into(), inp);
                let mut stat_buffer: HashMap<BungieHash, StatBump> = HashMap::new();
                for (key, value) in stat_mod {
                    stat_buffer.insert(key, value);
                }

                for (key, value) in perk.stat_buffs {
                    stat_buffer
                        .entry(key)
                        .and_modify(|stat| *stat += value)
                        .or_insert(value);
                }
                mod_response.statbump = Some(stat_buffer);
                mod_response
            });
            buffer.insert(perk.raw_hash, mod_buffer);
        }

        buffer
    }
}
