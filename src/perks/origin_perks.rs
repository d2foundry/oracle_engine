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
        |input: ModifierResponseInput| -> RefundResponse {
            let data = input.cached_data.get("veist_stinger");
            let last_proc = *data.unwrap_or(&1.0);
            let time_since_last_proc = input.calc_data.time_total - last_proc;
            let max_refund = input.calc_data.base_mag - input.calc_data.curr_mag;

            if input.value == 0 || time_since_last_proc < 4.0 || max_refund == 0.0 {
                return RefundResponse::default();
            };

            input
                .cached_data
                .insert("veist_stinger".to_string(), input.calc_data.time_total);
            let refund_amount = (input.calc_data.base_mag / 4.0).ceil() as i32;
            let final_refund_ammount = refund_amount.clamp(0, max_refund as i32);
            RefundResponse {
                requirement: 1,
                crit: false,
                refund_mag: refund_amount,
                refund_reserves: -final_refund_ammount,
            }
        },
    );

    add_fmr(
        Perks::VeistStinger,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_scale: if input.calc_data.weapon_type == &WeaponType::BOW
                    && input.value > 0
                {
                    0.85
                } else {
                    1.0
                },
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::HakkeBreach,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }

            let buff = match input.value {
                1 => 1.15,
                2 => 1.45,
                3 => 1.30,
                _ => 1.30,
            };

            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                crit_scale: 1.0,
            }
        },
    );

    add_rmr(
        Perks::Alacrity,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_add = if input.value > 0 { 20 } else { 0 };
            RangeModifierResponse {
                range_stat_add: range_add,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::Alacrity,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_add = if input.value > 0 { 50 } else { 0 };
            ReloadModifierResponse {
                reload_stat_add: reload_add,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::Alacrity,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let range = if input.value > 0 { 20 } else { 0 };
            let reload = if input.value > 0 { 50 } else { 0 };
            let stability = if input.value > 0 { 20 } else { 0 };
            let aim_assist = if input.value > 0 { 10 } else { 0 };
            map.insert(StatHashes::RANGE.into(), range);
            map.insert(StatHashes::RELOAD.into(), reload);
            map.insert(StatHashes::STABILITY.into(), stability);
            map.insert(StatHashes::AIM_ASSIST.into(), aim_assist);
            map
        },
    );

    add_sbr(
        Perks::Ambush,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let range = if input.is_enhanced { 30 } else { 20 };
            let handling = if input.is_enhanced { 40 } else { 20 };
            if input.calc_data.time_total < 2.0 && input.value > 0 {
                map.insert(StatHashes::RANGE.into(), range);
                map.insert(StatHashes::HANDLING.into(), handling);
            }
            map
        },
    );

    add_rmr(
        Perks::Ambush,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_add = if input.is_enhanced { 30 } else { 20 };
            if input.calc_data.time_total < 2.0 && input.value > 0 {
                RangeModifierResponse {
                    range_stat_add: range_add,
                    ..Default::default()
                }
            } else {
                RangeModifierResponse::default()
            }
        },
    );

    add_hmr(
        Perks::Ambush,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling_add = if input.is_enhanced { 40 } else { 20 };
            if input.calc_data.time_total < 2.0 && input.value > 0 {
                HandlingModifierResponse {
                    stat_add: handling_add,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::Ambush,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 || input.pvp {
                return DamageModifierResponse::default();
            }
            let damage_mult = if input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
                1.0888
            } else {
                1.1078
            };

            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_fmr(
        Perks::Ambush,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_scale: if input.calc_data.weapon_type == &WeaponType::BOW
                    && input.value > 0
                {
                    0.9
                } else {
                    1.0
                },
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::HotSwap,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling_add = if input.is_enhanced { 60 } else { 30 };
            if input.value > 0 {
                HandlingModifierResponse {
                    stat_add: handling_add,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_rsmr(
        Perks::FluidDynamics,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_add = if input.is_enhanced { 35 } else { 30 };
            if input.calc_data.shots_fired_this_mag <= input.calc_data.base_mag / 2.0 {
                ReloadModifierResponse {
                    reload_stat_add: reload_add,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::FluidDynamics,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let reload = if input.is_enhanced { 35 } else { 30 };
            let stability = if input.is_enhanced { 25 } else { 20 };
            if input.calc_data.shots_fired_this_mag <= input.calc_data.base_mag / 2.0
                && input.value > 0
            {
                map.insert(StatHashes::RELOAD.into(), reload);
                map.insert(StatHashes::STABILITY.into(), stability);
            }
            map
        },
    );

    add_rsmr(
        Perks::QuietMoment,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 40,
                    reload_time_scale: 0.95,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::QuietMoment,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if input.value > 0 {
                map.insert(StatHashes::RELOAD.into(), 40);
            }
            map
        },
    );

    add_rsmr(
        Perks::BitterSpite,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(input.value, 0, 5) as i32;
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
        },
    );

    add_sbr(
        Perks::BitterSpite,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let val = clamp(input.value, 0, 5) as i32;
            map.insert(StatHashes::RELOAD.into(), val * 10);
            map
        },
    );

    add_rmr(
        Perks::RightHook,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_add = if input.is_enhanced { 20 } else { 10 };
            if input.value > 0 {
                RangeModifierResponse {
                    range_stat_add: range_add,
                    ..Default::default()
                }
            } else {
                RangeModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::RightHook,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let stat_bump = if input.is_enhanced { 20 } else { 10 };
            if input.value > 0 {
                map.insert(StatHashes::AIM_ASSIST.into(), stat_bump);
                map.insert(StatHashes::RANGE.into(), stat_bump);
            }
            map
        },
    );

    add_hmr(
        Perks::SearchParty,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                HandlingModifierResponse {
                    ads_scale: 0.85,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_mmr(
        Perks::RunnethOver,
        |input: ModifierResponseInput| -> MagazineModifierResponse {
            let val = clamp(input.value, 0, 5) as f64;
            MagazineModifierResponse {
                magazine_scale: 1.0 + val * 0.1,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::TexBalancedStock,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if input.value > 0 {
                map.insert(StatHashes::HANDLING.into(), 20);
                map.insert(StatHashes::RELOAD.into(), 20);
                map.insert(StatHashes::RANGE.into(), 20);
            }
            map
        },
    );

    add_hmr(
        Perks::TexBalancedStock,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                HandlingModifierResponse {
                    stat_add: 50,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_rsmr(
        Perks::TexBalancedStock,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 20,
                    reload_time_scale: 0.9,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );
    add_rmr(
        Perks::TexBalancedStock,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            if input.value == 0 {
                return RangeModifierResponse::default();
            }
            RangeModifierResponse {
                range_stat_add: 20,
                ..Default::default()
            }
        },
    );
    add_sbr(
        Perks::SurosSynergy,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::HANDLING.into(), 40);
            }
            out
        },
    );

    add_hmr(
        Perks::SurosSynergy,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                HandlingModifierResponse {
                    stat_add: 40,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_flmr(
        Perks::SurosSynergy,
        |input: ModifierResponseInput| -> FlinchModifierResponse {
            if input.value > 0 {
                FlinchModifierResponse { flinch_scale: 0.80 }
            } else {
                FlinchModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::HarmonicResonance,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value == 1 {
                out.insert(StatHashes::HANDLING.into(), 10);
            }
            if input.value > 1 {
                out.insert(StatHashes::RELOAD.into(), 20);
                out.insert(StatHashes::HANDLING.into(), 20);
            }
            out
        },
    );

    add_rsmr(
        Perks::HarmonicResonance,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let stat_bump = if input.value > 1 { 20 } else { 0 };
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: stat_bump,
                    reload_time_scale: 0.95,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_hmr(
        Perks::HarmonicResonance,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let stat_bump = 10 * clamp(input.value, 0, 2);
            HandlingModifierResponse {
                stat_add: stat_bump as i32,
                ..Default::default()
            }
        },
    );
}
