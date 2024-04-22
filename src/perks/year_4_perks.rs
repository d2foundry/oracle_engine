use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    add_dmr, add_epr, add_flmr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr,
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, VelocityModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_4_perks() {
    add_dmr(
        Perks::Adagio,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let duration = if input.is_enhanced { 8.0 } else { 7.0 };
            let mut dmg_boost = 0.3;
            if *input.calc_data.weapon_type == WeaponType::BOW
                || *input.calc_data.weapon_type == WeaponType::SHOTGUN
            {
                dmg_boost = 0.2;
            };
            if input.calc_data.time_total > duration || input.value == 0 {
                dmg_boost = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + dmg_boost,
                explosive_dmg_scale: 1.0 + dmg_boost,
                crit_scale: 1.0,
            }
        },
    );

    add_fmr(
        Perks::Adagio,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let duration = if input.is_enhanced { 8.0 } else { 7.0 };
            let mut firing_slow = 1.2;
            if input.calc_data.time_total > duration || input.value == 0 {
                firing_slow = 1.0;
            };
            FiringModifierResponse {
                burst_delay_scale: firing_slow,
                burst_delay_add: 0.0,
                inner_burst_scale: firing_slow,
                burst_size_add: 0.0,
            }
        },
    );

    add_sbr(
        Perks::Adagio,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let duration = if input.is_enhanced { 8.0 } else { 7.0 };
            if input.calc_data.time_total <= duration && input.value > 0 {
                map.insert(StatHashes::RANGE.into(), 10);
            }
            map
        },
    );

    add_rmr(
        Perks::Adagio,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            if input.value == 0 {
                return RangeModifierResponse::default();
            }

            RangeModifierResponse {
                range_stat_add: 10,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::AdrenalineJunkie,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(input.value, 0, 5);
            let duration = if input.is_enhanced { 6.0 } else { 4.5 };
            let mut dmg_boost = 0.067 * val as f64;
            if input.calc_data.time_total > duration {
                dmg_boost = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + dmg_boost,
                explosive_dmg_scale: 1.0 + dmg_boost,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::AdrenalineJunkie,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let duration = if input.is_enhanced { 6.0 } else { 4.5 };
            let mut handling = 0;
            if input.calc_data.time_total <= duration && input.value > 0 {
                handling = 20;
            };
            let mut out = HashMap::new();
            out.insert(StatHashes::HANDLING.into(), handling);
            out
        },
    );

    add_hmr(
        Perks::AdrenalineJunkie,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling = if input.value > 0 { 20 } else { 0 };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::Cornered,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let mut delay_mult = 1.0;
            if input.value > 0 {
                delay_mult = 0.85;
            };
            FiringModifierResponse {
                burst_delay_scale: delay_mult,
                burst_delay_add: 0.0,
                inner_burst_scale: 1.0,
                burst_size_add: 0.0,
            }
        },
    );

    add_sbr(
        Perks::Ensemble,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let handling = if input.is_enhanced { 35 } else { 30 };
            let reload = if input.is_enhanced { 45 } else { 40 };
            if input.value > 0 {
                let mut out = HashMap::new();
                out.insert(StatHashes::HANDLING.into(), handling);
                out.insert(StatHashes::RELOAD.into(), reload);
                out
            } else {
                HashMap::new()
            }
        },
    );

    add_hmr(
        Perks::Ensemble,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling = if input.is_enhanced { 35 } else { 30 };
            if input.value > 0 {
                HandlingModifierResponse {
                    stat_add: handling,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );

    add_rsmr(
        Perks::Ensemble,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload = if input.is_enhanced { 45 } else { 40 };
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: reload,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse {
                    reload_stat_add: 0,
                    reload_time_scale: 1.0,
                }
            }
        },
    );

    add_rsmr(
        Perks::Frenzy,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if input.value > 0 {
                reload = 100;
            };
            if input.calc_data.time_total > 12.0 {
                reload = 100;
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: 1.0,
            }
        },
    );

    add_hmr(
        Perks::Frenzy,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling = 0;
            if input.value > 0 {
                handling = 100;
            };
            if input.calc_data.time_total > 12.0 {
                handling = 100;
            };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::Frenzy,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut dmg = 0.0;
            if input.value > 0 {
                dmg = 0.15;
            };
            if input.calc_data.time_total > 12.0 {
                dmg = 0.15;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + dmg,
                explosive_dmg_scale: 1.0 + dmg,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::Frenzy,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut handling = 0;
            let mut reload = 0;
            if input.value > 0 {
                handling = 100;
                reload = 100;
            };
            if input.calc_data.time_total > 12.0 {
                handling = 100;
                reload = 100;
            };
            let mut out = HashMap::new();
            out.insert(StatHashes::HANDLING.into(), handling);
            out.insert(StatHashes::RELOAD.into(), reload);
            out
        },
    );

    add_rsmr(
        Perks::ImpulseAmplifier,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload = if input.is_enhanced { 25 } else { 20 };
            let reload_mult = if *input.calc_data.weapon_type == WeaponType::ROCKET {
                0.8
            } else {
                0.85
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: reload_mult,
            }
        },
    );

    add_sbr(
        Perks::ImpulseAmplifier,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let reload = if input.is_enhanced { 25 } else { 20 };
            let mut out = HashMap::new();
            out.insert(StatHashes::RELOAD.into(), reload);
            out
        },
    );

    add_vmr(
        Perks::ImpulseAmplifier,
        |_: ModifierResponseInput| -> VelocityModifierResponse {
            VelocityModifierResponse {
                velocity_scaler: 1.35,
            }
        },
    );

    add_sbr(
        Perks::PerpetualMotion,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let val = clamp(input.value, 0, 2);
            let mut stat_bump = 0;
            if val == 1 {
                stat_bump = 10;
            } else if val == 2 {
                stat_bump = 20;
            };
            let mut out = HashMap::new();
            out.insert(StatHashes::RELOAD.into(), stat_bump);
            out.insert(StatHashes::HANDLING.into(), stat_bump);
            out.insert(StatHashes::STABILITY.into(), stat_bump);
            out
        },
    );

    add_hmr(
        Perks::PerpetualMotion,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let val = clamp(input.value, 0, 2);
            let mut stat_bump = 0;
            if val == 1 {
                stat_bump = 10;
            } else if val == 2 {
                stat_bump = 20;
            };
            HandlingModifierResponse {
                stat_add: stat_bump,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::PerpetualMotion,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(input.value, 0, 2);
            let mut stat_bump = 0;
            if val == 1 {
                stat_bump = 10;
            } else if val == 2 {
                stat_bump = 20;
            };
            ReloadModifierResponse {
                reload_stat_add: stat_bump,
                reload_time_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::PerfectFloat,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::AIRBORNE.into(), 30);
            };
            out
        },
    );

    add_flmr(
        Perks::PerfectFloat,
        |input: ModifierResponseInput| -> FlinchModifierResponse {
            let val = if input.value > 0 { 0.65 } else { 1.0 };
            FlinchModifierResponse { flinch_scale: val }
        },
    );

    add_sbr(
        Perks::Pugilist,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::HANDLING.into(), 35);
            };
            out
        },
    );

    add_hmr(
        Perks::Pugilist,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let mut handling = 0;
            if input.value > 0 {
                handling = 35;
            };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_mmr(
        Perks::Reconstruction,
        |input: ModifierResponseInput| -> MagazineModifierResponse {
            let mag_scale = if input.value > 0 { 2.0 } else { 1.0 };
            MagazineModifierResponse {
                magazine_stat_add: 0,
                magazine_scale: mag_scale,
                magazine_add: 0.0,
            }
        },
    );

    add_sbr(
        Perks::DangerZone,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::BLAST_RADIUS.into(), 100);
            };
            out
        },
    );

    add_dmr(
        Perks::OneForAll,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut dmg = 0.0;
            let duration = if input.is_enhanced { 11.0 } else { 10.0 };
            if input.value > 0 {
                dmg = 0.35;
            };
            if input.calc_data.time_total > duration {
                dmg = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + dmg,
                explosive_dmg_scale: 1.0 + dmg,
                crit_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::FireFly,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut buffer: HashMap<u32, i32> = HashMap::new();
            if input.value > 0 {
                buffer.insert(StatHashes::RELOAD.into(), 50);
            }
            buffer
        },
    );

    add_rsmr(
        Perks::FireFly,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 50,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::GoldenTricorn,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let val = clamp(input.value, 0, 2);
            let mut duration = if val == 2 { 10.0 } else { 7.0 };
            if input.is_enhanced && val == 1 {
                duration += 1.0;
            };
            let damage_mult = if val == 2 { 0.5 } else { 0.15 };
            if input.value > 0 && input.calc_data.time_total < duration {
                DamageModifierResponse {
                    impact_dmg_scale: 1.0 + damage_mult,
                    explosive_dmg_scale: 1.0 + damage_mult,
                    crit_scale: 1.0,
                }
            } else {
                DamageModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::Harmony,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut damage_mult = if input.value > 0 { 0.20 } else { 0.0 };
            let duration = if input.is_enhanced { 8.0 } else { 7.0 };
            if input.calc_data.time_total > duration {
                damage_mult = 0.0;
            };
            DamageModifierResponse {
                impact_dmg_scale: 1.0 + damage_mult,
                explosive_dmg_scale: 1.0 + damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_hmr(
        Perks::Harmony,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling = if input.value > 0 { 15 } else { 0 };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::Harmony,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::HANDLING.into(), 15);
            }
            out
        },
    );

    add_sbr(
        Perks::Surplus,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value == 1 {
                out.insert(StatHashes::HANDLING.into(), 10);
                out.insert(StatHashes::RELOAD.into(), 5);
                out.insert(StatHashes::STABILITY.into(), 5);
            } else if input.value == 2 {
                out.insert(StatHashes::HANDLING.into(), 25);
                out.insert(StatHashes::RELOAD.into(), 25);
                out.insert(StatHashes::STABILITY.into(), 15);
            } else if input.value == 3 {
                out.insert(StatHashes::HANDLING.into(), 50);
                out.insert(StatHashes::RELOAD.into(), 50);
                out.insert(StatHashes::STABILITY.into(), 25);
            }
            out
        },
    );

    add_hmr(
        Perks::Surplus,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling = if input.value == 1 {
                10
            } else if input.value == 2 {
                25
            } else if input.value == 3 {
                50
            } else {
                0
            };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::Surplus,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload = if input.value == 1 {
                5
            } else if input.value == 2 {
                25
            } else if input.value == 3 {
                50
            } else {
                0
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                reload_time_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::HeatingUp,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let val = clamp(input.value, 0, 2) as i32;
            let mut out = HashMap::new();
            out.insert(StatHashes::RECOIL_DIR.into(), 20 * val);
            out.insert(StatHashes::STABILITY.into(), 15 * val);
            out
        },
    );

    add_sbr(
        Perks::TunnelVision,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut out = HashMap::new();
            if input.value > 0 {
                out.insert(StatHashes::AIM_ASSIST.into(), 20);
            }
            out
        },
    );

    add_hmr(
        Perks::TunnelVision,
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

    add_dmr(
        Perks::KickStart,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            const DURATION: f64 = 1.0;
            const MULT: f64 = 1.15;
            if input.value == 0 || input.calc_data.time_total > DURATION {
                return DamageModifierResponse::default();
            }

            DamageModifierResponse {
                impact_dmg_scale: MULT,
                explosive_dmg_scale: MULT,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::KickStart,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let mut fire_rate_mult = if input.value > 0 { 0.20 } else { 0.0 };
            let duration = 1.0;
            if input.calc_data.time_total > duration {
                fire_rate_mult = 0.0;
            };
            FiringModifierResponse {
                burst_delay_scale: 1.0 - fire_rate_mult,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::Recombination,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            //to make sure it doesn't go over the max stacks
            let val = if input.is_enhanced {
                clamp(input.value, 0, 8) as f64
            } else {
                clamp(input.value, 0, 10) as f64
            };
            //dmg buff per stack depends on enhancement and pvp
            let buff = 1.0
                + if input.calc_data.total_shots_fired == 0.0 {
                    match (input.is_enhanced, input.pvp) {
                        (false, false) => 0.1 * val,
                        (false, true) => 0.05 * val,
                        (true, false) => 0.125 * val,
                        (true, true) => 0.0625 * val,
                    }
                } else {
                    0.0
                };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                ..Default::default()
            }
        },
    );
}
