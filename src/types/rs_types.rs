use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enemies::EnemyType;

#[derive(Debug, Clone)]
pub struct DataPointers {
    pub h: usize,
    pub r: usize,
    pub rl: usize,
    pub s: usize,
    pub f: usize,
    pub a: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct WeaponPath(pub u32, pub u32);

#[derive(Debug, Clone, Default, Copy, Serialize)]
pub struct FiringData {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub inner_burst_delay: f64,
    pub burst_size: i32,
    pub one_ammo: bool,
    pub charge: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DamageMods {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
    pub timestamp: u64,
}
impl Default for DamageMods {
    fn default() -> Self {
        DamageMods {
            pve: 1.0,
            minor: 1.0,
            elite: 1.0,
            miniboss: 1.0,
            champion: 1.0,
            boss: 1.0,
            vehicle: 1.0,
            timestamp: 0,
        }
    }
}
impl DamageMods {
    pub fn get_mod(&self, _type: &EnemyType) -> f64 {
        let combatant_scale = match _type {
            &EnemyType::MINOR => self.minor,
            &EnemyType::ELITE => self.elite,
            &EnemyType::MINIBOSS => self.miniboss,
            &EnemyType::CHAMPION => self.champion,
            &EnemyType::BOSS => self.boss,
            &EnemyType::VEHICLE => self.vehicle,
            _ => 1.0,
        };
        combatant_scale
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RangeFormula {
    pub start: StatQuadraticFormula,
    pub end: StatQuadraticFormula,
    pub floor_percent: f64,
    pub fusion: bool,
    pub timestamp: u64,
}

//even if just linear use this
#[derive(Debug, Clone, Default, Serialize)]
pub struct StatQuadraticFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
impl StatQuadraticFormula {
    pub fn solve_at(&self, _x: f64) -> f64 {
        self.evpp * _x * _x + self.vpp * _x + self.offset
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ReloadFormula {
    pub reload_data: StatQuadraticFormula,
    pub ammo_percent: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct HandlingFormula {
    pub ready: StatQuadraticFormula,
    pub stow: StatQuadraticFormula,
    pub ads: StatQuadraticFormula,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AmmoFormula {
    pub mag: StatQuadraticFormula,
    pub round_to: i32,
    pub reserve_id: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct RangeResponse {
    pub hip_falloff_start: f64,
    pub hip_falloff_end: f64,
    pub ads_falloff_start: f64,
    pub ads_falloff_end: f64,
    pub floor_percent: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default, Copy)]
pub struct HandlingResponse {
    pub ready_time: f64,
    pub stow_time: f64,
    pub ads_time: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct AmmoResponse {
    pub mag_size: i32,
    pub reserve_size: i32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ReloadResponse {
    pub reload_time: f64,
    pub ammo_time: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default)]
pub struct DpsResponse {
    pub dps_per_mag: Vec<f64>,
    pub time_damage_data: Vec<(f64, f64)>,
    pub total_damage: f64,
    pub total_time: f64,
    pub total_shots: i32,
}
impl DpsResponse {
    pub fn apply_rpl(&mut self, rpl: f64) {
        for mag in self.dps_per_mag.iter_mut() {
            *mag *= rpl;
        }
        for (_, damage) in self.time_damage_data.iter_mut() {
            *damage *= rpl;
        }
        self.total_damage *= rpl;
    }
    pub fn get_dps_over_time(&self) -> Vec<(f64, f64)> {
        let dps_data = &self.time_damage_data;
        let mut damage_so_far = dps_data[0].1;
        let mut dps_lst = Vec::new();
        for hit in dps_data {
            if hit.0 != 0.0 {
                dps_lst.push((hit.0, damage_so_far / hit.0));
            }
            damage_so_far += hit.1;
        }
        dps_lst
    }
}

#[derive(Debug, Clone, Default)]
pub struct FiringResponse {
    pub pvp_impact_damage: f64,
    pub pvp_explosion_damage: f64,
    pub pvp_crit_mult: f64,

    pub pve_impact_damage: f64,
    pub pve_explosion_damage: f64,
    pub pve_crit_mult: f64,

    pub burst_delay: f64,
    pub inner_burst_delay: f64,
    pub burst_size: i32,

    pub rpm: f64,

    pub timestamp: u64,
}
impl FiringResponse {
    pub fn apply_pve_bonuses(
        &mut self,
        _rpl_mult: f64,
        _gpl_mult: f64,
        _pve_mult: f64,
        _combatant_mult: f64,
    ) {
        crate::logging::log(
            format!(
                "rpl: {}, gpl: {}, pve: {}, combat_mult: {}",
                _rpl_mult, _gpl_mult, _pve_mult, _combatant_mult
            )
            .as_str(),
            crate::logging::LogLevel::Debug.into(),
        );
        self.pve_impact_damage *= _rpl_mult * _gpl_mult * _pve_mult * _combatant_mult;
        self.pve_explosion_damage *= _rpl_mult * _gpl_mult * _pve_mult * _combatant_mult;
    }
}

#[derive(Debug, Clone, Default)]
pub struct EDR {
    pub pvp_first_tick_damage: f64,
    pub pvp_avg_tick_damage: f64,
    pub pvp_last_tick_damage: f64,
    pub pvp_tick_duration: f64,
    pub pvp_num_ticks: i32,

    pub pvp_first_explo_damage: f64,
    pub pvp_avg_explo_damage: f64,
    pub pvp_last_explo_damage: f64,
    
    //delay before the explosion goes off
    pub explo_delay: f64,
    //delay between explosions if there are multiple
    pub multiple_explo_delay: f64,
    pub explo_num: i32,

    pub pve_first_tick_damage: f64,
    pub pve_avg_tick_damage: f64,
    pub pve_last_tick_damage: f64,
    pub pve_tick_duration: f64,
    pub pve_num_ticks: i32,
}

