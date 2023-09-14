use serde::{Serialize, Deserialize};


///Describes the addresses of all the formulas for a given weapon
/// based off the weapon path
#[derive(Debug, Clone)]
pub struct DataPointers {
    pub h: usize,
    pub r: usize,
    pub rl: usize,
    pub s: usize,
    pub f: usize,
    pub a: usize,
}

///The path to a weapon in the database
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct WeaponPath(
    //type id
    pub u32,
    //intrinsic hash
    pub u32
);

//even if just linear use this
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct StatQuadraticFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
impl StatQuadraticFormula {
    pub fn solve_at(&self, _x: f64) -> f64 {
        self.evpp * _x * _x + self.vpp * _x + self.offset
    }

    pub fn solve_at_i(&self, x: i32) -> f64 {
        let x = x.clamp(0, 100) as f64;
        self.evpp * x * x + self.vpp * x + self.offset
    }
}

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct FiringDataFormula {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub inner_burst_delay: f64,
    pub burst_size: i32,
    pub one_ammo: bool,
    pub charge: bool,
    #[serde(skip_deserializing)]
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct DamageModFormula {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
    #[serde(skip_deserializing)]
    pub timestamp: u64,
}
impl Default for DamageModFormula {
    fn default() -> Self {
        DamageModFormula {
            pve: 1.0,
            minor: 1.0,
            elite: 1.0,
            miniboss: 1.0,
            champion: 1.0,
            boss: 1.0,
            vehicle: 1.0,
            timestamp: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct RangeFormula {
    pub start: StatQuadraticFormula,
    pub end: StatQuadraticFormula,
    pub floor_percent: f64,
    pub fusion: bool,
    #[serde(skip_deserializing)]
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct ReloadFormula {
    pub reload_data: StatQuadraticFormula,
    pub ammo_percent: f64,
    #[serde(skip_deserializing)]
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct HandlingFormula {
    pub ready: StatQuadraticFormula,
    pub stow: StatQuadraticFormula,
    pub ads: StatQuadraticFormula,
    #[serde(skip_deserializing)]
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all(serialize = "camelCase"))]
pub struct AmmoFormula {
    pub mag: StatQuadraticFormula,
    pub round_to: i32,
    pub reserve_id: u32,
    #[serde(skip_deserializing)]
    pub timestamp: u64,
}