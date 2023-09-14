

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

#[derive(Debug, Clone, Default, Copy)]
pub struct AmmoResponse {
    pub mag_size: i32,
    pub reserve_size: i32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Default, Copy)]
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

#[derive(Debug, Clone, Default, Copy)]
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
        self.pve_impact_damage *= _rpl_mult * _gpl_mult * _pve_mult * _combatant_mult;
        self.pve_explosion_damage *= _rpl_mult * _gpl_mult * _pve_mult * _combatant_mult;
    }
}