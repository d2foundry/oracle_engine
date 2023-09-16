use crate::activity::Activity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
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
