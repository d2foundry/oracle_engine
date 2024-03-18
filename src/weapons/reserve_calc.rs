#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ReserveIDs {
    Primary,
    LeviathansBreath,
    Fusions,
    SpecialGrenadeLaunchers,
    SmallGrenadeLaunchers,
    LargeGrenadeLaunchers,
    ErianasVow,
    LinearFusions,
    SmallMachineGuns,
    LargeMachineGuns,
    Xenophage,
    Overture,
    RocketLaunchers,
    Shotguns,
    ForeRunner,
    SniperRifles,
    Glaive,
    TraceRifles,
    RapidFireSniper,
    RapidFireShotgun,

    //kinetic exotic special
    Arbalest,
    Bastion,
    ConditionalFinality,
    DelicateTomb,

    //energy exotic special
    BuriedBloodline,
    DeadMessenger,
    ExDiris,
    Jotunn,
    LordOfWolves,
    LorentzDriver,
    Merciless,
    Telesto,

    //exotic heavy
    Anarchy,
    DeathBringer,
    DragonsBreath,
    EyesOfTomorrow,
    Gjallarhorn,
    HierApparent,
    LegendOfAcrius,
    OneThousandVoices,
    Parasite,
    SleeperSimulant,
    TheColony,
    TheProspector,
    TheQueenbreaker,
    TheWardcliffCoil,
    TractorCannon,
    TwoTailedFox,
}
impl From<u32> for ReserveIDs {
    fn from(id: u32) -> Self {
        match id {
            0 => ReserveIDs::Primary,
            1699724249 => ReserveIDs::LeviathansBreath,
            1101 => ReserveIDs::Fusions,
            231 => ReserveIDs::LargeGrenadeLaunchers,
            232 => ReserveIDs::SpecialGrenadeLaunchers,
            233 => ReserveIDs::SmallGrenadeLaunchers,
            3174300811 => ReserveIDs::ErianasVow,
            2201 => ReserveIDs::LinearFusions,
            81 => ReserveIDs::SmallMachineGuns,
            82 => ReserveIDs::LargeMachineGuns,
            2261491232 => ReserveIDs::Xenophage,
            2940035732 => ReserveIDs::Overture,
            101 => ReserveIDs::RocketLaunchers,
            71 => ReserveIDs::Shotguns,
            2984682260 => ReserveIDs::ForeRunner,
            121 => ReserveIDs::SniperRifles,
            331 => ReserveIDs::Glaive,
            251 => ReserveIDs::TraceRifles,
            1201 => ReserveIDs::RapidFireSniper,
            701 => ReserveIDs::RapidFireShotgun,

            //kinetic exotic special
            2564164194 => ReserveIDs::Arbalest,
            1186480754 => ReserveIDs::Bastion,
            3787406018 => ReserveIDs::ConditionalFinality,

            //energy exotic special
            90392189 => ReserveIDs::BuriedBloodline,
            2733244971 => ReserveIDs::DeadMessenger,
            2585427437 => ReserveIDs::DelicateTomb,
            3183537623 => ReserveIDs::ExDiris,
            1656957541 => ReserveIDs::Jotunn,
            481338655 => ReserveIDs::LordOfWolves,
            2881100038 => ReserveIDs::LorentzDriver,
            656200654 => ReserveIDs::Merciless,
            1927916065 => ReserveIDs::Telesto,

            //heavy
            389268985 => ReserveIDs::Anarchy,
            411799453 => ReserveIDs::DeathBringer,
            2440389870 => ReserveIDs::DragonsBreath,
            2200569208 => ReserveIDs::EyesOfTomorrow,
            2962361451 => ReserveIDs::Gjallarhorn,
            2608508147 => ReserveIDs::HierApparent,
            372430833 => ReserveIDs::LegendOfAcrius,
            1657056865 => ReserveIDs::OneThousandVoices,
            1174163613 => ReserveIDs::Parasite,
            3884127242 => ReserveIDs::SleeperSimulant,
            3913463509 => ReserveIDs::TheColony,
            2977709078 => ReserveIDs::TheProspector,
            1531126198 => ReserveIDs::TheQueenbreaker,
            2473404935 => ReserveIDs::TheWardcliffCoil,
            1210807262 => ReserveIDs::TractorCannon,
            3649430342 => ReserveIDs::TwoTailedFox,

            _ => ReserveIDs::Primary,
        }
    }
}

