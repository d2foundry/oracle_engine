use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_5_perks() {
    add_fmr(
        Perks::CascadePoint,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let duration = if input.is_enhanced { 3.0 } else { 2.5 };
            let mut delay_mult = 1.0;
            if input.calc_data.time_total < duration && input.value > 0 {
                if *input.calc_data.weapon_type == WeaponType::MACHINEGUN
                    || *input.calc_data.weapon_type == WeaponType::SUBMACHINEGUN
                {
                    delay_mult = 0.7;
                } else {
                    delay_mult = 0.6;
                }
            }
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        },
    );

    add_sbr(
        Perks::Encore,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let val = clamp(input.value, 0, 4) as i32;
            let stability_boost = 8 * val;
            let range_boost = 5 * val;
            map.insert(StatHashes::RANGE.into(), range_boost);
            map.insert(StatHashes::STABILITY.into(), stability_boost);
            map
        },
    );

    add_rmr(
        Perks::Encore,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let val = clamp(input.value, 0, 4) as i32;
            let range_boost = 5 * val;
            RangeModifierResponse {
                range_stat_add: range_boost,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::FocusedFury,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let shots_needed = (input.calc_data.base_mag
                * (input.calc_data.curr_firing_data.burst_size as f64))
                / 2.0;

            let dmg_boost = if input.calc_data.total_shots_fired >= shots_needed || input.value > 0
            {
                1.2
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: dmg_boost,
                explosive_dmg_scale: dmg_boost,
                crit_scale: 1.0,
            }
        },
    );

    add_rmr(
        Perks::FragileFocus,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_bonus = if input.value > 0 { 20 } else { 0 };
            RangeModifierResponse {
                range_stat_add: range_bonus,
                range_all_scale: 1.0,
                range_hip_scale: 1.0,
                range_zoom_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::FragileFocus,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let mut range_bonus = 0;
            if input.value > 0 {
                range_bonus = 20;
            };
            map.insert(StatHashes::RANGE.into(), range_bonus);
            map
        },
    );

    add_dmr(
        Perks::GutShot,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let high_weapons = [
                WeaponType::AUTORIFLE,
                WeaponType::HANDCANNON,
                WeaponType::BOW,
                WeaponType::SCOUTRIFLE,
            ];
            let dmg_scale: f64;
            let crit_scale: f64;
            if high_weapons.contains(input.calc_data.weapon_type) {
                dmg_scale = 1.2;
                crit_scale = 1.0 / 1.2;
            } else {
                dmg_scale = 1.1;
                crit_scale = 1.0 / 1.1;
            };
            // if  input.calc_data.base_crit_mult <= 1.0 {
            //     crit_scale = 1.0;
            // }
            DamageModifierResponse {
                impact_dmg_scale: dmg_scale,
                explosive_dmg_scale: dmg_scale,
                crit_scale,
            }
        },
    );

    add_sbr(
        Perks::OffhandStrike,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let mut stability_boost = 0;
            if input.value > 0 {
                stability_boost = 30;
            };
            map.insert(StatHashes::STABILITY.into(), stability_boost);
            map
        },
    );

    add_rmr(
        Perks::OffhandStrike,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range_hip_mult = 1.0;
            if input.value > 0 {
                range_hip_mult = 1.45;
            };
            RangeModifierResponse {
                range_stat_add: 0,
                range_all_scale: 1.0,
                range_hip_scale: range_hip_mult,
                range_zoom_scale: 1.0,
            }
        },
    );

    add_hmr(
        Perks::Slickdraw,
        |_: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                stat_add: 100,
                stow_scale: 1.0,
                draw_scale: 0.95,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::Slickdraw,
        |_: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            map.insert(StatHashes::HANDLING.into(), 100);
            map
        },
    );

    add_sbr(
        Perks::StatsForAll,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            let mut stability_boost = 0;
            let mut range_boost = 0;
            let mut reload_boost = 0;
            let mut handling_boost = 0;
            if input.value > 0 {
                stability_boost = 10;
                range_boost = 10;
                reload_boost = 35;
                handling_boost = 35;
            };
            out.insert(StatHashes::STABILITY.into(), stability_boost);
            out.insert(StatHashes::RANGE.into(), range_boost);
            out.insert(StatHashes::RELOAD.into(), reload_boost);
            out.insert(StatHashes::HANDLING.into(), handling_boost);
            out
        },
    );

    add_hmr(
        Perks::StatsForAll,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling_boost = 0;
            let duration = if input.is_enhanced { 11.0 } else { 10.0 };
            if input.value > 0 && input.calc_data.time_total < duration {
                handling_boost = 35;
            };
            HandlingModifierResponse {
                stat_add: handling_boost,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::StatsForAll,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let mut range = 0;
            let mut range_mult = 1.0;
            if input.value > 0 {
                range = 10;
                range_mult = 1.05;
            };
            RangeModifierResponse {
                range_stat_add: range,
                range_all_scale: range_mult,
                range_hip_scale: 1.0,
                range_zoom_scale: 1.0,
            }
        },
    );

    add_rsmr(
        Perks::StatsForAll,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            let mut reload_mult = 1.0;
            let duration = if input.is_enhanced { 11.0 } else { 10.0 };
            if input.value > 0 && input.calc_data.time_total < duration {
                reload = 35;
                reload_mult = 0.95;
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: reload_mult,
            }
        },
    );

    add_sbr(
        Perks::SteadyHands,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let mut handling = 0;
            if input.value > 0 {
                handling = 100;
            };
            map.insert(StatHashes::HANDLING.into(), handling);
            map
        },
    );

    add_hmr(
        Perks::SteadyHands,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling_mult = 1.0;
            let mut handling = 0;
            let duration = if input.is_enhanced { 9.0 } else { 8.5 };
            if input.value > 0 && input.calc_data.time_total < duration {
                handling_mult = 0.825;
                handling = 100;
            };
            HandlingModifierResponse {
                stat_add: handling,
                stow_scale: handling_mult,
                draw_scale: handling_mult,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::TargetLock,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let nerf = 0.625; //patch 7.1.5
            let enh_increase = if input.is_enhanced { 1.2 } else { 1.0 };
            let low_end_dmg = 0.0934 * enh_increase * nerf;
            let high_end_dmg = 0.4005 * enh_increase * nerf;

            let formula_start = -0.35;
            let formula_end = 1.1350;

            let percent_of_mag = input.calc_data.shots_fired_this_mag / input.calc_data.base_mag;

            let buff = if (percent_of_mag < 0.125
                && *input.calc_data.weapon_type != WeaponType::SUBMACHINEGUN)
                || (percent_of_mag < 0.2
                    && *input.calc_data.weapon_type == WeaponType::SUBMACHINEGUN)
            {
                0.0
            } else if percent_of_mag > formula_end {
                high_end_dmg
            } else {
                let x = (percent_of_mag - formula_start) / (formula_end - formula_start);
                let smoothstep = 3.0 * (x.powf(2.0)) - 2.0 * (x.powf(3.0));
                low_end_dmg + (high_end_dmg - low_end_dmg) * smoothstep
            };

            DamageModifierResponse {
                impact_dmg_scale: buff + 1.0,
                explosive_dmg_scale: buff + 1.0,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::UnderOver,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut buff = 1.0_f64;
            if input.calc_data.has_overshield {
                buff += if input.pvp { 0.2 } else { 1.25 };
                if input.is_enhanced {
                    buff *= 1.05;
                }
            }
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::WellRounded,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let val = clamp(input.value, 0, 2) as i32;
            let mut map = HashMap::new();
            let stat_base = if input.is_enhanced { 12 } else { 10 };
            let stat_bump = stat_base * val;
            map.insert(StatHashes::STABILITY.into(), stat_bump);
            map.insert(StatHashes::RANGE.into(), stat_bump);
            map.insert(StatHashes::HANDLING.into(), stat_bump);
            map
        },
    );

    add_hmr(
        Perks::WellRounded,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let val = clamp(input.value, 0, 2) as i32;
            //due to ease of activation and upkeep will assume its always active
            // let mut duration = if  input.is_enhanced {9.0} else {8.5};
            let stat_base = if input.is_enhanced { 12 } else { 10 };
            let handling = stat_base * val;
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::WellRounded,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let val = clamp(input.value, 0, 2) as i32;
            let stat_base = if input.is_enhanced { 12 } else { 10 };
            let range = stat_base * val;
            RangeModifierResponse {
                range_stat_add: range,
                range_all_scale: 1.0,
                range_hip_scale: 1.0,
                range_zoom_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::BaitAndSwitch,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.30,
                explosive_dmg_scale: 1.30,
                crit_scale: 1.0,
            }
        },
    );

    add_rsmr(
        Perks::CompulsiveReloader,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_add = if input.is_enhanced { 55 } else { 50 };
            if input.calc_data.shots_fired_this_mag <= input.calc_data.base_mag / 2.0
                && input.value > 0
            {
                ReloadModifierResponse {
                    reload_stat_add: reload_add,
                    reload_time_scale: 0.95,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::CompulsiveReloader,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let reload_add = if input.is_enhanced { 55 } else { 50 };
            let mut map = HashMap::new();
            if input.calc_data.shots_fired_this_mag <= input.calc_data.base_mag / 2.0
                && input.value > 0
            {
                map.insert(StatHashes::RELOAD.into(), reload_add);
            }
            map
        },
    );

    add_sbr(
        Perks::SleightOfHand,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let val = clamp(input.value, 0, 3) as i32;
            let mut map = HashMap::new();
            let stat_base = 10;
            let stat_bump = stat_base * val;
            map.insert(StatHashes::STABILITY.into(), stat_bump);
            map.insert(StatHashes::RELOAD.into(), stat_bump);
            map.insert(StatHashes::HANDLING.into(), stat_bump);
            map
        },
    );

    add_hmr(
        Perks::SleightOfHand,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let val = clamp(input.value, 0, 3) as i32;
            let stat_base = 10;
            let handling = stat_base * val;
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::SleightOfHand,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(input.value, 0, 3) as i32;
            let stat_base = 10;
            let reload = stat_base * val;
            ReloadModifierResponse {
                reload_stat_add: reload,
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::ShotSwap,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling_mult = 1.0;
            let mut handling = 0;
            if input.value > 0 {
                handling_mult = 0.95;
                handling = 100;
            };
            HandlingModifierResponse {
                draw_add: handling,
                stow_add: handling,
                stow_scale: handling_mult,
                draw_scale: handling_mult,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::ShotSwap,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if input.value > 0 {
                map.insert(StatHashes::HANDLING.into(), 100);
            }
            map
        },
    );

    add_fmr(
        Perks::SuccesfulWarmup,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value == 0 {
                return FiringModifierResponse::default();
            }
            FiringModifierResponse {
                burst_delay_scale: 0.625,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::UnstoppableForce,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let scalar = if input.value >= 1 { 1.20 } else { 1.0 };
            DamageModifierResponse {
                impact_dmg_scale: scalar,
                explosive_dmg_scale: scalar,
                crit_scale: 1.0,
            }
        },
    );
}
