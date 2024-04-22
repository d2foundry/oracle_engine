#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ReserveIDs {
    Primary,
    LeviathansBreath,
    Fusions,
    SpecialGrenadeLaunchers,
    SmallGrenadeLaunchers,
    LargeGrenadeLaunchers,
    LinearFusions,
    SmallMachineGuns,
    LargeMachineGuns,
    RocketLaunchers,
    Shotguns,
    SniperRifles,
    Glaive,
    TraceRifles,
    RapidFireSniper,
    RapidFireShotgun,
    HighInventoryRockets,
    AggressiveLinearFusionRifle,
    RocketAssistedFrame,

    //kinetic exotic special
    Arbalest,
    Bastion,
    ConditionalFinality,
    DelicateTomb,
    ForeRunner,

    //energy exotic special
    BuriedBloodline,
    DeadMessenger,
    ExDiris,
    Jotunn,
    LordOfWolves,
    LorentzDriver,
    Merciless,
    Telesto,
    ErianasVow,
    Tessellation,

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
    Truth,
    TwoTailedFox,
    Winterbite,
    WhisperOfTheWorm,
    Xenophage,
    Overture,
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
            2201 => ReserveIDs::LinearFusions,
            81 => ReserveIDs::SmallMachineGuns,
            82 => ReserveIDs::LargeMachineGuns,
            101 => ReserveIDs::RocketLaunchers,
            71 => ReserveIDs::Shotguns,
            121 => ReserveIDs::SniperRifles,
            331 => ReserveIDs::Glaive,
            251 => ReserveIDs::TraceRifles,
            1201 => ReserveIDs::RapidFireSniper,
            701 => ReserveIDs::RapidFireShotgun,
            1002 => ReserveIDs::HighInventoryRockets,
            2202 => ReserveIDs::AggressiveLinearFusionRifle,
            1701 => ReserveIDs::RocketAssistedFrame,

            //kinetic exotic special
            2984682260 => ReserveIDs::ForeRunner,
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
            2769013282 => ReserveIDs::Tessellation,
            3174300811 => ReserveIDs::ErianasVow,

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
            2491817779 => ReserveIDs::Truth,
            3649430342 => ReserveIDs::TwoTailedFox,
            1207608520 => ReserveIDs::Winterbite,
            281315705 => ReserveIDs::WhisperOfTheWorm,
            2261491232 => ReserveIDs::Xenophage,
            2940035732 => ReserveIDs::Overture,

            _ => ReserveIDs::Primary,
        }
    }
}

