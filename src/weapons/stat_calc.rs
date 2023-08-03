use std::collections::HashMap;

use super::{reserve_calc::calc_reserves, Stat, Weapon};
use crate::{
    d2_enums::{MetersPerSecond, Seconds, StatHashes, WeaponType},
    perks::{
        get_dmg_modifier, get_explosion_data, get_extra_damage, get_firing_modifier,
        get_flinch_modifier, get_handling_modifier, get_magazine_modifier, get_range_modifier,
        get_reload_modifier, get_reserve_modifier, get_velocity_modifier,
        lib::{
            CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
            HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
            RangeModifierResponse, ReloadModifierResponse,
        },
        Perks,
    },
    types::rs_types::{
        AmmoFormula, AmmoResponse, FiringResponse, HandlingFormula, HandlingResponse, RangeFormula,
        RangeResponse, ReloadFormula, ReloadResponse, EDR,
    },
    Perk,
};

impl ReloadFormula {
    fn calc_reload_time_formula(&self, _reload_stat: i32) -> ReloadResponse {
        let reload_time = self.reload_data.solve_at(_reload_stat as f64);
        ReloadResponse {
            reload_time,
            ammo_time: reload_time * self.ammo_percent,
            timestamp: self.timestamp,
        }
    }
}
impl Weapon {
    //TODO: Change this to use piece wise linears instead of approx quadratics
    pub fn calc_reload_time(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> ReloadResponse {
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);

        let mut reload_stat = self
            .stats
            .get(&StatHashes::RELOAD.into())
            .unwrap_or(&Stat::new())
            .perk_val();

        if self.weapon_type == WeaponType::BOW {
            reload_stat = reload_stat.clamp(0, 85);
        }

        let modifiers = if let Some(calc_input) = _calc_input {
            get_reload_modifier(self.list_perks(), &calc_input, _pvp, cached_data)
        } else {
            ReloadModifierResponse::default()
        };

        let mut out = self.reload_formula.calc_reload_time_formula(reload_stat);
        out.reload_time *= modifiers.reload_time_scale;

        if self.weapon_type == WeaponType::BOW {
            out.reload_time = out.reload_time.clamp(0.6, 5.0);
        }

        out
    }
}

impl RangeFormula {
    fn calc_range_falloff_formula(
        &self,
        _range_stat: i32,
        _zoom_stat: i32,
        _modifiers: RangeModifierResponse,
        _floor: f64,
    ) -> RangeResponse {
        let range_stat = (_range_stat + _modifiers.range_stat_add).clamp(0, 100) as f64;
        let zoom_stat = _zoom_stat as f64;

        let zoom_mult = if self.fusion {
            1.0 + 0.02 * zoom_stat
        } else {
            0.1 * zoom_stat - 0.025
        };

        let start = self.start.solve_at(range_stat) * _modifiers.range_all_scale;
        let end = self.end.solve_at(range_stat) * _modifiers.range_all_scale;

        RangeResponse {
            hip_falloff_start: start * _modifiers.range_hip_scale,
            hip_falloff_end: end * _modifiers.range_hip_scale,

            ads_falloff_start: start * zoom_mult * _modifiers.range_zoom_scale,
            ads_falloff_end: end * zoom_mult * _modifiers.range_zoom_scale,

            floor_percent: _floor,
            timestamp: self.timestamp,
        }
    }
}
impl Weapon {
    pub fn calc_range_falloff(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> RangeResponse {
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);

        let range_stat = self
            .stats
            .get(&StatHashes::RANGE.into())
            .unwrap_or(&Stat::new())
            .val();
        let zoom_stat = self
            .stats
            .get(&StatHashes::ZOOM.into())
            .unwrap_or(&Stat::new())
            .val();

        let modifiers = if let Some(calc_input) = _calc_input {
            get_range_modifier(self.list_perks(), &calc_input, _pvp, cached_data)
        } else {
            RangeModifierResponse::default()
        };

        self.range_formula.calc_range_falloff_formula(
            range_stat,
            zoom_stat,
            modifiers,
            self.range_formula.floor_percent,
        )
    }
}

