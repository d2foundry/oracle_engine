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
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut crit_scale = 1.0;
            let mut dmg_scale = 1.0;
            if *input.calc_data.weapon_type == WeaponType::LINEARFUSIONRIFLE && !input.pvp {
                crit_scale *= 1.15;
            };
            if *input.calc_data.damage_type == DamageType::KINETIC && !input.pvp {
                if input.calc_data.ammo_type == &AmmoType::PRIMARY {
                    dmg_scale *= 1.1;
                } else if input.calc_data.ammo_type == &AmmoType::SPECIAL {
                    dmg_scale *= 1.15;
                };
            };

            if *input.calc_data.ammo_type == AmmoType::PRIMARY
                && input.calc_data.intrinsic_hash > 1000
                && *input.calc_data.enemy_type == EnemyType::MINOR
                && !input.pvp
            {
                dmg_scale *= 1.4;
            }

            if *input.calc_data.weapon_type == WeaponType::LINEARFUSIONRIFLE
                && input.calc_data.intrinsic_hash < 1000
            {
                let charge_time = input
                    .calc_data
                    .stats
                    .get(&StatHashes::CHARGE_TIME.into())
                    .unwrap();
                //source: https://docs.google.com/spreadsheets/d/1QaUwtOW2_RJCTK1uaIGkbCoEXDa8UStvjDQSHSDxLOM/edit#gid=497378026
                //damage value updated from harm and stardust during super DR testing
                let total_damage = input.calc_data.curr_firing_data.damage
                    * input.calc_data.curr_firing_data.burst_size as f64;
                let stat = (charge_time.perk_val() - charge_time.base_value) as f64;
                dmg_scale *= 1.0 - (0.6 * stat) / total_damage;
            }
            DamageModifierResponse {
                crit_scale,
                impact_dmg_scale: dmg_scale,
                explosive_dmg_scale: dmg_scale,
            }
        },
    );

    add_fmr(
        Perks::BuiltIn,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            #[allow(unused_mut)]
            let mut delay_add = 0.0;

            if matches!(
                input.calc_data.weapon_type,
                WeaponType::FUSIONRIFLE | WeaponType::LINEARFUSIONRIFLE
            ) && input.calc_data.intrinsic_hash < 1000
            {
                let charge_time = input
                    .calc_data
                    .stats
                    .get(&StatHashes::CHARGE_TIME.into())
                    .unwrap();
                let stat = (charge_time.perk_val() - charge_time.base_value) as f64;
                delay_add -= match input.calc_data.weapon_type {
                    WeaponType::FUSIONRIFLE => stat * 0.0040,
                    WeaponType::LINEARFUSIONRIFLE => stat * 0.0033,
                    _ => 0.0,
                }
            }

            if input.calc_data.weapon_type == &WeaponType::BOW {
                let draw_time = input
                    .calc_data
                    .stats
                    .get(&StatHashes::DRAW_TIME.into())
                    .unwrap()
                    .clone();
                delay_add += match input.calc_data.intrinsic_hash {
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
        },
    );

    add_epr(
        Perks::BuiltIn,
        |input: ModifierResponseInput| -> ExplosivePercentResponse {
            if *input.calc_data.weapon_type == WeaponType::GRENADELAUNCHER {
                let blast_radius_struct =
                    input.calc_data.stats.get(&StatHashes::BLAST_RADIUS.into());

                let blast_radius = blast_radius_struct.cloned().unwrap_or_default().perk_val();

                if input.calc_data.ammo_type == &AmmoType::SPECIAL {
                    return ExplosivePercentResponse {
                        percent: 0.5 + 0.003 * blast_radius as f64,
                        delyed: 0.0,
                        retain_base_total: true,
                    };
                } else if input.calc_data.ammo_type == &AmmoType::HEAVY {
                    return ExplosivePercentResponse {
                        percent: 0.7 + 0.00175 * blast_radius as f64,
                        delyed: 0.0,
                        retain_base_total: true,
                    };
                };
            }
            if *input.calc_data.weapon_type == WeaponType::ROCKET
                && input.calc_data.intrinsic_hash < 1000
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
    );

    add_hmr(
        Perks::DexterityMod,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let swap_scale = if input.value > 0 {
                0.85 - clamp(input.value, 1, 3) as f64 * 0.05
            } else {
                1.0
            };
            HandlingModifierResponse {
                stow_scale: swap_scale,
                draw_scale: swap_scale,
                ..Default::default()
            }
        },
    );

    add_hmr(
        Perks::TargetingMod,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                ads_scale: if input.value > 0 { 0.75 } else { 1.0 },
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::TargetingMod,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value == 1 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 5);
            } else if input.value == 2 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 8);
            } else if input.value > 2 {
                stats.insert(StatHashes::AIM_ASSIST.into(), 10);
            }
            stats
        },
    );

    add_imr(
        Perks::ReserveMod,
        |input: ModifierResponseInput| -> InventoryModifierResponse {
            let inv_buff = match input.value {
                0 => 0,
                1 => 20,
                2 => 40,
                3 => 50,
                _ => 50,
            };
            InventoryModifierResponse {
                inv_stat_add: inv_buff,
                inv_scale: 1.0,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::ReserveMod,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let inv_buff = match input.value {
                0 => 0,
                1 => 20,
                2 => 40,
                3 => 50,
                _ => 50,
            };
            let mut stats = HashMap::new();
            stats.insert(StatHashes::INVENTORY_SIZE.into(), inv_buff);
            stats
        },
    );

    add_rsmr(
        Perks::LoaderMod,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let stat = match input.value {
                0 => 0,
                1 => 10,
                2 => 15,
                3.. => 18,
            };
            let mult = if input.value > 0 { 0.85 } else { 1.0 };

            ReloadModifierResponse {
                reload_stat_add: stat,
                reload_time_scale: mult,
            }
        },
    );

    add_sbr(
        Perks::LoaderMod,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            let buff = match input.value {
                0 => 0,
                1 => 10,
                2 => 15,
                3.. => 18,
            };
            stats.insert(StatHashes::RELOAD.into(), buff);
            stats
        },
    );

    add_flmr(
        Perks::UnflinchingMod,
        |input: ModifierResponseInput| -> FlinchModifierResponse {
            if input.value > 2 {
                FlinchModifierResponse { flinch_scale: 0.6 }
            } else if input.value == 2 {
                FlinchModifierResponse { flinch_scale: 0.7 }
            } else if input.value == 1 {
                FlinchModifierResponse { flinch_scale: 0.75 }
            } else {
                FlinchModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::RallyBarricade,
        |_: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::STABILITY.into(), 30);
            stats.insert(StatHashes::RELOAD.into(), 100);
            stats
        },
    );

    add_flmr(
        Perks::RallyBarricade,
        |_input: ModifierResponseInput| -> FlinchModifierResponse {
            FlinchModifierResponse { flinch_scale: 0.5 }
        },
    );

    add_rsmr(
        Perks::RallyBarricade,
        |_: ModifierResponseInput| -> ReloadModifierResponse {
            ReloadModifierResponse {
                reload_stat_add: 100,
                reload_time_scale: 0.9,
            }
        },
    );

    add_rmr(
        Perks::RallyBarricade,
        |_: ModifierResponseInput| -> RangeModifierResponse {
            RangeModifierResponse {
                range_all_scale: 1.1,
                ..Default::default()
            }
        },
    );
    add_fmr(
        Perks::AdeptChargeTime,
        |input: ModifierResponseInput| -> FiringModifierResponse {
            FiringModifierResponse {
                burst_delay_add: match *input.calc_data.weapon_type {
                    WeaponType::FUSIONRIFLE => -0.040,
                    WeaponType::LINEARFUSIONRIFLE => -0.033,
                    _ => 0.0,
                },
                ..Default::default()
            }
        },
    );
}
