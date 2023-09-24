use self::damage_calc::{get_gear_delta_mult, get_wep_delta_mult, rpl_mult, DifficultyOptions};

pub mod damage_calc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayerClass {
    #[default]
    Unknown = 0,
    Titan = 1,
    Hunter = 2,
    Warlock = 3,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Player {
    pub power: u32,
    pub wep_power: u32,
    pub class: PlayerClass,
}

#[derive(Debug, Clone)]
pub struct Activity {
    pub name: String,
    pub difficulty: DifficultyOptions,
    pub rpl: u32,
    pub cap: i32,
    pub player: Player,
}
impl Default for Activity {
    fn default() -> Self {
        let expansion_base = 1600;
        Activity {
            name: "Default".to_string(),
            difficulty: DifficultyOptions::default(),
            rpl: expansion_base,
            cap: 100,
            player: Player {
                power: expansion_base + 210,
                wep_power: expansion_base + 210,
                class: PlayerClass::default(),
            },
        }
    }
}
impl Activity {
    pub fn get_pl_delta(&self) -> f64 {
        get_gear_delta_mult(self) * get_wep_delta_mult(self)
    }
    pub fn get_rpl_mult(&self) -> f64 {
        rpl_mult(self.rpl as f64)
    }
}
