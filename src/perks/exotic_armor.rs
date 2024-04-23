use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, BungieHash, DamageType, StatBump, StatHashes, WeaponType},
    logging::{extern_log, LogLevel},
};

use super::{
    add_dmr, add_epr, add_flmr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr,
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        FlinchModifierResponse, HandlingModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse, ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

pub fn exotic_armor() {
    add_dmr(
        Perks::BallindorseWrathweavers,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let mut modifier = DamageModifierResponse::default();
            let value = if input.pvp { 1.05 } else { 1.15 };
            if input.calc_data.damage_type == &DamageType::STASIS && input.value >= 1 {
                modifier.impact_dmg_scale = value;
                modifier.explosive_dmg_scale = value;
            }
            modifier
        },
    );

    //doesnt work for sturm overcharge, (maybe) memento
    add_dmr(
        Perks::LuckyPants,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let perks = input.calc_data.perk_value_map.clone();

            let perk_check =
                |hash: Perks| -> bool { matches!(perks.get(&hash.into()), Some(x) if x > &0) };
            //I hate this
            if perk_check(Perks::ParacausalShot)
            || perk_check(Perks::StormAndStress)
            //|| perk_check(Perks::ExplosiveShadow) //needs a way to remove only EDR?
            || input.pvp
            {
                return DamageModifierResponse::default();
            }

            let mult = if input.calc_data.ammo_type == &AmmoType::SPECIAL {
                0.3
            } else {
                0.6
            };

            DamageModifierResponse {
                impact_dmg_scale: 1.0 + mult * input.value.clamp(0, 10) as f64,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::TomeOfDawn,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 50);
            }
            stats
        },
    );

    add_flmr(
        Perks::TomeOfDawn,
        |input: ModifierResponseInput| -> FlinchModifierResponse {
            if input.value > 0 {
                FlinchModifierResponse { flinch_scale: 0.80 }
            } else {
                FlinchModifierResponse::default()
            }
        },
    );

    add_sbr(
        Perks::KnuckleheadRadar,
        |_: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            HashMap::from([(StatHashes::AIRBORNE.into(), 20)])
        },
    );

    add_dmr(
        Perks::KnuckleheadRadar,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            let health_percent = *input.cached_data.get("health%").unwrap_or(&1.0);
            if health_percent >= 0.3 || input.value == 0 {
                return DamageModifierResponse::default();
            }
            let modifier = 1.0 + (0.3 - health_percent);
            DamageModifierResponse {
                impact_dmg_scale: modifier,
                explosive_dmg_scale: modifier,
                crit_scale: 1.0,
            }
        },
    );

    //TODO: MECHANEER'S TRICKSLEEVES AUTORELOAD

    add_sbr(
        Perks::MechaneersTricksleeves,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::SIDEARM {
                stats.insert(StatHashes::AIRBORNE.into(), 50);
                stats.insert(StatHashes::HANDLING.into(), 100);
                stats.insert(StatHashes::RELOAD.into(), 100);
            };
            stats
        },
    );

    add_hmr(
        Perks::MechaneersTricksleeves,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.calc_data.weapon_type == &WeaponType::SIDEARM {
                HandlingModifierResponse {
                    stat_add: 100,
                    ..Default::default()
                }
            } else {
                HandlingModifierResponse::default()
            }
        },
    );
    add_rsmr(
        Perks::MechaneersTricksleeves,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.calc_data.weapon_type == &WeaponType::SIDEARM {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    ..Default::default()
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_dmr(
        Perks::MechaneersTricksleeves,
        |input: ModifierResponseInput| -> DamageModifierResponse {
            if input.value == 0 || *input.calc_data.weapon_type != WeaponType::SIDEARM {
                return DamageModifierResponse::default();
            };

            let damage_mult = if input.pvp { 1.10 } else { 2.0 };
            DamageModifierResponse {
                explosive_dmg_scale: damage_mult,
                impact_dmg_scale: damage_mult,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::Oathkeeper,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::BOW {
                stats.insert(StatHashes::AIRBORNE.into(), 40);
                stats.insert(StatHashes::DRAW_TIME.into(), 10);
            };
            stats
        },
    );

    /*add_fmr(
    Perks::Oathkeeper,
    |_input: ModifierResponsInput| -> FiringModifierResponse {
        FiringModifierResponse {
            burst_delay_add: match _input.calc_data.intrinsic_hash {
                906 => -36.0 / 1100.0,
                905 => -40.0 / 1100.0,
                _ => 0.0,
            },
            ..Default::default()
        }
    });*/

    add_sbr(
        Perks::SealedAhamkaraGrasps,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 50);
            };
            stats
        },
    );

    //TODO: AUTORELOAD FOR SEALED AHAMKARA GRASPS
    //TODO: LUCKY PANTS AFFECTING ACCURACY CONE
    //LUCKY PANTS ONLY WORKS FOR READY ?!?!?! crazy :(
    add_sbr(
        Perks::LuckyPants,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stat = HashMap::new();
            if input.value > 0 && input.calc_data.weapon_type == &WeaponType::HANDCANNON {
                stat.insert(StatHashes::AIRBORNE.into(), 20);
                stat.insert(StatHashes::HANDLING.into(), 100);
            };
            stat
        },
    );

    add_hmr(
        Perks::LuckyPants,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value > 0 && input.calc_data.weapon_type == &WeaponType::HANDCANNON {
                return HandlingModifierResponse {
                    draw_add: 100,
                    draw_scale: 0.6,
                    ..Default::default()
                };
            }
            HandlingModifierResponse::default()
        },
    );

    add_sbr(
        Perks::NoBackupPlans,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::SHOTGUN {
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            };
            stats
        },
    );

    add_sbr(
        Perks::ActiumWarRig,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::AUTORIFLE
                || input.calc_data.weapon_type == &WeaponType::MACHINEGUN
            {
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            }
            stats
        },
    );

    //TODO: AUTORELOAD ON ACTIUM WAR RIG

    add_sbr(
        Perks::HallowfireHeart,
        |_: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            HashMap::from([(StatHashes::AIRBORNE.into(), 20)])
        },
    );

    add_sbr(
        Perks::LionRampart,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 50);
            };
            stats
        },
    );

    add_sbr(
        Perks::Peacekeepers,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::SUBMACHINEGUN {
                stats.insert(StatHashes::AIRBORNE.into(), 40);
                stats.insert(StatHashes::HANDLING.into(), 50);
            };
            stats
        },
    );

    add_hmr(
        Perks::Peacekeepers,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.calc_data.weapon_type == &WeaponType::SUBMACHINEGUN {
                return HandlingModifierResponse {
                    stat_add: 50,
                    ads_scale: 1.0,
                    draw_scale: 0.8,
                    stow_scale: 0.8,
                    ..Default::default()
                };
            }
            HandlingModifierResponse::default()
        },
    );

    add_sbr(
        Perks::PeregrineGreaves,
        |_: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            HashMap::from([(StatHashes::AIRBORNE.into(), 20)])
        },
    );

    add_sbr(
        Perks::EyeOfAnotherWorld,
        |_: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            HashMap::from([(StatHashes::AIRBORNE.into(), 15)])
        },
    );

    add_sbr(
        Perks::AstrocyteVerse,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::AIRBORNE.into(), 30);
            if input.value > 0 {
                stats.insert(StatHashes::HANDLING.into(), 100);
            }
            stats
        },
    );

    add_hmr(
        Perks::AstrocyteVerse,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value == 0 {
                return HandlingModifierResponse::default();
            }
            HandlingModifierResponse {
                draw_add: 100,
                ..Default::default()
            }
        },
    );

    add_sbr(
        Perks::NecroticGrips,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.intrinsic_hash == 1863355414
                || input.calc_data.intrinsic_hash == 2965975126
                || input.calc_data.intrinsic_hash == 2724693746
            {
                //Thorn, Osteo Striga, Touch of Malice
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            };
            stats
        },
    );

    add_sbr(
        Perks::BootsOfTheAssembler,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.intrinsic_hash == 2144092201 {
                //Lumina
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            };
            stats
        },
    );

    add_sbr(
        Perks::RainOfFire,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.calc_data.weapon_type == &WeaponType::FUSIONRIFLE
                || input.calc_data.weapon_type == &WeaponType::LINEARFUSIONRIFLE
            {
                stats.insert(StatHashes::AIRBORNE.into(), 30);
            }
            stats
        },
    );

    add_sbr(
        Perks::SpeedloaderSlacks,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let modifiers = match input.value {
                0 => (0, 0, 0),
                1 => (40, 40, 30),
                2 => (40, 40, 35),
                3 => (45, 45, 40),
                4 => (50, 50, 45),
                5 => (55, 55, 50),
                _ => (55, 55, 50),
            };

            HashMap::from([
                (StatHashes::RELOAD.into(), modifiers.0),
                (StatHashes::HANDLING.into(), modifiers.1), //?
                (StatHashes::AIRBORNE.into(), modifiers.2),
            ])
        },
    );

    add_hmr(
        Perks::SpeedloaderSlacks,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            let handling = match input.value {
                0 => 0,
                1 => 40,
                2 => 40,
                3 => 45,
                4 => 50,
                5 => 55,
                _ => 55,
            };
            HandlingModifierResponse {
                stat_add: handling,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::SpeedloaderSlacks,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            let modifiers = match input.value {
                0 => (0, 1.0),
                1 => (40, 1.0),
                2 => (40, 0.925),
                3 => (45, 0.915),
                4 => (50, 0.91),
                5 => (55, 0.89),
                _ => (55, 0.89),
            };

            ReloadModifierResponse {
                reload_stat_add: modifiers.0,
                reload_time_scale: modifiers.1,
            }
        },
    );

    add_sbr(
        Perks::LunaFaction,
        |input: ModifierResponseInput| -> HashMap<BungieHash, StatBump> {
            let mut stat = HashMap::new();
            if input.value >= 1 {
                stat.insert(StatHashes::RELOAD.into(), 100);
            }
            stat
        },
    );

    add_rsmr(
        Perks::LunaFaction,
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value >= 1 {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 0.9,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );

    add_rmr(
        Perks::LunaFaction,
        |input: ModifierResponseInput| -> RangeModifierResponse {
            if input.value >= 2 {
                return RangeModifierResponse {
                    range_all_scale: 2.0,
                    ..Default::default()
                };
            }
            RangeModifierResponse::default()
        },
    );

    add_sbr(
        Perks::TritonVice,
        |input| -> HashMap<BungieHash, StatBump> {
            let mut stats = HashMap::new();
            if input.value > 0 && *input.calc_data.weapon_type == WeaponType::GLAIVE {
                stats.insert(StatHashes::RELOAD.into(), 50);
            }
            stats
        },
    );

    add_hmr(
        Perks::OphidianAspect,
        |_: ModifierResponseInput| -> HandlingModifierResponse {
            HandlingModifierResponse {
                stat_add: 35,
                ..Default::default()
            }
        },
    );

    add_rsmr(
        Perks::OphidianAspect,
        |_: ModifierResponseInput| -> ReloadModifierResponse {
            ReloadModifierResponse {
                reload_stat_add: 35,
                reload_time_scale: 1.0,
            }
        },
    );

    add_sbr(
        Perks::OphidianAspect,
        |_: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            stats.insert(StatHashes::HANDLING.into(), 35);
            stats.insert(StatHashes::RELOAD.into(), 35);
            stats.insert(StatHashes::AIRBORNE.into(), 10);
            stats
        },
    );

    add_sbr(
        Perks::DragonShadow,
        |input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if input.value >= 1 {
                stats.insert(StatHashes::HANDLING.into(), 100);
                stats.insert(StatHashes::RELOAD.into(), 100);
            }
            stats
        },
    );

    add_hmr(
        Perks::DragonShadow,
        |input: ModifierResponseInput| -> HandlingModifierResponse {
            if input.value >= 1 {
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
        |input: ModifierResponseInput| -> ReloadModifierResponse {
            if input.value >= 1 {
                ReloadModifierResponse {
                    reload_stat_add: 100,
                    reload_time_scale: 1.0,
                }
            } else {
                ReloadModifierResponse::default()
            }
        },
    );
}
