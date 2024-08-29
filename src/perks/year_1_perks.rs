use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, BungieHash, DamageType, StatBump, StatHashes, WeaponType},
    logging::extern_log,
    weapons::Weapon,
};

use super::{
    add_dmr, add_edr, add_epr, add_fmr, add_hmr, add_imr, add_mmr, add_rmr, add_rr, add_rsmr,
    add_sbr, add_vmr, clamp, lerp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        VelocityModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_1_perks() {
    add_sbr(
        Perks::ThreatDetector,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let stats = match _input.value {
                    0 => (0, 0, 0),
                    1 => (15, 15, 25),
                    _ => (40, 55, 100),
                };
                let mut out = HashMap::new();
                out.insert(StatHashes::STABILITY.into(), stats.0);
                out.insert(StatHashes::RELOAD.into(), stats.1);
                out.insert(StatHashes::HANDLING.into(), stats.2);
                out
            },
        ),
    );

    add_dmr(
        Perks::HighImpactReserves,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut out_dmg_scale = 1.0;
            let base = if _input.pvp { 0.03 } else { 0.121 };
            let max = if _input.pvp { 0.06 } else { 0.256 };
            let threshold_divisor = if _input.is_enhanced { 4.0 / 3.0 } else { 2.0 };
            if _input.calc_data.curr_mag <= _input.calc_data.curr_mag / threshold_divisor {
                let t = 1.0
                    - (_input.calc_data.curr_mag - 1.0)
                        / ((_input.calc_data.base_mag / threshold_divisor) - 1.0);
                if t > 0.0 {
                    out_dmg_scale = lerp(base, max, t);
                }
            };
            DamageModifierResponse {
                impact_dmg_scale: out_dmg_scale,
                explosive_dmg_scale: out_dmg_scale,
                ..Default::default()
            }
        }),
    );

    //Confirmed by harm <3
    add_hmr(
        Perks::ThreatDetector,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let val = clamp(_input.value, 0, 2) as i32;
                let stat = match val {
                    0 => 0,
                    1 => 25,
                    2 => 100,
                    _ => 100,
                };
                let time_scale = 0.9_f64.powi(val);
                HandlingModifierResponse {
                    stat_add: stat,
                    draw_scale: time_scale,
                    stow_scale: time_scale,
                    ads_scale: time_scale,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::ThreatDetector,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if _input.value == 1 {
                reload = 15;
            } else if _input.value == 2 {
                reload = 55;
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: 1.0,
            }
        }),
    );

    add_mmr(
        Perks::AmbitiousAssassin,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                let val = clamp(_input.value, 0, 15) as f64;
                if _input.calc_data.total_shots_fired == 0.0 {
                    let mut mag_mult = 1.0;
                    if *_input.calc_data.ammo_type == AmmoType::PRIMARY {
                        mag_mult += 0.2 * val;
                    } else {
                        mag_mult += 0.1 * val;
                    };
                    return MagazineModifierResponse {
                        magazine_stat_add: 0,
                        magazine_scale: clamp(mag_mult, 1.0, 2.5),
                        magazine_add: 0.0,
                    };
                };
                MagazineModifierResponse {
                    magazine_stat_add: 0,
                    magazine_scale: 1.0,
                    magazine_add: 0.0,
                }
            },
        ),
    );

    add_dmr(
        Perks::BoxBreathing,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.calc_data.total_shots_fired == 0.0 && _input.value > 0 {
                let mut crit_mult =
                    (_input.calc_data.base_crit_mult + 1.0) / _input.calc_data.base_crit_mult;
                if *_input.calc_data.weapon_type == WeaponType::SCOUTRIFLE {
                    crit_mult *= 0.95;
                }
                return DamageModifierResponse {
                    crit_scale: crit_mult,
                    ..Default::default()
                };
            };
            DamageModifierResponse::default()
        }),
    );

    add_fmr(
        Perks::Desperado,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            let duration = if _input.is_enhanced { 7.0 } else { 6.0 };
            if _input.calc_data.time_total < duration && _input.value > 0 {
                delay_mult = 0.7;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::ExplosivePayload,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.pvp {
                DamageModifierResponse::default()
            } else {
                DamageModifierResponse {
                    explosive_dmg_scale: 1.3,
                    ..Default::default()
                }
            }
        }),
    );

    add_epr(
        Perks::ExplosivePayload,
        Box::new(
            |_input: ModifierResponseInput| -> ExplosivePercentResponse {
                ExplosivePercentResponse {
                    percent: 0.5,
                    delyed: 0.0,
                    retain_base_total: true,
                }
            },
        ),
    );

    add_dmr(
        Perks::TimedPayload,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.pvp {
                DamageModifierResponse::default()
            } else {
                // let damage_mult = ((1.0 /  _input.calc_data.base_crit_mult) * 0.15) + 1.0;
                DamageModifierResponse {
                    explosive_dmg_scale: 1.3,
                    ..Default::default()
                }
            }
        }),
    );

    add_epr(
        Perks::TimedPayload,
        Box::new(
            |_input: ModifierResponseInput| -> ExplosivePercentResponse {
                ExplosivePercentResponse {
                    percent: 0.5,
                    delyed: 0.6,
                    retain_base_total: true,
                }
            },
        ),
    );

    add_sbr(
        Perks::FieldPrep,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut out = HashMap::new();
                if _input.value > 0 {
                    let reload = if _input.is_enhanced { 55 } else { 50 };
                    out.insert(StatHashes::RELOAD.into(), reload);
                };
                let mut reserves = if _input.is_enhanced { 40 } else { 30 };
                if *_input.calc_data.weapon_type == WeaponType::GRENADELAUNCHER {
                    reserves -= 10;
                };
                out.insert(StatHashes::INVENTORY_SIZE.into(), reserves);
                out
            },
        ),
    );

    add_rsmr(
        Perks::FieldPrep,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            let mut reload_mult = 1.0;
            if _input.value > 0 {
                reload = if _input.is_enhanced { 55 } else { 50 };
                reload_mult = if _input.is_enhanced { 0.77 } else { 0.8 };
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: reload_mult,
            }
        }),
    );

    add_imr(
        Perks::FieldPrep,
        Box::new(
            |_input: ModifierResponseInput| -> InventoryModifierResponse {
                InventoryModifierResponse {
                    inv_stat_add: if _input.is_enhanced { 40 } else { 30 },
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::FieldPrep,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value >= 1 {
                    HandlingModifierResponse {
                        stow_scale: 0.8,
                        draw_scale: 0.8,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::FirmlyPlanted,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut handling = if _input.is_enhanced { 35 } else { 30 };
                let mut stabiltiy = if _input.is_enhanced { 25 } else { 20 };
                if *_input.calc_data.weapon_type == WeaponType::FUSIONRIFLE {
                    handling /= 2;
                    stabiltiy /= 2;
                };
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::HANDLING.into(), handling);
                    out.insert(StatHashes::STABILITY.into(), stabiltiy);
                }
                out
            },
        ),
    );

    add_hmr(
        Perks::FirmlyPlanted,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let mut handling = if _input.is_enhanced { 35 } else { 30 };
                if *_input.calc_data.weapon_type == WeaponType::FUSIONRIFLE {
                    handling /= 2;
                };
                if _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: handling,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_fmr(
        Perks::FullAutoTrigger,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            if *_input.calc_data.weapon_type == WeaponType::SHOTGUN {
                delay_mult = 0.91;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        }),
    );

    add_rr(
        Perks::TripleTap,
        Box::new(|_input: ModifierResponseInput| -> RefundResponse {
            RefundResponse {
                crit: true,
                requirement: 3,
                refund_mag: 1,
                refund_reserves: 0,
            }
        }),
    );

    add_sbr(
        Perks::HipFireGrip,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::AIM_ASSIST.into(), 15);
                    out.insert(StatHashes::STABILITY.into(), 25);
                };
                out
            },
        ),
    );

    add_rmr(
        Perks::HipFireGrip,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let mut hf_range_scale = 1.2;
            if *_input.calc_data.weapon_type == WeaponType::FUSIONRIFLE
                || *_input.calc_data.weapon_type == WeaponType::SHOTGUN
                || _input.calc_data.intrinsic_hash == 2770223582
            //last word
            {
                hf_range_scale = 1.0;
            };
            RangeModifierResponse {
                range_stat_add: 0,
                range_all_scale: 1.0,
                range_hip_scale: hf_range_scale,
                range_zoom_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::ImpactCasing,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.1,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::MovingTarget,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let aim_assist = if _input.is_enhanced { 11 } else { 10 };
                let mut out = HashMap::new();
                if _input.value >= 1 {
                    out.insert(StatHashes::AIM_ASSIST.into(), aim_assist);
                }
                out
            },
        ),
    );

    add_sbr(
        Perks::OpeningShot,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut aim_assist: f64 = if _input.is_enhanced { 25.0 } else { 20.0 };
                let mut range: f64 = if _input.is_enhanced { 30.0 } else { 25.0 };
                let mut out = HashMap::new();
                if *_input.calc_data.ammo_type == AmmoType::SPECIAL {
                    aim_assist /= 2.0;
                    range /= 2.0;
                }
                if _input.value > 0 {
                    out.insert(StatHashes::AIM_ASSIST.into(), aim_assist.ceil() as i32);
                    out.insert(StatHashes::RANGE.into(), range.ceil() as i32);
                }
                out
            },
        ),
    );

    add_rmr(
        Perks::OpeningShot,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range: f64 = if _input.is_enhanced { 30.0 } else { 25.0 };
            if _input.calc_data.total_shots_fired != 0.0 || _input.value == 0 {
                range = 0.0;
            };
            if *_input.calc_data.ammo_type == AmmoType::SPECIAL {
                range /= 2.0;
            }
            RangeModifierResponse {
                range_stat_add: range.ceil() as i32,
                range_all_scale: 1.0,
                range_hip_scale: 1.0,
                range_zoom_scale: 1.0,
            }
        }),
    );

    add_sbr(
        Perks::Outlaw,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::RELOAD.into(), 70);
                }
                out
            },
        ),
    );

    add_rsmr(
        Perks::Outlaw,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let duration = if _input.is_enhanced { 7.0 } else { 6.0 };
            if _input.value > 0 && _input.calc_data.time_total < duration {
                ReloadModifierResponse {
                    reload_stat_add: 70,
                    reload_time_scale: 0.9,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_vmr(
        Perks::RangeFinder,
        Box::new(
            |_input: ModifierResponseInput| -> VelocityModifierResponse {
                VelocityModifierResponse {
                    velocity_scaler: 1.05,
                }
            },
        ),
    );

    add_sbr(
        Perks::SlideShot,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let stability = if _input.is_enhanced { 35 } else { 30 };
                let range = if _input.is_enhanced { 25 } else { 20 };
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::STABILITY.into(), stability);
                    out.insert(StatHashes::RANGE.into(), range);
                }
                out
            },
        ),
    );

    add_rmr(
        Perks::SlideShot,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range;
            if *_input.calc_data.weapon_type == WeaponType::FUSIONRIFLE {
                range = 0; //only applies to first proj so like should do alot less
            } else if _input.value > 0 {
                range = if _input.is_enhanced { 25 } else { 20 }
            } else {
                range = 0;
            }
            RangeModifierResponse {
                range_stat_add: range,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::SlideWays,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let stability = if _input.is_enhanced { 25 } else { 20 };
                let handling = if _input.is_enhanced { 25 } else { 20 };
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::STABILITY.into(), stability);
                    out.insert(StatHashes::HANDLING.into(), handling);
                }
                out
            },
        ),
    );

    add_hmr(
        Perks::SlideWays,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let handling = if _input.value > 0 { 20 } else { 0 };
                HandlingModifierResponse {
                    stat_add: handling,
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::Snapshot,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let mut ads_mult = 0.5;
                if *_input.calc_data.ammo_type == AmmoType::SPECIAL {
                    ads_mult = 0.8; //its 0.8 from my testing idk
                };
                HandlingModifierResponse {
                    ads_scale: ads_mult,
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::TapTheTrigger,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut stability = if _input.is_enhanced { 44 } else { 40 };
                if *_input.calc_data.weapon_type == WeaponType::FUSIONRIFLE {
                    stability /= 4;
                }
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::STABILITY.into(), stability);
                }
                out
            },
        ),
    );

    add_dmr(
        Perks::Rampage,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(_input.value, 0, 3);
            let mut damage_mult = 1.1_f64.powi(val as i32) - 1.0;
            let duration = if _input.is_enhanced { 5.0 } else { 4.0 };
            if _input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            if _input.calc_data.perk_value_map.contains_key(&630329983) && !_input.pvp {
                //huckleberry
                damage_mult *= 2.0;
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::KillClip,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = if _input.value > 0 { 0.25 } else { 0.0 };
            let duration = if _input.is_enhanced { 5.0 } else { 4.0 };
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
        Perks::BackupPlan,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = if _input.value > 0 { 0.2 } else { 0.0 };
            let duration = if _input.is_enhanced { 2.2 } else { 2.0 };
            if _input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 - damage_mult,
                explosive_dmg_scale: 1.0 - damage_mult,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::BackupPlan,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let mut firing_mult = if _input.value > 0 { 0.7 } else { 1.0 };
            let duration = if _input.is_enhanced { 2.2 } else { 2.0 };
            if _input.calc_data.time_total > duration {
                firing_mult = 0.0;
            };
            FiringModifierResponse {
                burst_delay_scale: firing_mult,
                ..Default::default()
            }
        }),
    );

    add_hmr(
        Perks::BackupPlan,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let mut handling_add = if _input.value > 0 { 100 } else { 0 };
                let duration = if _input.is_enhanced { 2.2 } else { 2.0 };
                if _input.calc_data.time_total > duration {
                    handling_add = 0;
                };
                HandlingModifierResponse {
                    stat_add: handling_add,
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::BackupPlan,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut handling = if _input.value > 0 { 100 } else { 0 };
                let duration = if _input.is_enhanced { 2.2 } else { 2.0 };
                if _input.calc_data.time_total > duration {
                    handling = 0;
                };
                let mut out = HashMap::new();
                if _input.value > 0 {
                    out.insert(StatHashes::HANDLING.into(), handling);
                }
                out
            },
        ),
    );

    add_edr(
        Perks::ClusterBomb,
        Box::new(|_input: ModifierResponseInput| -> ExtraDamageResponse {
            ExtraDamageResponse {
                additive_damage: 350.0 * 0.04,
                combatant_scale: true,
                crit_scale: false,
                increment_total_time: false,
                time_for_additive_damage: 0.8,
                times_to_hit: 6,
                weapon_scale: true,
                hit_at_same_time: true,
                is_dot: false,
            }
        }),
    );

    add_dmr(
        Perks::DisruptionBreak,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = if _input.value > 0 { 0.5 } else { 0.0 };
            let duration = if _input.is_enhanced { 5.0 } else { 4.0 };
            if _input.calc_data.time_total > duration
                || _input.calc_data.damage_type != &DamageType::KINETIC
            {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                ..Default::default()
            }
        }),
    );

    add_hmr(
        Perks::QuickDraw,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    HandlingModifierResponse {
                        draw_add: 100,
                        draw_scale: 0.95,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::QuickDraw,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut map = HashMap::new();
                if _input.value > 0 {
                    map.insert(StatHashes::HANDLING.into(), 100);
                }
                map
            },
        ),
    );

    add_hmr(
        Perks::PulseMonitor,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: 50,
                        draw_scale: 0.95,
                        stow_scale: 0.95,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::PulseMonitor,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut map = HashMap::new();
                if _input.value > 0 {
                    map.insert(StatHashes::HANDLING.into(), 50);
                }
                map
            },
        ),
    );

    add_sbr(
        Perks::UnderDog,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut map = HashMap::new();
                if _input.value > 0 {
                    map.insert(StatHashes::RELOAD.into(), 100);
                }
                map
            },
        ),
    );

    add_rsmr(
        Perks::UnderDog,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 0.9,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::UnderPressure,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let mut map = HashMap::new();
                let buff = if _input.is_enhanced { 35 } else { 30 };
                if _input.value > 0 {
                    map.insert(StatHashes::STABILITY.into(), buff);
                }
                map
            },
        ),
    );
    add_fmr(
        Perks::PhaseMag,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: 1.0 / 30.0,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::PhaseMag,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            //set up precision smg to get damage values from
            let precision = Weapon::generate_weapon(
                0, 24,         //smg
                1636108362, //precision
                1,          //primary
                3949783978, //strand
            )
            .unwrap();

            let p_data = precision.get_damage_profile(_input.pvp);

            let lightweight_body = _input.calc_data.curr_firing_data.damage;
            let lightweight_crit = _input.calc_data.curr_firing_data.crit_mult;

            let precision_body = p_data.impact_dmg;
            let precision_crit = p_data.crit_mult;
            DamageModifierResponse {
                impact_dmg_scale: precision_body / lightweight_body,
                explosive_dmg_scale: precision_body / lightweight_body,
                crit_scale: precision_crit / lightweight_crit,
                ..Default::default()
            }
        }),
    );
}
