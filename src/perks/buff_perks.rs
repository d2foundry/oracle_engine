use std::collections::HashMap;

use crate::d2_enums::{AmmoType, BungieHash, DamageType, StatBump, StatHashes, WeaponType};

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
        1.0
    } else {
        _cached_data.insert("empowering".to_string(), _desired_buff);
        _desired_buff / current_buff
    }
}

fn surge_buff(_cached_data: &mut HashMap<String, f64>, _value: u32, _pvp: bool) -> f64 {
    let desired_buff = match (_pvp, _value) {
        (_, 0) => 1.00,
        (true, 1) => 1.03,
        (true, 2) => 1.045,
        (true, 3) => 1.055,
        (true, 4..) => 1.060,
        (false, 1) => 1.10,
        (false, 2) => 1.17,
        (false, 3) => 1.22,
        (false, 4..) => 1.25,
    };

    let current_buff = _cached_data.get("surge").unwrap_or(&1.0).to_owned();
    if current_buff >= desired_buff {
        1.0
    } else {
        _cached_data.insert("surge".to_string(), desired_buff);
        desired_buff / current_buff
    }
}

fn gbl_debuff(_cached_data: &mut HashMap<String, f64>, _desired_buff: f64) -> f64 {
    let current_buff = _cached_data.get("debuff").unwrap_or(&1.0).to_owned();
    if current_buff >= _desired_buff {
        1.0
    } else {
        _cached_data.insert("debuff".to_string(), _desired_buff);
        _desired_buff / current_buff
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
            if _input.value == 0 || _input.calc_data.damage_type != &DamageType::SOLAR {
                return DamageModifierResponse::default();
            }
            let buff = surge_buff(_input.cached_data, _input.value, _input.pvp);
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
            if _input.value > 0 {
                let des_buff = if _input.pvp { 1.0 } else { 1.35 };
                let buff = emp_buff(_input.cached_data, des_buff);
                DamageModifierResponse {
                    impact_dmg_scale: buff,
                    explosive_dmg_scale: buff,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
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

    add_dmr(
        Perks::WormByproduct,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value > 0 {
                DamageModifierResponse {
                    impact_dmg_scale: 1.15,
                    explosive_dmg_scale: 1.15,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
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
        Perks::TractorCannon,
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
            if _input.value > 0 {
                let debuff = gbl_debuff(_input.cached_data, 1.3);
                DamageModifierResponse {
                    impact_dmg_scale: debuff,
                    explosive_dmg_scale: debuff,
                    ..Default::default()
                }
            } else {
                DamageModifierResponse::default()
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
    add_dmr(
        Perks::SurgeMod,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mod = surge_buff(_input.cached_data, _input.value, _input.pvp);
            DamageModifierResponse {
                explosive_dmg_scale: damage_mod,
                impact_dmg_scale: damage_mod,
                ..Default::default()
            }
        }),
    );
    add_sbr(
        Perks::LucentBlades,
        Box::new(|_input: ModifierResponseInput| -> HashMap<u32, i32> {
            if _input.calc_data.weapon_type != &WeaponType::SWORD {
                return HashMap::new();
            }
            let stat_bump = match _input.value {
                0 => return HashMap::new(),
                1 => 30,
                2 => 50,
                3.. => 60,
            };
            HashMap::from([(StatHashes::CHARGE_RATE.into(), stat_bump)])
        }),
    );
    add_dmr(
        Perks::EternalWarrior,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let damage_mod = surge_buff(_input.cached_data, _input.value, _input.pvp);
            DamageModifierResponse {
                explosive_dmg_scale: damage_mod,
                impact_dmg_scale: damage_mod,
                ..Default::default()
            }
        }),
    );

    add_dmr(
        Perks::MantleOfBattleHarmony,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = if _input.value > 0 {
                surge_buff(_input.cached_data, 4, _input.pvp)
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::MaskOfBakris,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            let buff = if _input.value > 0
                && matches!(
                    _input.calc_data.damage_type,
                    DamageType::STASIS | DamageType::ARC
                ) {
                surge_buff(_input.cached_data, 4, _input.pvp)
            } else {
                1.0
            };
            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::SanguineAlchemy,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 || *_input.calc_data.damage_type == DamageType::KINETIC {
                return DamageModifierResponse::default();
            }

            let buff = surge_buff(_input.cached_data, 2, _input.pvp);

            DamageModifierResponse {
                impact_dmg_scale: buff,
                explosive_dmg_scale: buff,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::Foetracers,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 {
                return DamageModifierResponse::default();
            }
            let mult = surge_buff(_input.cached_data, 4, _input.pvp);
            DamageModifierResponse {
                impact_dmg_scale: mult,
                explosive_dmg_scale: mult,
                ..Default::default()
            }
        }),
    );
    add_dmr(
        Perks::GlacialGuard,
        Box::new(|_input: ModifierResponseInput| -> DamageModifierResponse {
            if _input.value == 0 || _input.calc_data.damage_type != &DamageType::STASIS {
                return DamageModifierResponse::default();
            }
            let mult = surge_buff(_input.cached_data, 4, _input.pvp);
            DamageModifierResponse {
                impact_dmg_scale: mult,
                explosive_dmg_scale: mult,
                ..Default::default()
            }
        }),
    );
}
