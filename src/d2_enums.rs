use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AmmoType {
    Primary = 1,
    Special = 2,
    Heavy = 3,
    Unknown = 0,
}
impl From<u32> for AmmoType {
    fn from(_value: u32) -> AmmoType {
        match _value {
            1 => AmmoType::Primary,
            2 => AmmoType::Special,
            3 => AmmoType::Heavy,
            _ => AmmoType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatHashes {
    Accuracy,
    AimAssist,
    Airborne,
    AmmoCapacity,
    Attack,
    BlastRadius,
    ChargeRate,
    ChargeTime,
    Discipline,
    DrawTime,
    GuardEfficiency,
    GuardEndurance,
    GuardResistance,
    Handling,
    Impact,
    Itellect,
    InventorySize,
    Magazine,
    Mobility,
    Range,
    RecoilDir,
    Recovery,
    Reload,
    Resilience,
    Rpm,
    ShieldDuration,
    Stability,
    Strength,
    SwingSpeed,
    Velocity,
    Zoom,
    Unknown,
}
impl From<u32> for StatHashes {
    fn from(_value: u32) -> StatHashes {
        match _value {
            1591432999 => StatHashes::Accuracy,
            1345609583 => StatHashes::AimAssist,
            2714457168 => StatHashes::Airborne,
            925767036 => StatHashes::AmmoCapacity,
            1480404414 => StatHashes::Attack,
            3614673599 => StatHashes::BlastRadius,
            3022301683 => StatHashes::ChargeRate,
            2961396640 => StatHashes::ChargeTime,
            1735777505 => StatHashes::Discipline,
            447667954 => StatHashes::DrawTime,
            2762071195 => StatHashes::GuardEfficiency,
            3736848092 => StatHashes::GuardEndurance,
            209426660 => StatHashes::GuardResistance,
            943549884 => StatHashes::Handling,
            4043523819 => StatHashes::Impact,
            144602215 => StatHashes::Itellect,
            1931675084 => StatHashes::InventorySize,
            3871231066 => StatHashes::Magazine,
            2996146975 => StatHashes::Mobility,
            1240592695 => StatHashes::Range,
            2715839340 => StatHashes::RecoilDir,
            1943323491 => StatHashes::Recovery,
            4188031367 => StatHashes::Reload,
            392767087 => StatHashes::Resilience,
            4284893193 => StatHashes::Rpm,
            1842278586 => StatHashes::ShieldDuration,
            155624089 => StatHashes::Stability,
            4244567218 => StatHashes::Strength,
            2837207746 => StatHashes::SwingSpeed,
            2523465841 => StatHashes::Velocity,
            3555269338 => StatHashes::Zoom,
            _ => StatHashes::Unknown,
        }
    }
}
impl From<StatHashes> for u32 {
    fn from(val: StatHashes) -> Self {
        match val {
            StatHashes::Accuracy => 1591432999,
            StatHashes::AimAssist => 1345609583,
            StatHashes::Airborne => 2714457168,
            StatHashes::AmmoCapacity => 925767036,
            StatHashes::Attack => 1480404414,
            StatHashes::BlastRadius => 3614673599,
            StatHashes::ChargeRate => 3022301683,
            StatHashes::ChargeTime => 2961396640,
            StatHashes::Discipline => 1735777505,
            StatHashes::DrawTime => 447667954,
            StatHashes::GuardEfficiency => 2762071195,
            StatHashes::GuardEndurance => 3736848092,
            StatHashes::GuardResistance => 209426660,
            StatHashes::Handling => 943549884,
            StatHashes::Impact => 4043523819,
            StatHashes::Itellect => 144602215,
            StatHashes::InventorySize => 1931675084,
            StatHashes::Magazine => 3871231066,
            StatHashes::Mobility => 2996146975,
            StatHashes::Range => 1240592695,
            StatHashes::RecoilDir => 2715839340,
            StatHashes::Recovery => 1943323491,
            StatHashes::Reload => 4188031367,
            StatHashes::Resilience => 392767087,
            StatHashes::Rpm => 4284893193,
            StatHashes::ShieldDuration => 1842278586,
            StatHashes::Stability => 155624089,
            StatHashes::Strength => 4244567218,
            StatHashes::SwingSpeed => 2837207746,
            StatHashes::Velocity => 2523465841,
            StatHashes::Zoom => 3555269338,
            StatHashes::Unknown => 0,
        }
    }
}
impl StatHashes {
    pub fn is_weapon_stat(&self) -> bool {
        matches!(
            self,
            StatHashes::Accuracy
                | StatHashes::AimAssist
                | StatHashes::Airborne
                | StatHashes::AmmoCapacity
                | StatHashes::Zoom
                | StatHashes::Range
                | StatHashes::Stability
                | StatHashes::Reload
                | StatHashes::Magazine
                | StatHashes::Handling
                | StatHashes::Velocity
                | StatHashes::BlastRadius
                | StatHashes::ChargeTime
                | StatHashes::InventorySize
                | StatHashes::RecoilDir
                | StatHashes::Rpm
                | StatHashes::GuardEfficiency
                | StatHashes::GuardEndurance
                | StatHashes::GuardResistance
                | StatHashes::DrawTime
                | StatHashes::SwingSpeed
                | StatHashes::ShieldDuration
                | StatHashes::Impact
                | StatHashes::ChargeRate
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageType {
    Arc,
    Void,
    Solar,
    Stasis,
    Kinetic,
    Strand,
    Unknown,
}

impl From<u32> for DamageType {
    fn from(_value: u32) -> DamageType {
        match _value {
            2303181850 => DamageType::Arc,
            3454344768 => DamageType::Void,
            1847026933 => DamageType::Solar,
            151347233 => DamageType::Stasis,
            3373582085 => DamageType::Kinetic,
            3949783978 => DamageType::Strand,
            _ => DamageType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageSource {
    Sniper,
    Melee,
    Eplosion,
    Enviromental,
    Unknown,
}

pub type Seconds = f64;
pub type MetersPerSecond = f64;
pub type StatBump = i32;
pub type BungieHash = u32;
