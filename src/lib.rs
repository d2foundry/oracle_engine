#![allow(dead_code)]
#![allow(unused_imports)]
extern crate alloc;

use logging::LogLevel;
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

use crate::perks::{Perk, Perks};
use crate::weapons::{Stat, Weapon};
use abilities::Ability;
use activity::Activity;
use d2_enums::StatHashes;
use enemies::Enemy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod database {
    include!(concat!(env!("OUT_DIR"), "/formulas.rs"));
}

//JavaScript
#[cfg(feature = "wasm")]
use crate::types::js_types::{
    JsAmmoResponse, JsDifficultyOptions, JsDpsResponse, JsEnemyType, JsFiringResponse,
    JsHandlingResponse, JsMetaData, JsRangeResponse, JsReloadResponse, JsResillienceSummary,
    JsStat,
};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "wasm", feature = "foundry"))]
use crate::types::js_types::JsScalarResponse;

//python
#[cfg(feature = "python")]
use crate::types::py_types::{
    PyActivity, PyDifficultyOptions, PyDpsResponse, PyEnemy, PyEnemyType, PyFiringResponse,
    PyHandlingResponse, PyPerk, PyPlayer, PyPlayerClass, PyRangeResponse, PyResillienceSummary,
};
#[cfg(feature = "python")]
use pyo3::{prelude::*, types::PyDict};

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

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    //foreign function interface
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}
#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    perks::map_perks();
    console_log!("D2 Calculator Loaded");
}

//---------------WEAPONS---------------//

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getMetadata")]
pub fn get_metadata() -> Result<JsMetaData, JsValue> {
    let metadata = JsMetaData {
        api_timestamp: built_info::BUILT_TIME_UTC,
        api_version: built_info::PKG_VERSION,
        api_commit: built_info::GIT_COMMIT_HASH.unwrap(),
        api_branch: built_info::GIT_HEAD_REF.unwrap(),
    };
    Ok(metadata)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "stringifyWeapon")]
pub fn weapon_as_string() -> Result<String, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    Ok(format!("{:?}", weapon))
}