impl HandlingFormula {
    fn calc_handling_times_formula(
        &self,
        _handling_stat: i32,
        _modifiers: HandlingModifierResponse,
    ) -> HandlingResponse {
        let handling_stat = _handling_stat + _modifiers.stat_add;

        let ready_time =
            self.ready.solve_at_i(handling_stat + _modifiers.draw_add) * _modifiers.draw_scale;

        let stow_time = (self.stow.solve_at_i(handling_stat + _modifiers.stow_add)
            * _modifiers.stow_scale)
            .clamp(self.stow.solve_at(100.0), f64::INFINITY);

        let ads_time =
            self.ads.solve_at_i(handling_stat + _modifiers.ads_add) * _modifiers.ads_scale;

        HandlingResponse {
            ready_time,
            stow_time,
            ads_time,
            timestamp: self.timestamp,
        }
    }
}
impl Weapon {
    pub fn calc_handling_times(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> HandlingResponse {
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);

        let handling_stat = self
            .stats
            .get(&StatHashes::HANDLING.into())
            .unwrap_or(&Stat::new())
            .val();

        let modifiers = if let Some(calc_input) = _calc_input {
            get_handling_modifier(self.list_perks(), &calc_input, _pvp, cached_data)
        } else {
            HandlingModifierResponse::default()
        };

        self.handling_formula
            .calc_handling_times_formula(handling_stat, modifiers)
    }
}

impl AmmoFormula {
    fn calc_ammo_size_formula(
        &self,
        _mag_stat: i32,
        _mag_modifiers: MagazineModifierResponse,
        _reserve_stat: i32,
        _inv_modifiers: InventoryModifierResponse,
        _calc_inv: bool,
        _inv_id: u32,
    ) -> AmmoResponse {
        let mag_stat = (_mag_stat + _mag_modifiers.magazine_stat_add).clamp(0, 100) as f64;
        let inv_stat = (_reserve_stat + _inv_modifiers.inv_stat_add).clamp(0, 100) as f64;

        let raw_mag_size =
            (self.mag.evpp * (mag_stat.powi(2))) + (self.mag.vpp * mag_stat) + self.mag.offset;

        let mut mag_size =
            (((self.mag.evpp * (mag_stat.powi(2))) + (self.mag.vpp * mag_stat) + self.mag.offset)
                .ceil()
                * _mag_modifiers.magazine_scale
                + _mag_modifiers.magazine_add)
                .ceil() as i32;
        if mag_size < 1 {
            mag_size = 1;
        }

        let mut reserve_size = 1;
        if _calc_inv {
            reserve_size = calc_reserves(
                raw_mag_size,
                _mag_stat as i32,
                inv_stat as i32,
                _inv_id,
                _inv_modifiers.inv_scale,
            );
        }
        AmmoResponse {
            mag_size,
            reserve_size: reserve_size + _inv_modifiers.inv_add,
            timestamp: self.timestamp,
        }
    }
}
impl Weapon {
    pub fn calc_ammo_sizes(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> AmmoResponse {
        let mag_stat = self
            .stats
            .get(&StatHashes::MAGAZINE.into())
            .unwrap_or(&Stat::new())
            .val();
        let inv_stat = self
            .stats
            .get(&StatHashes::INVENTORY_SIZE.into())
            .unwrap_or(&Stat::new())
            .val();
        let mut out;
        let mut default_chd_dt = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_chd_dt);
        if _calc_input.is_some() {
            let mag_modifiers = get_magazine_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                cached_data,
            );
            let inv_modifiers = get_reserve_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                cached_data,
            );
            out = self.ammo_formula.calc_ammo_size_formula(
                mag_stat,
                mag_modifiers,
                inv_stat,
                inv_modifiers,
                true,
                self.ammo_formula.reserve_id,
            );
        } else {
            out = self.ammo_formula.calc_ammo_size_formula(
                mag_stat,
                MagazineModifierResponse::default(),
                inv_stat,
                InventoryModifierResponse::default(),
                true,
                self.ammo_formula.reserve_id,
            );
        }
        if mag_stat > 90 && self.weapon_type == WeaponType::SNIPER {
            out.mag_size += 1;
        }
        if self.ammo_formula.round_to != i32::default() {
            out.mag_size = ((out.mag_size as f64 / self.ammo_formula.round_to as f64).ceil()
                * self.ammo_formula.round_to as f64) as i32;
        }
        out
    }
}