pub fn calc_reserves(mag_size: f64, mag_stat: i32, inv_stat: i32, id: u32, scale: f64) -> i32 {
    let id = ReserveIDs::from(id);
    let raw_size: f64 = match id {
        ReserveIDs::Primary => 9999.0,
        ReserveIDs::SmallMachineGuns => small_machinegun(mag_size, mag_stat, inv_stat),
        ReserveIDs::TraceRifles => trace_rifle(mag_size, inv_stat),
        ReserveIDs::Glaive => glaives(mag_stat, inv_stat),
        ReserveIDs::SniperRifles => sniper_rifles(mag_stat, inv_stat),
        ReserveIDs::Shotguns => shotguns(mag_size, inv_stat),
        ReserveIDs::RocketLaunchers => rockets(inv_stat),
        ReserveIDs::RapidFireSniper => rapid_fire_sniper(mag_stat, inv_stat),
        ReserveIDs::RapidFireShotgun => rapid_fire_shotgun(mag_size, inv_stat),
        ReserveIDs::Fusions => fusions(inv_stat),
        ReserveIDs::LinearFusions => linear_fusion_rifle(inv_stat),
        ReserveIDs::LargeMachineGuns => rapid_fire_machinegun(mag_size, inv_stat),
        ReserveIDs::HighInventoryRockets => high_inventory_rockets(inv_stat),
        ReserveIDs::AggressiveLinearFusionRifle => {
            aggressive_linear_fusion_rifle(mag_stat, inv_stat)
        }
        ReserveIDs::SpecialGrenadeLaunchers => {
            special_grenade_launcher(inv_stat)
        }
        ReserveIDs::SmallGrenadeLaunchers => {
            adaptive_grenade_launcher(inv_stat)
        }
        ReserveIDs::LargeGrenadeLaunchers => {
            rapid_grenade_launcher(inv_stat)
        }
        ReserveIDs::RocketAssistedFrame => rocket_assisted(inv_stat),

        //exotic kinetic special
        ReserveIDs::ForeRunner => forerunner(inv_stat),
        ReserveIDs::Arbalest => arbalest(inv_stat),
        ReserveIDs::Bastion => bastion(inv_stat),

        //exotic energy special
        ReserveIDs::BuriedBloodline => buried_bloodline(inv_stat),
        ReserveIDs::ErianasVow => eriana_vow(inv_stat),
        ReserveIDs::ConditionalFinality => conditional_finality(inv_stat),
        ReserveIDs::DeadMessenger => dead_messenger(inv_stat),
        ReserveIDs::DelicateTomb => delicate_tomb(inv_stat),
        ReserveIDs::ExDiris => ex_diris(inv_stat),
        ReserveIDs::Jotunn => jotunn(inv_stat),
        ReserveIDs::LordOfWolves => 100.0,
        ReserveIDs::LorentzDriver => lorentz_driver(inv_stat),
        ReserveIDs::Merciless => merciless(inv_stat),
        ReserveIDs::Telesto => telesto(inv_stat),
        ReserveIDs::Tessellation => tessellation(inv_stat),

        //exotic heavy
        ReserveIDs::Anarchy => anarchy(inv_stat),
        ReserveIDs::DeathBringer => deathbringer(inv_stat),
        ReserveIDs::DragonsBreath => dragons_breath(inv_stat),
        ReserveIDs::EyesOfTomorrow => eyes_of_tomorrow(inv_stat),
        ReserveIDs::Gjallarhorn => gjallarhorn(inv_stat),
        ReserveIDs::HierApparent => hier_apparent(inv_stat),
        ReserveIDs::LegendOfAcrius => legend_of_acrius(inv_stat),
        ReserveIDs::OneThousandVoices => one_thousand_voices(inv_stat),
        ReserveIDs::Overture => overture(inv_stat),
        ReserveIDs::SleeperSimulant => sleeper_simulant(inv_stat),
        ReserveIDs::Xenophage => xenophage(inv_stat),
        ReserveIDs::LeviathansBreath => leviathans_breath(inv_stat),
        ReserveIDs::Parasite => parasite(inv_stat),
        ReserveIDs::TheColony => 28.0,
        ReserveIDs::TheProspector => 35.0,
        ReserveIDs::TheQueenbreaker => queenbreaker(inv_stat),
        ReserveIDs::TheWardcliffCoil => wardcliff_coil(inv_stat),
        ReserveIDs::TractorCannon => tractor_cannon(inv_stat),
        ReserveIDs::Truth => truth(inv_stat),
        ReserveIDs::TwoTailedFox => two_tailed_fox(inv_stat),
        ReserveIDs::Winterbite => winterbite(inv_stat),
        ReserveIDs::WhisperOfTheWorm => whisper_of_the_worm(mag_stat, inv_stat),
    };
    let size = raw_size * scale;
    size.ceil() as i32
}

fn small_machinegun(mag_size: f64, mag_stat: i32, inv_stat: i32) -> f64 {
    let base_machine_gun = 225.0;
    let round_amount = mag_size.ceil() - mag_size;
    let offset = (-0.875 + round_amount * 2.0) * (2.0 - ((100.0 - mag_stat as f64) / 100.0)) * 1.5;

    base_machine_gun
        + offset
        + inv_stat as f64 * ((base_machine_gun + offset) * 2.0 - (base_machine_gun + offset))
            / 100.0
}
fn rapid_fire_machinegun(mag_size: f64, inv_stat: i32) -> f64 {
    let rapid_fire_mg = 345.0;
    let round_amount = mag_size.ceil() - mag_size;
    let offset = (-0.25 + round_amount * 2.85) * 1.5;

    rapid_fire_mg
        + offset
        + inv_stat as f64 * ((rapid_fire_mg + offset) * 2.0 - (rapid_fire_mg + offset)) / 100.0
}
fn trace_rifle(mag_size: f64, inv_stat: i32) -> f64 {
    let mult = inv_stat as f64 * 0.025 + 3.5;
    mag_size.ceil() * mult
}

fn glaives(mag_stat: i32, inv_stat: i32) -> f64 {
    let vpp = if mag_stat >= 100 { 0.16875 } else { 0.18 };
    let offset = if mag_stat >= 100 { 13.5 } else { 14.4 };
    vpp * inv_stat as f64 + offset
}

