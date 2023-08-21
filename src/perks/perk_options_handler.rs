use std::collections::HashMap;

use serde::Serialize;

use super::{enhanced_check, Perk, Perks};

#[derive(Debug, Clone, Serialize)]
pub enum PerkValueVariant {
    STATIC,
    TOGGLE,
    SLIDER,
    OPTIONS,
}
impl Default for PerkValueVariant {
    fn default() -> Self {
        PerkValueVariant::STATIC
    }
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
    pub fn toggled_off() -> PerkOptionData {
        PerkOptionData {
            stacks: (0, 1),
            options: vec![],
            option_type: PerkValueVariant::TOGGLE,
        }
    }
    pub fn toggled_on() -> PerkOptionData {
        PerkOptionData {
            stacks: (1, 0),
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
        Perks::OnYourMark => Some(PerkOptionData::stacking(3)),
        Perks::Frequency => Some(PerkOptionData::toggled_off()),
        Perks::Tempering => Some(PerkOptionData::toggled_off()),
        Perks::Hedrons => Some(PerkOptionData::toggled_off()),
        Perks::HeatRises => Some(PerkOptionData::toggled_off()),
        Perks::FlowState => Some(PerkOptionData::toggled_off()),
        Perks::ThreadOfAscent => Some(PerkOptionData::toggled_off()),
        Perks::WellOfRadiance => Some(PerkOptionData::static_()),
        Perks::Amplified => Some(PerkOptionData::static_()),
        Perks::Radiant => Some(PerkOptionData::static_()),
        Perks::Weaken => Some(PerkOptionData::static_()),
        Perks::WardOfDawn => Some(PerkOptionData::static_()),
        Perks::BannerShield => Some(PerkOptionData::static_()),
        Perks::DeadFall => Some(PerkOptionData::static_()),
        Perks::MoebiusQuiver => Some(PerkOptionData::static_()),

        //intrinsics
        Perks::RapidFireFrame => Some(PerkOptionData::toggled_off()),
        Perks::PrecisionFrame => Some(PerkOptionData::static_()),

        //armor
        Perks::DexterityMod => Some(PerkOptionData::stacking(3)),
        Perks::ReserveMod => Some(PerkOptionData::stacking(3)),
        Perks::LoaderMod => Some(PerkOptionData::stacking(3)),
        Perks::TargetingMod => Some(PerkOptionData::stacking(3)),
        Perks::UnflinchingMod => Some(PerkOptionData::stacking(3)),
        Perks::SurgeMod => Some(PerkOptionData::stacking(3)),

        //parts
        Perks::ImpactCasing => Some(PerkOptionData::static_()),
        Perks::SwapMag => Some(PerkOptionData::static_()),
        Perks::FullChoke => Some(PerkOptionData::static_()),
        Perks::SpikeGrenades => Some(PerkOptionData::static_()),
        Perks::AlloyMag => Some(PerkOptionData::toggled_off()),
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
        Perks::TakenSpec => Some(PerkOptionData::toggled_off()),
        Perks::FreehandGrip => Some(PerkOptionData::static_()),

        //origin | year 5+
        Perks::VeistStinger => Some(PerkOptionData::toggled_off()),
        Perks::HakkeBreach => Some(PerkOptionData::toggled_off()),
        Perks::Alacrity => Some(PerkOptionData::toggled_off()),
        Perks::FluidDynamics => Some(PerkOptionData::toggled_off()),
        Perks::QuietMoment => Some(PerkOptionData::toggled_off()),
        Perks::SurosSynergy => Some(PerkOptionData::toggled_off()),
        Perks::BitterSpite => Some(PerkOptionData::stacking(5)),
        Perks::RunnethOver => Some(PerkOptionData::stacking(5)),
        Perks::HotSwap => Some(PerkOptionData::toggled_off()),
        Perks::RightHook => Some(PerkOptionData::toggled_off()),
        Perks::Ambush => Some(PerkOptionData::toggled_off()),
        Perks::TexBalancedStock => Some(PerkOptionData::toggled_off()),
        Perks::SearchParty => Some(PerkOptionData::toggled_off()),
        Perks::HarmonicResonance => Some(PerkOptionData::stacking(2)),
        Perks::FieldTested => Some(PerkOptionData::stacking(5)),

        //season 1 | year 1
        Perks::KillClip => Some(PerkOptionData::toggled_off()),
        Perks::Outlaw => Some(PerkOptionData::toggled_off()),
        Perks::BackupPlan => Some(PerkOptionData::toggled_off()),
        Perks::FieldPrep => Some(PerkOptionData::toggled_off()),
        Perks::Rampage => Some(PerkOptionData::stacking(3)),
        Perks::OpeningShot => Some(PerkOptionData::toggled_off()),
        Perks::MovingTarget => Some(PerkOptionData::toggled_off()),
        Perks::AmbitiousAssassin => Some(PerkOptionData::stacking(15)),
        Perks::ClusterBomb => Some(PerkOptionData::static_()),
        Perks::ExplosivePayload => Some(PerkOptionData::static_()),
        Perks::FirmlyPlanted => Some(PerkOptionData::toggled_off()),
        Perks::FullAutoTrigger => Some(PerkOptionData::static_()),
        Perks::HeadSeeker => Some(PerkOptionData::static_()),
        Perks::HighImpactReserves => Some(PerkOptionData::static_()),
        Perks::HipFireGrip => Some(PerkOptionData::toggled_off()),
        Perks::Snapshot => Some(PerkOptionData::static_()),
        Perks::TapTheTrigger => Some(PerkOptionData::toggled_off()),
        Perks::SlideWays => Some(PerkOptionData::toggled_off()),
        Perks::QuickDraw => Some(PerkOptionData::toggled_off()),
        Perks::TimedPayload => Some(PerkOptionData::static_()),
        Perks::ThreatDetector => Some(PerkOptionData::stacking(2)),
        Perks::SlideShot => Some(PerkOptionData::toggled_off()),
        Perks::TripleTap => Some(PerkOptionData::static_()),
        Perks::UnderPressure => Some(PerkOptionData::toggled_off()),
        Perks::PulseMonitor => Some(PerkOptionData::toggled_off()),

        //season 2 | year 1
        //lmao bozo

        //season 3 | year 1
        Perks::RangeFinder => Some(PerkOptionData::static_()),
        Perks::DisruptionBreak => Some(PerkOptionData::toggled_off()),
        Perks::TrenchBarrel => Some(PerkOptionData::toggled_off()),
        Perks::Desperado => Some(PerkOptionData::toggled_off()),
        Perks::BoxBreathing => Some(PerkOptionData::toggled_off()),

        //season 4 | year 2
        Perks::ArchersTempo => Some(PerkOptionData::toggled_off()),
        Perks::ExplosiveHead => Some(PerkOptionData::static_()),
        Perks::FeedingFrenzy => Some(PerkOptionData::stacking(5)),
        Perks::FourthTimesTheCharm => Some(PerkOptionData::static_()),
        Perks::RapidHit => Some(PerkOptionData::stacking(5)),

        //season 5 | year 2
        Perks::ResevoirBurst => Some(PerkOptionData::static_()),
        Perks::Surrounded => Some(PerkOptionData::toggled_off()),
        Perks::AirAssault => Some(PerkOptionData::stacking(2)),

        //season 6 | year 2
        Perks::FiringLine => Some(PerkOptionData::toggled_off()),
        Perks::FullCourt => Some(PerkOptionData::toggled_off()),
        Perks::KillingTally => Some(PerkOptionData::stacking(3)),
        // Perks::Demolitionist => Some(PerkOptionData::options(vec!["Once", "Every 3s"])),
        Perks::MultikillClip => Some(PerkOptionData::stacking(3)),
        Perks::Swashbuckler => Some(PerkOptionData::stacking(5)),
        Perks::OverFlow => Some(PerkOptionData::toggled_off()),

        //season 7 | year 2
        Perks::UnderDog => Some(PerkOptionData::toggled_off()),
        Perks::ExplosiveLight => Some(PerkOptionData::toggled_off()),
        Perks::EyeOfTheStorm => Some(PerkOptionData::toggled_off()),
        Perks::NoDistractions => Some(PerkOptionData::toggled_off()),

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
        Perks::KillingWind => Some(PerkOptionData::toggled_off()),

        //season 12 | year 4
        Perks::DualLoader => Some(PerkOptionData::toggled_off()),
        Perks::OneForAll => Some(PerkOptionData::toggled_off()),
        Perks::Recombination => Some(PerkOptionData::stacking(10)),
        Perks::Reconstruction => Some(PerkOptionData::toggled_off()),
        Perks::Surplus => Some(PerkOptionData::stacking(3)),

        //season 13 | year 4
        Perks::ImpulseAmplifier => Some(PerkOptionData::static_()),
        Perks::Frenzy => Some(PerkOptionData::toggled_off()),
        Perks::LastingImpression => Some(PerkOptionData::static_()),
        Perks::KickStart => Some(PerkOptionData::toggled_off()),

        //season 14 | year 4
        Perks::Cornered => Some(PerkOptionData::toggled_off()),
        Perks::AdrenalineJunkie => Some(PerkOptionData::stacking(5)),
        Perks::RewindRounds => Some(PerkOptionData::static_()),
        Perks::HeatingUp => Some(PerkOptionData::stacking(2)),
        Perks::FireFly => Some(PerkOptionData::toggled_off()),
        Perks::DangerZone => Some(PerkOptionData::toggled_off()),
        Perks::TunnelVision => Some(PerkOptionData::toggled_off()),

        //season 15 | year 4
        Perks::Encore => Some(PerkOptionData::stacking(4)),
        Perks::Ensemble => Some(PerkOptionData::toggled_off()),
        Perks::GoldenTricorn => Some(PerkOptionData::stacking(2)),
        Perks::Harmony => Some(PerkOptionData::toggled_off()),
        Perks::PerpetualMotion => Some(PerkOptionData::stacking(2)),
        Perks::Adagio => Some(PerkOptionData::toggled_off()),

        //season 16 | year 5
        Perks::BaitAndSwitch => Some(PerkOptionData::toggled_off()),
        Perks::CompulsiveReloader => Some(PerkOptionData::toggled_off()),
        Perks::FocusedFury => Some(PerkOptionData::toggled_off()),
        Perks::ChillClip => Some(PerkOptionData::static_()),
        Perks::SleightOfHand => Some(PerkOptionData::stacking(3)),
        Perks::StatsForAll => Some(PerkOptionData::toggled_off()),
        Perks::SteadyHands => Some(PerkOptionData::toggled_off()),
        Perks::SuccesfulWarmup => Some(PerkOptionData::toggled_off()),
        Perks::UnstoppableForce => Some(PerkOptionData::toggled_off()),

        //season 17 | year 5
        Perks::FragileFocus => Some(PerkOptionData::toggled_off()),
        Perks::WellRounded => Some(PerkOptionData::stacking(2)),

        //season 18 | year 5
        Perks::GutShot => Some(PerkOptionData::static_()),
        Perks::Pugilist => Some(PerkOptionData::toggled_off()),
        Perks::Slickdraw => Some(PerkOptionData::static_()),
        Perks::OverUnder => Some(PerkOptionData::static_()),

        //season 19 | year 5
        Perks::CascadePoint => Some(PerkOptionData::toggled_off()),
        Perks::CloseToMelee => Some(PerkOptionData::toggled_off()),
        Perks::OffhandStrike => Some(PerkOptionData::toggled_off()),
        Perks::PerfectFloat => Some(PerkOptionData::toggled_off()),
        Perks::ShotSwap => Some(PerkOptionData::toggled_off()),
        Perks::TargetLock => Some(PerkOptionData::static_()),

        //season 20 | year 6
        Perks::KeepAway => Some(PerkOptionData::toggled_off()),
        Perks::ParacausalAffinity => Some(PerkOptionData::toggled_off()),
        Perks::EnviousAssasin => Some(PerkOptionData::stacking(15)),

        //season 21 | year 6
        Perks::CollectiveAction => Some(PerkOptionData::toggled_off()),
        Perks::Bipod => Some(PerkOptionData::static_()),
        Perks::ControlledBurst => Some(PerkOptionData::toggled_off()),
        Perks::InvisibleHand => Some(PerkOptionData::toggled_off()),
        Perks::UnsatedHunger => Some(PerkOptionData::toggled_off()),
        Perks::Discord => Some(PerkOptionData::toggled_off()),

        //exotic weapon
        Perks::CranialSpike => Some(PerkOptionData::stacking(5)),
        Perks::DarkForgedTrigger => Some(PerkOptionData::options_raw(["Hip-Fire", "ADS"].to_vec())),
        Perks::AgersCall => Some(PerkOptionData::static_()),
        Perks::LagragianSight => Some(PerkOptionData::toggled_off()),
        Perks::StringofCurses => Some(PerkOptionData::stacking(5)),
        Perks::WormsHunger => Some(PerkOptionData::stacking(20)),
        Perks::WormByproduct => Some(PerkOptionData::toggled_off()),
        Perks::RocketTracers => Some(PerkOptionData::static_()),
        Perks::ParacausalShot => Some(PerkOptionData::stacking(7)),
        Perks::CorruptionSpreads => Some(PerkOptionData::static_()),
        Perks::TimeSlip => Some(PerkOptionData::toggled_off()),
        Perks::ToM => Some(PerkOptionData::toggled_off()),
        Perks::IgnitionTrigger => Some(PerkOptionData::toggled_off()),
        Perks::GuidanceRing => Some(PerkOptionData::toggled_off()),
        Perks::ConserveMomentum => Some(PerkOptionData::stacking(15)),
        Perks::Impetus => Some(PerkOptionData::toggled_off()),
        Perks::FirstGlance => Some(PerkOptionData::toggled_off()),
        Perks::PerfectFith => Some(PerkOptionData::static_()),
        Perks::Broadside => Some(PerkOptionData::stacking(4)),
        Perks::FourthHorsemanCatalyst => Some(PerkOptionData::toggled_off()),
        Perks::Stormbringer => Some(PerkOptionData::static_()),
        Perks::PrismaticInferno => Some(PerkOptionData::static_()),
        Perks::ReignHavoc => Some(PerkOptionData::toggled_off()),
        Perks::WhisperCatalyst => Some(PerkOptionData::toggled_off()),
        Perks::Roadborn => Some(PerkOptionData::toggled_off()),
        Perks::SwoopingTalons => Some(PerkOptionData::toggled_off()),
        Perks::CalculatedBalance => Some(PerkOptionData::toggled_off()),
        Perks::RavenousBeast => Some(PerkOptionData::toggled_off()),
        Perks::LordOfWolvesCatalyst => Some(PerkOptionData::static_()),
        Perks::ReleaseTheWolves => Some(PerkOptionData::toggled_off()),
        Perks::Fundamentals => Some(PerkOptionData::options(["Void", "Solar", "Arc"].to_vec())),
        Perks::ThinTheHerd => Some(PerkOptionData::toggled_off()),
        Perks::Chimera => Some(PerkOptionData::toggled_off()),
        Perks::FateOfAllFools => Some(PerkOptionData::stacking(3)),
        Perks::HonedEdge => Some(PerkOptionData::stacking_min(4, 1)),
        Perks::TakenPredator => Some(PerkOptionData::options(
            ["Taken", "Witherhoard", "Both"].to_vec(),
        )),
        Perks::MarkovChain => Some(PerkOptionData::stacking(5)),
        Perks::StormAndStress => Some(PerkOptionData::toggled_off()),
        Perks::DualSpeedReceiver => Some(PerkOptionData::toggled_off()),
        Perks::ExplosiveShadow => Some(PerkOptionData::static_()),
        Perks::SurosLegacy => Some(PerkOptionData::static_()),
        Perks::SpinningUp => Some(PerkOptionData::stacking(2)),
        Perks::DarkDescent => Some(PerkOptionData::toggled_off()),
        Perks::SleeperCatalyst => Some(PerkOptionData::static_()),
        Perks::TargetAquired => Some(PerkOptionData::toggled_off()),
        Perks::RatPack => Some(PerkOptionData::stacking_min(5, 1)),
        Perks::HuntersTrance => Some(PerkOptionData::static_()),
        Perks::RideTheBull => Some(PerkOptionData::stacking(2)),
        Perks::NobleRounds => Some(PerkOptionData::toggled_off()),
        Perks::MementoMori => Some(PerkOptionData::toggled_off()),
        Perks::TractorCannon => Some(PerkOptionData::static_()),
        Perks::HarmonicLaser => Some(PerkOptionData::stacking(2)),
        Perks::AgersScepterCatalyst => Some(PerkOptionData::toggled_off()),
        Perks::ColdFusion => Some(PerkOptionData::toggled_off()),
        Perks::BlackHole => Some(PerkOptionData::static_()),
        Perks::TemporalUnlimiter => Some(PerkOptionData::toggled_off()),
        Perks::MarksmanSights => Some(PerkOptionData::static_()),
        Perks::Broadhead => Some(PerkOptionData::static_()),
        Perks::HuntersTrace => Some(PerkOptionData::toggled_off()),

        //exotic armor
        Perks::DragonShadow => Some(PerkOptionData::toggled_off()),
        Perks::OphidianAspect => Some(PerkOptionData::static_()),
        Perks::TomeOfDawn => Some(PerkOptionData::toggled_off()),
        Perks::PathOfTheBurningSteps => Some(PerkOptionData::stacking(4)),
        Perks::MantleOfBattleHarmony => Some(PerkOptionData::static_()),
        Perks::MaskOfBakris => Some(PerkOptionData::toggled_off()),
        Perks::BallindorseWrathweavers => Some(PerkOptionData::toggled_off()),
        Perks::LunaFaction => Some(PerkOptionData::options(
            ["Heal Rift", "Empowering Rift / Well"].to_vec(),
        )),
        Perks::Foetracer => Some(PerkOptionData::toggled_off()),
        Perks::MechaneersTricksleeves => Some(PerkOptionData::toggled_off()),
        Perks::Oathkeeper => Some(PerkOptionData::static_()),
        Perks::SealedAhamkaraGrasps => Some(PerkOptionData::toggled_off()),
        Perks::LuckyPants => Some(PerkOptionData::stacking(10)),
        Perks::NoBackupPlans => Some(PerkOptionData::toggled_off()),
        Perks::ActiumWarRig => Some(PerkOptionData::static_()),
        Perks::HallowfireHeart => Some(PerkOptionData::static_()),
        Perks::LionRampart => Some(PerkOptionData::toggled_off()),
        Perks::Peacekeepers => Some(PerkOptionData::static_()),
        Perks::EyeOfAnotherWorld => Some(PerkOptionData::static_()),
        Perks::AstrocyteVerse => Some(PerkOptionData::toggled_off()),
        Perks::NecroticGrips => Some(PerkOptionData::static_()),
        Perks::BootsOfTheAssembler => Some(PerkOptionData::static_()),
        Perks::RainOfFire => Some(PerkOptionData::static_()),
        Perks::SpeedloaderSlacks => Some(PerkOptionData::stacking(5)),
        Perks::PeregrineGreaves => Some(PerkOptionData::static_()),
        Perks::Gyrfalcon => Some(PerkOptionData::toggled_off()),
        Perks::AeonInsight => Some(PerkOptionData::toggled_off()),
        Perks::Felwinters => Some(PerkOptionData::toggled_off()),
        Perks::SanguineAlchemy => Some(PerkOptionData::toggled_off()),
        Perks::TritonVice => Some(PerkOptionData::toggled_off()),

        //misc
        Perks::UmbralSharpening => Some(PerkOptionData::stacking(5)),
        Perks::EnhancedScannerAugment => Some(PerkOptionData::toggled_off()),
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
        if data.is_some() {
            options.insert(perk, data.unwrap());
        }
    }
    options
}
