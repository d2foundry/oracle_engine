use crate::activity::Activity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Deserialize)]
pub enum EnemyType {
    Minor,
    Elite,
    Miniboss,
    Boss,
    Vehicle,
    #[default]
    Enclave,
    Player,
    Champion,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Enemy {
    pub health: f64,
    pub damage: f64,
    pub damage_resistance: f64,
    pub r#type: EnemyType,
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
            EnemyType::Minor => self.minor,
            EnemyType::Elite => self.elite,
            EnemyType::Miniboss => self.miniboss,
            EnemyType::Champion => self.champion,
            EnemyType::Boss => self.boss,
            EnemyType::Vehicle => self.vehicle,
            _ => 1.0,
        }
    }
}
