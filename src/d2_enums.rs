#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum AmmoType {
    PRIMARY = 1,
    SPECIAL = 2,
    HEAVY = 3,
    UNKNOWN = 0,
}
impl From<u32> for AmmoType {
    fn from(_value: u32) -> AmmoType {
        match _value {
            1 => AmmoType::PRIMARY,
            2 => AmmoType::SPECIAL,
            3 => AmmoType::HEAVY,
            _ => AmmoType::UNKNOWN,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum WeaponType {
    AUTORIFLE = 6,
    BOW = 31,
    FUSIONRIFLE = 11,
    GLAIVE = 33,
    GRENADELAUNCHER = 23,
    HANDCANNON = 9,
    LINEARFUSIONRIFLE = 22,
    MACHINEGUN = 8,
    PULSERIFLE = 13,
    ROCKET = 10,
    SCOUTRIFLE = 14,
    SHOTGUN = 7,
    SIDEARM = 17,
    SNIPER = 12,
    SUBMACHINEGUN = 24,
    SWORD = 18,
    TRACERIFLE = 25,
    UNKNOWN = 0,
}
impl From<u32> for WeaponType {
    fn from(_value: u32) -> WeaponType {
        match _value {
            6 => WeaponType::AUTORIFLE,
            31 => WeaponType::BOW,
            11 => WeaponType::FUSIONRIFLE,
            33 => WeaponType::GLAIVE,
            23 => WeaponType::GRENADELAUNCHER,
            9 => WeaponType::HANDCANNON,
            22 => WeaponType::LINEARFUSIONRIFLE,
            8 => WeaponType::MACHINEGUN,
            13 => WeaponType::PULSERIFLE,
            10 => WeaponType::ROCKET,
            14 => WeaponType::SCOUTRIFLE,
            7 => WeaponType::SHOTGUN,
            17 => WeaponType::SIDEARM,
            12 => WeaponType::SNIPER,
            24 => WeaponType::SUBMACHINEGUN,
            18 => WeaponType::SWORD,
            25 => WeaponType::TRACERIFLE,
            _ => WeaponType::UNKNOWN,
        }
    }
}

#[allow(non_snake_case, non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatHashes {
    ACCURACY,
    AIM_ASSIST,
    AIRBORNE,
    AMMO_CAPACITY,
    ATTACK,
    BLAST_RADIUS,
    CHARGE_RATE,
    CHARGE_TIME,
    DISCIPLINE,
    DRAW_TIME,
    GUARD_EFFICIENCY,
    GUARD_ENDURANCE,
    GUARD_RESISTANCE,
    HANDLING,
    IMPACT,
    INTELLECT,
    INVENTORY_SIZE,
    MAGAZINE,
    MOBILITY,
    POWER,
    RANGE,
    RECOIL_DIR,
    RECOVERY,
    RELOAD,
    RESILIENCE,
    RPM,
    SHIELD_DURATION,
    STABILITY,
    STRENGTH,
    SWING_SPEED,
    VELOCITY,
    ZOOM,
    UNKNOWN,
}
impl From<u32> for StatHashes {
    fn from(_value: u32) -> StatHashes {
        match _value {
            1591432999 => StatHashes::ACCURACY,
            1345609583 => StatHashes::AIM_ASSIST,
            2714457168 => StatHashes::AIRBORNE,
            925767036 => StatHashes::AMMO_CAPACITY,
            1480404414 => StatHashes::ATTACK,
            3614673599 => StatHashes::BLAST_RADIUS,
            3022301683 => StatHashes::CHARGE_RATE,
            2961396640 => StatHashes::CHARGE_TIME,
            1735777505 => StatHashes::DISCIPLINE,
            447667954 => StatHashes::DRAW_TIME,
            2762071195 => StatHashes::GUARD_EFFICIENCY,
            3736848092 => StatHashes::GUARD_ENDURANCE,
            209426660 => StatHashes::GUARD_RESISTANCE,
            943549884 => StatHashes::HANDLING,
            4043523819 => StatHashes::IMPACT,
            144602215 => StatHashes::INTELLECT,
            1931675084 => StatHashes::INVENTORY_SIZE,
            3871231066 => StatHashes::MAGAZINE,
            2996146975 => StatHashes::MOBILITY,
            1935470627 => StatHashes::POWER,
            1240592695 => StatHashes::RANGE,
            2715839340 => StatHashes::RECOIL_DIR,
            1943323491 => StatHashes::RECOVERY,
            4188031367 => StatHashes::RELOAD,
            392767087 => StatHashes::RESILIENCE,
            4284893193 => StatHashes::RPM,
            1842278586 => StatHashes::SHIELD_DURATION,
            155624089 => StatHashes::STABILITY,
            4244567218 => StatHashes::STRENGTH,
            2837207746 => StatHashes::SWING_SPEED,
            2523465841 => StatHashes::VELOCITY,
            3555269338 => StatHashes::ZOOM,
            _ => StatHashes::UNKNOWN,
        }
    }
}
impl From<StatHashes> for u32 {
    fn from(val: StatHashes) -> Self {
        match val {
            StatHashes::ACCURACY => 1591432999,
            StatHashes::AIM_ASSIST => 1345609583,
            StatHashes::AIRBORNE => 2714457168,
            StatHashes::AMMO_CAPACITY => 925767036,
            StatHashes::ATTACK => 1480404414,
            StatHashes::BLAST_RADIUS => 3614673599,
            StatHashes::CHARGE_RATE => 3022301683,
            StatHashes::CHARGE_TIME => 2961396640,
            StatHashes::DISCIPLINE => 1735777505,
            StatHashes::DRAW_TIME => 447667954,
            StatHashes::GUARD_EFFICIENCY => 2762071195,
            StatHashes::GUARD_ENDURANCE => 3736848092,
            StatHashes::GUARD_RESISTANCE => 209426660,
            StatHashes::HANDLING => 943549884,
            StatHashes::IMPACT => 4043523819,
            StatHashes::INTELLECT => 144602215,
            StatHashes::INVENTORY_SIZE => 1931675084,
            StatHashes::MAGAZINE => 3871231066,
            StatHashes::MOBILITY => 2996146975,
            StatHashes::POWER => 1935470627,
            StatHashes::RANGE => 1240592695,
            StatHashes::RECOIL_DIR => 2715839340,
            StatHashes::RECOVERY => 1943323491,
            StatHashes::RELOAD => 4188031367,
            StatHashes::RESILIENCE => 392767087,
            StatHashes::RPM => 4284893193,
            StatHashes::SHIELD_DURATION => 1842278586,
            StatHashes::STABILITY => 155624089,
            StatHashes::STRENGTH => 4244567218,
            StatHashes::SWING_SPEED => 2837207746,
            StatHashes::VELOCITY => 2523465841,
            StatHashes::ZOOM => 3555269338,
            StatHashes::UNKNOWN => 0,
        }
    }
}
impl StatHashes {
    pub fn is_weapon_stat(&self) -> bool {
        match self {
            StatHashes::ACCURACY => true,
            StatHashes::AIM_ASSIST => true,
            StatHashes::AIRBORNE => true,
            StatHashes::AMMO_CAPACITY => true,
            StatHashes::ZOOM => true,
            StatHashes::RANGE => true,
            StatHashes::STABILITY => true,
            StatHashes::RELOAD => true,
            StatHashes::MAGAZINE => true,
            StatHashes::HANDLING => true,
            StatHashes::VELOCITY => true,
            StatHashes::BLAST_RADIUS => true,
            StatHashes::CHARGE_TIME => true,
            StatHashes::INVENTORY_SIZE => true,
            StatHashes::RECOIL_DIR => true,
            StatHashes::RPM => true,
            StatHashes::GUARD_EFFICIENCY => true,
            StatHashes::GUARD_ENDURANCE => true,
            StatHashes::GUARD_RESISTANCE => true,
            StatHashes::DRAW_TIME => true,
            StatHashes::SWING_SPEED => true,
            StatHashes::SHIELD_DURATION => true,
            StatHashes::IMPACT => true,
            StatHashes::CHARGE_RATE => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum DamageType {
    ARC,
    VOID,
    SOLAR,
    STASIS,
    KINETIC,
    STRAND,
    UNKNOWN,
}

impl From<u32> for DamageType {
    fn from(_value: u32) -> DamageType {
        match _value {
            2303181850 => DamageType::ARC,
            3454344768 => DamageType::VOID,
            1847026933 => DamageType::SOLAR,
            151347233 => DamageType::STASIS,
            3373582085 => DamageType::KINETIC,
            3949783978 => DamageType::STRAND,
            _ => DamageType::UNKNOWN,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum DamageSource {
    SNIPER,
    MELEE,
    EXPLOSION,
    ENVIRONMENTAL,
    UNKNOWN,
}

pub type Seconds = f64;
pub type MetersPerSecond = f64;
pub type StatBump = i32;
pub type BungieHash = u32;
