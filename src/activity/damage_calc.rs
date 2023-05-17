#![allow(dead_code)]
use super::Activity;
use crate::{enemies::EnemyType, types::rs_types::DamageMods};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
struct TableKey {
    time: f64,
    value: f64,
}
#[derive(Debug, Clone)]
struct LinearTable {
    table: Vec<TableKey>,
}
impl LinearTable {
    fn evaluate(&self, time: f64) -> f64 {
        if time > 0.0 {
            return self.table[self.table.len() - 1].value;
        }
        let mut index = 0;
        for i in 0..self.table.len() {
            if self.table[i].time >= time {
                index = i;
                break;
            }
        }
        if index == 0 {
            return self.table[0].value;
        }
        if index == self.table.len() {
            return self.table[self.table.len() - 1].value;
        }
        let a = self.table[index - 1];
        let b = self.table[index];
        let t = (time - a.time) / (b.time - a.time);
        return a.value + (b.value - a.value) * t;
    }
    fn from_vecs(_times: [f64; 11], _values: [f64; 11]) -> LinearTable {
        let mut table = Vec::new();
        let times = _times.clone().to_vec();
        let values = _values.clone().to_vec();
        for i in 0..times.len() {
            table.push(TableKey {
                time: times[i],
                value: values[i],
            });
        }
        table.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        return LinearTable { table };
    }
}

#[derive(Debug, Clone)]
pub struct DifficultyData {
    name: String,
    cap: i32,
    table: LinearTable,
}

const MASTER_VALUES: [f64; 11] = [
    0.85, 0.68, 0.58, 0.535, 0.510, 0.490, 0.475, 0.46, 0.44, 0.42, 0.404,
];
const MASTER_TIMES: [f64; 11] = [
    0.0, -10.0, -20.0, -30.0, -40.0, -50.0, -60.0, -70.0, -80.0, -90.0, -99.0,
];

const NORMAL_VALUES: [f64; 11] = [
    1.0, 0.78, 0.66, 0.5914, 0.541, 0.5, 0.475, 0.46, 0.44, 0.42, 0.402,
];
const NORMAL_TIMES: [f64; 11] = [
    0.0, -10.0, -20.0, -30.0, -40.0, -50.0, -60.0, -70.0, -80.0, -90.0, -99.0,
];

const RAID_VALUES: [f64; 11] = [
    0.925, 0.73, 0.62, 0.563, 0.525, 0.495, 0.475, 0.46, 0.44, 0.42, 0.402,
];
const RAID_TIMES: [f64; 11] = [
    0.0, -10.0, -20.0, -30.0, -40.0, -50.0, -60.0, -70.0, -80.0, -90.0, -99.0,
];

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
                table: LinearTable::from_vecs(NORMAL_TIMES, NORMAL_VALUES),
            },
            DifficultyOptions::MASTER => DifficultyData {
                name: "Master".to_string(),
                cap: 20,
                table: LinearTable::from_vecs(MASTER_TIMES, MASTER_VALUES),
            },
            DifficultyOptions::RAID => DifficultyData {
                name: "Raid & Dungeon".to_string(),
                cap: 20,
                table: LinearTable::from_vecs(RAID_TIMES, RAID_VALUES),
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
    // good - harm
    return (1.0 + ((1.0 / 30.0) * _rpl)) / (4.0 / 3.0);
}

pub(super) fn gpl_delta(_activity: &Activity) -> f64 {
    let difficulty_data: DifficultyData = _activity.difficulty.get_difficulty_data();

    let rpl = _activity.rpl;
    let cap = if _activity.cap < difficulty_data.cap {
        _activity.cap
    } else {
        difficulty_data.cap
    };
    let mut delta = _activity.player.pl as i32 - rpl as i32;
    if delta < -99 {
        return 0.0;
    } else if delta > cap {
        delta = cap;
    }
    let wep_delta_mult = std::f64::consts::E.powf(WEAPON_DELTA_EXPONENT * (delta as f64));

    let curve = difficulty_data.table;
    let gear_delta_mult = curve.evaluate(delta as f64);
    wep_delta_mult * gear_delta_mult
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
    let gpl_delta = gpl_delta(_activity);

    _damage / (gpl_delta * rpl_mult * _combatant_mult)
}
