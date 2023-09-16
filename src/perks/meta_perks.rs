use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, DamageType, Seconds, StatHashes, WeaponType},
    enemies::EnemyType,
    weapons::Stat,
};

use super::{
    add_dmr, add_epr, add_flmr, add_fmr, add_hmr, add_imr, add_mmr, add_rmr, add_rsmr, add_sbr,
    add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, FlinchModifierResponse, HandlingModifierResponse,
        InventoryModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn meta_perks() {
    add_dmr(
        Perks::BuiltIn,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let mut crit_scale = 1.0;
            let mut dmg_scale = 1.0;
            if *_input.calc_data.weapon_type == WeaponType::LINEARFUSIONRIFLE && !_input.pvp {
                crit_scale *= 1.15;
            };
            if *_input.calc_data.damage_type == DamageType::KINETIC && !_input.pvp {
                if _input.calc_data.ammo_type == &AmmoType::PRIMARY {
                    dmg_scale *= 1.1;
                } else if _input.calc_data.ammo_type == &AmmoType::SPECIAL {
                    dmg_scale *= 1.15;
                };
            };

            if *_input.calc_data.ammo_type == AmmoType::PRIMARY
                && _input.calc_data.intrinsic_hash > 1000
                && *_input.calc_data.enemy_type == EnemyType::MINOR
                && !_input.pvp
            {
                dmg_scale *= 1.4;
            }

            if *_input.calc_data.weapon_type == WeaponType::LINEARFUSIONRIFLE
                && _input.calc_data.intrinsic_hash < 1000
            {
                let charge_time = _input
                    .calc_data
                    .stats
                    .get(&StatHashes::CHARGE_TIME.into())
                    .unwrap();
                //source: https://docs.google.com/spreadsheets/d/1QaUwtOW2_RJCTK1uaIGkbCoEXDa8UStvjDQSHSDxLOM/edit#gid=497378026
                //damage value updated from harm and stardust during super DR testing
                let total_damage = _input.calc_data.curr_firing_data.damage
                    * _input.calc_data.curr_firing_data.burst_size as f64;
                let stat = (charge_time.perk_val() - charge_time.base_value) as f64;
                dmg_scale *= 1.0 - (0.6 * stat) / total_damage;
            }

            DamageModifierResponse {
                crit_scale,
                impact_dmg_scale: dmg_scale,
                explosive_dmg_scale: dmg_scale,
            }
        }),
    );

    add_fmr(
        Perks::BuiltIn,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            #[allow(unused_mut)]
            let mut delay_add = 0.0;

            if matches!(
                _input.calc_data.weapon_type,
                WeaponType::FUSIONRIFLE | WeaponType::LINEARFUSIONRIFLE
            ) && _input.calc_data.intrinsic_hash < 1000
            {
                let charge_time = _input
                    .calc_data
                    .stats
                    .get(&StatHashes::CHARGE_TIME.into())
                    .unwrap();
                let stat = (charge_time.perk_val() - charge_time.base_value) as f64;
                delay_add -= match _input.calc_data.weapon_type {
                    WeaponType::FUSIONRIFLE => stat * 0.0040,
                    WeaponType::LINEARFUSIONRIFLE => stat * 0.0033,
                    _ => 0.0,
                }
            }

            if _input.calc_data.weapon_type == &WeaponType::BOW {
                let draw_time = _input
                    .calc_data
                    .stats
                    .get(&StatHashes::DRAW_TIME.into())
                    .unwrap()
                    .clone();
                delay_add += match _input.calc_data.intrinsic_hash {
                    //Lightweights, Wishender, Ticcus, Verglas
                    905 | 1470121888 | 3239299468 | 2636679416 => {
                        (draw_time.perk_val() as f64 * -4.0 + 900.0) / 1100.0
                    }
                    //Precisions, Lemon, Trinity, Hierarchy
                    906 | 2186532310 | 1573888036 | 2226793914 => {
                        (draw_time.perk_val() as f64 * -3.6 + 900.0) / 1100.0
                    }
                    //Levi Breath lol
                    1699724249 => (draw_time.perk_val() as f64 * -5.0 + 1428.0) / 1100.0,
                    _ => 0.0,
                };
            }
            FiringModifierResponse {
                burst_delay_add: delay_add,
                ..Default::default()
            }
        }),
    );

    add_epr(
        Perks::BuiltIn,
        Box::new(
            |_input: ModifierResponseInput| -> ExplosivePercentResponse {
                if *_input.calc_data.weapon_type == WeaponType::ROCKET
                    && _input.calc_data.intrinsic_hash < 1000
                //ensures not exotic
                {
                    return ExplosivePercentResponse {
                        percent: 0.778,
                        delyed: 0.0,
                        retain_base_total: true,
                    };
                }
                ExplosivePercentResponse {
                    percent: 0.0,
                    delyed: 0.0,
                    retain_base_total: true,
                }
            },
        ),
    );

    add_hmr(
        Perks::DexterityMod,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                let swap_scale = if _input.value > 0 {
                    0.85 - clamp(_input.value, 1, 3) as f64 * 0.05
                } else {
                    1.0
                };
                HandlingModifierResponse {
                    stow_scale: swap_scale,
                    draw_scale: swap_scale,
                    ..Default::default()
                }
            },
        ),
    );

    add_hmr(
        Perks::TargetingMod,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                HandlingModifierResponse {
                    ads_scale: if _input.value > 0 { 0.75 } else { 1.0 },
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::TargetingMod,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value == 1 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 5);
            } else if _input.value == 2 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 8);
            } else if _input.value > 2 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 10);
            }
            stats
        }),
    );

    add_imr(
        Perks::ReserveMod,
        Box::new(
            |_input: ModifierResponseInput| -> InventoryModifierResponse {
                let mut inv_buff = if _input.value > 0 { 20 } else { 0 };
                if _input.value == 2 {
                    inv_buff += 20;
                }
                if _input.value > 2 {
                    inv_buff += 30;
                }
                InventoryModifierResponse {
                    inv_stat_add: inv_buff,
                    inv_scale: 1.0,
                    ..Default::default()
                }
            },
        ),
    );

    add_sbr(
        Perks::ReserveMod,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut inv_buff = if _input.value > 0 { 20 } else { 0 };
            if _input.value == 2 {
                inv_buff += 15;
            }
            if _input.value > 2 {
                inv_buff += 20;
            }
            let mut stats = HashMap::new();
            stats.insert(StatHashes::INVENTORY_SIZE.into(), inv_buff);
            stats
        }),
    );

    add_rsmr(
        Perks::LoaderMod,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let stat = match _input.value {
                0 => 0,
                1 => 10,
                2 => 15,
                3.. => 18,
            };
            let mult = if _input.value > 0 { 0.85 } else { 1.0 };

            ReloadModifierResponse {
                reload_stat_add: stat,
                reload_time_scale: mult,
            }
        }),
    );

    add_sbr(
        Perks::LoaderMod,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let buff = match _input.value {
                0 => 0,
                1 => 10,
                2 => 15,
                3.. => 18,
            };
            stats.insert(StatHashes::RELOAD.into(), buff);
            stats
        }),
    );

    add_flmr(
        Perks::UnflinchingMod,
        Box::new(|_input: ModifierResponseInput| -> FlinchModifierResponse {
            if _input.value > 2 {
                FlinchModifierResponse { flinch_scale: 0.6 }
            } else if _input.value == 2 {
                FlinchModifierResponse { flinch_scale: 0.7 }
            } else if _input.value == 1 {
                FlinchModifierResponse { flinch_scale: 0.75 }
            } else {
                FlinchModifierResponse::default()
            }
        }),
    );

    add_sbr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::STABILITY.into(), 30);
            stats.insert(StatHashes::RELOAD.into(), 100);
            stats
        }),
    );

    add_flmr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> FlinchModifierResponse {
            FlinchModifierResponse { flinch_scale: 0.5 }
        }),
    );

    add_rsmr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            ReloadModifierResponse {
                reload_stat_add: 100,
                reload_time_scale: 0.9,
            }
        }),
    );

    add_rmr(
        Perks::RallyBarricade,
        Box::new(|_input: ModifierResponseInput| -> RangeModifierResponse {
            RangeModifierResponse {
                range_all_scale: 1.1,
                ..Default::default()
            }
        }),
    );
    add_fmr(
        Perks::AdeptChargeTime,
        Box::new(|_input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: match *_input.calc_data.weapon_type {
                    WeaponType::FUSIONRIFLE => -0.040,
                    WeaponType::LINEARFUSIONRIFLE => -0.033,
                    _ => 0.0,
                },
                ..Default::default()
            }
        }),
    );
}
