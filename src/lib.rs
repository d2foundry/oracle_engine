// #![forbid(clippy::all)]//, clippy::pedantic, clippy::nursery, clippy::cargo
#![warn(
    missing_copy_implementations,
    single_use_lifetimes,
    variant_size_differences,
    clippy::many_single_char_names,
    clippy::get_unwrap,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panicking_unwrap,
    arithmetic_overflow,
    missing_debug_implementations
)]
#![forbid(
    while_true,
    absolute_paths_not_starting_with_crate,
    bare_trait_objects,
    semicolon_in_expressions_from_macros,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_lifetimes,
    redundant_semicolons,
    unreachable_pub
)]

extern crate alloc;

use logging::LogLevel;
use perks::lib::CalculationInput;
pub mod abilities;
pub mod activity;
pub mod d2_enums;
pub mod enemies;
pub mod logging;
pub mod perks;
#[cfg(test)]
mod test;
pub mod types;
pub mod weapons;

use crate::perks::Perk;
use crate::weapons::{Stat, Weapon};
use abilities::Ability;
use activity::Activity;
use enemies::Enemy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct MetaData {
    pub api_version: &'static str,
    pub api_timestamp: &'static str,
    pub api_commit: &'static str,
    pub api_branch: &'static str,
}

pub(crate) mod database {
    include!(concat!(env!("OUT_DIR"), "/formulas.rs"));
}

//JavaScript

use crate::types::*;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct PersistentData {
    pub weapon: Weapon,
    pub activity: Activity,
    pub ability: Ability,
    pub enemy: Enemy,
    pub log_level: LogLevel,
}
impl PersistentData {
    pub fn new() -> PersistentData {
        Self::default()
    }
}

thread_local! {
    static PERS_DATA: RefCell<PersistentData> = RefCell::new(PersistentData::new());
}

#[wasm_bindgen]
extern "C" {
    //foreign function interface
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    perks::map_perks();
    console_log!("D2 Calculator Loaded");
}

//---------------WEAPONS---------------//

#[wasm_bindgen(js_name = "getMetadata")]
pub fn get_metadata() -> Result<JsValue, JsValue> {
    #[allow(clippy::unwrap_used)]
    let metadata = MetaData {
        api_timestamp: built_info::BUILT_TIME_UTC,
        api_version: built_info::PKG_VERSION,
        api_commit: built_info::GIT_COMMIT_HASH.unwrap(),
        api_branch: built_info::GIT_HEAD_REF.unwrap(),
    };
    serde_wasm_bindgen::to_value(&metadata).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "stringifyWeapon")]
pub fn weapon_as_string() -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    serde_wasm_bindgen::to_value(&weapon).map_err(JsValue::from)
}

//
// #[wasm_bindgen(js_name = "weaponJSON")]
// ///Returns the weapon as a JSON structure, snake case fields
// pub fn weapon_as_json() -> Result<JsValue, JsValue> {
//     let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
//     Ok(serde_wasm_bindgen::to_value(&weapon).unwrap())
// }

#[wasm_bindgen(js_name = "setWeapon")]
pub fn set_weapon(
    _hash: u32,
    _weapon_type_id: u8,
    _intrinsic_hash: u32,
    _ammo_type_id: u32,
    _damage_type_id: u32,
) {
    PERS_DATA.with(|perm_data| {
        let new_weapon = Weapon::generate_weapon(
            _hash,
            _weapon_type_id,
            _intrinsic_hash,
            _ammo_type_id,
            _damage_type_id,
        );

        if let Ok(weapon) = new_weapon {
            perm_data.borrow_mut().weapon = weapon;
        } else {
            logging::log(
                format!(
                    "Could not find weapon data for type: {}, intrinsic: {}, Err: {:?}",
                    _weapon_type_id, _intrinsic_hash, new_weapon
                )
                .as_str(),
                LogLevel::Error.into(),
            );
            perm_data.borrow_mut().weapon = Weapon::default();
        }
    });
}

#[wasm_bindgen(js_name = "getStats")]
pub fn get_stats() -> Result<JsValue, JsValue> {
    let stat_map = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.stats.clone());
    serde_wasm_bindgen::to_value(&stat_map).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "setStats")]
pub fn set_stats(_stats: JsValue) -> Result<(), JsValue> {
    let in_stats: HashMap<u32, i32> =
        serde_wasm_bindgen::from_value(_stats).map_err(JsValue::from)?;
    let mut stats = HashMap::new();
    for (key, value) in in_stats {
        stats.insert(key, Stat::from(value));
    }
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.stats = stats);
    Ok(())
}

