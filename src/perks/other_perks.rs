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
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
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
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 || _input.calc_data.weapon_type == &WeaponType::SHOTGUN {
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
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.calc_data.weapon_type == &WeaponType::HANDCANNON {
                stats.insert(StatHashes::AIRBORNE.into(), 25);
            }
            stats
        }),
    );

    add_hmr(
        Perks::SwapMag,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
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
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
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
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    draw_scale: 0.95,
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::OphidianAspect,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    stat_add: 35,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::OphidianAspect,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            ReloadModifierResponse {
                reload_stat_add: 35,
                reload_time_scale: 1.0,
            }
        }),
    );

    add_sbr(
        Perks::OphidianAspect,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::HANDLING.into(), 35);
            stats.insert(StatHashes::RELOAD.into(), 35);
            stats.insert(StatHashes::AIRBORNE.into(), 10);
            stats
        }),
    );

    add_sbr(
        Perks::DragonShadow,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value >= 1 {
                stats.insert(StatHashes::HANDLING.into(), 100);
                stats.insert(StatHashes::RELOAD.into(), 100);
            }
            stats
        }),
    );

    add_hmr(
        Perks::DragonShadow,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value >= 1 {
                    HandlingModifierResponse {
                        stat_add: 100,
                        draw_scale: 0.7,
                        stow_scale: 0.7,
                        ..Default::default()
                    }
                } else {
                    HandlingModifierResponse::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::DragonShadow,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value >= 1 {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::Amplified,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::HANDLING.into(), 40);
            stats
        }),
    );

    add_hmr(
        Perks::Amplified,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
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
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
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
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::RELOAD.into(), 50);
            };
            stats
        }),
    );

    add_rsmr(
        Perks::FlowState,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
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
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::RELOAD.into(), 50);
            };
            stats
        }),
    );

    add_sbr(
        Perks::Tempering,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 20);
            };
            stats
        }),
    );

    add_sbr(
        Perks::OnYourMark,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let val = clamp(_input.value, 0, 3) as i32;
            if _input.value > 0 {
                stats.insert(StatHashes::HANDLING.into(), 20 * val);
                stats.insert(StatHashes::RELOAD.into(), 20 * val);
            };
            stats
        }),
    );

    add_hmr(
        Perks::OnYourMark,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let val = clamp(_input.value, 0, 3) as i32;
                HandlingModifierResponse {
                    stat_add: 20 * val,
                    ..Default::default()
                }
            },
        ),
    );

    add_rsmr(
        Perks::OnYourMark,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(_input.value, 0, 3) as i32;
            ReloadModifierResponse {
                reload_stat_add: 20 * val,
                reload_time_scale: if _input.value > 0 { 0.93 } else { 1.0 },
            }
        }),
    );

    add_sbr(
        Perks::HeatRises,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let mut buff = 20;
            if _input.value > 0 {
                buff += 50;
            };
            stats.insert(StatHashes::AIRBORNE.into(), buff);
            stats
        }),
    );

    add_sbr(
        Perks::Hedrons,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 20);
                stats.insert(StatHashes::AIM_ASSIST.into(), 15);
                stats.insert(StatHashes::STABILITY.into(), 30);
            };
            stats
        }),
    );

    add_dmr(
        Perks::BossSpec,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(
                *_input.calc_data.enemy_type,
                EnemyType::BOSS | EnemyType::CHAMPION | EnemyType::MINIBOSS | EnemyType::VEHICLE
            ) || _input.pvp
            {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.0777;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MajorSpec,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(*_input.calc_data.enemy_type, EnemyType::ELITE) || _input.pvp {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.0777;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::BigOnesSpec,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(
                *_input.calc_data.enemy_type,
                EnemyType::ELITE
                    | EnemyType::MINIBOSS
                    | EnemyType::CHAMPION
                    | EnemyType::BOSS
                    | EnemyType::VEHICLE
            ) || _input.pvp
            {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.0777;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MinorSpec,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(*_input.calc_data.enemy_type, EnemyType::MINOR) || _input.pvp {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.0777;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::TakenSpec,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if _input.value > 0 && !_input.pvp {
                1.1
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::SpikeGrenades,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.125,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::DisorientingGrenades,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.75,
                explosive_dmg_scale: 0.75,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::FullChoke,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.calc_data.weapon_type == &WeaponType::SHOTGUN
                && _input.calc_data.base_crit_mult < 1.15
            {
                DamageModifierResponse {
                    crit_scale: 0.92,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
            }
        }),
    );

    add_fmr(
        Perks::AcceleratedCoils,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
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
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE {
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
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.02,
                explosive_dmg_scale: 1.02,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::AcceleratedCoils,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.98,
                explosive_dmg_scale: 0.98,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::AssaultMag,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            let hash = _input.calc_data.intrinsic_hash;
            let tick_amount = if hash == 904 {
                3.0
            } else if hash == 906 {
                2.0
            } else {
                1.0
            };
            if _input.calc_data.weapon_type == &WeaponType::SHOTGUN {
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
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if _input.value > 0 {
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
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
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
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
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
        Perks::SupportFrame,
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
    )
}
