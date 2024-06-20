//This also includes intrinsic perks, not just exotic
use std::collections::HashMap;

use serde::__private::de;

use crate::{d2_enums::StatHashes, enemies::EnemyType, weapons::Stat};

use super::{
    add_dmr, add_edr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rr, add_rsmr, add_sbr,
    add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, InventoryModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn exotic_perks() {
    add_dmr(
        Perks::ParacausalShot,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let bufflist_pve = vec![1.0, 3.92, 4.0, 4.4, 5.25, 7.67, 11.71, 18.36];
            let bufflist_pvp = vec![1.0, 1.01, 1.03, 1.13, 1.41, 1.96, 3.0, 4.73];
            let mut damage_buff = 1.0;
            if _input.calc_data.curr_mag == 1.0 {
                let num_of_crits = clamp(_input.calc_data.shots_fired_this_mag as i32, 0, 7);
                let bufflist = if _input.pvp {
                    &bufflist_pvp
                } else {
                    &bufflist_pve
                };
                damage_buff = bufflist[num_of_crits as usize];
            };
            if _input.calc_data.time_this_mag < 0.0 {
                let num_of_crits = clamp(_input.value as i32, 0, 7);
                let bufflist = if _input.pvp {
                    &bufflist_pvp
                } else {
                    &bufflist_pve
                };
                damage_buff = bufflist[num_of_crits as usize];
            }
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::HuntersTrance,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            let inter_val = *_input
                .calc_data
                .perk_value_map
                .get(&213689231)
                .unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            out.insert(StatHashes::RELOAD.into(), buff_val);
            out.insert(StatHashes::RANGE.into(), buff_val);
            out.insert(StatHashes::HANDLING.into(), buff_val);
            out
        }),
    );

    add_rsmr(
        Perks::HuntersTrance,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let inter_val = *_input
                .calc_data
                .perk_value_map
                .get(&213689231)
                .unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            ReloadModifierResponse {
                reload_stat_add: buff_val,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::HuntersTrance,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let inter_val = *_input
                .calc_data
                .perk_value_map
                .get(&213689231)
                .unwrap_or(&0);
            let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
            RangeModifierResponse {
                range_stat_add: buff_val,
                ..Default::default()
            }
        }),
    );

    add_hmr(
        Perks::HuntersTrance,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let inter_val = *_input
                    .calc_data
                    .perk_value_map
                    .get(&213689231)
                    .unwrap_or(&0);
                let buff_val = (clamp(inter_val, 0, 7) * 5) as i32;
                HandlingModifierResponse {
                    stat_add: buff_val,
                    ..Default::default()
                }
            },
        ),
    );

    add_rmr(
        Perks::HuntersTrace,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_ads_scale = if _input.value > 0 { 4.5 / 1.7 } else { 1.0 };
            RangeModifierResponse {
                range_zoom_scale: range_ads_scale,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MementoMori,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_buff = 1.0;
            if _input.value > 0 && _input.calc_data.total_shots_fired < 7.0 {
                damage_buff = if _input.pvp { 1.285 } else { 1.5 };
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::MementoMori,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_all_scale = if _input.value > 0 && _input.calc_data.total_shots_fired < 7.0 {
                0.85
            } else {
                1.0
            };
            RangeModifierResponse {
                range_all_scale,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::Roadborn,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if _input.value > 0 {
                out.insert(StatHashes::HANDLING.into(), 20);
                out.insert(StatHashes::RELOAD.into(), 40);
            };
            out
        }),
    );

    add_dmr(
        Perks::Roadborn,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut crit_mult = 1.0;
            if _input.value > 0 {
                crit_mult = 1.17;
            };
            DamageModifierResponse {
                crit_scale: crit_mult,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::Roadborn,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            if _input.value > 0 {
                delay_mult = 0.583;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        }),
    );

    add_rmr(
        Perks::Roadborn,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range_scale = 1.05;
            if _input.value > 0 {
                range_scale = 1.15; //roughly
            };
            RangeModifierResponse {
                range_stat_add: 0,
                range_all_scale: range_scale,
                range_hip_scale: 1.0,
                range_zoom_scale: 1.0,
            }
        }),
    );

    add_rsmr(
        Perks::Roadborn,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if _input.value > 0 {
                reload = 40;
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: 1.0,
            }
        }),
    );

    add_fmr(
        Perks::ReignHavoc,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            if _input.calc_data.shots_fired_this_mag >= _input.calc_data.base_mag * 0.2 {
                delay_mult = 0.75;
            };
            if _input.calc_data.shots_fired_this_mag >= _input.calc_data.base_mag * 0.4 {
                delay_mult = 0.625;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        }),
    );

    add_edr(
        Perks::ReignHavoc,
        Box::new(|_input: ModifierResponseInput| -> ExtraDamageResponse {
            let dmg = if _input.pvp { 65.0 } else { 65.0 * 1.3 };
            ExtraDamageResponse {
                additive_damage: dmg,
                increment_total_time: false,
                times_to_hit: 1,
                time_for_additive_damage: 0.0,
                hit_at_same_time: true,
                is_dot: false,
                weapon_scale: true,
                crit_scale: false,
                combatant_scale: true,
            }
        }),
    );

    add_dmr(
        Perks::WormsHunger,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(_input.value, 0, 20);
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + (val as f64) * 0.1,
                explosive_dmg_scale: 1.0 + (val as f64) * 0.1,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::LagragianSight,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_buff = 1.0;
            if _input.value > 0 && _input.calc_data.time_total < 30.0 {
                damage_buff = 1.4;
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::ToM,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_buff = 1.0;
            if _input.calc_data.curr_mag == 1.0 {
                damage_buff = if _input.pvp { 2.0 } else { 2.4 };
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_buff,
                explosive_dmg_scale: damage_buff,
                ..Default::default()
            }
        }),
    );

    add_rr(
        Perks::ToM,
        Box::new(|_input: ModifierResponseInput| -> RefundResponse {
            RefundResponse {
                refund_mag: if _input.calc_data.curr_mag == 0.0 {
                    1
                } else {
                    0
                },
                refund_reserves: 0,
                crit: false,
                requirement: 1,
            }
        }),
    );

    add_edr(
        Perks::RocketTracers,
        Box::new(|_input: ModifierResponseInput| -> ExtraDamageResponse {
            let dmg = if _input.pvp { 24.0 } else { 105.0 };
            ExtraDamageResponse {
                additive_damage: dmg,
                times_to_hit: 1,
                increment_total_time: false,
                time_for_additive_damage: 0.0,
                hit_at_same_time: true,
                is_dot: false,
                weapon_scale: true,
                crit_scale: false,
                combatant_scale: true,
            }
        }),
    );

    // add_edr_guidance_ring(
    //     _input: &CalculationInput,
    //     _input.value: u32,
    //     is_enhanced: bool,
    //     _pvp: bool,
    //     _cached_data: &mut HashMap<String, f64>,
    // ) -> ExtraDamageResponse {
    //     ExtraDamageResponse {
    //         additive_damage: if _input.value > 0 {
    //              _input.calc_data.base_damage *  _input.calc_data.base_crit_mult
    //         } else {
    //             0.0
    //         },
    //         times_to_hit: 2,
    //         increment_total_time: false,
    //         time_for_additive_damage: 0.1,
    //         hit_at_same_time: true,
    //         is_dot: false,
    //         weapon_scale: true,
    //         crit_scale: false,
    //         combatant_scale: true,
    //     }
    // }

    // add_edr_poison_arrows(
    //     _input: &CalculationInput,
    //     _input.value: u32,
    //     is_enhanced: bool,
    //     _pvp: bool,
    //     _cached_data: &mut HashMap<String, f64>,
    // ) -> ExtraDamageResponse {
    //     let last_proc = _cached_data.get("poison_arrows").unwrap_or(&0.0);
    //     let time_diff =  _input.calc_data.time_total - last_proc;
    //     return ExtraDamageResponse {
    //         additive_damage: if _input.value > 0 {
    //              _input.calc_data.base_damage *  _input.calc_data.base_crit_mult
    //         } else {
    //             0.0
    //         },
    //         times_to_hit: (time_diff / 0.5).ceil() as i32,
    //         increment_total_time: false,
    //         time_for_additive_damage: 0.5,
    //         hit_at_same_time: false,
    //         is_dot: true,
    //         weapon_scale: true,
    //         crit_scale: false,
    //         combatant_scale: true,
    //     };
    // }

    add_fmr(
        Perks::HakkeHeavyBurst,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_size_add: -2.0,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::HakkeHeavyBurst,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let crit_scale = 1.828 / _input.calc_data.base_crit_mult;
            DamageModifierResponse {
                explosive_dmg_scale: 1.48,
                impact_dmg_scale: 1.48,
                crit_scale,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::SwoopingTalons,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut dmg_mult = 1.0;
            if _input.value > 0 {
                dmg_mult = 1.4;
            }
            dmg_mult += _input.calc_data.total_shots_fired * 0.04;
            dmg_mult = clamp(dmg_mult, 1.0, 1.4);
            DamageModifierResponse {
                impact_dmg_scale: dmg_mult,
                explosive_dmg_scale: dmg_mult,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::IgnitionTrigger,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut dmg_mult = 1.0;
            if _input.value > 0 || _input.calc_data.total_shots_fired > 20.0 {
                dmg_mult = if _input.pvp { 1.55 } else { 1.99 };
            }
            DamageModifierResponse {
                impact_dmg_scale: dmg_mult,
                explosive_dmg_scale: dmg_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::CalculatedBalance,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = if _input.value > 0 { 0.2 } else { 0.0 };
            let duration = 5.0;
            if _input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::RavenousBeast,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.value > 0 {
                FiringModifierResponse {
                    burst_delay_scale: 0.8,
                    ..Default::default()
                }
            } else {
                FiringModifierResponse::default()
            }
        }),
    );

    add_dmr(
        Perks::RavenousBeast,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let mut crit_mult = 1.0;
            if _input.value > 0 {
                damage_mult = if _input.pvp { 2.2 } else { 2.87 };
                crit_mult = if _input.pvp {
                    1.0 / (1.5 + -3.0 / 51.0)
                } else {
                    1.99 / 2.87
                };
            }
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: crit_mult,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::ReleaseTheWolves,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let has_cat = _input.calc_data.perk_value_map.contains_key(&431220296);
            let mut out = HashMap::new();
            if has_cat {
                if _input.value == 0 {
                    out.insert(StatHashes::STABILITY.into(), 40);
                } else if _input.value == 1 {
                    out.insert(StatHashes::RELOAD.into(), 100);
                }
            }
            out
        }),
    );

    add_rsmr(
        Perks::ReleaseTheWolves,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let has_cat = _input.calc_data.perk_value_map.contains_key(&431220296);
            if _input.value == 1 && has_cat {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 0.85,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_fmr(
        Perks::ReleaseTheWolves,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.value > 0 {
                FiringModifierResponse {
                    burst_delay_scale: 0.4,
                    ..Default::default()
                }
            } else {
                FiringModifierResponse::default()
            }
        }),
    );

    add_dmr(
        Perks::ReleaseTheWolves,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if _input.value > 0 { 1.4 } else { 1.0 };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::Fundamentals,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value == 1 {
                stats.insert(StatHashes::STABILITY.into(), 20);
                stats.insert(StatHashes::AIM_ASSIST.into(), 10);
            } else if _input.value == 2 {
                stats.insert(StatHashes::AIRBORNE.into(), 20);
                stats.insert(StatHashes::RELOAD.into(), 35);
            } else if _input.value == 3 {
                stats.insert(StatHashes::RANGE.into(), 5);
                stats.insert(StatHashes::HANDLING.into(), 25);
            };
            stats
        }),
    );

    add_hmr(
        Perks::Fundamentals,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let mut handling = 0;
                if _input.value == 3 {
                    handling = 25;
                }
                HandlingModifierResponse {
                    stat_add: handling,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::Fundamentals,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if _input.value == 2 {
                reload = 35;
            }
            ReloadModifierResponse {
                reload_stat_add: reload,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::Fundamentals,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range = 0;
            if _input.value == 3 {
                range = 5;
            }
            RangeModifierResponse {
                range_stat_add: range,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::ThinTheHerd,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if _input.value > 0 {
                out.insert(StatHashes::RELOAD.into(), 70);
            }
            out
        }),
    );

    add_rsmr(
        Perks::ThinTheHerd,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 70,
                    ..Default::default()
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_hmr(
        Perks::Chimera,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: 100,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::Chimera,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if _input.value > 0 {
                out.insert(StatHashes::RELOAD.into(), 100);
            }
            out
        }),
    );

    add_dmr(
        Perks::FirstGlance,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let mut crit_mult = 1.0;
            if _input.value > 0 {
                if _input.calc_data.total_shots_fired == 0.0 {
                    damage_mult = 1.33;
                } else {
                    crit_mult = 1.33;
                };
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                crit_scale: crit_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::FateOfAllFools,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let mut crit_mult = 1.0;
            if _input.value as f64 > _input.calc_data.total_shots_fired {
                let cc = _input.calc_data.base_crit_mult;
                damage_mult = cc;
                crit_mult = 1.0 / cc;
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                crit_scale: crit_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::HonedEdge,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            let has_cat = _input.calc_data.perk_value_map.contains_key(&529188544);
            if _input.value == 2 {
                damage_mult = if _input.pvp { 1.183 } else { 2.0 };
            } else if _input.value == 3 {
                damage_mult = if _input.pvp { 1.412 } else { 3.0 };
            } else if _input.value == 4 && has_cat {
                damage_mult = if _input.pvp { 1.504 * 1.2 } else { 4.0 * 1.2 };
            } else if _input.value == 4 {
                damage_mult = if _input.pvp { 1.504 } else { 4.0 };
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::TakenPredator,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = 1.0;
            if _input.value == 1 || _input.value == 2 {
                damage_mult = 1.25;
            } else if _input.value == 3 {
                damage_mult = 1.25 * 1.25;
            };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MarkovChain,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(_input.value, 0, 5);
            let damage_mult = (1.0 / 15.0) * val as f64 * if _input.pvp { 1.0 } else { 2.0 };
            DamageModifierResponse {
                explosive_dmg_scale: 1.0 + damage_mult,
                impact_dmg_scale: 1.0 + damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::StringofCurses,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(_input.value, 0, 5);
            let mut damage_mult = 0.2 * val as f64;
            if _input.pvp {
                damage_mult = ((damage_mult * 100.0 / 2.0) / 4.0).ceil() * 0.04;
            }
            let duration = 3.5;
            if _input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::StormAndStress,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }

            let damage_mult = if _input.pvp { 1.8 } else { 3.62 };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::DualSpeedReceiver,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            if _input.value == 0 {
                return RangeModifierResponse::default();
            }
            RangeModifierResponse {
                range_stat_add: 30,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::DualSpeedReceiver,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if _input.value > 0 {
                out.insert(StatHashes::ZOOM.into(), 3);
                out.insert(StatHashes::RANGE.into(), 30);
            }
            out
        }),
    );

    add_dmr(
        Perks::FullStop,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.pvp {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                crit_scale: 2.9,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::RatPack,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.value == 0 {
                return FiringModifierResponse::default();
            }
            let val = clamp(_input.value - 1, 0, 4);

            FiringModifierResponse {
                burst_delay_add: val as f64 * (-0.625 / 30.0),
                ..Default::default()
            }
        }),
    );

    add_mmr(
        Perks::RatPack,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                let val = clamp(_input.value - 1, 0, 4);
                MagazineModifierResponse {
                    magazine_add: val as f64 * if val == 4 { 2.25 } else { 2.0 },
                    ..Default::default()
                }
            },
        ),
    );

    add_fmr(
        Perks::RideTheBull,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let extra_value = _input.calc_data.shots_fired_this_mag / 10.0;
            let val = clamp(_input.value + extra_value as u32, 0, 2);
            FiringModifierResponse {
                burst_delay_add: val as f64 * (-0.25 / 30.0),
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::SpinningUp,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let extra_value = _input.calc_data.shots_fired_this_mag / 12.0;
            let val = clamp(_input.value + extra_value as u32, 0, 2);
            FiringModifierResponse {
                burst_delay_add: val as f64 * (-0.5 / 30.0),
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::CranialSpike,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            let val = clamp(_input.value, 0, 5) as i32;
            out.insert(StatHashes::RANGE.into(), 8 * val);
            out.insert(StatHashes::AIM_ASSIST.into(), 4 * val);
            out
        }),
    );

    add_rsmr(
        Perks::CranialSpike,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(_input.value, 0, 5) as i32;
            let rel = 0.97_f64.powi(val);
            ReloadModifierResponse {
                reload_time_scale: rel,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::CranialSpike,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let val = clamp(_input.value, 0, 5) as i32;
            RangeModifierResponse {
                range_stat_add: 8 * val,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::DarkForgedTrigger,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.value == 0 {
                return FiringModifierResponse::default();
            }
            if _input
                .calc_data
                .perk_value_map
                .get(&1319823571)
                .unwrap_or(&0)
                > &4
            {
                FiringModifierResponse {
                    burst_delay_add: -5.0 / 30.0,
                    ..Default::default()
                }
            } else {
                FiringModifierResponse {
                    burst_delay_add: -1.0 / 30.0,
                    ..Default::default()
                }
            }
        }),
    );

    add_dmr(
        Perks::HarmonicLaser,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (_input.value, _input.pvp) {
                (0, _) => 1.0,
                (1, true) => 1.03,
                (1, false) => 1.323,
                (2.., true) => 1.0625,
                (2.., false) => 1.687,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::AgersScepterCatalyst,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.8,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        }),
    );

    add_mmr(
        Perks::AgersScepterCatalyst,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                let mag_buff = if _input.value > 0 && _input.calc_data.total_shots_fired == 0.0 {
                    2.0
                } else {
                    1.0
                };
                MagazineModifierResponse {
                    magazine_scale: mag_buff,
                    ..Default::default()
                }
            },
        ),
    );

    add_dmr(
        Perks::ColdFusion,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = 0.0195 * clamp(_input.calc_data.total_shots_hit, 0.0, 41.0);
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + buff,
                ..Default::default()
            }
        }),
    );

    //Queenbreaker's sights
    add_dmr(
        Perks::MarksmanSights,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.38,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::MarksmanSights,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: (1800.0 / (60000.0 / 333.0)), // 300 + 333 = 633 ,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::Broadside,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match _input.value {
                0 => 1.0,
                1 => 1.18,
                2 => 1.39,
                3 => 1.59,
                4.. => 1.81,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::TemporalUnlimiter,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.value > 0 {
                return FiringModifierResponse {
                    burst_delay_add: 0.366,
                    ..Default::default()
                };
            }
            FiringModifierResponse::default()
        }),
    );

    add_dmr(
        Perks::TemporalUnlimiter,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let mut buff = if _input.pvp { 7.545 } else { 14.0 };
            //season 23
            //https://www.bungie.net/7/en/News/Article/season-23-weapons-preview
            if *_input.calc_data.enemy_type == EnemyType::CHAMPION {
                buff *= 2.0;
            }
            DamageModifierResponse {
                impact_dmg_scale: buff,
                crit_scale: 1.875,
                ..Default::default()
            }
        }),
    );

    add_mmr(
        Perks::FourthHorsemanCatalyst,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                MagazineModifierResponse {
                    magazine_add: 1.0,
                    ..Default::default()
                }
            },
        ),
    );

    add_dmr(
        Perks::BlackHole,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = if _input.calc_data.total_shots_hit % 2.0 == 1.0 {
                1.35
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::Impetus,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.5,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        }),
    );

    add_fmr(
        Perks::MarksmanSights,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: 0.333, // 300 + 333 = 633 ,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::Broadhead,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let broadhead_damage = if _input.pvp { 30.0 } else { 60.0 };
            let impact_damage = _input.calc_data.curr_firing_data.damage;
            let crit_mult = _input.calc_data.curr_firing_data.crit_mult;

            let impact_dmg_scale = (broadhead_damage + impact_damage) / impact_damage;

            let crit_scale = (impact_damage * crit_mult + broadhead_damage)
                / (impact_damage * impact_dmg_scale * crit_mult);

            DamageModifierResponse {
                impact_dmg_scale,
                crit_scale,
                ..Default::default()
            }
        }),
    );
    add_fmr(
        Perks::Desperation,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let duration = 7.0;
            if _input.value == 0 || _input.calc_data.time_total > duration {
                return FiringModifierResponse::default();
            }
            FiringModifierResponse {
                burst_delay_scale: 0.8,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::IonicReturn,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let current_crit = _input.calc_data.curr_firing_data.crit_mult;
            let crit_scale = (current_crit + (34.0 / 51.0)) / current_crit;
            DamageModifierResponse {
                impact_dmg_scale: 1.15,
                crit_scale,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::Unrepentant,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 || _input.pvp {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 3.0,
                ..Default::default()
            }
        }),
    );
    add_fmr(
        Perks::Unrepentant,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let shots_in_super_burst: f64 = 6.0;
            if _input.calc_data.total_shots_hit >= shots_in_super_burst || _input.value == 0 {
                return FiringModifierResponse::default();
            }
            FiringModifierResponse {
                burst_size_add: 3.0,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::ArcConductor,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.1,
                explosive_dmg_scale: 1.1,
                ..Default::default()
            }
        }),
    );
    add_hmr(
        Perks::ArcConductor,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value == 0 {
                    return HandlingModifierResponse::default();
                }
                HandlingModifierResponse {
                    stat_add: 100,
                    ..Default::default()
                }
            },
        ),
    );
    add_sbr(
        Perks::ArcConductor,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value == 1 {
                stats.insert(StatHashes::HANDLING.into(), 100);
            }
            stats
        }),
    );
    add_dmr(
        Perks::VoidLeech,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 || _input.pvp {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.2,
                explosive_dmg_scale: 1.2,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::InverseRelationship,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (_input.value, _input.pvp) {
                (0, _) => 1.0,
                (1, false) => 1.1,
                (2, false) => 1.2,
                (3.., false) => 1.4,
                (1, true) => 1.01,
                (2, true) => 1.025,
                (3.., true) => 1.05,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::Spindle,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let buff = 1.0 + (0.02 * _input.value as f64);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::TheRightChoice,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if ((_input.calc_data.total_shots_fired - 1.0) / 7.0) == 0.0 {
                // every 1,8,15... so on
                let buff = if _input.pvp { 1.15 } else { 3.525 };
                return DamageModifierResponse {
                    impact_dmg_scale: buff,
                    explosive_dmg_scale: buff,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        }),
    )
}
