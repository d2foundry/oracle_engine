use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

use super::Weapon;
use crate::d2_enums::{AmmoType, WeaponType};
use crate::enemies::Enemy;
use crate::perks::lib::{
    CalculationInput, ExtraDamageResponse, RefundResponse, ReloadOverrideResponse,
};
use crate::perks::*;
use crate::types::rs_types::DpsResponse;

//first entry in tuple is refund to mag, second is too reserves
pub fn calc_refund(_shots_hit_this_mag: i32, _refunds: Vec<RefundResponse>) -> (i32, i32) {
    let mut refund_ammount = (0, 0);
    for refund in _refunds {
        if _shots_hit_this_mag % refund.requirement == 0 {
            refund_ammount.0 += refund.refund_mag;
            refund_ammount.1 += refund.refund_reserves;
        }
    }
    refund_ammount
}

#[derive(Debug, Clone)]
pub struct ExtraDamageResult {
    pub extra_time: f64,
    pub extra_dmg: f64,
    pub extra_hits: i32,
    pub extra_time_dmg: Vec<(f64, f64)>,
}
#[derive(Debug, Clone)]
pub struct ExtraDamageBuffInfo {
    pub pl_buff: f64,
    pub pve_buff: f64,
    pub impact_buff: f64,
    pub explosive_buff: f64,
    pub crit_buff: f64,
    pub combatant_buff: f64,
}
impl ExtraDamageBuffInfo {
    pub fn get_buff_amount(&self, entry: &ExtraDamageResponse) -> f64 {
        let mut buff = self.pl_buff;
        if entry.weapon_scale {
            buff *= (self.impact_buff + self.explosive_buff) / 2.0;
            buff *= self.pve_buff
        };
        if entry.crit_scale {
            buff *= self.crit_buff
        };
        if entry.combatant_scale {
            buff *= self.combatant_buff
        };
        buff
    }
}
pub fn calc_extra_dmg(
    _total_time: f64,
    _extra_dmg_entries: Vec<ExtraDamageResponse>,
    _dmg_buffs: ExtraDamageBuffInfo,
) -> ExtraDamageResult {
    let mut extra_time = 0.0;
    let mut extra_dmg = 0.0;
    let mut extra_hits = 0;
    let mut extra_time_dmg: Vec<(f64, f64)> = Vec::new();
    for entry in _extra_dmg_entries {
        if entry.additive_damage > 0.0 {
            if entry.hit_at_same_time {
                let mut bonus_dmg = entry.additive_damage * entry.times_to_hit as f64;
                bonus_dmg *= _dmg_buffs.get_buff_amount(&entry);
                extra_dmg += bonus_dmg;
                if entry.increment_total_time {
                    extra_time += entry.time_for_additive_damage
                };
                extra_time_dmg.push((_total_time + entry.time_for_additive_damage, bonus_dmg));
                extra_hits += entry.times_to_hit;
            } else if !entry.is_dot {
                for i in 0..entry.times_to_hit {
                    let mut bonus_dmg = entry.additive_damage;
                    bonus_dmg *= _dmg_buffs.get_buff_amount(&entry);
                    extra_dmg += bonus_dmg;
                    if entry.increment_total_time {
                        extra_time += entry.time_for_additive_damage
                    };
                    extra_time_dmg.push((
                        _total_time + entry.time_for_additive_damage * i as f64,
                        bonus_dmg,
                    ));
                    extra_hits += 1;
                }
            } else {
                //all dot does is increment time backwards
                for i in 0..entry.times_to_hit {
                    let mut bonus_dmg = entry.additive_damage;
                    bonus_dmg *= _dmg_buffs.get_buff_amount(&entry);
                    extra_dmg += bonus_dmg;
                    if entry.increment_total_time {
                        extra_time += entry.time_for_additive_damage
                    };
                    extra_time_dmg.push((
                        _total_time - entry.time_for_additive_damage * i as f64,
                        bonus_dmg,
                    ));
                    extra_hits += 1;
                }
            }
        }
    }
    ExtraDamageResult {
        extra_time,
        extra_dmg,
        extra_hits,
        extra_time_dmg,
    }
}

