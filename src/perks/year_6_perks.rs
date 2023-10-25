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
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            let mut range_bonus = 0;
            let mut reload_bonus = 0;
            let ev = if _input.is_enhanced { 2 } else { 0 };
            if _input.value > 0 {
                range_bonus = 10 + ev;
                reload_bonus = 30 + ev;
            };
            map.insert(StatHashes::RANGE.into(), range_bonus);
            map.insert(StatHashes::RELOAD.into(), reload_bonus);
            map
        }),
    );

    add_rmr(
        Perks::KeepAway,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let ev = if _input.is_enhanced { 2 } else { 0 };
            let range_bonus = if _input.value > 0 { 10 + ev } else { 0 };
            RangeModifierResponse {
                range_stat_add: range_bonus,
                ..Default::default()
            }
        }),
    );

    add_rsmr(
        Perks::KeepAway,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let ev = if _input.is_enhanced { 2 } else { 0 };
            let reload_bonus = if _input.value > 0 { 30 + ev } else { 0 };
            ReloadModifierResponse {
                reload_stat_add: reload_bonus,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::FieldTested,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut map = HashMap::new();
            if _input.value > 4 {
                map.insert(StatHashes::RANGE.into(), 20);
                map.insert(StatHashes::RELOAD.into(), 55);
            } else if _input.value == 4 {
                map.insert(StatHashes::RANGE.into(), 12);
                map.insert(StatHashes::RELOAD.into(), 35);
            } else if _input.value == 3 {
                map.insert(StatHashes::RANGE.into(), 9);
                map.insert(StatHashes::RELOAD.into(), 20);
            } else if _input.value == 2 {
                map.insert(StatHashes::RANGE.into(), 6);
                map.insert(StatHashes::RELOAD.into(), 10);
            } else if _input.value == 1 {
                map.insert(StatHashes::RELOAD.into(), 5);
                map.insert(StatHashes::RANGE.into(), 3);
            }
            map
        }),
    );

    // add_hmr(
    //     Perks::FieldTested,
    //     Box::new(
    //         |_input: ModifierResponseInput| -> HandlingModifierResponse {
    //             let val = clamp(_input.value, 0, 5) as i32;
    //             HandlingModifierResponse {
    //                 stat_add: val * 5,
    //                 ..Default::default()
    //             }
    //         },
    //     ),
    // );

    add_rsmr(
        Perks::FieldTested,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let reload_bump;
            if _input.value > 4 {
                reload_bump = 55;
            } else if _input.value == 4 {
                reload_bump = 35;
            } else if _input.value == 3 {
                reload_bump = 20;
            } else if _input.value == 2 {
                reload_bump = 10;
            } else if _input.value == 1 {
                reload_bump = 5;
            } else {
                reload_bump = 0;
            };
            ReloadModifierResponse {
                reload_stat_add: reload_bump,
                ..Default::default()
            }
        }),
    );

    add_rmr(
        Perks::FieldTested,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            let range_bump;
            if _input.value > 4 {
                range_bump = 20;
            } else if _input.value == 4 {
                range_bump = 12;
            } else if _input.value == 3 {
                range_bump = 9;
            } else if _input.value == 2 {
                range_bump = 6;
            } else if _input.value == 1 {
                range_bump = 3;
            } else {
                range_bump = 0;
            };
            RangeModifierResponse {
                range_stat_add: range_bump,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::ParacausalAffinity,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value > 0 {
                DamageModifierResponse {
                    explosive_dmg_scale: 1.2,
                    impact_dmg_scale: 1.2,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
            }
        }),
    );

    add_mmr(
        Perks::EnviousAssasin,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                let val = _input.value as f64;
                //i dont know why this if is here? - harm
                if _input.calc_data.total_shots_fired == 0.0 {
                    return MagazineModifierResponse {
                        magazine_scale: 1.0 + clamp(0.1 * val, 0.0, 1.5),
                        ..Default::default()
                    };
                };
                MagazineModifierResponse::default()
            },
        ),
    );

    add_dmr(
        Perks::CollectiveAction,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (_input.pvp, _input.value) {
                (_, 0) => 1.0,
                (true, 1..) => 1.1,
                (false, 1..) => 1.2,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::Bipod,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            DamageModifierResponse {
                impact_dmg_scale: 0.75,
                explosive_dmg_scale: 0.75,
                ..Default::default()
            }
        }),
    );

    add_mmr(
        Perks::Bipod,
        Box::new(
            |_input: ModifierResponseInput| -> MagazineModifierResponse {
                MagazineModifierResponse {
                    magazine_scale: 2.0,
                    ..Default::default()
                }
            },
        ),
    );

    add_imr(
        Perks::Bipod,
        Box::new(
            |_input: ModifierResponseInput| -> InventoryModifierResponse {
                InventoryModifierResponse {
                    inv_add: 5,
                    ..Default::default()
                }
            },
        ),
    );

    add_fmr(
        Perks::Bipod,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_scale: 0.75,
                ..Default::default()
            }
        }),
    );

    add_fmr(
        Perks::ControlledBurst,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            if _input.value > 0 {
                return FiringModifierResponse {
                    burst_delay_scale: 0.9,
                    ..Default::default()
                };
            }
            FiringModifierResponse::default()
        }),
    );
    add_dmr(
        Perks::ControlledBurst,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value > 0 {
                return DamageModifierResponse {
                    impact_dmg_scale: 1.2,
                    explosive_dmg_scale: 1.2,
                    ..Default::default()
                };
            }
            DamageModifierResponse::default()
        }),
    );
    add_sbr(
        Perks::InvisibleHand,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::STABILITY.into(), 25);
            }
            stats
        }),
    );
    add_sbr(
        Perks::UnsatedHunger,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::STABILITY.into(), 20);
                stats.insert(StatHashes::HANDLING.into(), 60);
                stats.insert(StatHashes::RELOAD.into(), 60);
            }
            stats
        }),
    );

    add_rsmr(
        Perks::UnsatedHunger,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value > 0 {
                return ReloadModifierResponse {
                    reload_stat_add: 60,
                    ..Default::default()
                };
            }
            ReloadModifierResponse::default()
        }),
    );
    add_hmr(
        Perks::UnsatedHunger,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    return HandlingModifierResponse {
                        stat_add: 60,
                        ..Default::default()
                    };
                }
                HandlingModifierResponse::default()
            },
        ),
    );

    add_hmr(
        Perks::Discord,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value > 0 {
                    return HandlingModifierResponse {
                        ads_scale: 0.75,
                        ..Default::default()
                    };
                }
                HandlingModifierResponse::default()
            },
        ),
    );

    add_sbr(
        Perks::Discord,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            }
            stats
        }),
    );

    add_dmr(
        Perks::PrecisionInstrument,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let max_percent = if _input.is_enhanced { 0.30 } else { 0.25 };
            let max_stacks = 6.0;
            let shots_hit = _input.calc_data.total_shots_hit;

            let stacks = clamp(_input.value as f64 + shots_hit, 0.0, max_stacks);

            DamageModifierResponse {
                crit_scale: 1.0 + stacks * max_percent / max_stacks,
                ..Default::default()
            }
        }),
    );

    add_rsmr(
        Perks::LooseChange,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: 50,
                ..Default::default()
            }
        }),
    );

    add_sbr(
        Perks::LooseChange,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                if _input.value == 0 {
                    return HashMap::new();
                }
                HashMap::from([(StatHashes::RELOAD.into(), 50)])
            },
        ),
    );

    add_dmr(
        Perks::HighGround,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let mult = if _input.pvp { 1.1 } else { 1.2 };

            DamageModifierResponse {
                impact_dmg_scale: mult,
                explosive_dmg_scale: mult,
                ..Default::default()
            }
        }),
    );
    add_sbr(
        Perks::HeadRush,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                if _input.value == 0 {
                    return HashMap::new();
                }
                HashMap::from([
                    (StatHashes::RELOAD.into(), 10),
                    (StatHashes::HANDLING.into(), 0),
                ])
            },
        ),
    );
    add_hmr(
        Perks::HeadRush,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value == 0 {
                    return HandlingModifierResponse::default();
                }
                //unknown at time
                HandlingModifierResponse {
                    ..Default::default()
                }
            },
        ),
    );
    add_rsmr(
        Perks::HeadRush,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: 10,
                ..Default::default()
            }
        }),
    );
    add_sbr(
        Perks::EnlightendAction,
        Box::new(
            |_input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
                let shots_hit = _input.calc_data.total_shots_hit as i32;
                let value = _input.value as i32;
                let stat_per_stack = 10;
                let max_stacks = 5;

                let stat_bump = clamp(value + shots_hit, 0, max_stacks) * stat_per_stack;
                HashMap::from([
                    (StatHashes::RELOAD.into(), stat_bump),
                    (StatHashes::HANDLING.into(), stat_bump),
                ])
            },
        ),
    );
    add_hmr(
        Perks::EnlightendAction,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let shots_hit = _input.calc_data.total_shots_hit as i32;
                let value = _input.value as i32;
                let stat_per_stack = 10;
                let max_stacks = 5;

                let stat_bump = clamp(value + shots_hit, 0, max_stacks) * stat_per_stack;
                HandlingModifierResponse {
                    stat_add: stat_bump,
                    ..Default::default()
                }
            },
        ),
    );
    add_rsmr(
        Perks::EnlightendAction,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let shots_hit = _input.calc_data.total_shots_hit as i32;
            let value = _input.value as i32;
            let stat_per_stack = 10;
            let max_stacks = 5;

            let stat_bump = clamp(value + shots_hit, 0, max_stacks) * stat_per_stack;
            ReloadModifierResponse {
                reload_stat_add: stat_bump,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::SwordLogic,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = match (_input.pvp, _input.value) {
                (_, 0) => 1.0,
                (false, 1..=3) => 1.05 + (0.1 * _input.value as f64),
                (false, 4..) => 1.5,
                (true, 1 | 2) => 1.2,
                (true, 3..) => 1.35,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_sbr(
        Perks::NobleDeeds,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            if _input.value == 0 {
                return HashMap::new();
            }
            HashMap::from([
                (StatHashes::HANDLING.into(), 30),
                (StatHashes::RELOAD.into(), 30),
            ])
        }),
    );
    add_rsmr(
        Perks::NobleDeeds,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            if _input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_stat_add: 30,
                ..Default::default()
            }
        }),
    );
    add_hmr(
        Perks::NobleDeeds,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value == 0 {
                    return HandlingModifierResponse::default();
                }
                HandlingModifierResponse {
                    stat_add: 30,
                    ..Default::default()
                }
            },
        ),
    );
}
