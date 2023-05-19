#![allow(dead_code)]
use super::Activity;
use crate::{enemies::EnemyType, logging, types::rs_types::DamageMods};
use piecewise_linear::PiecewiseLinearFunction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DifficultyData {
    name: String,
    cap: i32,
    table: PiecewiseLinearFunction<f64>,
}

const WEAPON_DELTA_EXPONENT: f64 = 0.00672;

#[derive(Debug, Clone)]
pub enum DifficultyOptions {
    NORMAL = 1,
    RAID = 2,
    MASTER = 3,
}
impl Default for DifficultyOptions {
    fn default() -> Self {
        DifficultyOptions::NORMAL
    }
}
impl DifficultyOptions {
    pub fn get_difficulty_data(&self) -> DifficultyData {
        match self {
            DifficultyOptions::NORMAL => DifficultyData {
                name: "Normal".to_string(),
                cap: 50,
                table: PiecewiseLinearFunction::try_from(vec![
                    (-99.0, 0.4018),
                    (-90.0, 0.4200),
                    (-80.0, 0.4400),
                    (-70.0, 0.4600),
                    (-60.0, 0.4750),
                    (-50.0, 0.5000),
                    (-40.0, 0.5405),
                    (-30.0, 0.5915),
                    (-20.0, 0.6600),
                    (-10.0, 0.7800),
                    (0.0, 1.0),
                ])
                .unwrap(),
            },
            DifficultyOptions::MASTER => DifficultyData {
                name: "Master".to_string(),
                cap: 20,
                table: PiecewiseLinearFunction::try_from(vec![
                    (-99.0, 0.4018),
                    (-90.0, 0.4200),
                    (-80.0, 0.4400),
                    (-70.0, 0.4600),
                    (-60.0, 0.4750),
                    (-50.0, 0.4900),
                    (-40.0, 0.5100),
                    (-30.0, 0.5350),
                    (-20.0, 0.5800),
                    (-10.0, 0.6800),
                    (0.0, 0.8500),
                ])
                .unwrap(),
            },
            DifficultyOptions::RAID => DifficultyData {
                name: "Raid & Dungeon".to_string(),
                cap: 20,
                table: PiecewiseLinearFunction::try_from(vec![
                    (-99.0, 0.4018),
                    (-90.0, 0.4200),
                    (-80.0, 0.4400),
                    (-70.0, 0.4600),
                    (-60.0, 0.4750),
                    (-50.0, 0.4950),
                    (-40.0, 0.5253),
                    (-30.0, 0.5632),
                    (-20.0, 0.62),
                    (-10.0, 0.73),
                    (0.0, 0.925),
                ])
                .unwrap(),
            },
        }
    }
}
impl From<i32> for DifficultyOptions {
    fn from(i: i32) -> Self {
        match i {
            1 => DifficultyOptions::NORMAL,
            2 => DifficultyOptions::RAID,
            3 => DifficultyOptions::MASTER,
            _ => DifficultyOptions::NORMAL,
        }
    }
}

pub(super) fn rpl_mult(_rpl: f64) -> f64 {
    return (1.0 + ((1.0 / 30.0) * _rpl)) / (4.0 / 3.0);
}

pub(super) fn get_gear_delta_mult(_activity: &Activity) -> f64 {
    let difficulty_data: DifficultyData = _activity.difficulty.get_difficulty_data();

    let epl = _activity.player.power as i32 - _activity.rpl as i32;

    if epl < -99 {
        return 0.0;
    }

    let gear_delta_mult = if epl <= 0 {
        difficulty_data.table.y_at_x(epl as f64)
    } else {
        difficulty_data.table.y_at_x(0.0)
    }
    .unwrap();

    crate::logging::log(
        format!("gear_delta_mult: {}", gear_delta_mult).as_str(),
        logging::LogLevel::Debug.into(),
    );
    gear_delta_mult
}

pub(super) fn get_wep_delta_mult(_activity: &Activity) -> f64 {
    let difficulty_data: DifficultyData = _activity.difficulty.get_difficulty_data();

    let cap = if _activity.cap < difficulty_data.cap {
        _activity.cap
    } else {
        difficulty_data.cap
    };

    let epl = (_activity.player.wep_power as i32 - _activity.rpl as i32).clamp(-100, cap);

    if epl < -99 {
        return 0.0;
    }

    let wep_delta_mult = match epl {
        //More accurate formula from mossy
        -50..=50 => {
            (epl as f64) * 0.00683343 + (0.5 * 0.0000441279650846838 * (epl as f64).powi(2)) + 1.0
        }
        //Formula gets messy past these values so we go to a more generic formula
        _ => std::f64::consts::E.powf(WEAPON_DELTA_EXPONENT * (epl as f64)),
    };
    crate::logging::log(
        format!("wep_delta: {}", wep_delta_mult).as_str(),
        logging::LogLevel::Debug.into(),
    );
    wep_delta_mult
}

// add_remove_pve_bonuses(
//     _rpl: f64,
//     _pl: u32,
//     _combatant_mult: f64,
//     _difficulty: DifficultyOptions,
//     _damage: f64,
// ) -> f64 {
//     let rpl_mult = rpl_mult(_rpl);
//     let mut tmp_activity = Activity::default();
//     tmp_activity.difficulty = _difficulty;
//     tmp_activity.rpl = _rpl as u32;
//     let gpl_delta = gpl_delta(tmp_activity, _pl);

//     _damage / (gpl_delta * rpl_mult * _combatant_mult)
// }

pub fn remove_pve_bonuses(_damage: f64, _combatant_mult: f64, _activity: &Activity) -> f64 {
    let rpl_mult = rpl_mult(_activity.rpl as f64);
    let gpl_delta = get_gear_delta_mult(_activity);

    _damage / (gpl_delta * rpl_mult * _combatant_mult)
}
