#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AbilityType {
    GRENADE,
    MELEE,
    CLASS,
    SUPER,

    //these will typically behave the same but are diff cuz i said so
    WEAPON,
    ARMOR,
    MISC,
    #[default]
    UNKNOWN,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct AbilityDamageProfile {
    impact: f64,
    secondary: f64,
    sec_hit_count: u32,
    lin_hit_scalar: f64,
    crit_mult: f64, // if 1.0, no crit
}

#[derive(Debug, Clone, Default)]
pub struct Ability {
    pub name: String,
    pub hash: u32,
    pub ability_type: AbilityType,
    pub damage_profile: AbilityDamageProfile,
    pub is_initialized: bool,
}
