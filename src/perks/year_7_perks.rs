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

pub fn year_7_perks() {
    add_dmr(
        Perks::ChaosReshaped,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let buff = match _input.value {
                0 => unreachable!(),
                1 => 1.2,
                2.. => 1.35,
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::CircleOfLife,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let buff = if _input.pvp { 1.125 } else { 1.25 };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_sbr(
        Perks::AirTrigger,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();

            if _input.value > 0 {
                stats.insert(StatHashes::RELOAD.into(), 30);
            }
            stats
        }),
    );
    add_rsmr(
        Perks::AirTrigger,
        Box::new(|_input: ModifierResponseInput| -> ReloadModifierResponse {
            let buff = 0.8;
            if _input.value == 0 {
                return ReloadModifierResponse::default();
            }
            ReloadModifierResponse {
                reload_time_scale: buff,
                reload_stat_add: 30,
            }
        }),
    );
    add_sbr(
        Perks::LoneWolf,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            let mut stats = HashMap::new();
            if _input.value > 0 {
                stats.insert(StatHashes::AIRBORNE.into(), 10 * _input.value as i32);
                stats.insert(StatHashes::AIM_ASSIST.into(), 10 * _input.value as i32);
            }
            stats
        }),
    );
    add_hmr(
        Perks::LoneWolf,
        Box::new(
            |_input: ModifierResponseInput| -> HandlingModifierResponse {
                if _input.value == 0 {
                    HandlingModifierResponse::default();
                }
                HandlingModifierResponse {
                    ads_scale: 1.0 - (0.1 * _input.value as f64),
                    ..Default::default()
                }
            },
        ),
    )
}
