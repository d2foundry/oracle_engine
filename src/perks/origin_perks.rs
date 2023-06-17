use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    add_dmr, add_epr, add_flmr, add_fmr, add_hmr, add_mmr, add_rmr, add_rr, add_rsmr, add_sbr,
    add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn origin_perks() {
    add_rr(
        Perks::VeistStinger,
        Box::new(|_input: ModifierResponseInput| -> RefundResponse {
            if !(_input.value > 0) {
                return RefundResponse::default();
            };
            let data = _input.cached_data.get("veist_stinger");
            let last_proc;
            if data.is_none() {
                last_proc = 0.0;
            } else {
                last_proc = *data.unwrap();
            };
            let time_since_last_proc = _input.calc_data.time_total - last_proc;
            if time_since_last_proc >= 4.0 && _input.value > 0 {
                let max_refund = _input.calc_data.base_mag - _input.calc_data.curr_mag;
                let refund_amount = (_input.calc_data.base_mag / 4.0).ceil() as i32;
                if max_refund > 0.0 {
                    _input
                        .cached_data
                        .insert("veist_stinger".to_string(), _input.calc_data.time_total);
                    let final_refund_ammount = clamp(refund_amount, 0, max_refund as i32);
                    return RefundResponse {
                        requirement: 1,
                        crit: false,
                        refund_mag: refund_amount,
                        refund_reserves: -final_refund_ammount,
                    };
                } else {
                    RefundResponse::default()
                }
            } else {
                RefundResponse::default()
            }
        }),
    );

    add_fmr(
        Perks::VeistStinger,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_scale: if _input.calc_data.weapon_type == &WeaponType::BOW
                    && _input.value > 0
                {
                    0.85
                } else {
                    1.0
                },
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::HakkeBreach,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if _input.value > 0 { 0.3 } else { 0.0 };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                crit_scale: 1.0,
            }
        }),
    );

    add_rmr(
        Perks::Alacrity,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_add = if _input.value > 0 { 20 } else { 0 };
            RangeModifierResponse {
                range_stat_add: range_add,
                ..Default::default()
            }
        }),
    );

    add_rsmr(
        Perks::Alacrity,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_add = if _input.value > 0 { 50 } else { 0 };
            ReloadModifierResponse {
                reload_stat_add: reload_add,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::Alacrity,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let range = if _input.value > 0 { 20 } else { 0 };
            let reload = if _input.value > 0 { 50 } else { 0 };
            let stability = if _input.value > 0 { 20 } else { 0 };
            let aim_assist = if _input.value > 0 { 10 } else { 0 };
            map.insert(StatHashes::RANGE.into(), range);
            map.insert(StatHashes::RELOAD.into(), reload);
            map.insert(StatHashes::STABILITY.into(), stability);
            map.insert(StatHashes::AIM_ASSIST.into(), aim_assist);
            map
        }),
    );

    add_sbr(
        Perks::Ambush,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let range = if _input.is_enhanced { 30 } else { 20 };
            let handling = if _input.is_enhanced { 40 } else { 20 };
            if _input.calc_data.time_total < 2.0 && _input.value > 0 {
                map.insert(StatHashes::RANGE.into(), range);
                map.insert(StatHashes::HANDLING.into(), handling);
            }
            map
        }),
    );

    add_rmr(
        Perks::Ambush,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_add = if _input.is_enhanced { 30 } else { 20 };
            if _input.calc_data.time_total < 2.0 && _input.value > 0 {
                RangeModifierResponse {
                    range_stat_add: range_add,
                    ..Default::default()
                }
            } else {
                RangeModifierResponse::default()
            }
        }),
    );

    add_hmr(
        Perks::Ambush,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let handling_add = if _input.is_enhanced { 40 } else { 20 };
                if _input.calc_data.time_total < 2.0 && _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: handling_add,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_dmr(
        Perks::Ambush,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 || _input.pvp {
                return DamageModifierResponse::default();
            }
            let damage_mult = if _input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
                1.0888
            } else {
                1.1078
            };

            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        }),
    );

    add_fmr(
        Perks::Ambush,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_scale: if _input.calc_data.weapon_type == &WeaponType::BOW
                    && _input.value > 0
                {
                    0.9
                } else {
                    1.0
                },
                ..Default::default()
            }
        }),
    );

    add_hmr(
        Perks::HotSwap,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let handling_add = if _input.is_enhanced { 60 } else { 30 };
                if _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: handling_add,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::FluidDynamics,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_add = if _input.is_enhanced { 35 } else { 30 };
            if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0 {
                ReloadModifierResponse {
                    reload_stat_add: reload_add,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::FluidDynamics,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let reload = if _input.is_enhanced { 35 } else { 30 };
            let stability = if _input.is_enhanced { 25 } else { 20 };
            if _input.calc_data.shots_fired_this_mag <= _input.calc_data.base_mag / 2.0
                && _input.value > 0
            {
                map.insert(StatHashes::RELOAD.into(), reload);
                map.insert(StatHashes::STABILITY.into(), stability);
            }
            map
        }),
    );

    add_rsmr(
        Perks::QuietMoment,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 40,
                    reload_time_scale: 0.95,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::QuietMoment,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if _input.value > 0 {
                map.insert(StatHashes::RELOAD.into(), 40);
            }
            map
        }),
    );

    add_rsmr(
        Perks::BitterSpite,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(_input.value, 0, 5) as i32;
            let mult = match val {
                0 => 1.0,
                1 => 0.97,
                2 => 0.96,
                3 => 0.95,
                4 => 0.92,
                5 => 0.90,
                _ => 0.90,
            };
            ReloadModifierResponse {
                reload_stat_add: val * 10,
                reload_time_scale: mult,
            }
        }),
    );

    add_sbr(
        Perks::BitterSpite,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let val = clamp(_input.value, 0, 5) as i32;
            map.insert(StatHashes::RELOAD.into(), val * 10);
            map
        }),
    );

    add_rmr(
        Perks::RightHook,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_add = if _input.is_enhanced { 20 } else { 10 };
            if _input.value > 0 {
                RangeModifierResponse {
                    range_stat_add: range_add,
                    ..Default::default()
                }
            } else {
                RangeModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::RightHook,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let stat_bump = if _input.is_enhanced { 20 } else { 10 };
            if _input.value > 0 {
                map.insert(StatHashes::AIM_ASSIST.into(), stat_bump);
                map.insert(StatHashes::RANGE.into(), stat_bump);
            }
            map
        }),
    );

    add_hmr(
        Perks::SearchParty,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    HandlingModifierResponse {
                        ads_scale: 0.85,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_mmr(
        Perks::RunnethOver,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                let val = clamp(_input.value, 0, 5) as f64;
                MagazineModifierResponse {
                    magazine_scale: 1.0 + val * 0.1,
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::TexBalancedStock,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if _input.value > 0 {
                map.insert(StatHashes::HANDLING.into(), 20);
                map.insert(StatHashes::RELOAD.into(), 20);
            }
            map
        }),
    );

    add_hmr(
        Perks::TexBalancedStock,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: 50,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::TexBalancedStock,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 20,
                    reload_time_scale: 0.9,
                    ..Default::default()
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::SurosSynergy,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if _input.value > 0 {
                out.insert(StatHashes::HANDLING.into(), 40);
            }
            out
        }),
    );

    add_hmr(
        Perks::SurosSynergy,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: 40,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_flmr(
        Perks::SurosSynergy,
        Box::new(|_input: ModifierResponseInput| -> FlinchModifierResponse {
            if _input.value > 0 {
                FlinchModifierResponse { flinch_scale: 0.80 }
            } else {
                FlinchModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::HarmonicResonance,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if _input.value == 1 {
                out.insert(StatHashes::HANDLING.into(), 10);
            }
            if _input.value > 1 {
                out.insert(StatHashes::RELOAD.into(), 20);
                out.insert(StatHashes::HANDLING.into(), 20);
            }
            out
        }),
    );

    add_rsmr(
        Perks::HarmonicResonance,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let stat_bump = if _input.value > 1 { 20 } else { 0 };
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: stat_bump,
                    reload_time_scale: 0.95,
                    ..Default::default()
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_hmr(
        Perks::HarmonicResonance,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let stat_bump = 10 * clamp(_input.value, 0, 2);
                HandlingModifierResponse {
                    stat_add: stat_bump as i32,
                    ..Default::default()
                }
            },
        ),
    );
}