#[wasm_bindgen(js_name = "addTrait")]
pub fn add_perk(_stats: JsValue, _value: u32, _hash: u32) -> Result<(), JsValue> {
    let data = perks::enhanced_check(_hash);
    let perk = Perk {
        stat_buffs: serde_wasm_bindgen::from_value(_stats).map_err(JsValue::from)?,
        enhanced: data.1,
        value: _value,
        raw_hash: _hash,
        hash: data.0,
    };
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(perk));
    Ok(())
}

#[wasm_bindgen(js_name = "resetTraits")]
pub fn reset_perks() {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.reset_perks());
}

#[wasm_bindgen(js_name = "getTraitHashes")]
pub fn query_perks() -> Vec<u32> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.list_perk_ids())
}

#[wasm_bindgen(js_name = "setTraitValue")]
pub fn change_perk_value(perk_hash: u32, new_value: u32) {
    let data = perks::enhanced_check(perk_hash);
    PERS_DATA.with(|perm_data| {
        perm_data
            .borrow_mut()
            .weapon
            .change_perk_val(data.0, new_value)
    });
}

#[wasm_bindgen(js_name = "getTraitOptions")]
pub fn get_perk_options_js(_perks: Vec<u32>) -> Result<JsValue, JsValue> {
    let options = perks::perk_options_handler::get_perk_options(_perks);
    match serde_wasm_bindgen::to_value(&options) {
        Ok(value) => Ok(value),
        Err(_) => Err(JsValue::from_str(
            "Could not convert perk options to JsValue",
        )),
    }
}

#[wasm_bindgen(js_name = "getWeaponRangeFalloff")]
pub fn get_weapon_range(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let out = if _dynamic_traits {
        weapon.calc_range_falloff(Some(weapon.static_calc_input()), None, _pvp)
    } else {
        weapon.calc_range_falloff(None, None, _pvp)
    };
    serde_wasm_bindgen::to_value(&out).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "getWeaponHandlingTimes")]
pub fn get_weapon_handling(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let out = if _dynamic_traits {
        weapon.calc_handling_times(Some(weapon.static_calc_input()), None, _pvp)
    } else {
        weapon.calc_handling_times(None, None, _pvp)
    };
    serde_wasm_bindgen::to_value(&out).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "getWeaponReloadTimes")]
pub fn get_weapon_reload(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let out = if _dynamic_traits {
        weapon.calc_reload_time(Some(weapon.static_calc_input()), None, _pvp)
    } else {
        weapon.calc_reload_time(None, None, _pvp)
    };
    serde_wasm_bindgen::to_value(&out).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "getWeaponAmmoSizes")]
pub fn get_weapon_ammo(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let out = if _dynamic_traits {
        weapon.calc_ammo_sizes(Some(weapon.static_calc_input()), None, _pvp)
    } else {
        weapon.calc_ammo_sizes(None, None, _pvp)
    };
    serde_wasm_bindgen::to_value(&out).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "getWeaponTtk")]
pub fn get_weapon_ttk(_overshield: f64) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let ttk_data = weapon.calc_ttk(_overshield);
    serde_wasm_bindgen::to_value(&ttk_data).map_err(JsValue::from)
}

///DEPRECATED for now
//
// #[wasm_bindgen(js_name = "getWeaponDps")]
// pub fn get_weapon_dps(_use_rpl: bool) -> Result<JsDpsResponse, JsValue> {
//     let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
//     let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
//     let pl_dmg_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_pl_delta());
//     let mut dps_response = weapon.calc_dps(enemy, pl_dmg_mult);
//     let rpl_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_rpl_mult());
//     if _use_rpl {
//         dps_response.apply_rpl(rpl_mult)
//     }
//     Ok(dps_response.into())
// }

