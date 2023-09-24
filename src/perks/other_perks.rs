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
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 0,
                    reload_time_scale: 0.85,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_rsmr(
        Perks::RapidFireFrame,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 || _input.calc_data.weapon_type == &WeaponType::SHOTGUN {
                ReloadModifierResponse {
                    reload_stat_add: 0,
                    reload_time_scale: 0.80,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_imr(
        Perks::RapidFireFrame,
        |_input: ModifierResponseInput| -> InventoryModifierResponse {
            if _input.calc_data.weapon_type == &WeaponType::SNIPER {
                InventoryModifierResponse {
                    inv_scale: 1.3,
                    ..Default::default()
                }
            } else {
                InventoryModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::PrecisionFrame,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.calc_data.weapon_type == &WeaponType::HANDCANNON {
                stats.insert(StatHashes::Airborne.into(), 25);
            }
            stats
        },
    );

    add_hmr(
        Perks::SwapMag,
        |_input: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                draw_scale: 0.9,
                stow_scale: 0.9,
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::QuickAccessSling,
        |_input: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                draw_scale: 0.9,
                stow_scale: 0.9,
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::FreehandGrip,
        |_input: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                draw_scale: 0.95,
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::OphidianAspect,
        |_input: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                stat_add: 35,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::OphidianAspect,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            ReloadModifierResponse {
                reload_stat_add: 35,
                reload_time_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::OphidianAspect,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::Handling.into(), 35);
            stats.insert(StatHashes::Reload.into(), 35);
            stats.insert(StatHashes::Airborne.into(), 10);
            stats
        },
    );

    add_sbr(
        Perks::DragonShadow,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value >= 1 {
                stats.insert(StatHashes::Handling.into(), 100);
                stats.insert(StatHashes::Reload.into(), 100);
            }
            stats
        },
    );

    add_hmr(
        Perks::DragonShadow,
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
    );

    add_rsmr(
        Perks::DragonShadow,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value >= 1 {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::Amplified,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::Handling.into(), 40);
            stats
        },
    );

    add_hmr(
        Perks::Amplified,
        |_input: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                stat_add: 40,
                draw_scale: 0.95,
                stow_scale: 0.95,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::Frequency,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 50,
                    reload_time_scale: 0.8,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::Tempering,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::Reload.into(), 50);
            };
            stats
        },
    );

    add_rsmr(
        Perks::FlowState,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_stat_add: 50,
                    reload_time_scale: 0.8,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::FlowState,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::Reload.into(), 50);
            };
            stats
        },
    );

    add_sbr(
        Perks::Tempering,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::Airborne.into(), 20);
            };
            stats
        },
    );

    add_sbr(
        Perks::OnYourMark,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let val = clamp(_input.value, 0, 3) as i32;
            if _input.value > 0 {
                stats.insert(StatHashes::Handling.into(), 20 * val);
                stats.insert(StatHashes::Reload.into(), 20 * val);
            };
            stats
        },
    );

    add_hmr(
        Perks::OnYourMark,
        |_input: ModifierResponseInput| -> HandlingModifierResponse {
            let val = clamp(_input.value, 0, 3) as i32;
            HandlingModifierResponse {
                stat_add: 20 * val,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::OnYourMark,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            let val = clamp(_input.value, 0, 3) as i32;
            ReloadModifierResponse {
                reload_stat_add: 20 * val,
                reload_time_scale: if _input.value > 0 { 0.93 } else { 1.0 },
            }
        },
    );

    add_sbr(
        Perks::HeatRises,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let mut buff = 20;
            if _input.value > 0 {
                buff += 50;
            };
            stats.insert(StatHashes::Airborne.into(), buff);
            stats
        },
    );

    add_sbr(
        Perks::Hedrons,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::Airborne.into(), 20);
                stats.insert(StatHashes::AimAssist.into(), 15);
                stats.insert(StatHashes::Stability.into(), 30);
            };
            stats
        },
    );

    add_dmr(
        Perks::BossSpec,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if *_input.calc_data.enemy_type == EnemyType::Boss && !_input.pvp {
                1.077
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::MajorSpec,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(
                *_input.calc_data.enemy_type,
                EnemyType::Elite | EnemyType::Miniboss | EnemyType::Champion
            ) || _input.pvp
            {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.077;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::BigOnesSpec,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            if !matches!(
                *_input.calc_data.enemy_type,
                EnemyType::Elite | EnemyType::Miniboss | EnemyType::Champion | EnemyType::Boss
            ) || _input.pvp
            {
                return DamageModifierResponse::default();
            }
            let damage_mult = 1.077;
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::MinorSpec,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if _input.calc_data.enemy_type == &EnemyType::Minor && !_input.pvp {
                1.077
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::TakenSpec,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mult = if _input.value > 0 && !_input.pvp {
                1.1
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: damage_mult,
                explosive_dmg_scale: damage_mult,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::SpikeGrenades,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.5,
                explosive_dmg_scale: 1.0,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::DisorientingGrenades,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.75,
                explosive_dmg_scale: 0.75,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::FullChoke,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.calc_data.weapon_type == &WeaponType::SHOTGUN
                && _input.calc_data.base_crit_mult < 1.15
            {
                DamageModifierResponse {
                    impact_dmg_scale: 1.0,
                    explosive_dmg_scale: 1.0,
                    crit_scale: 0.92,
                }
            } else {
                DamageModifierResponse::default()
            }
        },
    );

    add_fmr(
        Perks::AcceleratedCoils,
        |_input: ModifierResponseInput| -> FiringModifierResponse {
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
        },
    );

    add_fmr(
        Perks::LiquidCoils,
        |_input: ModifierResponseInput| -> FiringModifierResponse {
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
        },
    );

    add_dmr(
        Perks::LiquidCoils,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 1.02,
                explosive_dmg_scale: 1.02,
                crit_scale: 1.0,
            }
        },
    );

    add_dmr(
        Perks::AcceleratedCoils,
        |_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.98,
                explosive_dmg_scale: 0.98,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::AssaultMag,
        |_input: ModifierResponseInput| -> FiringModifierResponse {
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
        },
    );

    add_sbr(
        Perks::ThreadOfAscent,
        |_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if _input.value > 0 {
                map.insert(StatHashes::Airborne.into(), 30);
                map.insert(StatHashes::Reload.into(), 40);
                map.insert(StatHashes::Handling.into(), 40);
            }
            map
        },
    );

    add_hmr(
        Perks::ThreadOfAscent,
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
    );
    add_rsmr(
        Perks::ThreadOfAscent,
        |_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                ReloadModifierResponse {
                    reload_time_scale: 0.925,
                    reload_stat_add: 40,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );
}
