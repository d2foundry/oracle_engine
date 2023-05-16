use std::collections::HashMap;

use crate::d2_enums::{AmmoType, DamageType, StatHashes, WeaponType};

use super::{
    add_dmr, add_epr, add_fmr, add_hmr, add_mmr, add_rmr, add_rsmr, add_sbr, add_vmr, clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, RangeModifierResponse, RefundResponse, ReloadModifierResponse,
        ReloadOverrideResponse,
    },
    ModifierResponseInput, Perks,
};

fn emp_buff(_cached_data: &mut HashMap<String, f64>, _desired_buff: f64) -> f64 {
    let current_buff = _cached_data.get("empowering").unwrap_or(&1.0).to_owned();
    if current_buff >= _desired_buff {
        return 1.0;
    } else {
        _cached_data.insert("empowering".to_string(), _desired_buff);
        return _desired_buff / current_buff;
    }
}

fn gbl_debuff(_cached_data: &mut HashMap<String, f64>, _desired_buff: f64) -> f64 {
    let current_buff = _cached_data.get("debuff").unwrap_or(&1.0).to_owned();
    if current_buff >= _desired_buff {
        return 1.0;
    } else {
        _cached_data.insert("debuff".to_string(), _desired_buff);
        return _desired_buff / current_buff;
    }
}

//surge mod dmr is in meta_perks.rs

//
// BUFFS
//
pub fn buff_perks() {
    add_dmr(
        Perks::WellOfRadiance,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = emp_buff(_input.cached_data, 1.25);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::NobleRounds,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let des_buff = if _input.pvp { 1.15 } else { 1.35 };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::Radiant,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_buff = if _input.pvp { 1.1 } else { 1.25 };
            let buff = emp_buff(_input.cached_data, des_buff);
            _input.cached_data.insert("radiant".to_string(), 1.0);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::PathOfTheBurningSteps,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let pvp_values = [1.15, 1.25, 1.2, 1.35];
            let pve_values = [1.2, 1.25, 1.35, 1.4];
            let des_buff = if _input.pvp {
                pvp_values[clamp(_input.value - 1, 0, 3) as usize]
            } else {
                pve_values[clamp(_input.value - 1, 0, 3) as usize]
            };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::BannerShield,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_buff = if _input.pvp { 1.35 } else { 1.4 };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::EmpRift,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_buff = if _input.pvp { 1.15 } else { 1.2 };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MantleOfBattleHarmony,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_buff = if _input.pvp { 1.15 } else { 1.2 };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::WardOfDawn,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = emp_buff(_input.cached_data, 1.25);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::Gyrfalcon,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_buff = if _input.pvp { 1.0 } else { 1.35 };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::AeonInsight,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_buff = if _input.pvp { 1.0 } else { 1.35 };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::UmbralSharpening,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let pve_values = [1.2, 1.25, 1.35, 1.4];
            let des_buff = if _input.pvp {
                1.0
            } else {
                pve_values[clamp(_input.value, 0, 3) as usize]
            };
            let buff = emp_buff(_input.cached_data, des_buff);
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );

    //
    // DEBUFFS
    //

    add_dmr(
        Perks::Weaken,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_debuff = if _input.pvp { 1.075 } else { 1.15 };
            let debuff = gbl_debuff(_input.cached_data, des_debuff);
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::RepulsorForce,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_debuff = if _input.pvp { 1.5 } else { 1.3 };
            let debuff = gbl_debuff(_input.cached_data, des_debuff);
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MoebiusQuiver,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_debuff = if _input.pvp { 1.5 } else { 1.3 };
            let debuff = gbl_debuff(_input.cached_data, des_debuff);
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::DeadFall,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let des_debuff = if _input.pvp { 1.5 } else { 1.3 };
            let debuff = gbl_debuff(_input.cached_data, des_debuff);
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::Felwinters,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let debuff = gbl_debuff(_input.cached_data, 1.3);
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::EnhancedScannerAugment,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let pve_values = [1.08, 1.137, 1.173, 1.193, 1.2];
            let des_debuff = if _input.pvp {
                1.0
            } else {
                pve_values[clamp(_input.value, 0, 4) as usize]
            };
            let debuff = gbl_debuff(_input.cached_data, des_debuff);
            DamageModifierResponse {
                impact_dmg_scale: debuff,
                explosive_dmg_scale: debuff,
                ..Default::default()
            }
        }),
    );
}