impl Weapon {
    pub fn calc_firing_data(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> FiringResponse {
        let pve_damage_modifiers: DamageModifierResponse;
        let pvp_damage_modifiers: DamageModifierResponse;
        let firing_modifiers: FiringModifierResponse;
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);
        if _calc_input.is_some() {
            firing_modifiers = get_firing_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                cached_data,
            );
            pvp_damage_modifiers = get_dmg_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                true,
                &mut cached_data.clone(),
            );
            pve_damage_modifiers = get_dmg_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                false,
                &mut cached_data.clone(),
            );
        } else {
            firing_modifiers = FiringModifierResponse::default();
            pvp_damage_modifiers = DamageModifierResponse::default();
            pve_damage_modifiers = DamageModifierResponse::default();
        };
        let tmp_dmg_prof = self.get_damage_profile();
        let impact_dmg = tmp_dmg_prof.0;
        let explosion_dmg = tmp_dmg_prof.1;
        let crit_mult = tmp_dmg_prof.2;

        let fd = self.firing_data;
        let extra_charge_delay = if self.weapon_type == WeaponType::FUSIONRIFLE {
            0.45
        } else if self.weapon_type == WeaponType::LINEARFUSIONRIFLE {
            0.95
        } else {
            0.0
        };
        let burst_delay = (fd.burst_delay + firing_modifiers.burst_delay_add)
            * firing_modifiers.burst_delay_scale;
        let burst_size = fd.burst_size + firing_modifiers.burst_size_add as i32;
        let inner_burst_delay = fd.inner_burst_delay * firing_modifiers.inner_burst_scale;
        let raw_rpm = 60.0
            / ((burst_delay
                + (inner_burst_delay * (burst_size as f64 - 1.0))
                + extra_charge_delay)
                / burst_size as f64);
        let rpm: f64;
        if self.firing_data.one_ammo {
            rpm = raw_rpm / burst_size as f64
        } else {
            rpm = raw_rpm
        };
        let out = FiringResponse {
            pvp_impact_damage: impact_dmg * pvp_damage_modifiers.impact_dmg_scale,
            pvp_explosion_damage: explosion_dmg * pvp_damage_modifiers.explosive_dmg_scale,
            pvp_crit_mult: crit_mult * pvp_damage_modifiers.crit_scale,

            pve_impact_damage: impact_dmg * pve_damage_modifiers.impact_dmg_scale,
            pve_explosion_damage: explosion_dmg * pve_damage_modifiers.explosive_dmg_scale,
            pve_crit_mult: crit_mult * pve_damage_modifiers.crit_scale,

            burst_delay,
            burst_size,
            inner_burst_delay,

            rpm,

            timestamp: fd.timestamp,
        };
        out
    }
}

impl Weapon {
    pub fn get_damage_profile(&self) -> (f64, f64, f64, f64) {
        let impact;
        let mut explosion = 0.0_f64;
        let mut crit = 1.0_f64;
        let delay;

        let epr = get_explosion_data(self.list_perks(), &self.static_calc_input(), false);
        if epr.percent <= 0.0 {
            impact = self.firing_data.damage;
            crit = self.firing_data.crit_mult;
            delay = 0.0;
        } else {
            impact = self.firing_data.damage * (1.0 - epr.percent);
            explosion = self.firing_data.damage * epr.percent;
            if epr.retain_base_total && self.firing_data.crit_mult > 1.0 {
                crit = (self.firing_data.crit_mult - 1.0) / (1.0 - epr.percent) + 1.0
            }
            delay = epr.delyed;
        }
        (impact, explosion, crit, delay)
    }
}

