use std::{collections::HashMap, default, hash::Hash};

use crate::d2_enums::{AmmoType, BungieHash, DamageType, StatBump, StatHashes, WeaponType};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_imr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr,
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, InventoryModifierResponse,
        MagazineModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        VelocityModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn year_6_perks() {
    add_sbr(
        Perks::KeepAway,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let mut range_bonus = 0;
            let mut reload_bonus = 0;
            let ev = if input.is_enhanced { 2 } else { 0 };
            if input.value > 0 {
                range_bonus = 10 + ev;
                reload_bonus = 30 + ev;
            };
            map.insert(StatHashes::RANGE.into(), range_bonus);
            map.insert(StatHashes::RELOAD.into(), reload_bonus);
            map
        },
    );

    add_rmr(
        Perks::KeepAway,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let ev = if input.is_enhanced { 2 } else { 0 };
            let range_bonus = if input.value > 0 { 10 + ev } else { 0 };
            RangeModifierResponse {
                range_stat_add: range_bonus,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::KeepAway,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let ev = if input.is_enhanced { 2 } else { 0 };
            let reload_bonus = if input.value > 0 { 30 + ev } else { 0 };
            ReloadModifierResponse {
                reload_stat_add: reload_bonus,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::FieldTested,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if input.value > 4 {
                map.insert(StatHashes::RANGE.into(), 20);
                map.insert(StatHashes::RELOAD.into(), 55);
            } else if input.value == 4 {
                map.insert(StatHashes::RANGE.into(), 12);
                map.insert(StatHashes::RELOAD.into(), 35);
            } else if input.value == 3 {
                map.insert(StatHashes::RANGE.into(), 9);
                map.insert(StatHashes::RELOAD.into(), 20);
            } else if input.value == 2 {
                map.insert(StatHashes::RANGE.into(), 6);
                map.insert(StatHashes::RELOAD.into(), 10);
            } else if input.value == 1 {
                map.insert(StatHashes::RELOAD.into(), 5);
                map.insert(StatHashes::RANGE.into(), 3);
            }
            map
        },
    );

    // add_hmr(
    //     Perks::FieldTested,
    //
    //         |input: ModifierResponseInput| -> HandlingModifierResponse {
    //             let val = clamp(input.value, 0, 5) as i32;
    //             HandlingModifierResponse {
    //                 stat_add: val * 5,
    //                 ..Default::default()
    //             }
    //         },
    //     ),
    // );

    add_rsmr(
        Perks::FieldTested,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_bump;
            if input.value > 4 {
                reload_bump = 55;
            } else if input.value == 4 {
                reload_bump = 35;
            } else if input.value == 3 {
                reload_bump = 20;
            } else if input.value == 2 {
                reload_bump = 10;
            } else if input.value == 1 {
                reload_bump = 5;
            } else {
                reload_bump = 0;
            };
            ReloadModifierResponse {
                reload_stat_add: reload_bump,
                ..Default::default()
            }
        },
    );

    add_rmr(
        Perks::FieldTested,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            let range_bump;
            if input.value > 4 {
                range_bump = 20;
            } else if input.value == 4 {
                range_bump = 12;
            } else if input.value == 3 {
                range_bump = 9;
            } else if input.value == 2 {
                range_bump = 6;
            } else if input.value == 1 {
                range_bump = 3;
            } else {
                range_bump = 0;
            };
            RangeModifierResponse {
                range_stat_add: range_bump,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::ParacausalAffinity,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value > 0 {
                DamageModifierResponse {
                    explosive_dmg_scale: 1.2,
                    impact_dmg_scale: 1.2,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
            }
        },
    );

    add_mmr(
        Perks::EnviousAssassin,
        |input: ModifierResponseInput| -> MagazineModifierResponse {
            let val = input.value as f64;
            MagazineModifierResponse {
                magazine_scale: 1.0 + clamp(0.1 * val, 0.0, 2.0),
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::CollectiveAction,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (input.pvp, input.value) {
                (_, 0) => 1.0,
                (true, 1..) => 1.1,
                (false, 1..) => 1.2,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        },
    );

    add_dmr(
        Perks::Bipod,
        |_: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.75,
                explosive_dmg_scale: 0.75,
                ..Default::default()
            }
        },
    );

    add_mmr(
        Perks::Bipod,
        |_: ModifierResponseInput| -> MagazineModifierResponse {
            MagazineModifierResponse {
                magazine_scale: 2.0,
                ..Default::default()
            }
        },
    );

    add_imr(
        Perks::Bipod,
        |_: ModifierResponseInput| -> InventoryModifierResponse {
            InventoryModifierResponse {
                inv_add: 5,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::Bipod,
        |_: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_scale: 0.75,
                ..Default::default()
            }
        },
    );

    add_fmr(
        Perks::ControlledBurst,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            if input.value > 0 {
                return FiringModifierResponse {
                    burst_delay_scale: 0.9,
                    ..Default::default()
                };
            }
            FiringModifierResponse::default()
        },
    );
    add_dmr(
        Perks::ControlledBurst,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.2,
                    explosive_dmg_scale: 1.2,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        },
    );
    add_sbr(
        Perks::InvisibleHand,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::STABILITY.into(), 25);
            }
            stats
        },
    );
    add_sbr(
        Perks::UnsatedHunger,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::STABILITY.into(), 20);
                stats.insert(StatHashes::HANDLING.into(), 60);
                stats.insert(StatHashes::RELOAD.into(), 60);
            }
            stats
        },
    );

    add_rsmr(
        Perks::UnsatedHunger,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value > 0 {
                return ReloadModifierResponse {
                    reload_stat_add: 60,
                    ..Default::default()
                };
            }
            ReloadModifierResponse::default()
        },
    );
    add_hmr(
        Perks::UnsatedHunger,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                return HandlingModifierResponse {
                    stat_add: 60,
                    ..Default::default()
                };
            }
            HandlingModifierResponse::default()
        },
    );

    add_hmr(
        Perks::Discord,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 {
                return HandlingModifierResponse {
                    ads_scale: 0.75,
                    ..Default::default()
                };
            }
            HandlingModifierResponse::default()
        },
    );

    add_sbr(
        Perks::Discord,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            }
            stats
        },
    );

    add_dmr(
        Perks::PrecisionInstrument,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let max_percent = if input.is_enhanced { 0.30 } else { 0.25 };
            let max_stacks = 6.0;
            let shots_hit = input.calc_data.total_shots_hit;

            let stacks = clamp(input.value as f64 + shots_hit, 0.0, max_stacks);

            DamageModifierResponse {
                crit_scale: 1.0 + stacks * max_percent / max_stacks,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::LooseChange,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: 50,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::LooseChange,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            if input.value == 0 {
                return HashMap::new();
            }
            HashMap::from([
                (StatHashes::RELOAD.into(), 50),
                (StatHashes::AIM_ASSIST.into(), 20),
            ])
        },
    );

    add_dmr(
        Perks::HighGround,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }
            let mult = if input.pvp { 1.1 } else { 1.2 };

            DamageModifierResponse {
                impact_dmg_scale: mult,
                explosive_dmg_scale: mult,
                ..Default::default()
            }
        },
    );
    add_sbr(
        Perks::HeadRush,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            if input.value == 0 {
                return HashMap::new();
            }
            HashMap::from([
                (StatHashes::RELOAD.into(), 10),
                (StatHashes::HANDLING.into(), 0),
            ])
        },
    );
    add_hmr(
        Perks::HeadRush,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value == 0 {
                return HandlingModifierResponse::default();
            }
            //unknown at time
            HandlingModifierResponse {
                ..Default::default()
            }
        },
    );
    add_rsmr(
        Perks::HeadRush,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: 10,
                ..Default::default()
            }
        },
    );
    add_sbr(
        Perks::EnlightendAction,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let shots_hit = input.calc_data.total_shots_hit as i32;
            let value = input.value as i32;
            let stat_per_stack = 10;
            let max_stacks = 5;

            let stat_bump = clamp(value + shots_hit, 0, max_stacks) * stat_per_stack;
            HashMap::from([
                (StatHashes::RELOAD.into(), stat_bump),
                (StatHashes::HANDLING.into(), stat_bump),
            ])
        },
    );
    add_hmr(
        Perks::EnlightendAction,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let shots_hit = input.calc_data.total_shots_hit as i32;
            let value = input.value as i32;
            let stat_per_stack = 10;
            let max_stacks = 5;

            let stat_bump = clamp(value + shots_hit, 0, max_stacks) * stat_per_stack;
            HandlingModifierResponse {
                stat_add: stat_bump,
                ..Default::default()
            }
        },
    );
    add_rsmr(
        Perks::EnlightendAction,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let shots_hit = input.calc_data.total_shots_hit as i32;
            let value = input.value as i32;
            let stat_per_stack = 10;
            let max_stacks = 5;

            let stat_bump = clamp(value + shots_hit, 0, max_stacks) * stat_per_stack;
            ReloadModifierResponse {
                reload_stat_add: stat_bump,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::SwordLogic,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (input.pvp, input.value) {
                (_, 0) => 1.0,
                (false, 1..=3) => 1.05 + (0.1 * input.value as f64),
                (false, 4..) => 1.5,
                (true, 1 | 2) => 1.2,
                (true, 3..) => 1.35,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        },
    );
    add_sbr(
        Perks::NobleDeeds,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            if input.value == 0 {
                return HashMap::new();
            }
            HashMap::from([
                (StatHashes::HANDLING.into(), 30),
                (StatHashes::RELOAD.into(), 30),
            ])
        },
    );
    add_rsmr(
        Perks::NobleDeeds,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: 30,
                ..Default::default()
            }
        },
    );
    add_hmr(
        Perks::NobleDeeds,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value == 0 {
                return HandlingModifierResponse::default();
            }
            HandlingModifierResponse {
                stat_add: 30,
                ..Default::default()
            }
        },
    );
    add_fmr(
        Perks::Onslaught,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            let buff = match input.value {
                0 => 1.0,
                1 => 0.84,
                2 => 0.72,
                3 => 0.63,
                _ => 0.63,
            };
            FiringModifierResponse {
                burst_delay_scale: buff,
                ..Default::default()
            }
        },
    );
    add_rsmr(
        Perks::Onslaught,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let buff = match input.value {
                0 => 0,
                1 => 15,
                2 => 25,
                3 => 35,
                _ => 35,
            };
            ReloadModifierResponse {
                reload_stat_add: buff,
                ..Default::default()
            }
        },
    );
    add_sbr(
        Perks::Onslaught,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let buff = match input.value {
                0 => 0,
                1 => 15,
                2 => 25,
                3 => 35,
                _ => 35,
            };
            HashMap::from([(StatHashes::RELOAD.into(), buff)])
        },
    );
    add_dmr(
        Perks::Sever,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let debuff = if input.pvp { 0.85 } else { 1.0 };
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::DesperateMeasures,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = input.value.clamp(0, 3) as f64 * 0.1 + 1.0;
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        },
    );
    add_dmr(
        Perks::MasterOfArms,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 {
                return DamageModifierResponse::default();
            }
            DamageModifierResponse {
                impact_dmg_scale: 1.15,
                explosive_dmg_scale: 1.15,
                ..Default::default()
            }
        },
    );
}
