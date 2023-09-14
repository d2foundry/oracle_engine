// TODO: get dps back in while coping with it wont be perfect
//      under current system
// pub mod dps_calc;
pub mod reserve_calc;
pub mod stat_calc;
pub mod ttk_calc;
pub mod weapon_constructor;

use std::collections::HashMap;

use serde::Serialize;

use crate::d2_enums::{AmmoType, DamageType, WeaponType};
use crate::perks::{get_stat_bumps, lib::CalculationInput, Perk};

use crate::types::formula_types::{
    AmmoFormula, DamageModFormula, FiringDataFormula, HandlingFormula, RangeFormula, ReloadFormula,
};


#[derive(Debug, Clone, Copy, Serialize)]
pub struct Stat {
    pub base_value: i32,
    pub part_value: i32,
    pub perk_value: i32,
}
impl Stat {
    pub fn new() -> Stat {
        Stat {
            base_value: 0,
            part_value: 0,
            perk_value: 0,
        }
    }
    pub fn val(&self) -> i32 {
        (self.base_value + self.part_value).clamp(0, 100)
    }
    pub fn perk_val(&self) -> i32 {
        (self.base_value + self.part_value + self.perk_value).clamp(0, 100)
    }
}

impl Default for Stat {
    fn default() -> Self {
        Self::new()
    }
}

impl From<i32> for Stat {
    fn from(_val: i32) -> Self {
        Stat {
            base_value: _val,
            part_value: 0,
            perk_value: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Weapon {
    pub hash: u32,
    pub intrinsic_hash: u32,

    pub perks: HashMap<u32, Perk>,
    pub stats: HashMap<u32, Stat>,
    #[serde(skip)]
    pub perk_value_map: HashMap<u32, u32>,

    pub damage_mods: DamageModFormula,
    pub firing_data: FiringDataFormula,
    pub range_formula: RangeFormula,
    pub ammo_formula: AmmoFormula,
    pub handling_formula: HandlingFormula,
    pub reload_formula: ReloadFormula,

    pub weapon_type: WeaponType,
    pub damage_type: DamageType,
    pub ammo_type: AmmoType,
}
impl Weapon {
    pub fn add_perk(&mut self, _perk: Perk) {
        self.perks.insert(_perk.hash, _perk);
        self.update_stats();
    }
    pub fn remove_perk(&mut self, _perk_hash: u32) {
        self.perks.remove(&_perk_hash);
        self.update_stats();
    }
    pub fn reset_perks(&mut self) {
        self.perks = HashMap::new();
        self.update_stats();
    }
    pub fn list_perk_ids(&self) -> Vec<u32> {
        self.perks.keys().cloned().collect()
    }
    pub fn list_perks(&self) -> Vec<Perk> {
        let mut perk_list: Vec<Perk> = Vec::new();
        for perk in self.perks.values() {
            perk_list.push(perk.clone());
        }
        perk_list
    }
    pub fn perk_value_map_update(&self) -> HashMap<u32, u32> {
        let mut perk_map: HashMap<u32, u32> = HashMap::new();
        for perk in self.perks.values() {
            perk_map.insert(perk.hash, perk.value);
        }
        perk_map
    }
    pub fn change_perk_val(&mut self, _perk_hash: u32, _val: u32) {
        let perk_opt = self.perks.get_mut(&_perk_hash);
        if let Some(perk) = perk_opt {
            perk.value = _val;
        }
        self.update_stats();
    }
    pub fn get_stats(&mut self) -> HashMap<u32, Stat> {
        self.update_stats();
        self.stats.clone()
    }
    pub fn set_stats(&mut self, _stats: HashMap<u32, Stat>) {
        self.stats = _stats;
        self.update_stats()
    }
    pub fn reset(&mut self) {
        self.perks = HashMap::new();
        self.stats = HashMap::new();
        self.hash = 0;
        self.damage_mods = DamageModFormula::default();
        self.firing_data = FiringDataFormula::default();
        self.range_formula = RangeFormula::default();
        self.ammo_formula = AmmoFormula::default();
        self.handling_formula = HandlingFormula::default();
        self.reload_formula = ReloadFormula::default();
    }

    pub fn static_calc_input(&self) -> CalculationInput {
        CalculationInput::construct_static(
            self.intrinsic_hash,
            &self.firing_data,
            &self.stats,
            &self.perk_value_map,
            &self.weapon_type,
            &self.ammo_type,
            &self.damage_type,
            self.firing_data.crit_mult,
        )
    }

    pub fn sparse_calc_input(&self, _total_shots_fired: i32, _total_time: f64) -> CalculationInput {
        CalculationInput::construct_pve_sparse(
            self.intrinsic_hash,
            &self.firing_data,
            &self.stats,
            &self.perk_value_map,
            &self.weapon_type,
            &self.ammo_type,
            &self.damage_type,
            self.firing_data.damage,
            self.firing_data.crit_mult,
            self.calc_ammo_sizes(None, None, false).mag_size,
            _total_shots_fired,
            _total_time,
        )
    }
    pub fn pvp_calc_input(
        &self,
        _total_shots_fired: f64,
        _total_shots_hit: f64,
        _total_time: f64,
        _overshield: bool,
    ) -> CalculationInput {
        let base_mag = self.calc_ammo_sizes(None, None, true).mag_size as f64;
        let mut tmp = CalculationInput::construct_pvp(
            self.intrinsic_hash,
            &self.firing_data,
            &self.stats,
            &self.perk_value_map,
            &self.weapon_type,
            &self.ammo_type,
            self.firing_data.damage,
            self.firing_data.crit_mult,
            base_mag,
            _overshield,
            self.calc_handling_times(None, None, true),
        );
        tmp.time_this_mag = _total_time;
        tmp.time_total = _total_time;
        tmp.shots_fired_this_mag = _total_shots_fired;
        tmp.total_shots_fired = _total_shots_fired;
        tmp.total_shots_hit = _total_shots_hit;
        tmp.damage_type = &self.damage_type;
        tmp
    }
    pub fn update_stats(&mut self) {
        self.perk_value_map = self.perk_value_map_update();
        let input = CalculationInput::construct_static(
            self.intrinsic_hash,
            &self.firing_data,
            &self.stats,
            &self.perk_value_map,
            &self.weapon_type,
            &self.ammo_type,
            &self.damage_type,
            self.firing_data.crit_mult,
        );
        let inter_var = get_stat_bumps(self.list_perks(), input, false, &mut HashMap::new());
        let dynamic_stats = &inter_var[0];
        let static_stats = &inter_var[1];
        for (key, stat) in &mut self.stats {
            let a = static_stats.get(key);
            let b = dynamic_stats.get(key);
            if let Some(value) = a {
                stat.part_value = *value;
            }
            if let Some(value) = b {
                stat.perk_value = *value;
            }
        }
    }
}
impl Default for Weapon {
    fn default() -> Weapon {
        Weapon {
            intrinsic_hash: 0,
            hash: 0,

            perks: HashMap::new(),
            stats: HashMap::new(),
            perk_value_map: HashMap::new(),

            damage_mods: DamageModFormula::default(),
            firing_data: FiringDataFormula::default(),

            range_formula: RangeFormula::default(),
            ammo_formula: AmmoFormula::default(),
            handling_formula: HandlingFormula::default(),
            reload_formula: ReloadFormula::default(),

            weapon_type: WeaponType::UNKNOWN,
            damage_type: DamageType::UNKNOWN,
            ammo_type: AmmoType::UNKNOWN,
        }
    }
}

// //making this separate for organization
// impl Weapon {
//     pub fn get_damage(&self)
// }