fn sniper_rifles(mag_stat: i32, inv_stat: i32) -> f64 {
    let vpp = if mag_stat >= 100 { 0.14 } else { 0.12 };
    let offset = if mag_stat >= 100 { 14.0 } else { 12.0 };
    vpp * inv_stat as f64 + offset
}
fn whisper_of_the_worm(mag_stat: i32, inv_stat: i32) -> f64 {
    let vpp = if mag_stat >= 100 { 0.14 } else { 0.12 };
    let offset = if mag_stat >= 100 { 20.0 } else { 18.0 };
    vpp * inv_stat as f64 + offset
}
fn rapid_fire_sniper(mag_stat: i32, inv_stat: i32) -> f64 {
    let vpp = if mag_stat >= 100 { 0.182 } else { 0.156 };
    let offset: f64 = if mag_stat >= 100 { 18.2 } else { 15.6 };
    (vpp * inv_stat as f64) + offset
}
fn shotguns(mag_size: f64, inv_stat: i32) -> f64 {
    let offset = match mag_size.ceil() as i32 {
        4 => 14.0,
        5 => 13.133,
        6 => 12.6,
        7 => 12.267,
        8 => 12.0,
        _ => 12.0,
    };

    let vpp = ((offset * (5.0 / 3.0)) - offset) / 100.0;
    vpp * inv_stat as f64 + offset
}

fn rapid_fire_shotgun(mag_size: f64, inv_stat: i32) -> f64 {
    let offset = match mag_size.ceil() as i32 {
        4 => 14.0,
        5 => 13.133,
        6 => 12.6,
        7 => 12.267,
        8 => 12.0,
        _ => 12.0,
    };

    let vpp = ((offset * (5.0 / 3.0)) - offset) / 100.0;
    vpp * inv_stat as f64 + offset + 8.0
}
fn linear_fusion_rifle(inv_stat: i32) -> f64 {
    let offset = 15.6;
    0.08 * inv_stat as f64 + offset
}
fn rockets(inv_stat: i32) -> f64 {
    inv_stat as f64 * 0.05 + 4.5
}
fn high_inventory_rockets(inv_stat: i32) -> f64 {
    inv_stat as f64 * 0.05 + 6.5
}
fn fusions(inv_stat: i32) -> f64 {
    let vpp = 0.12;
    let offset = 9.6;
    vpp * inv_stat as f64 + offset
}
fn aggressive_linear_fusion_rifle(mag_stat: i32, inv_stat: i32) -> f64 {
    let offset = match mag_stat {
        0..=69 => 16.5,
        70..=90 => 16.0,
        91..=100 => 15.5,
        _ => 15.5,
    };
    let vpp = ((offset * 1.4375) - offset) / 100.0;
    vpp * inv_stat as f64 + offset
}
fn rocket_assisted(inv_stat: i32) -> f64 {
    let offset = 15.6;
    0.08 * inv_stat as f64 + offset
}
fn heavy_compressed_wave(inv_stat: i32) -> f64 {
    let offset = 20.6;
    0.075 * inv_stat as f64 + offset
}
fn special_grenade_launcher(inv_stat: i32) -> f64 {
    let vpp = 0.05;
    let offset = 18.0;
    vpp * inv_stat as f64 + offset
}
fn adaptive_grenade_launcher(inv_stat: i32) -> f64 {
    let vpp = 0.08;
    let offset = 20.0;
    vpp * inv_stat as f64 + offset
}
fn rapid_grenade_launcher(inv_stat: i32) -> f64 {
    let vpp = 0.1;
    let offset = 25.0;
    vpp * inv_stat as f64 + offset
}
fn forerunner(inv_stat: i32) -> f64 {
    match inv_stat {
        56 => 72.0,
        76 => 79.0,
        96 => 85.0,
        _ => 87.0,
    }
}

fn overture(inv_stat: i32) -> f64 {
    match inv_stat {
        45 => 60.0,
        65 => 63.0,
        85 => 67.0,
        _ => 69.0,
    }
}

fn xenophage(inv_stat: i32) -> f64 {
    match inv_stat {
        3 => 28.0,
        _ => 34.0,
    }
}

