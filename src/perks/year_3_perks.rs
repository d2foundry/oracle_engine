use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, StatHashes},
    enemies::EnemyType,
};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr, add_msmr,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse, MovementSpeedModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_3_perks() {
    add_mmr(
        Perks::ClownCartridge,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                MagazineModifierResponse {
                    magazine_add: 0.0,
                    magazine_scale: 1.5,
                    magazine_stat_add: 0,
                }
            },
        ),
    );

    add_sbr(
        Perks::ElementalCapacitor,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let ev = if _input.is_enhanced { 5 } else { 0 };
            if _input.value == 1 {
                stats.insert(StatHashes::STABILITY.into(), 20 + ev);
            } else if _input.value == 2 {
                stats.insert(StatHashes::RELOAD.into(), 50 + ev);
            } else if _input.value == 3 {
                stats.insert(StatHashes::HANDLING.into(), 50 + ev);
            } else if _input.value == 4 {
                stats.insert(StatHashes::RECOIL_DIR.into(), 20 + ev);
            } else if _input.value == 5 {
                stats.insert(StatHashes::AIRBORNE.into(), 20 + ev);
            };
            stats
        }),
    );

    add_hmr(
        Perks::ElementalCapacitor,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let mut handling = 0;
                if _input.value == 3 {
                    handling = if _input.is_enhanced { 55 } else { 50 };
                };
                HandlingModifierResponse {
                    stat_add: handling,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::ElementalCapacitor,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let mut reload = 0;
            if _input.value == 2 {
                reload = if _input.is_enhanced { 55 } else { 50 };
            };
            ReloadModifierResponse {
                reload_stat_add: reload,
                ..Default::default()
            }
        }),
    );

    add_msmr(
        Perks::ElementalCapacitor,
        Box::new(|_input: ModifierResponseInput| -> MovementSpeedModifierResponse {
            MovementSpeedModifierResponse {
                ads_scalar: 1.05,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::KillingWind,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::HANDLING.into(), 40);
                stats.insert(StatHashes::RANGE.into(), 20);
            };
            stats
        }),
    );

    add_rmr(
        Perks::KillingWind,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            if _input.value > 0 {
                RangeModifierResponse {
                    range_stat_add: 20,
                    range_all_scale: 1.05,
                    range_zoom_scale: 1.0,
                    range_hip_scale: 1.0,
                }
            } else {
                RangeModifierResponse {
                    range_stat_add: 0,
                    range_all_scale: 1.0,
                    range_zoom_scale: 1.0,
                    range_hip_scale: 1.0,
                }
            }
        }),
    );

    add_hmr(
        Perks::KillingWind,
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

    add_msmr(
        Perks::KillingWind,
        Box::new(|_input: ModifierResponseInput| -> MovementSpeedModifierResponse {
            MovementSpeedModifierResponse {
                extra_mobility: 50,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::LastingImpression,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.0,
                explosive_dmg_scale: 1.25,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::Vorpal,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut buff = 1.0;
            if (*_input.calc_data.enemy_type == EnemyType::BOSS
                || *_input.calc_data.enemy_type == EnemyType::MINIBOSS
                || *_input.calc_data.enemy_type == EnemyType::CHAMPION
                || *_input.calc_data.enemy_type == EnemyType::VEHICLE)
                && _input.pvp == false
            {
                buff = match *_input.calc_data.ammo_type {
                    AmmoType::HEAVY => 1.1,
                    AmmoType::SPECIAL => 1.15,
                    AmmoType::PRIMARY => 1.2,
                    AmmoType::UNKNOWN => 0.0, //this should make someone point out a bug? whats error handling lol
                };
            }
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                crit_scale: 1.0,
            }
        }),
    );
}
