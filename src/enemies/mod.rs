use crate::activity::Activity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnemyType {
    MINOR,
    ELITE,
    MINIBOSS,
    BOSS,
    VEHICLE,
    #[default]
    ENCLAVE,
    PLAYER,
    CHAMPION,
}

#[derive(Debug, Clone, Default)]
pub struct Enemy {
    pub health: f64,
    pub damage: f64,
    pub damage_resistance: f64,
    pub type_: EnemyType,
    pub tier: u8,
}
impl Enemy {
    pub fn get_adjusted_health(&self, _activity: Activity) -> f64 {
        self.health * (1.0 - self.damage_resistance)
    }
}

//is here to sanitize the formulas_types.rs file for build script
impl crate::types::formula_types::DamageModFormula {
    pub fn get_mod(&self, _type: &EnemyType) -> f64 {
        match *_type {
            EnemyType::MINOR => self.minor,
            EnemyType::ELITE => self.elite,
            EnemyType::MINIBOSS => self.miniboss,
            EnemyType::CHAMPION => self.champion,
            EnemyType::BOSS => self.boss,
            EnemyType::VEHICLE => self.vehicle,
            _ => 1.0,
        }
    }
}