fn eriana_vow(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 30.0,
        20 => 34.0,
        40 => 38.0,
        _ => 40.0,
    }
}
fn leviathans_breath(inv_stat: i32) -> f64 {
    if inv_stat >= 80 {
        15.0
    } else {
        8.0
    }
}
fn anarchy(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 23.0,
        20 => 25.0,
        40 => 27.0,
        _ => 28.0,
    }
}
fn arbalest(inv_stat: i32) -> f64 {
    match inv_stat {
        34 => 20.0,
        54 => 22.0,
        74 => 23.0,
        _ => 24.0,
    }
}
fn bastion(inv_stat: i32) -> f64 {
    match inv_stat {
        30 => 15.0,
        50 => 17.0,
        70 => 20.0,
        _ => 21.0,
    }
}
fn buried_bloodline(inv_stat: i32) -> f64 {
    match inv_stat {
        50 => 62.0,
        70 => 67.0,
        90 => 72.0,
        _ => 75.0,
    }
}
fn conditional_finality(inv_stat: i32) -> f64 {
    match inv_stat {
        51 => 18.0,
        71 => 20.0,
        91 => 22.0,
        _ => 22.0,
    }
}
fn dead_messenger(inv_stat: i32) -> f64 {
    match inv_stat {
        0..=89 => 22.0,
        90 => 23.0,
        _ => 23.0,
    }
}
fn deathbringer(inv_stat: i32) -> f64 {
    match inv_stat {
        36 => 9.0,
        56 => 10.0,
        86 => 11.0,
        _ => 11.0,
    }
}
fn delicate_tomb(inv_stat: i32) -> f64 {
    match inv_stat {
        55 => 23.0,
        75 => 26.0,
        95 => 29.0,
        _ => 30.0,
    }
}
fn dragons_breath(inv_stat: i32) -> f64 {
    match inv_stat {
        50 => 9.0,
        70 => 10.0,
        90 => 11.0,
        _ => 12.0,
    }
}
fn ex_diris(inv_stat: i32) -> f64 {
    match inv_stat {
        70 => 32.0,
        _ => 33.0,
    }
}
fn eyes_of_tomorrow(inv_stat: i32) -> f64 {
    match inv_stat {
        20 => 8.0,
        40 => 9.0,
        60 => 10.0,
        _ => 10.0,
    }
}
fn gjallarhorn(inv_stat: i32) -> f64 {
    match inv_stat {
        50 => 9.0,
        70 => 10.0,
        90 => 11.0,
        _ => 12.0,
    }
}
fn hier_apparent(inv_stat: i32) -> f64 {
    match inv_stat {
        50 => 500.0,
        70 => 540.0,
        90 => 580.0,
        _ => 600.0,
    }
}
fn jotunn(inv_stat: i32) -> f64 {
    match inv_stat {
        26 => 17.0,
        46 => 20.0,
        66 => 22.0,
        _ => 24.0,
    }
}
fn legend_of_acrius(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 16.0,
        20 => 17.0,
        40 => 19.0,
        _ => 19.0,
    }
}
fn lorentz_driver(inv_stat: i32) -> f64 {
    match inv_stat {
        35 => 20.0,
        55 => 21.0,
        75 => 22.0,
        _ => 23.0,
    }
}
fn merciless(inv_stat: i32) -> f64 {
    match inv_stat {
        55 => 17.0,
        75 => 19.0,
        95 => 21.0,
        _ => 22.0,
    }
}
fn one_thousand_voices(inv_stat: i32) -> f64 {
    match inv_stat {
        80 => 11.0,
        _ => 12.0,
    }
}
fn parasite(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 13.0,
        20 => 15.0,
        40 => 16.0,
        _ => 17.0,
    }
}
fn sleeper_simulant(inv_stat: i32) -> f64 {
    match inv_stat {
        10 => 13.0,
        30 => 14.0,
        50 => 16.0,
        _ => 16.0,
    }
}
fn telesto(inv_stat: i32) -> f64 {
    match inv_stat {
        55 => 21.0,
        75 => 22.0,
        95 => 22.0,
        _ => 22.0,
    }
}
fn tessellation(inv_stat: i32) -> f64 {
    match inv_stat {
        33 => 16.0,
        53 => 19.0,
        73 => 21.0,
        _ => 23.0,
    }
}
fn queenbreaker(inv_stat: i32) -> f64 {
    match inv_stat {
        40 => 21.0,
        60 => 22.0,
        80 => 24.0,
        _ => 24.0,
    }
}
fn wardcliff_coil(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 6.0,
        20 => 7.0,
        40 => 8.0,
        _ => 8.0,
    }
}
fn tractor_cannon(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 17.0,
        20 => 18.0,
        40 => 20.0,
        _ => 21.0,
    }
}
fn truth(inv_stat: i32) -> f64 {
    match inv_stat {
        40 => 9.0,
        60 => 10.0,
        80 => 11.0,
        _ => 11.0,
    }
}
fn two_tailed_fox(inv_stat: i32) -> f64 {
    match inv_stat {
        30 => 8.0,
        50 => 9.0,
        70 => 10.0,
        _ => 10.0,
    }
}
fn winterbite(inv_stat: i32) -> f64 {
    match inv_stat {
        0 => 9.0,
        20 => 12.0,
        40 => 15.0,
        _ => 17.0,
    }
}
