use std::collections::{btree_map::Range, HashMap};

use crate::{
    d2_enums::{AmmoType, BungieHash, DamageType, StatBump, StatHashes, WeaponType},
    enemies::EnemyType,
};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_imr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr,
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn other_perks() {
    add_rsmr(
        Perks::AlloyMag,
        Box::new(|input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 0,
                    reload_time_scale: 0.85,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_rsmr(
        Perks::RapidFireFrame,
        Box::new(|input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 || input.calc_data.weapon_type == &WeaponType::SHOTGUN {
                ReloadModifierResponse {
                    reload_stat_add: 0,
                    reload_time_scale: 0.80,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::PrecisionFrame,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::HANDCANNON {
                stats.insert(StatHashes::AIRBORNE.into(), 25);
            }
            stats
        }),
    );

    add_hmr(
        Perks::SwapMag,
        Box::new(
            |_: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    draw_scale: 0.9,
                    stow_scale: 0.9,
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::QuickAccessSling,
        Box::new(
            |_: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    draw_scale: 0.9,
                    stow_scale: 0.9,
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::FreehandGrip,
        Box::new(
            |_: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    draw_scale: 0.95,
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::Amplified,
        Box::new(|_: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::HANDLING.into(), 40);
            stats
        }),
    );

    add_hmr(
        Perks::Amplified,
        Box::new(
            |_: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    stat_add: 40,
                    draw_scale: 0.95,
                    stow_scale: 0.95,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::Frequency,
        Box::new(|input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 50,
                    reload_time_scale: 0.8,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::Tempering,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::RELOAD.into(), 50);
            };
            stats
        }),
    );

    add_rsmr(
        Perks::FlowState,
        Box::new(|input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 50,
                    reload_time_scale: 0.8,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::FlowState,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::RELOAD.into(), 50);
            };
            stats
        }),
    );

    add_sbr(
        Perks::Tempering,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 20);
            };
            stats
        }),
    );

    add_sbr(
        Perks::OnYourMark,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let val = clamp(input.value, 0, 3) as i32;
            if input.value > 0 {
                stats.insert(StatHashes::HANDLING.into(), 20 * val);
                stats.insert(StatHashes::RELOAD.into(), 20 * val);
            };
            stats
        }),
    );

    add_hmr(
        Perks::OnYourMark,
        Box::new(
            |input: ModifierResponseInput| -> HandlingModifierResponse {
                let val = clamp(input.value, 0, 3) as i32;
                HandlingModifierResponse {
                    stat_add: 20 * val,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::OnYourMark,
        Box::new(|input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(input.value, 0, 3) as i32;
            ReloadModifierResponse {
                reload_stat_add: 20 * val,
                reload_time_scale: if input.value > 0 { 0.93 } else { 1.0 },
            }
        }),
    );

    add_sbr(
        Perks::HeatRises,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let mut buff = 20;
            if input.value > 0 {
                buff += 50;
            };
            stats.insert(StatHashes::AIRBORNE.into(), buff);
            stats
        }),
    );

    add_sbr(
        Perks::Hedrons,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 20);
                stats.insert(StatHashes::AIM_ASSIST.into(), 15);
                stats.insert(StatHashes::STABILITY.into(), 30);
            };
            stats
        }),
    );

    add_dmr(
        Perks::BossSpec,
        Box::new(|input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if *input.calc_data.enemy_type == EnemyType::BOSS && !input.pvp {
                1.077
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::MajorSpec,
        Box::new(|input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(
                *input.calc_data.enemy_type,
                EnemyType::ELITE | EnemyType::MINIBOSS | EnemyType::CHAMPION
            ) || input.pvp
            {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.077;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::BigOnesSpec,
        Box::new(|input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(
                *input.calc_data.enemy_type,
                EnemyType::ELITE | EnemyType::MINIBOSS | EnemyType::CHAMPION | EnemyType::BOSS
            ) || input.pvp
            {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.077;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MinorSpec,
        Box::new(|input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if input.calc_data.enemy_type == &EnemyType::MINOR && !input.pvp {
                1.077
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::TakenSpec,
        Box::new(|input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if input.value > 0 && !input.pvp {
                1.1
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::SpikeGrenades,
        Box::new(|_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.125,
                explosive_dmg_scale: 1.0,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::DisorientingGrenades,
        Box::new(|_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.75,
                explosive_dmg_scale: 0.75,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::FullChoke,
        Box::new(|input: ModifierResponseInput| -> DamageModifierResponse {
            if input.calc_data.weapon_type == &WeaponType::SHOTGUN
                && input.calc_data.base_crit_mult < 1.15
            {
                DamageModifierResponse {
                    impact_dmg_scale: 1.0,
                    explosive_dmg_scale: 1.0,
                    crit_scale: 0.92,
                }
            } else {
                DamageModifierResponse::default()
            }
        }),
    );

    add_fmr(
        Perks::AcceleratedCoils,
        Box::new(|input: ModifierResponseInput| -> FiringModifierResponse {
            if input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
                return FiringModifierResponse {
                    burst_delay_add: -0.033,
                    ..Default::default()
                };
            }
            FiringModifierResponse {
                burst_delay_add: -0.040,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::LiquidCoils,
        Box::new(|input: ModifierResponseInput| -> FiringModifierResponse {
            if input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
                return FiringModifierResponse {
                    burst_delay_add: 0.033,
                    ..Default::default()
                };
            }
            FiringModifierResponse {
                burst_delay_add: 0.040,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::LiquidCoils,
        Box::new(|_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.02,
                explosive_dmg_scale: 1.02,
                crit_scale: 1.0,
            }
        }),
    );

    add_dmr(
        Perks::AcceleratedCoils,
        Box::new(|_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.98,
                explosive_dmg_scale: 0.98,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::AssaultMag,
        Box::new(|input: ModifierResponseInput| -> FiringModifierResponse {
            let hash = input.calc_data.intrinsic_hash;
            let tick_amount = if hash == 904 {
                3.0
            } else if hash == 906 {
                2.0
            } else {
                1.0
            };
            if input.calc_data.weapon_type == &WeaponType::SHOTGUN {
                FiringModifierResponse {
                    burst_delay_add: -(tick_amount / 30.0),
                    ..Default::default()
                }
            } else {
                FiringModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::ThreadOfAscent,
        Box::new(|input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if input.value > 0 {
                map.insert(StatHashes::AIRBORNE.into(), 30);
                map.insert(StatHashes::RELOAD.into(), 40);
                map.insert(StatHashes::HANDLING.into(), 40);
            }
            map
        }),
    );

    add_hmr(
        Perks::ThreadOfAscent,
        Box::new(
            |input: ModifierResponseInput| -> HandlingModifierResponse {
                if input.value > 0 {
                    HandlingModifierResponse {
                        stat_add: 40,
                        draw_scale: 0.925,
                        stow_scale: 0.925,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::ThreadOfAscent,
        Box::new(|input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                ReloadModifierResponse {
                    reload_time_scale: 0.925,
                    reload_stat_add: 40,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_dmr(
        Perks::ImpactCasing,
        Box::new(|_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.1,
                explosive_dmg_scale: 1.0,
                crit_scale: 1.0,
            }
        }),
    );
}
