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
    LordOfWolves,
    ForeRunner,
    SniperRifles,
    Glaive,
    TraceRifles,
}
impl From<u32> for ReserveIDs {
    fn from(id: u32) -> Self {
        match id {
            0 => ReserveIDs::Primary,
            1699724249 => ReserveIDs::LeviathansBreath,
            111 => ReserveIDs::Fusions,
            231 => ReserveIDs::LargeGrenadeLaunchers,
            232 => ReserveIDs::SpecialGrenadeLaunchers,
            233 => ReserveIDs::SmallGrenadeLaunchers,
            3174300811 => ReserveIDs::ErianasVow,
            221 => ReserveIDs::LinearFusions,
            81 => ReserveIDs::SmallMachineGuns,
            82 => ReserveIDs::LargeMachineGuns,
            2261491232 => ReserveIDs::Xenophage,
            2940035732 => ReserveIDs::Overture,
            101 => ReserveIDs::RocketLaunchers,
            71 => ReserveIDs::Shotguns,
            481338655 => ReserveIDs::LordOfWolves,
            2984682260 => ReserveIDs::ForeRunner,
            121 => ReserveIDs::SniperRifles,
            331 => ReserveIDs::Glaive,
            251 => ReserveIDs::TraceRifles,
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
        ReserveIDs::Xenophage => xenophage(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::Overture => overture(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::ForeRunner => forerunner(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::ErianasVow => eriana_vow(_mag_size, _mag_stat, _inv_stat),
        ReserveIDs::RocketLaunchers => rockets(_mag_size, _mag_stat, _inv_stat),

        //placeholders
        ReserveIDs::LeviathansBreath => 8.0,
        ReserveIDs::Fusions => 21.0,
        ReserveIDs::SmallGrenadeLaunchers => 18.0,
        ReserveIDs::LargeGrenadeLaunchers => 20.0,
        ReserveIDs::SpecialGrenadeLaunchers => 21.0,
        ReserveIDs::LinearFusions => 21.0,
        ReserveIDs::LargeMachineGuns => 400.0,
        ReserveIDs::LordOfWolves => 120.0,
    };
    let size = raw_size * _scale;
    size.ceil() as i32
}

fn small_machinegun(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let round_amount = _mag_size.ceil() - _mag_size;
    let offset = (-0.875 + round_amount * 2.0) * (2.0 - ((100.0 - _mag_stat as f64) / 100.0));
    
    225.0 + offset + _inv_stat as f64 * ((225.0 + offset) * 2.0 - (225.0 + offset)) / 100.0
}

fn trace_rifle(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let mult = _inv_stat as f64 * 0.025 + 3.5;
    _mag_size * mult
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

fn shotguns(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let real_mag_size = _mag_size.ceil() as i32;
    let base_offset = match real_mag_size {
        8 => 0.0,
        7 => 4.0,
        6 => 9.0,
        5 => 17.0,
        4 => 30.0,
        _ => 0.0,
    };
    let base = (base_offset / 15.0) + 12.0;
    let mult_vpp = (2.0 / 3.0) / 100.0;
    base * (1.0 + mult_vpp * _inv_stat as f64)
}

fn forerunner(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    _inv_stat as f64 * 0.325 + 53.45
}

fn overture(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let inv_stat = _inv_stat as f64;
    0.005 * (inv_stat * inv_stat) + inv_stat * -0.4 + 67.375
}

fn xenophage(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let inv_stat = _inv_stat as f64;
    0.01 * (inv_stat * inv_stat) + inv_stat * 0.56 + 25.91
}

fn eriana_vow(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    let inv_stat = _inv_stat as f64;
    -0.00126 * (inv_stat * inv_stat) + inv_stat * 0.225 + 29.5
}

fn rockets(_mag_size: f64, _mag_stat: i32, _inv_stat: i32) -> f64 {
    _inv_stat as f64 * 0.05 + 4.5
}