// #[cfg(feature = "wasm")]
// #[wasm_bindgen(js_name = "weaponJSON")]
// ///Returns the weapon as a JSON structure, snake case fields
// pub fn weapon_as_json() -> Result<JsValue, JsValue> {
//     let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
//     Ok(serde_wasm_bindgen::to_value(&weapon).unwrap())
// }

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setWeapon")]
pub fn set_weapon(
    _hash: u32,
    _weapon_type_id: u8,
    _intrinsic_hash: u32,
    _ammo_type_id: u32,
    _damage_type_id: u32,
) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        let new_weapon = Weapon::generate_weapon(
            _hash,
            _weapon_type_id,
            _intrinsic_hash,
            _ammo_type_id,
            _damage_type_id,
        );
        if new_weapon.is_err() {
            console_log!(
                "Could not find weapon data for type: {}, intrinsic: {}, Err: {:?}",
                _weapon_type_id,
                _intrinsic_hash,
                new_weapon
            );
            perm_data.borrow_mut().weapon = Weapon::default();
        } else {
            perm_data.borrow_mut().weapon = new_weapon.unwrap();
        };
    });
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getStats")]
pub fn get_stats() -> Result<JsValue, JsValue> {
    let stat_map = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.stats.clone());
    let mut js_stat_map = HashMap::new();
    for (key, value) in stat_map {
        js_stat_map.insert(key, JsStat::from(value));
    }
    let value = serde_wasm_bindgen::to_value(&js_stat_map);
    if value.is_err() {
        return Err(JsValue::from_str("Could not convert stats to JsValue"));
    }
    Ok(value.unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setStats")]
pub fn set_stats(_stats: JsValue) -> Result<(), JsValue> {
    let in_stats: HashMap<u32, i32> = serde_wasm_bindgen::from_value(_stats).unwrap();
    let mut stats = HashMap::new();
    for (key, value) in in_stats {
        stats.insert(key, Stat::from(value));
    }
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.stats = stats);
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "addTrait")]
pub fn add_perk(_stats: JsValue, _value: u32, _hash: u32) -> Result<(), JsValue> {
    let data = perks::enhanced_check(_hash);
    let perk = Perk {
        stat_buffs: serde_wasm_bindgen::from_value(_stats).unwrap(),
        enhanced: data.1,
        value: _value,
        raw_hash: _hash,
        hash: data.0,
    };
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(perk));
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getTraitHashes")]
pub fn query_perks() -> Vec<u32> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.list_perk_ids())
}

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getTraitOptions")]
pub fn get_perk_options_js(_perks: Vec<u32>) -> Result<JsValue, JsValue> {
    let options = perks::perk_options_handler::get_perk_options(_perks);
    let value = serde_wasm_bindgen::to_value(&options);
    if value.is_err() {
        return Err(JsValue::from_str(
            "Could not convert perk options to JsValue",
        ));
    }
    Ok(value.unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponRangeFalloff")]
pub fn get_weapon_range(_dynamic_traits: bool, _pvp: bool) -> Result<JsRangeResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_range_falloff(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_range_falloff(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponHandlingTimes")]
pub fn get_weapon_handling(
    _dynamic_traits: bool,
    _pvp: bool,
) -> Result<JsHandlingResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_handling_times(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_handling_times(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponReloadTimes")]
pub fn get_weapon_reload(_dynamic_traits: bool, _pvp: bool) -> Result<JsReloadResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_reload_time(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_reload_time(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponAmmoSizes")]
pub fn get_weapon_ammo(_dynamic_traits: bool, _pvp: bool) -> Result<JsAmmoResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(weapon
            .calc_ammo_sizes(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_ammo_sizes(None, None, _pvp).into())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponTtk")]
pub fn get_weapon_ttk(_overshield: f64) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let ttk_data = weapon.calc_ttk(_overshield);
    let js_ttk_data: Vec<JsResillienceSummary> = ttk_data.into_iter().map(|r| r.into()).collect();
    Ok(serde_wasm_bindgen::to_value(&js_ttk_data).unwrap())
}

///DEPRECATED for now
// #[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getWeaponFiringData")]
pub fn get_weapon_firing_data(
    _dynamic_traits: bool,
    _pvp: bool,
    _use_rpl: bool,
) -> Result<JsFiringResponse, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let mut response: types::rs_types::FiringResponse;
    if _dynamic_traits {
        response = weapon.calc_firing_data(Some(weapon.static_calc_input()), None, _pvp);
    } else {
        response = weapon.calc_firing_data(None, None, _pvp);
    };
    PERS_DATA.with(|perm_data| {
        response.apply_pve_bonuses(
            perm_data.borrow().activity.get_rpl_mult(),
            perm_data.borrow().activity.get_pl_delta(),
            perm_data.borrow().weapon.damage_mods.pve,
            perm_data
                .borrow()
                .weapon
                .damage_mods
                .get_mod(&perm_data.borrow().enemy.type_),
        )
    });
    Ok(response.into())
}

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getMiscData")]
pub fn get_misc_data(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _dynamic_traits {
        Ok(serde_wasm_bindgen::to_value(
            &weapon.get_misc_stats(Some(weapon.static_calc_input()), _pvp),
        )
        .unwrap())
    } else {
        Ok(serde_wasm_bindgen::to_value(&weapon.get_misc_stats(None, _pvp)).unwrap())
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setEncounter")]
pub fn set_encounter(
    _reccomended_pl: u32,
    _player_pl: u32,
    _override_cap: i32,
    _difficulty: JsDifficultyOptions,
    _enemy_type: JsEnemyType,
) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        let mut activity = &mut perm_data.borrow_mut().activity;
        activity.rpl = _reccomended_pl;
        activity.cap = _override_cap;
        activity.difficulty = _difficulty.into();
        activity.player.pl = _player_pl;
    });
    PERS_DATA.with(|perm_data| {
        let mut enemy = &mut perm_data.borrow_mut().enemy;
        enemy.type_ = _enemy_type.into();
    });
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "setLoggingLevel")]
pub fn set_logging_level(_level: usize) -> Result<(), JsValue> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().log_level = _level.into();
    });
    Ok(())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen(js_name = "getModifierResponseSummary")]
pub fn get_modifier_response(_dynamic_traits: bool, _pvp: bool) -> Result<JsValue, JsValue> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let modifier = weapon.get_modifier_summary(
        _dynamic_traits.then_some(weapon.static_calc_input()),
        _pvp,
        None,
    );
    Ok(serde_wasm_bindgen::to_value(&modifier).unwrap())
}

#[cfg(all(feature = "wasm", feature = "foundry"))]
#[wasm_bindgen(js_name = "getScalarResponseSummary")]
pub fn get_scalar_response(_pvp: bool) -> Result<JsScalarResponse, JsValue> {
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
    Ok(JsScalarResponse {
        ads_range_scalar: rmr.range_zoom_scale,
        global_range_scalar: rmr.range_all_scale,
        hipfire_range_scalar: rmr.range_hip_scale,
        ads_scalar: hmr.ads_scale,
        draw_scalar: hmr.draw_scale,
        stow_scalar: hmr.stow_scale,
        reload_scalar: rsmr.reload_time_scale,
        mag_size_scalar: mmr.magazine_scale,
        reserve_size_scalar: imr.inv_scale,
    })
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "python")]
#[pyfunction(name = "get_hash")]
fn get_weapon_hash() -> PyResult<u32> {
    Ok(PERS_DATA.with(|perm_data| perm_data.borrow().weapon.hash))
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_weapon")]
fn set_weapon(
    _hash: u32,
    _weapon_type_id: u8,
    _intrinsic_hash: u32,
    _ammo_type_id: u32,
    _damage_type_id: u32,
) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        let new_weapon = Weapon::generate_weapon(
            _hash,
            _weapon_type_id,
            _intrinsic_hash,
            _ammo_type_id,
            _damage_type_id,
        );
        if new_weapon.is_err() {
            #[cfg(feature = "python")]
            println!(
                "Could not find weapon data for type: {}, intrinsic: {}",
                _weapon_type_id, _intrinsic_hash
            );
            perm_data.borrow_mut().weapon = Weapon::default();
        } else {
            perm_data.borrow_mut().weapon = new_weapon.unwrap();
        };
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "add_trait")]
fn add_perk(_perk: PyPerk) -> PyResult<()> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.add_perk(_perk.into()));
    Ok(())
}
#[cfg(feature = "python")]
#[pyfunction(name = "remove_trait")]
fn remove_perk(_perk_hash: u32) -> PyResult<()> {
    PERS_DATA.with(|perm_data| perm_data.borrow_mut().weapon.remove_perk(_perk_hash));
    Ok(())
}
#[cfg(feature = "python")]
#[pyfunction(name = "stringify_weapon")]
fn weapon_as_string() -> PyResult<String> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    Ok(format!("{:?}", weapon))
}
#[cfg(feature = "python")]
#[pyfunction(name = "get_range_falloff")]
fn get_weapon_range(_use_traits: bool, _pvp: bool) -> PyResult<PyRangeResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _use_traits {
        Ok(weapon
            .calc_range_falloff(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_range_falloff(None, None, _pvp).into())
    }
}
#[cfg(feature = "python")]
#[pyfunction(name = "get_handling_times")]
fn get_weapon_handling(_use_traits: bool, _pvp: bool) -> PyResult<PyHandlingResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    if _use_traits {
        Ok(weapon
            .calc_handling_times(Some(weapon.static_calc_input()), None, _pvp)
            .into())
    } else {
        Ok(weapon.calc_handling_times(None, None, _pvp).into())
    }
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_firing_data")]
fn get_firing_data(
    _dynamic_traits: bool,
    _pvp: bool,
    _use_rpl: bool,
) -> PyResult<PyFiringResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let mut response: types::rs_types::FiringResponse;
    if _dynamic_traits {
        response = weapon.calc_firing_data(Some(weapon.static_calc_input()), None, _pvp);
    } else {
        response = weapon.calc_firing_data(None, None, _pvp);
    };
    PERS_DATA.with(|perm_data| {
        response.apply_pve_bonuses(
            perm_data.borrow().activity.get_rpl_mult(),
            perm_data.borrow().activity.get_pl_delta(),
            perm_data.borrow().weapon.damage_mods.pve,
            perm_data
                .borrow()
                .weapon
                .damage_mods
                .get_mod(&perm_data.borrow().enemy.type_),
        )
    });
    Ok(response.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_dps")]
fn get_weapon_dps(_do_rpl_mult: bool) -> PyResult<PyDpsResponse> {
    let weapon = PERS_DATA.with(|perm_data| perm_data.borrow().weapon.clone());
    let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
    let pl_dmg_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_pl_delta());
    let mut dps_response = weapon.calc_dps(enemy, pl_dmg_mult);
    let rpl_mult = PERS_DATA.with(|perm_data| perm_data.borrow().activity.get_rpl_mult());
    if _do_rpl_mult {
        dps_response.apply_rpl(rpl_mult)
    }
    Ok(dps_response.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_ttk")]
fn get_weapon_ttk(_overshield: f64) -> PyResult<Vec<PyResillienceSummary>> {
    let rs_resill_array =
        PERS_DATA.with(|perm_data| perm_data.borrow().weapon.calc_ttk(_overshield));
    //call into on every item in the array
    let mut py_resill_array = Vec::new();
    for rs_resill in rs_resill_array {
        py_resill_array.push(rs_resill.into());
    }
    Ok(py_resill_array)
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_stats")]
fn set_weapon_stats(_in: &PyDict) {
    let mut stats = HashMap::new();
    for (key, value) in _in.iter() {
        let key = key.extract::<u32>().unwrap();
        let value: Stat = value.extract::<i32>().unwrap().into();
        stats.insert(key, value);
    }
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().weapon.stats = stats;
    });
}

#[cfg(feature = "python")]
#[pyfunction(name = "reverse_pve_calc")]
fn reverse_pve_calc(
    _damage: f64,
    _combatant_mult: Option<f64>,
    _pve_mult: Option<f64>,
) -> PyResult<f64> {
    use logging::extern_log;
    let output = PERS_DATA.with(|perm_data| {
        let combatant_mult = _combatant_mult.unwrap_or(1.0);
        let pve_mult = _pve_mult.unwrap_or(1.0);
        if perm_data.borrow().activity.name == "Default" {
            extern_log(
                "Activity is default and can return bad values",
                LogLevel::Warning,
            )
        }
        activity::damage_calc::remove_pve_bonuses(
            _damage,
            combatant_mult,
            &perm_data.borrow().activity,
        ) / pve_mult
    });
    Ok(output)
}

#[cfg(feature = "python")]
fn register_weapon_interface(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let weapon_interface = PyModule::new(py, "WeaponInterface")?;
    //functions
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_handling, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_range, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(weapon_as_string, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(remove_perk, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(add_perk, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_hash, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_dps, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(set_weapon, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_weapon_ttk, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(set_weapon_stats, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(get_firing_data, weapon_interface)?)?;
    weapon_interface.add_function(wrap_pyfunction!(reverse_pve_calc, weapon_interface)?)?;

    //classes
    weapon_interface.add_class::<PyPerk>()?;
    weapon_interface.add_class::<PyRangeResponse>()?;
    weapon_interface.add_class::<PyHandlingResponse>()?;
    weapon_interface.add_class::<PyDpsResponse>()?;
    weapon_interface.add_class::<PyResillienceSummary>()?;
    parent_module.add_submodule(weapon_interface)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_activity")]
fn set_activity(_activity: PyActivity) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().activity.cap = _activity.cap;
        perm_data.borrow_mut().activity.name = _activity.name;
        perm_data.borrow_mut().activity.rpl = _activity.rpl;
        perm_data.borrow_mut().activity.difficulty = _activity.difficulty.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_activity")]
fn get_activity() -> PyResult<PyActivity> {
    let activity = PERS_DATA.with(|perm_data| perm_data.borrow().activity.clone());
    Ok(activity.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_player")]
fn set_player(_player: PyPlayer) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().activity.player = _player.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_player")]
fn get_player() -> PyResult<PyPlayer> {
    let player = PERS_DATA.with(|perm_data| perm_data.borrow().activity.player.clone());
    Ok(player.into())
}

#[cfg(feature = "python")]
fn register_activity_interface(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let activity_interface = PyModule::new(py, "ActivityInterface")?;
    //functions
    activity_interface.add_function(wrap_pyfunction!(get_activity, activity_interface)?)?;
    activity_interface.add_function(wrap_pyfunction!(set_activity, activity_interface)?)?;
    activity_interface.add_function(wrap_pyfunction!(get_player, activity_interface)?)?;
    activity_interface.add_function(wrap_pyfunction!(set_player, activity_interface)?)?;

    //classes
    activity_interface.add_class::<PyActivity>()?;
    activity_interface.add_class::<PyPlayer>()?;
    activity_interface.add_class::<PyDifficultyOptions>()?;
    activity_interface.add_class::<PyPlayerClass>()?;

    parent_module.add_submodule(activity_interface)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "get_enemy")]
fn get_enemy() -> PyResult<PyEnemy> {
    let enemy = PERS_DATA.with(|perm_data| perm_data.borrow().enemy.clone());
    Ok(enemy.into())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_enemy")]
fn set_enemy(_enemy: PyEnemy) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().enemy = _enemy.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
#[pyfunction(name = "set_enemy_type")]
fn set_enemy_type(_enemy_type: PyEnemyType) -> PyResult<()> {
    PERS_DATA.with(|perm_data| {
        perm_data.borrow_mut().enemy.type_ = _enemy_type.into();
    });
    Ok(())
}

#[cfg(feature = "python")]
fn register_enemy_interface(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let enemy_interface = PyModule::new(py, "EnemyInterface")?;
    //functions
    enemy_interface.add_function(wrap_pyfunction!(get_enemy, enemy_interface)?)?;
    enemy_interface.add_function(wrap_pyfunction!(set_enemy, enemy_interface)?)?;
    enemy_interface.add_function(wrap_pyfunction!(set_enemy_type, enemy_interface)?)?;

    //classes
    enemy_interface.add_class::<PyEnemy>()?;
    enemy_interface.add_class::<PyEnemyType>()?;

    parent_module.add_submodule(enemy_interface)?;
    Ok(())
}

#[cfg(feature = "python")]
#[pymodule]
fn d2_calculation_api(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    register_weapon_interface(_py, m)?;
    register_activity_interface(_py, m)?;
    register_enemy_interface(_py, m)?;
    Ok(())
}