impl Weapon {
    //Returns the flinch scaler from Resillience, Stability, Perks, and Buffs
    //flinch resist = 1.0 - flinch scaler
    //flinch resist can be negative
    pub fn calc_flinch_resist(
        &self,
        _calc_input: Option<CalculationInput>,
        _resillience: i32,
        _pvp: bool,
        _cached_data: Option<&mut HashMap<String, f64>>,
    ) -> f64 {
        /*
        Todo:
        X3 Unflinching
        Perfect Float
         */
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);
        let mut total_scaler = 1.0;

        //resil
        let resillience: f64 = _resillience.clamp(0, 10).into();
        total_scaler *= 1.0 - resillience * 0.01;

        //stability
        let stability_percent = match self.weapon_type {
            WeaponType::AUTORIFLE => 0.25,
            WeaponType::SUBMACHINEGUN => 0.25,
            WeaponType::BOW => 0.25,
            WeaponType::PULSERIFLE => 0.2,
            WeaponType::SCOUTRIFLE => 0.2,
            WeaponType::SIDEARM => 0.2,
            WeaponType::MACHINEGUN => 0.2,
            WeaponType::HANDCANNON => 0.15,
            WeaponType::TRACERIFLE => 0.25,
            WeaponType::FUSIONRIFLE => 0.1,
            WeaponType::SHOTGUN => 0.1,
            WeaponType::SNIPER => 0.1,
            WeaponType::GRENADELAUNCHER => 0.1,
            WeaponType::LINEARFUSIONRIFLE => 0.1,
            WeaponType::ROCKET => 0.1,
            _ => 0.0,
        };

        let total_stability: f64 = self
            .stats
            .get(&StatHashes::STABILITY.into())
            .unwrap_or(&Stat::new())
            .perk_val()
            .clamp(0, 100)
            .into();
        total_scaler *= 1.0 - ((total_stability - 20.0) / 80.0 * stability_percent);

        if _calc_input.is_some() {
            total_scaler *=
                get_flinch_modifier(self.list_perks(), &_calc_input.unwrap(), _pvp, cached_data)
                    .flinch_scale;
        }

        total_scaler
    }
}

//returns the m/s of projectile
impl Weapon {
    pub fn calc_projectile_velocity(
        &self,
        _calc_input: Option<CalculationInput>,
        _pvp: bool,
        _cached_data: Option<&mut HashMap<String, f64>>,
    ) -> MetersPerSecond {
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);

        //Range/Velocity stat to m/s
        let mut velocity = match self.weapon_type {
            WeaponType::GLAIVE => {
                f64::from(
                    self.stats
                        .get(&StatHashes::RANGE.into())
                        .unwrap_or(&Stat::new())
                        .perk_val()
                        .clamp(0, 100),
                ) * 0.4
                    + 60.0
            }
            WeaponType::GRENADELAUNCHER => {
                f64::from(
                    self.stats
                        .get(&StatHashes::VELOCITY.into())
                        .unwrap_or(&Stat::new())
                        .perk_val()
                        .clamp(0, 100),
                ) * 0.384
                    + 29.6
            }
            WeaponType::ROCKET => {
                f64::from(
                    self.stats
                        .get(&StatHashes::VELOCITY.into())
                        .unwrap_or(&Stat::new())
                        .perk_val()
                        .clamp(0, 100),
                ) * 0.13
                    + 29.0
            }
            _ => 0.0,
        };

        if _calc_input.is_some() {
            velocity *=
                get_velocity_modifier(self.list_perks(), &_calc_input.unwrap(), _pvp, cached_data)
                    .velocity_scaler;
        }
        velocity
    }
}

impl Weapon {
    pub fn calc_perfect_draw(&self) -> Seconds {
        let stability: f64 = self
            .stats
            .get(&StatHashes::STABILITY.into())
            .unwrap_or(&Stat::new())
            .perk_val()
            .clamp(0, 100)
            .into();
        if self.perks.get(&1449897496).is_some() && self.weapon_type == WeaponType::BOW {
            return Seconds::INFINITY;
        }
        match self.intrinsic_hash {
            906 => stability * 1.0 / 400.0 + 0.3,
            905 => stability * 3.0 / 1000.0 + 0.5,
            _ => Seconds::NAN,
        }
    }
}