#[wasm_bindgen(js_name = "getWeaponFiringData")]
pub fn get_weapon_firing_data(
    _dynamic_traits: bool,
    _pvp: bool,
    _use_rpl: bool,
) -> Result<JsValue, JsValue> {
    let persistent = PERS_DATA.with(|_perm_data| _perm_data.borrow().clone());
    let mut response: types::rs_types::FiringResponse;
    let calc_input: Option<CalculationInput> = if _dynamic_traits {
        let mut buffer = persistent.weapon.static_calc_input();
        buffer.enemy_type = &persistent.enemy.r#type;
        Some(buffer)
    } else {
        None
    };
    response = persistent.weapon.calc_firing_data(calc_input, None, _pvp);
    response.apply_pve_bonuses(
        persistent.activity.get_rpl_mult(),
        persistent.activity.get_pl_delta(),
        persistent.weapon.damage_mods.pve,
        persistent
            .weapon
            .damage_mods
            .get_mod(&persistent.enemy.r#type),
    );
    serde_wasm_bindgen::to_value(&response).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "getWeaponFlinch")]
pub fn get_weapon_flinch(
    _dynamic_traits: bool,
    _pvp: bool,
    _resilience: u8,
) -> Result<f64, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(weapon.calc_flinch_resist(
            Some(weapon.static_calc_input()),
            _resilience as i32,
            _pvp,
            None,
        ))
    } else {
        Ok(weapon.calc_flinch_resist(None, _resilience as i32, _pvp, None))
    }
}

#[wasm_bindgen(js_name = "getMiscData")]
pub fn get_misc_data(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        serde_wasm_bindgen::to_value(&weapon.get_misc_stats(Some(weapon.static_calc_input()), _pvp))
            .map_err(JsValue::from)
    } else {
        serde_wasm_bindgen::to_value(&weapon.get_misc_stats(None, _pvp)).map_err(JsValue::from)
    }
}

#[wasm_bindgen(js_name = "setEncounter")]
pub fn set_encounter(
    _recommend_pl: u32,
    _player_pl: u32,
    _weapon_pl: u32,
    _override_cap: i32,
    _difficulty: JsValue,
    _enemy_type: JsValue,
) -> Result<(), JsValue> {
    let difficulty = serde_wasm_bindgen::from_value(_difficulty).map_err(JsValue::from)?;
    PERS_DATA.with(|perm_data| {
        let activity = &mut perm_data.borrow_mut().activity;
        activity.rpl = _recommend_pl;
        activity.cap = _override_cap;
        activity.difficulty = difficulty;
        activity.player.power = _player_pl;
        activity.player.wep_power = _weapon_pl;
    });
    let enemy_type = serde_wasm_bindgen::from_value(_enemy_type).map_err(JsValue::from)?;
    PERS_DATA.with(|perm_data| {
        let enemy = &mut perm_data.borrow_mut().enemy;
        enemy.r#type = enemy_type;
    });
    Ok(())
}

#[wasm_bindgen(js_name = "setLoggingLevel")]
pub fn set_logging_level(_level: usize) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().log_level = _level.into();
    });
    Ok(())
}

#[wasm_bindgen(js_name = "getModifierResponseSummary")]
pub fn get_modifier_response(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let modifier = weapon.get_modifier_summary(
        _dynamic_traits.then_some(weapon.static_calc_input()),
        _pvp,
        None,
    );
    serde_wasm_bindgen::to_value(&modifier).map_err(JsValue::from)
}

#[wasm_bindgen(js_name = "getScalarResponseSummary")]
pub fn get_scalar_response(_pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let input_data = weapon.static_calc_input();
    let mut cached_data = HashMap::new();
    let rmr = perks::get_range_modifier(weapon.list_perks(), &input_data, _pvp, &mut cached_data);
    let rsmr = perks::get_reload_modifier(weapon.list_perks(), &input_data, _pvp, &mut cached_data);
    let mmr =
        perks::get_magazine_modifier(weapon.list_perks(), &input_data, _pvp, &mut cached_data);
    let hmr =
        perks::get_handling_modifier(weapon.list_perks(), &input_data, _pvp, &mut cached_data);
    let imr = perks::get_reserve_modifier(weapon.list_perks(), &input_data, _pvp, &mut cached_data);
    let out = ScalarResponse {
        ads_range_scalar: rmr.range_zoom_scale,
        global_range_scalar: rmr.range_all_scale,
        hipfire_range_scalar: rmr.range_hip_scale,
        ads_scalar: hmr.ads_scale,
        draw_scalar: hmr.draw_scale,
        stow_scalar: hmr.stow_scale,
        reload_scalar: rsmr.reload_time_scale,
        mag_size_scalar: mmr.magazine_scale,
        reserve_size_scalar: imr.inv_scale,
    };
    serde_wasm_bindgen::to_value(&out).map_err(JsValue::from)
}
