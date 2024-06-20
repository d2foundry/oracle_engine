use std::collections::HashMap;

use serde::Serialize;

use super::{enhanced_check, Perk, Perks};

#[derive(Debug, Clone, Serialize, Default)]
pub enum PerkValueVariant {
    #[default]
    STATIC,
    TOGGLE,
    SLIDER,
    OPTIONS,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerkOptionData {
    stacks: (u32, u32),
    options: Vec<String>,
    #[serde(rename = "optionType")]
    option_type: PerkValueVariant,
}
impl PerkOptionData {
    pub fn static_() -> PerkOptionData {
        PerkOptionData {
            stacks: (0, 0),
            options: vec![],
            option_type: PerkValueVariant::STATIC,
        }
    }
    pub fn toggle() -> PerkOptionData {
        PerkOptionData {
            stacks: (0, 1),
            options: vec![],
            option_type: PerkValueVariant::TOGGLE,
        }
    }
    pub fn stacking(_stacks: u32) -> PerkOptionData {
        PerkOptionData {
            stacks: (0, _stacks),
            options: vec![],
            option_type: PerkValueVariant::SLIDER,
        }
    }
    pub fn stacking_min(_stacks: u32, _min_stacks: u32) -> PerkOptionData {
        PerkOptionData {
            stacks: (_min_stacks, _stacks),
            options: vec![],
            option_type: PerkValueVariant::SLIDER,
        }
    }
    pub fn options(_options: Vec<&str>) -> PerkOptionData {
        let mut options = vec!["None".to_string()];
        for option in _options {
            options.push(option.to_string());
        }
        PerkOptionData {
            stacks: (0, options.len() as u32 - 1),
            options,
            option_type: PerkValueVariant::OPTIONS,
        }
    }
    pub fn options_raw(_options: Vec<&str>) -> PerkOptionData {
        let mut options = vec![];
        for option in _options {
            options.push(option.to_string());
        }
        PerkOptionData {
            stacks: (0, options.len() as u32 - 1),
            options,
            option_type: PerkValueVariant::OPTIONS,
        }
    }
}

fn hash_to_perk_option_data(_hash: u32) -> Option<PerkOptionData> {
    let perk: Perks = enhanced_check(_hash).0.into();
    match perk {
        //Meta perks
        Perks::BuiltIn => None,
        Perks::RallyBarricade => Some(PerkOptionData::static_()),
        Perks::EmpRift => Some(PerkOptionData::static_()),

        //intrinsics
        Perks::RapidFireFrame => Some(PerkOptionData::toggle()),
        Perks::PrecisionFrame => Some(PerkOptionData::static_()),
        Perks::SupportFrame => Some(PerkOptionData::toggle()),

        //armor

        //parts
        Perks::ImpactCasing => Some(PerkOptionData::static_()),
        Perks::SwapMag => Some(PerkOptionData::static_()),
        Perks::FullChoke => Some(PerkOptionData::static_()),
        Perks::SpikeGrenades => Some(PerkOptionData::static_()),
        Perks::AlloyMag => Some(PerkOptionData::toggle()),
        Perks::LiquidCoils => Some(PerkOptionData::static_()),
        Perks::AcceleratedCoils => Some(PerkOptionData::static_()),
        Perks::ChargetimeMW => Some(PerkOptionData::static_()),
        Perks::DisorientingGrenades => Some(PerkOptionData::static_()),
        Perks::AssaultMag => Some(PerkOptionData::static_()),
        Perks::AdeptChargeTime => Some(PerkOptionData::static_()),
        Perks::PhaseMag => Some(PerkOptionData::static_()),
        //bow strings
        Perks::SlowerStringT1 => Some(PerkOptionData::static_()),
        Perks::FasterStringT1 => Some(PerkOptionData::static_()),
        Perks::FasterStringT2 => Some(PerkOptionData::static_()),

        //mods
        Perks::QuickAccessSling => Some(PerkOptionData::static_()),
        Perks::BossSpec => Some(PerkOptionData::static_()),
        Perks::MajorSpec => Some(PerkOptionData::static_()),
        Perks::MinorSpec => Some(PerkOptionData::static_()),
        Perks::BigOnesSpec => Some(PerkOptionData::static_()),
        Perks::TakenSpec => Some(PerkOptionData::toggle()),
        Perks::FreehandGrip => Some(PerkOptionData::static_()),

        //origin | year 5+
        Perks::VeistStinger => Some(PerkOptionData::toggle()),
        Perks::HakkeBreach => Some(PerkOptionData::options(
            ["Vehicle", "Stasis Crystals", "Other Constructs"].to_vec(),
        )),
        Perks::Alacrity => Some(PerkOptionData::toggle()),
        Perks::FluidDynamics => Some(PerkOptionData::toggle()),
        Perks::QuietMoment => Some(PerkOptionData::toggle()),
        Perks::SurosSynergy => Some(PerkOptionData::toggle()),
        Perks::BitterSpite => Some(PerkOptionData::stacking(5)),
        Perks::RunnethOver => Some(PerkOptionData::stacking(5)),
        Perks::HotSwap => Some(PerkOptionData::toggle()),
        Perks::RightHook => Some(PerkOptionData::toggle()),
        Perks::Ambush => Some(PerkOptionData::toggle()),
        Perks::TexBalancedStock => Some(PerkOptionData::toggle()),
        Perks::SearchParty => Some(PerkOptionData::toggle()),
        Perks::HarmonicResonance => Some(PerkOptionData::stacking(2)),
        Perks::FieldTested => Some(PerkOptionData::stacking(5)),
        Perks::NobleDeeds => Some(PerkOptionData::toggle()),

        //season 1 | year 1
        Perks::KillClip => Some(PerkOptionData::toggle()),
        Perks::Outlaw => Some(PerkOptionData::toggle()),
        Perks::BackupPlan => Some(PerkOptionData::toggle()),
        Perks::FieldPrep => Some(PerkOptionData::toggle()),
        Perks::Rampage => Some(PerkOptionData::stacking(3)),
        Perks::OpeningShot => Some(PerkOptionData::toggle()),
        Perks::MovingTarget => Some(PerkOptionData::toggle()),
        Perks::AmbitiousAssassin => Some(PerkOptionData::stacking(15)),
        Perks::ClusterBomb => Some(PerkOptionData::static_()),
        Perks::ExplosivePayload => Some(PerkOptionData::static_()),
        Perks::FirmlyPlanted => Some(PerkOptionData::toggle()),
        Perks::FullAutoTrigger => Some(PerkOptionData::static_()),
        Perks::HeadSeeker => Some(PerkOptionData::static_()),
        Perks::HighImpactReserves => Some(PerkOptionData::static_()),
        Perks::HipFireGrip => Some(PerkOptionData::toggle()),
        Perks::Snapshot => Some(PerkOptionData::static_()),
        Perks::TapTheTrigger => Some(PerkOptionData::toggle()),
        Perks::SlideWays => Some(PerkOptionData::toggle()),
        Perks::QuickDraw => Some(PerkOptionData::toggle()),
        Perks::TimedPayload => Some(PerkOptionData::static_()),
        Perks::ThreatDetector => Some(PerkOptionData::stacking(2)),
        Perks::SlideShot => Some(PerkOptionData::toggle()),
        Perks::TripleTap => Some(PerkOptionData::static_()),
        Perks::UnderPressure => Some(PerkOptionData::toggle()),
        Perks::PulseMonitor => Some(PerkOptionData::toggle()),

        //season 2 | year 1
        //lmao bozo

        //season 3 | year 1
        Perks::RangeFinder => Some(PerkOptionData::static_()),
        Perks::DisruptionBreak => Some(PerkOptionData::toggle()),
        Perks::TrenchBarrel => Some(PerkOptionData::toggle()),
        Perks::Desperado => Some(PerkOptionData::toggle()),
        Perks::BoxBreathing => Some(PerkOptionData::toggle()),

        //season 4 | year 2
        Perks::ArchersTempo => Some(PerkOptionData::toggle()),
        Perks::ExplosiveHead => Some(PerkOptionData::static_()),
        Perks::FeedingFrenzy => Some(PerkOptionData::stacking(5)),
        Perks::FourthTimesTheCharm => Some(PerkOptionData::static_()),
        Perks::RapidHit => Some(PerkOptionData::stacking(5)),

        //season 5 | year 2
        Perks::ResevoirBurst => Some(PerkOptionData::static_()),
        Perks::Surrounded => Some(PerkOptionData::toggle()),
        Perks::AirAssault => Some(PerkOptionData::stacking(2)),

        //season 6 | year 2
        Perks::FiringLine => Some(PerkOptionData::toggle()),
        Perks::FullCourt => Some(PerkOptionData::toggle()),
        Perks::KillingTally => Some(PerkOptionData::stacking(3)),
        // Perks::Demolitionist => Some(PerkOptionData::options(vec!["Once", "Every 3s"])),
        Perks::MultikillClip => Some(PerkOptionData::stacking(3)),
        Perks::Swashbuckler => Some(PerkOptionData::stacking(5)),
        Perks::OverFlow => Some(PerkOptionData::toggle()),

        //season 7 | year 2
        Perks::UnderDog => Some(PerkOptionData::toggle()),
        Perks::ExplosiveLight => Some(PerkOptionData::toggle()),
        Perks::EyeOfTheStorm => Some(PerkOptionData::toggle()),
        Perks::NoDistractions => Some(PerkOptionData::toggle()),
        Perks::ArchersGambit => Some(PerkOptionData::toggle()),

        //season 8 | year 3
        //TODO

        //season 9 | year 3
        Perks::ClownCartridge => Some(PerkOptionData::static_()),
        Perks::ElementalCapacitor => Some(PerkOptionData::options(
            ["Void", "Solar", "Arc", "Stasis", "Strand"].to_vec(),
        )),
        Perks::Vorpal => Some(PerkOptionData::static_()),

        //season 10 | year 3
        //bad season lmao

        //season 11 | year 3
        Perks::KillingWind => Some(PerkOptionData::toggle()),

        //season 12 | year 4
        Perks::DualLoader => Some(PerkOptionData::static_()),
        Perks::OneForAll => Some(PerkOptionData::toggle()),
        Perks::Recombination => Some(PerkOptionData::stacking(10)),
        Perks::Reconstruction => Some(PerkOptionData::toggle()),
        Perks::Surplus => Some(PerkOptionData::stacking(3)),

        //season 13 | year 4
        Perks::ImpulseAmplifier => Some(PerkOptionData::static_()),
        Perks::Frenzy => Some(PerkOptionData::toggle()),
        Perks::LastingImpression => Some(PerkOptionData::static_()),
        Perks::KickStart => Some(PerkOptionData::toggle()),

        //season 14 | year 4
        Perks::Cornered => Some(PerkOptionData::toggle()),
        Perks::AdrenalineJunkie => Some(PerkOptionData::stacking(5)),
        Perks::RewindRounds => Some(PerkOptionData::static_()),
        Perks::HeatingUp => Some(PerkOptionData::stacking(2)),
        Perks::FireFly => Some(PerkOptionData::toggle()),
        Perks::DangerZone => Some(PerkOptionData::toggle()),
        Perks::TunnelVision => Some(PerkOptionData::toggle()),

        //season 15 | year 4
        Perks::Encore => Some(PerkOptionData::stacking(4)),
        Perks::Ensemble => Some(PerkOptionData::toggle()),
        Perks::GoldenTricorn => Some(PerkOptionData::stacking(2)),
        Perks::Harmony => Some(PerkOptionData::toggle()),
        Perks::PerpetualMotion => Some(PerkOptionData::stacking(2)),
        Perks::Adagio => Some(PerkOptionData::toggle()),
        Perks::BluntExecutionRounds => Some(PerkOptionData::toggle()),

        //season 16 | year 5
        Perks::BaitAndSwitch => Some(PerkOptionData::toggle()),
        Perks::CompulsiveReloader => Some(PerkOptionData::toggle()),
        Perks::FocusedFury => Some(PerkOptionData::toggle()),
        Perks::ChillClip => Some(PerkOptionData::static_()),
        Perks::SleightOfHand => Some(PerkOptionData::stacking(3)),
        Perks::StatsForAll => Some(PerkOptionData::toggle()),
        Perks::SteadyHands => Some(PerkOptionData::toggle()),
        Perks::SuccesfulWarmup => Some(PerkOptionData::toggle()),
        Perks::UnstoppableForce => Some(PerkOptionData::toggle()),

        //season 17 | year 5
        Perks::FragileFocus => Some(PerkOptionData::toggle()),
        Perks::WellRounded => Some(PerkOptionData::stacking(2)),

        //season 18 | year 5
        Perks::GutShot => Some(PerkOptionData::toggle()),
        Perks::Pugilist => Some(PerkOptionData::toggle()),
        Perks::Slickdraw => Some(PerkOptionData::static_()),
        Perks::UnderOver => Some(PerkOptionData::static_()),

        //season 19 | year 5
        Perks::CascadePoint => Some(PerkOptionData::toggle()),
        Perks::CloseToMelee => Some(PerkOptionData::toggle()),
        Perks::OffhandStrike => Some(PerkOptionData::toggle()),
        Perks::PerfectFloat => Some(PerkOptionData::toggle()),
        Perks::ShotSwap => Some(PerkOptionData::toggle()),
        Perks::TargetLock => Some(PerkOptionData::static_()),

        //season 20 | year 6
        Perks::KeepAway => Some(PerkOptionData::toggle()),
        Perks::ParacausalAffinity => Some(PerkOptionData::toggle()),
        Perks::EnviousAssassin => Some(PerkOptionData::stacking(20)),

        //season 21 | year 6
        Perks::CollectiveAction => Some(PerkOptionData::toggle()),
        Perks::Bipod => Some(PerkOptionData::static_()),
        Perks::ControlledBurst => Some(PerkOptionData::toggle()),
        Perks::InvisibleHand => Some(PerkOptionData::toggle()),
        Perks::UnsatedHunger => Some(PerkOptionData::toggle()),
        Perks::Discord => Some(PerkOptionData::toggle()),
        Perks::EddyCurrent => Some(PerkOptionData::options(["Base", "Amplified"].to_vec())),

        //season 22 | year 6
        Perks::PrecisionInstrument => Some(PerkOptionData::stacking(6)),
        Perks::LooseChange => Some(PerkOptionData::toggle()),
        Perks::HighGround => Some(PerkOptionData::toggle()),
        Perks::HeadRush => Some(PerkOptionData::toggle()),
        Perks::EnlightendAction => Some(PerkOptionData::stacking(12)),
        Perks::SwordLogic => Some(PerkOptionData::stacking(4)),
        //season 23 | year 6
        Perks::Onslaught => Some(PerkOptionData::stacking(3)),
        Perks::DesperateMeasures => Some(PerkOptionData::stacking(3)),
        Perks::MasterOfArms => Some(PerkOptionData::toggle()),

        //episode 1 | year 7
        Perks::ChaosReshaped => Some(PerkOptionData::stacking(2)),
        Perks::CircleOfLife => Some(PerkOptionData::toggle()),

        //exotics
        Perks::CranialSpike => Some(PerkOptionData::stacking(5)),
        Perks::DarkForgedTrigger => Some(PerkOptionData::toggle()),
        Perks::AgersCall => Some(PerkOptionData::static_()),
        Perks::LagragianSight => Some(PerkOptionData::toggle()),
        Perks::StringofCurses => Some(PerkOptionData::stacking(5)),
        Perks::WormsHunger => Some(PerkOptionData::stacking(20)),
        Perks::WormByproduct => Some(PerkOptionData::toggle()),
        Perks::RocketTracers => Some(PerkOptionData::static_()),
        Perks::ParacausalShot => Some(PerkOptionData::stacking(7)),
        Perks::CorruptionSpreads => Some(PerkOptionData::static_()),
        Perks::TimeSlip => Some(PerkOptionData::toggle()),
        Perks::ToM => Some(PerkOptionData::toggle()),
        Perks::IgnitionTrigger => Some(PerkOptionData::toggle()),
        Perks::GuidanceRing => Some(PerkOptionData::toggle()),
        Perks::ConserveMomentum => Some(PerkOptionData::stacking(15)),
        Perks::Impetus => Some(PerkOptionData::toggle()),
        Perks::FirstGlance => Some(PerkOptionData::toggle()),
        Perks::PerfectFith => Some(PerkOptionData::static_()),
        Perks::Broadside => Some(PerkOptionData::stacking(4)),
        Perks::FourthHorsemanCatalyst => Some(PerkOptionData::toggle()),
        Perks::Stormbringer => Some(PerkOptionData::static_()),
        Perks::PrismaticInferno => Some(PerkOptionData::static_()),
        Perks::ReignHavoc => Some(PerkOptionData::toggle()),
        Perks::WhisperCatalyst => Some(PerkOptionData::toggle()),
        Perks::Roadborn => Some(PerkOptionData::toggle()),
        Perks::SwoopingTalons => Some(PerkOptionData::toggle()),
        Perks::CalculatedBalance => Some(PerkOptionData::toggle()),
        Perks::RavenousBeast => Some(PerkOptionData::toggle()),
        Perks::LordOfWolvesCatalyst => Some(PerkOptionData::static_()),
        Perks::ReleaseTheWolves => Some(PerkOptionData::toggle()),
        Perks::Fundamentals => Some(PerkOptionData::options(["Void", "Solar", "Arc"].to_vec())),
        Perks::ThinTheHerd => Some(PerkOptionData::toggle()),
        Perks::Chimera => Some(PerkOptionData::toggle()),
        Perks::FateOfAllFools => Some(PerkOptionData::stacking(3)),
        Perks::HonedEdge => Some(PerkOptionData::stacking_min(4, 1)),
        Perks::TakenPredator => Some(PerkOptionData::options(
            ["Taken", "Witherhoard", "Both"].to_vec(),
        )),
        Perks::MarkovChain => Some(PerkOptionData::stacking(5)),
        Perks::StormAndStress => Some(PerkOptionData::toggle()),
        Perks::DualSpeedReceiver => Some(PerkOptionData::toggle()),
        Perks::ExplosiveShadow => Some(PerkOptionData::static_()),
        Perks::SurosLegacy => Some(PerkOptionData::static_()),
        Perks::SpinningUp => Some(PerkOptionData::stacking(2)),
        Perks::DarkDescent => Some(PerkOptionData::toggle()),
        Perks::TargetAquired => Some(PerkOptionData::toggle()),
        Perks::RatPack => Some(PerkOptionData::stacking_min(5, 1)),
        Perks::HuntersTrance => Some(PerkOptionData::static_()),
        Perks::RideTheBull => Some(PerkOptionData::stacking(2)),
        Perks::NobleRounds => Some(PerkOptionData::toggle()),
        Perks::MementoMori => Some(PerkOptionData::toggle()),
        Perks::TractorCannon => Some(PerkOptionData::static_()),
        Perks::HarmonicLaser => Some(PerkOptionData::stacking(2)),
        Perks::AgersScepterCatalyst => Some(PerkOptionData::toggle()),
        Perks::ColdFusion => Some(PerkOptionData::toggle()),
        Perks::BlackHole => Some(PerkOptionData::static_()),
        Perks::TemporalUnlimiter => Some(PerkOptionData::toggle()),
        Perks::MarksmanSights => Some(PerkOptionData::static_()),
        Perks::DexterityMod => Some(PerkOptionData::stacking(3)),
        Perks::ReserveMod => Some(PerkOptionData::stacking(3)),
        Perks::LoaderMod => Some(PerkOptionData::stacking(3)),
        Perks::TargetingMod => Some(PerkOptionData::stacking(3)),
        Perks::UnflinchingMod => Some(PerkOptionData::stacking(3)),
        Perks::SurgeMod => Some(PerkOptionData::stacking(3)),
        Perks::LucentBlades => Some(PerkOptionData::stacking(3)),
        Perks::OnYourMark => Some(PerkOptionData::stacking(3)),
        Perks::Frequency => Some(PerkOptionData::toggle()),
        Perks::Tempering => Some(PerkOptionData::toggle()),
        Perks::Hedrons => Some(PerkOptionData::toggle()),
        Perks::HeatRises => Some(PerkOptionData::toggle()),
        Perks::FlowState => Some(PerkOptionData::toggle()),
        Perks::ThreadOfAscent => Some(PerkOptionData::toggle()),
        Perks::WellOfRadiance => Some(PerkOptionData::static_()),
        Perks::Amplified => Some(PerkOptionData::static_()),
        Perks::Radiant => Some(PerkOptionData::static_()),
        Perks::Weaken => Some(PerkOptionData::static_()),
        Perks::Sever => Some(PerkOptionData::static_()),
        Perks::WardOfDawn => Some(PerkOptionData::static_()),
        Perks::BannerShield => Some(PerkOptionData::static_()),
        Perks::DeadFall => Some(PerkOptionData::static_()),
        Perks::MoebiusQuiver => Some(PerkOptionData::static_()),
        Perks::Broadhead => Some(PerkOptionData::static_()),
        Perks::HuntersTrace => Some(PerkOptionData::toggle()),
        Perks::Desperation => Some(PerkOptionData::toggle()),
        Perks::IonicReturn => Some(PerkOptionData::toggle()),
        Perks::Unrepentant => Some(PerkOptionData::toggle()),
        Perks::ArcConductor => Some(PerkOptionData::toggle()),
        Perks::VoidLeech => Some(PerkOptionData::toggle()),
        Perks::InverseRelationship => Some(PerkOptionData::stacking(3)),
        Perks::Spindle => Some(PerkOptionData::stacking(25)),
        Perks::TheRightChoice => Some(PerkOptionData::static_()),

        Perks::DragonShadow => Some(PerkOptionData::toggle()),
        Perks::OphidianAspect => Some(PerkOptionData::static_()),
        Perks::TomeOfDawn => Some(PerkOptionData::toggle()),
        Perks::PathOfTheBurningSteps => Some(PerkOptionData::stacking(4)),
        Perks::MantleOfBattleHarmony => Some(PerkOptionData::static_()),
        Perks::MaskOfBakris => Some(PerkOptionData::toggle()),
        Perks::BallindorseWrathweavers => Some(PerkOptionData::toggle()),
        Perks::LunaFaction => Some(PerkOptionData::options(
            ["Heal Rift", "Empowering Rift / Well"].to_vec(),
        )),
        Perks::KnuckleheadRadar => Some(PerkOptionData::toggle()),
        Perks::MechaneersTricksleeves => Some(PerkOptionData::toggle()),
        Perks::Oathkeeper => Some(PerkOptionData::static_()),
        Perks::SealedAhamkaraGrasps => Some(PerkOptionData::toggle()),
        Perks::LuckyPants => Some(PerkOptionData::stacking(10)),
        Perks::NoBackupPlans => Some(PerkOptionData::toggle()),
        Perks::ActiumWarRig => Some(PerkOptionData::static_()),
        Perks::HallowfireHeart => Some(PerkOptionData::static_()),
        Perks::LionRampart => Some(PerkOptionData::toggle()),
        Perks::Peacekeepers => Some(PerkOptionData::static_()),
        Perks::EyeOfAnotherWorld => Some(PerkOptionData::static_()),
        Perks::AstrocyteVerse => Some(PerkOptionData::toggle()),
        Perks::NecroticGrips => Some(PerkOptionData::static_()),
        Perks::BootsOfTheAssembler => Some(PerkOptionData::static_()),
        Perks::RainOfFire => Some(PerkOptionData::static_()),
        Perks::SpeedloaderSlacks => Some(PerkOptionData::stacking(5)),
        Perks::PeregrineGreaves => Some(PerkOptionData::static_()),
        Perks::Gyrfalcon => Some(PerkOptionData::toggle()),
        Perks::AeonInsight => Some(PerkOptionData::toggle()),
        Perks::Felwinters => Some(PerkOptionData::toggle()),
        Perks::SanguineAlchemy => Some(PerkOptionData::toggle()),
        Perks::TritonVice => Some(PerkOptionData::toggle()),
        Perks::Foetracers => Some(PerkOptionData::toggle()),
        Perks::GlacialGuard => Some(PerkOptionData::toggle()),

        //misc
        Perks::UmbralSharpening => Some(PerkOptionData::stacking(5)),
        Perks::EnhancedScannerAugment => Some(PerkOptionData::toggle()),
        Perks::Demolitionist => Some(PerkOptionData::static_()),
        Perks::FullStop => Some(PerkOptionData::static_()),
        Perks::HakkeHeavyBurst => Some(PerkOptionData::static_()),
        Perks::EternalWarrior => Some(PerkOptionData::stacking(4)),

        Perks::Ignore => None,
    }
}

pub fn enh_hash_to_perk_option_data(_hash: u32) -> Option<PerkOptionData> {
    let perk: Perks = enhanced_check(_hash).0.into();
    match perk {
        Perks::Recombination => Some(PerkOptionData::stacking(8)),
        Perks::ExplosiveLight => Some(PerkOptionData::stacking(7)),
        _ => hash_to_perk_option_data(_hash),
    }
}

pub fn get_perk_options(_perks: Vec<u32>) -> HashMap<u32, PerkOptionData> {
    let mut options = HashMap::new();
    for perk in _perks {
        // let data = if  _input._is_enhanced {enh_hash_to_perk_option_data(perk)} else {hash_to_perk_option_data(perk)};
        let data = hash_to_perk_option_data(perk);
        if let Some(value) = data {
            options.insert(perk, value);
        }
    }
    options
}