pub fn calc_reserves(_mag_size: f64, _mag_stat: i32, _inv_stat: i32, _id: u32, _scale: f64) -> i32 {
    let id = ReserveIDs::from(_id);
    let raw_size: f64 = match id {
        ReserveIDs::Primary => 9999.0,
        ReserveIDs::SmallMachineGuns => small_machinegun(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::TraceRifles => trace_rifle(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::Glaive => glaives(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::SniperRifles => sniper_rifles(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::Shotguns => shotguns(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::RocketLaunchers => rockets(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::RapidFireSniper => rapid_fire_sniper(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::RapidFireShotgun => rapid_fire_shotgun(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::Fusions => fusions(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::LinearFusions => linear_fusion_rifle(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::LargeMachineGuns => rapid_fire_machinegun(_mag_size, _mag_stat, _inv_stat),

        //exotic kinetic special
        ReserveIDs::ForeRunner => forerunner(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::Arbalest => arbalest(_inv_stat),
        ReserveIDs::Bastion => bastion(_inv_stat),

        //exotic energy special
        ReserveIDs::BuriedBloodline => buried_bloodline(_inv_stat),
        ReserveIDs::ErianasVow => eriana_vow(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::ConditionalFinality => conditional_finality(_inv_stat),
        ReserveIDs::DeadMessenger => dead_messenger(_inv_stat),
        ReserveIDs::DelicateTomb => delicate_tomb(_inv_stat),
        ReserveIDs::ExDiris => ex_diris(_inv_stat),
        ReserveIDs::Jotunn => jotunn(_inv_stat),
        ReserveIDs::LordOfWolves => 100.0,
        ReserveIDs::LorentzDriver => lorentz_driver(_inv_stat),
        ReserveIDs::Merciless => merciless(_inv_stat),
        ReserveIDs::Telesto => telesto(_inv_stat),

        //exotic heavy
        ReserveIDs::Anarchy => anarchy(_inv_stat),
        ReserveIDs::DeathBringer => deathbringer(_inv_stat),
        ReserveIDs::DragonsBreath => dragons_breath(_inv_stat),
        ReserveIDs::EyesOfTomorrow => eyes_of_tomorrow(_inv_stat),
        ReserveIDs::Gjallarhorn => gjallarhorn(_inv_stat),
        ReserveIDs::HierApparent => hier_apparent(_inv_stat),
        ReserveIDs::LegendOfAcrius => legend_of_acrius(_inv_stat),
        ReserveIDs::OneThousandVoices => one_thousand_voices(_inv_stat),
        ReserveIDs::Overture => overture(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::SleeperSimulant => sleeper_simulant(_inv_stat),
        ReserveIDs::Xenophage => xenophage(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::LeviathansBreath => leviathans_breath(_inv_stat),
        ReserveIDs::Parasite => parasite(_inv_stat),
        ReserveIDs::TheColony => 28.0,
        ReserveIDs::TheProspector => 35.0,
        ReserveIDs::TheQueenbreaker => queenbreaker(_inv_stat),
        ReserveIDs::TheWardcliffCoil => wardcliff_coil(_inv_stat),
        ReserveIDs::TractorCannon => tractor_cannon(_inv_stat),
        ReserveIDs::TwoTailedFox => two_tailed_fox(_inv_stat),


        //placeholders
        ReserveIDs::SmallGrenadeLaunchers => 18.0,
        ReserveIDs::LargeGrenadeLaunchers => 20.0,
        ReserveIDs::SpecialGrenadeLaunchers => 21.0,
    };
    let size = raw_size * _scale;
    size.ceil() as i32
}

fn small_machinegun(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let non_rapid_fire_mg = 225.0;
    let round_amount = _mag_size - (29.5 + 0.45 * _mag_stat as f64);
    let offset = if _mag_stat as f64 >= 100.0 {
        0.0
    } else {
        (-0.875 + round_amount * 2.0) * (2.0 - ((100.0 - _mag_stat as f64) / 100.0))
    };
    let _mossy_complex = non_rapid_fire_mg
        + offset
        + _inv_stat as f64 * ((non_rapid_fire_mg + offset) * 2.25 - (non_rapid_fire_mg + offset))
            / 100.0;
    let mossy_simple = 2.25 * _inv_stat as f64 + 225.0 - offset;
    //225.0 + offset + _inv_stat as f64 * ((225.0 + offset) * 2.0 - (225.0 + offset)) / 100.0
    mossy_simple
}
fn rapid_fire_machinegun(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let rapid_fire_mg = 345.0;
    let round_amount = _mag_size - (29.5 + 0.45 * _mag_stat as f64);
    let offset = if _mag_stat as f64 >= 100.0 {
        0.0
    } else {
        (-0.875 + round_amount * 2.0) * (2.0 - ((100.0 - _mag_stat as f64) / 100.0))
    };
    let _mossy_complex = rapid_fire_mg
        + offset
        + _inv_stat as f64 * ((rapid_fire_mg + offset) * 3.45 - (rapid_fire_mg + offset)) / 100.0;
    let mossy_simple = 3.45 * _inv_stat as f64 + 345.0 - offset;
    mossy_simple
}
fn trace_rifle(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = 2.6;
    let offset = 364.0;
    //let mult = 0.025 * _mag_stat as f64 + 3.5;
    vpp * _inv_stat as f64 + offset
}

fn glaives(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = if _mag_stat >= 100 { 0.1681 } else { 0.1792 };
    let offset = if _mag_stat >= 100 { 13.44 } else { 14.44 };
    vpp * _inv_stat as f64 + offset
}

fn sniper_rifles(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = if _mag_stat >= 100 { 0.14 } else { 0.12 };
    let offset = if _mag_stat >= 100 { 14.0 } else { 12.0 };
    vpp * _inv_stat as f64 + offset
}

fn rapid_fire_sniper(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = if _mag_stat >= 100 { 0.182 } else { 0.156 };
    let offset: f64 = if _mag_stat >= 100 { 18.2 } else { 15.6 };
    (vpp * _inv_stat as f64) + offset
}
fn shotguns(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = 0.08;
    let offset = 12.0;
    vpp * _inv_stat as f64 + offset
}

fn rapid_fire_shotgun(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = 0.08;
    let offset = 20.0;
    vpp * _inv_stat as f64 + offset
}
fn linear_fusion_rifle(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let offset = 15.6;
    0.08 * _inv_stat as f64 + offset
}
fn forerunner(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    match _inv_stat {
        56 => 72.0,
        76 => 79.0,
        96 => 85.0,
        _ => 87.0,
    }
}

fn overture(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    match _inv_stat {
        45 => 60.0,
        65 => 63.0,
        85 => 67.0,
        _ => 69.0,
    }
}

fn xenophage(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    match _inv_stat {
        3 => 28.0,
        _ => 34.0,
    }
}

fn eriana_vow(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    match _inv_stat {
        0 => 30.0,
        20 => 34.0,
        40 => 38.0,
        _ => 40.0,
    }
}

fn rockets(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    _inv_stat as f64 * 0.05 + 4.5
}

fn fusions(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let vpp = 0.12;
    let offset = 9.6;
    vpp * _inv_stat as f64 + offset
}
fn leviathans_breath(_inv_stat: i32) -> f64 {
    if _inv_stat >= 80 {
        15.0
    } else {
        8.0
    }
}
fn anarchy(_inv_stat: i32) -> f64 {
    match _inv_stat {
        0 => 23.0,
        20 => 25.0,
        40 => 27.0,
        _ => 28.0,
    }
}
fn arbalest(_inv_stat: i32) -> f64 {
    match _inv_stat {
        34 => 20.0,
        54 => 22.0,
        74 => 23.0,
        _ => 24.0,
    }
}
fn bastion(_inv_stat: i32) -> f64 {
    match _inv_stat {
        30 => 15.0,
        50 => 17.0,
        70 => 20.0,
        _ => 21.0,
    }
}
fn buried_bloodline(_inv_stat: i32) -> f64 {
    match _inv_stat {
        50 => 62.0,
        70 => 67.0,
        90 => 72.0,
        _ => 75.0,
    }
}
fn conditional_finality(_inv_stat: i32) -> f64 {
    match _inv_stat {
        51 => 18.0,
        71 => 20.0,
        91 => 22.0,
        _ => 22.0,
    }
}
fn dead_messenger(_inv_stat: i32) -> f64 {
    match _inv_stat {
        0..=89 => 22.0,
        90 => 23.0,
        _ => 23.0,
    }
}
fn deathbringer(_inv_stat: i32) -> f64 {
    match _inv_stat {
        36 => 9.0,
        56 => 10.0,
        86 => 11.0,
        _ => 11.0,
    }
}
fn delicate_tomb(_inv_stat: i32) -> f64 {
    match _inv_stat {
        55 => 23.0,
        75 => 26.0,
        95 => 29.0,
        _ => 30.0,
    }
}
fn dragons_breath(_inv_stat: i32) -> f64 {
    match _inv_stat {
        50 => 9.0,
        70 => 10.0,
        90 => 11.0,
        _ => 12.0,
    }
}
fn ex_diris(_inv_stat: i32) -> f64 {
    match _inv_stat {
        70 => 32.0,
        _ => 33.0,
    }
}
fn eyes_of_tomorrow(_inv_stat: i32) -> f64 {
    match _inv_stat {
        20 => 8.0,
        40 => 9.0,
        60 => 10.0,
        _ => 10.0,
    }
}
fn gjallarhorn(_inv_stat: i32) -> f64 {
    match _inv_stat {
        50 => 9.0,
        70 => 10.0,
        90 => 11.0,
        _ => 12.0,
    }
}
fn hier_apparent(_inv_stat: i32) -> f64 {
    match _inv_stat {
        50 => 500.0,
        70 => 540.0,
        90 => 580.0,
        _ => 600.0,
    }
}
fn jotunn(_inv_stat: i32) -> f64 {
    match _inv_stat {
        26 => 17.0,
        46 => 20.0,
        66 => 22.0,
        _ => 24.0,
    }
}
fn legend_of_acrius(_inv_stat: i32) -> f64 {
    match _inv_stat {
        0 => 16.0,
        20 => 17.0,
        40 => 19.0,
        _ => 19.0,
    }
}
fn lorentz_driver(_inv_stat: i32) -> f64 {
    match _inv_stat {
        35 => 20.0,
        55 => 21.0,
        75 => 22.0,
        _ => 23.0,
    }
}
fn merciless(_inv_stat: i32) -> f64 {
    match _inv_stat {
        55 => 17.0,
        75 => 19.0,
        95 => 21.0,
        _ => 22.0,
    }
}
fn one_thousand_voices(_inv_stat: i32) -> f64 {
    match _inv_stat {
        80 => 7.0,
        _ => 8.0,
    }
}
fn parasite(_inv_stat: i32) -> f64 {
    match _inv_stat {
        0 => 13.0,
        20 => 15.0,
        40 => 16.0,
        _ => 17.0,
    }
}
fn sleeper_simulant(_inv_stat: i32) -> f64 {
    match _inv_stat {
        10 => 13.0,
        30 => 14.0,
        50 => 16.0,
        _ => 16.0,
    }
}
fn telesto(_inv_stat: i32) -> f64 {
    match _inv_stat {
        55 => 21.0,
        75 => 22.0,
        95 => 22.0,
        _ => 22.0,
    }
}
fn queenbreaker(_inv_stat: i32) -> f64 {
    match _inv_stat {
        40 => 21.0,
        60 => 22.0,
        80 => 24.0,
        _ => 24.0,
    }
}
fn wardcliff_coil(_inv_stat: i32) -> f64 {
    match _inv_stat {
        0 => 6.0,
        20 => 7.0,
        40 => 8.0,
        _ => 8.0,
    }
}
fn tractor_cannon(_inv_stat: i32) -> f64 {
    match _inv_stat {
        0 => 17.0,
        20 => 18.0,
        40 => 20.0,
        _ => 21.0,
    }
}
fn two_tailed_fox(_inv_stat: i32) -> f64 {
    match _inv_stat {
        30 => 8.0,
        50 => 9.0,
        70 => 10.0,
        _ => 10.0,
    }
}