pub fn complex_dps_calc(_weapon: Weapon, _enemy: Enemy, _pl_dmg_mult: f64) -> DpsResponse {
    let weapon = Rc::new(_weapon.clone());
    let stats = weapon.stats.clone();
    let weapon_type = weapon.weapon_type;
    let ammo_type = weapon.ammo_type;

    let tmp_dmg_prof = weapon.get_damage_profile(true);
    let impact_dmg = tmp_dmg_prof.impact_dmg;
    let explosion_dmg = tmp_dmg_prof.explosion_dmg;
    let crit_mult = tmp_dmg_prof.crit_mult;
    // let damage_delay = tmp_dmg_prof.damage_delay;

    let base_mag = weapon.calc_ammo_sizes(None, None, false).mag_size;
    let maximum_shots = if base_mag * 5 < 15 { 15 } else { base_mag * 5 };

    let firing_settings = _weapon.firing_data;
    let perks = weapon.list_perks();

    let burst_size = firing_settings.burst_size as f64;
    let burst_delay = firing_settings.burst_delay;
    let inner_burst_delay = firing_settings.inner_burst_delay;

    let mut total_damage = 0.0_f64;
    let mut total_time = 0.0_f64;

    let mut time_damage_data: Vec<(f64, f64)> = Vec::new(); //used for chart stuff
    let mut dps_per_mag: Vec<f64> = Vec::new(); //used for chart stuff

    let mut total_shots_fired = 0_i32;
    let mut total_shots_hit = 0_i32;
    let mut num_reloads = 0_i32;

    let mut pers_calc_data: HashMap<String, f64> = HashMap::new();

    let mut reserve = weapon
        .calc_ammo_sizes(
            Some(weapon.static_calc_input()),
            Some(&mut pers_calc_data),
            false,
        )
        .reserve_size;

    #[allow(unused_mut)]
    while reserve > 0 {
        let mut shots_this_mag = 0;
        //MAGAZINE/////////////////////
        let mag_calc_input = weapon.sparse_calc_input(total_shots_fired, total_time);
        let mut mag = weapon
            .calc_ammo_sizes(Some(mag_calc_input), Some(&mut pers_calc_data), false)
            .mag_size;
        if mag > reserve {
            mag = reserve
        }
        ///////////////////////////////

        //HANDLING/////////////////////
        //This is for stuff like weapon swapping, demo or trench barrel
        let handling_calc_input = weapon.sparse_calc_input(total_shots_fired, total_time);
        let handling_data =
            weapon.calc_handling_times(Some(handling_calc_input), Some(&mut pers_calc_data), false);
        ///////////////////////////////
        let mut start_time = total_time;
        while mag > 0 {
            //DMG MODIFIERS////////////////
            let before_shot_input_data = CalculationInput {
                intrinsic_hash: weapon.intrinsic_hash,
                curr_firing_data: &firing_settings,
                base_crit_mult: crit_mult,
                base_mag: base_mag as f64,
                curr_mag: mag as f64,
                ammo_type: &ammo_type,
                weapon_type: &weapon_type,
                stats: &stats,
                perk_value_map: &weapon.perk_value_map_update(),
                enemy_type: &_enemy.type_,
                shots_fired_this_mag: shots_this_mag as f64,
                total_shots_fired: total_shots_fired as f64,
                total_shots_hit: total_shots_hit as f64,
                reserves_left: reserve as f64,
                time_total: total_time,
                time_this_mag: (total_time - start_time),
                damage_type: &weapon.damage_type,
                handling_data,
                num_reloads: num_reloads as f64,
                has_overshield: false,
            };
            let dmg_mods = get_dmg_modifier(
                perks.clone(),
                &before_shot_input_data,
                false,
                &mut pers_calc_data,
            );
            ///////////////////////////////

            //FIRING MODIFIERS/////////////
            let firing_mods = get_firing_modifier(
                perks.clone(),
                &before_shot_input_data,
                false,
                &mut pers_calc_data,
            );
            ///////////////////////////////

            let dmg = {
                ((impact_dmg * dmg_mods.impact_dmg_scale) * (crit_mult * dmg_mods.crit_scale)
                    + (explosion_dmg * dmg_mods.explosive_dmg_scale))
                    * _pl_dmg_mult
                    * weapon.damage_mods.get_mod(&_enemy.type_)
                    * weapon.damage_mods.pve
            };

            let shot_burst_delay =
                (burst_delay + firing_mods.burst_delay_add) * firing_mods.burst_delay_scale;
            let shot_inner_burst_delay = inner_burst_delay * firing_mods.inner_burst_scale;
            let shot_burst_size = burst_size + firing_mods.burst_size_add;

            // if total_shots_fired == 0 && firing_settings.is_charge {
            //     total_time += shot_burst_delay*0.5;
            // }

            if firing_settings.one_ammo && burst_size > 1.0 {
                total_shots_fired += 1;
                shots_this_mag += 1;
                total_shots_hit += shot_burst_size as i32;
                total_damage += dmg * shot_burst_size;
                for i in 0..shot_burst_size as i32 {
                    time_damage_data.push((total_time + shot_inner_burst_delay * i as f64, dmg));
                }
                total_time += inner_burst_delay * (shot_burst_size - 1.0);
            } else {
                let spec_delay = if shots_this_mag % burst_size as i32 == 0 {
                    shot_burst_delay
                } else {
                    shot_inner_burst_delay
                };
                total_shots_fired += 1;
                shots_this_mag += 1;
                total_shots_hit += 1;
                if inner_burst_delay == 0.0 {
                    total_damage += dmg * burst_size;
                    time_damage_data.push((total_time, dmg * burst_size));
                } else {
                    total_damage += dmg;
                    time_damage_data.push((total_time, dmg));
                }
                if total_shots_fired > 0 {
                    total_time += spec_delay;
                }
            }
            mag -= 1;

            //REFUNDS//////////////////////
            let mut refund_calc_input = weapon.sparse_calc_input(total_shots_fired, total_time);
            refund_calc_input.shots_fired_this_mag = shots_this_mag as f64;
            let refunds = get_refund_modifier(
                perks.clone(),
                &refund_calc_input,
                false,
                &mut pers_calc_data,
            );
            let ammo_to_refund = calc_refund(shots_this_mag, refunds);
            mag += ammo_to_refund.0;
            reserve += ammo_to_refund.1;
            ///////////////////////////////

            //COMPLEX CALC PRECURSOR//////
            let after_shot_input_data = CalculationInput {
                intrinsic_hash: weapon.intrinsic_hash,
                curr_firing_data: &firing_settings,
                base_crit_mult: crit_mult,
                base_mag: base_mag as f64,
                curr_mag: mag as f64,
                ammo_type: &ammo_type,
                weapon_type: &weapon_type,
                stats: &stats,
                perk_value_map: &weapon.perk_value_map_update(),
                enemy_type: &_enemy.type_,
                shots_fired_this_mag: shots_this_mag as f64,
                total_shots_fired: total_shots_fired as f64,
                total_shots_hit: total_shots_hit as f64,
                reserves_left: reserve as f64,
                time_total: total_time,
                time_this_mag: (total_time - start_time),
                damage_type: &weapon.damage_type,
                handling_data,
                num_reloads: num_reloads as f64,
                has_overshield: false,
            };
            ///////////////////////////////

            //EXTRA DMG////////////////////
            let extra_dmg_responses = get_extra_damage(
                perks.clone(),
                &after_shot_input_data,
                false,
                &mut pers_calc_data,
            );
            let buffs = ExtraDamageBuffInfo {
                pl_buff: _pl_dmg_mult,
                impact_buff: dmg_mods.impact_dmg_scale,
                explosive_buff: dmg_mods.explosive_dmg_scale,
                pve_buff: weapon.damage_mods.pve,
                crit_buff: crit_mult * dmg_mods.crit_scale,
                combatant_buff: weapon.damage_mods.get_mod(&_enemy.type_),
            };
            let tmp_out_data = calc_extra_dmg(total_time, extra_dmg_responses, buffs);
            total_damage += tmp_out_data.extra_dmg;
            total_time += tmp_out_data.extra_time;
            total_shots_hit += tmp_out_data.extra_hits;
            time_damage_data.extend(tmp_out_data.extra_time_dmg);
            ///////////////////////////////

            //RELOAD OVERRIDE//////////////
            // if mag == 0 {
            //     let reload_override_responses = get_reload_overrides(
            //         perks.clone(),
            //         &after_shot_input_data,
            //         false,
            //         &mut pers_calc_data,
            //     );
            //     if reload_override_responses.len() > 0 {
            //         let mut final_response = ReloadOverrideResponse::invalid();
            //         for response in reload_override_responses {
            //             if response.priority > final_response.priority {
            //                 final_response = response;
            //             }
            //         }
            //         if final_response.valid {
            //             total_time += final_response.reload_time;
            //             if final_response.uses_ammo {
            //                 let ammo_to_add = if final_response.ammo_to_reload > reserve {
            //                     reserve
            //                 } else {
            //                     final_response.ammo_to_reload
            //                 };
            //                 mag = ammo_to_add;
            //                 reserve -= ammo_to_add;
            //             } else {
            //                 mag = final_response.ammo_to_reload;
            //             }
            //             if final_response.count_as_reload {
            //                 num_reloads += 1;
            //                 start_time = total_time;
            //                 shots_this_mag = 0;
            //             }
            //         }
            //     }
            // }
            if mag != 0 {
                if weapon.weapon_type == WeaponType::FUSIONRIFLE {
                    total_time += shot_burst_delay * 0.45
                } else if weapon.weapon_type == WeaponType::LINEARFUSIONRIFLE {
                    total_time += shot_burst_delay * 0.95
                }
            }
            ///////////////////////////////
            if weapon.ammo_type == AmmoType::PRIMARY {
                if total_shots_fired > maximum_shots {
                    reserve = 0;
                    break;
                }
            } else if total_shots_fired > base_mag * 8 + 20 {
                reserve = 0;
                break;
            }
            if reserve <= 0 {
                break;
            }
        }

        reserve -= base_mag;
        dps_per_mag.push(total_damage / total_time);

        //RELOAD///////////////////////
        let reload_input_data = CalculationInput {
            intrinsic_hash: weapon.intrinsic_hash,
            curr_firing_data: &firing_settings,
            base_crit_mult: crit_mult,
            base_mag: base_mag as f64,
            curr_mag: mag as f64,
            ammo_type: &ammo_type,
            weapon_type: &weapon_type,
            stats: &stats,
            perk_value_map: &weapon.perk_value_map_update(),
            enemy_type: &_enemy.type_,
            shots_fired_this_mag: shots_this_mag as f64,
            total_shots_fired: total_shots_fired as f64,
            total_shots_hit: total_shots_hit as f64,
            reserves_left: reserve as f64,
            time_total: total_time,
            time_this_mag: (total_time - start_time),
            damage_type: &weapon.damage_type,
            handling_data,
            num_reloads: num_reloads as f64,
            has_overshield: false,
        };
        let reload_responses =
            weapon.calc_reload_time(Some(reload_input_data), Some(&mut pers_calc_data), false);
        total_time += reload_responses.reload_time;
        ///////////////////////////////
        num_reloads += 1;
    }
    //sort time_damage_data by time
    time_damage_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    DpsResponse {
        dps_per_mag,
        time_damage_data,
        total_damage,
        total_time,
        total_shots: total_shots_fired,
    }
}