impl Weapon {
    pub fn calc_shield_duration(&self) -> Seconds {
        if self.weapon_type == WeaponType::GLAIVE {
            let shield_duration: f64 = self
                .stats
                .get(&StatHashes::SHIELD_DURATION.into())
                .unwrap_or(&Stat::new())
                .perk_val()
                .clamp(0, 100)
                .into();
            return shield_duration * 0.11 + 6.65;
        } else if self.weapon_type == WeaponType::SWORD {
            let guard_endruance: f64 = self
                .stats
                .get(&StatHashes::GUARD_ENDURANCE.into())
                .unwrap_or(&Stat::new())
                .perk_val()
                .clamp(0, 100)
                .into();
            return 100.0 / (guard_endruance * -0.75 + 80.0);
        }
        Seconds::NAN
    }
}

impl Weapon {
    pub fn get_misc_stats(
        &self,
        _calc_input: Option<CalculationInput>,
        _pvp: bool,
    ) -> HashMap<String, f64> {
        let mut buffer: HashMap<String, f64> = HashMap::new();
        let mut cached_data: HashMap<String, f64> = HashMap::new();

        if matches!(
            self.weapon_type,
            WeaponType::ROCKET | WeaponType::GRENADELAUNCHER | WeaponType::GLAIVE
        ) {
            buffer.insert(
                "velocity".to_string(),
                self.calc_projectile_velocity(_calc_input, _pvp, Some(&mut cached_data)),
            );
        };

        if matches!(self.weapon_type, WeaponType::GLAIVE) {
            // add "| WeaponType::SWORD" to matches when swords work
            buffer.insert("shield_duration".to_string(), self.calc_shield_duration());
        }

        if self.weapon_type == WeaponType::BOW {
            buffer.insert("perfect_draw".to_string(), self.calc_perfect_draw());
        }

        buffer
    }
}

impl Weapon {
    pub fn get_edr(
        &self,
        _calc_input: Option<CalculationInput>,
        _cached_data: Option<&mut HashMap<String, f64>>,
        _pvp: bool,
    ) -> EDR {
        let mut default_cached_data = HashMap::new();
        let cached_data = _cached_data.unwrap_or(&mut default_cached_data);
        let damage_modifiers: DamageModifierResponse;
        let extra_damage: Vec<ExtraDamageResponse>;

        if _calc_input.is_some() {
            damage_modifiers = get_dmg_modifier(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                &mut cached_data.clone(),
            );

            extra_damage = get_extra_damage(
                self.list_perks(),
                &_calc_input.clone().unwrap(),
                _pvp,
                &mut cached_data.clone(),
            );
        } else {
            damage_modifiers = DamageModifierResponse::default();
            extra_damage = vec![ExtraDamageResponse::default()];
        }

        let mut average_tick_damage = 0.0;
        // goes thru the vec to find the avg tick damage
        for edr in &extra_damage {
            average_tick_damage += edr.additive_damage;
        }
        let length = extra_damage.len() - 1;
        average_tick_damage /= length as f64 + 1.0;

        return EDR {
            first_tick_damage: (extra_damage[0].additive_damage
                * extra_damage[0].explosive_percent
                * damage_modifiers.explosive_dmg_scale)
                + (extra_damage[0].additive_damage
                    * (1.0 - extra_damage[0].explosive_percent)
                    * damage_modifiers.impact_dmg_scale),
            tick_duration: extra_damage[0].time_for_additive_damage,
            num_ticks: extra_damage[0].times_to_hit,
            last_tick_damage: (extra_damage[length].additive_damage
                * extra_damage[length].explosive_percent
                * damage_modifiers.explosive_dmg_scale)
                + (extra_damage[length].additive_damage
                    * (1.0 - extra_damage[length].explosive_percent)
                    * damage_modifiers.impact_dmg_scale),
            avg_tick_damage: average_tick_damage * damage_modifiers.impact_dmg_scale,
        };
    }
}
