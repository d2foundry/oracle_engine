use std::collections::HashMap;

use crate::{
    d2_enums::{AmmoType, DamageType, WeaponType},
    database,
    perks::{enhanced_check, Perk},
    types::formula_types::*,
};

use super::Weapon;

fn get_data_pointers(_weapon_type_id: u8, _intrinsic_hash: u32) -> Result<DataPointers, String> {
    let pointer_map: HashMap<WeaponPath, DataPointers> = HashMap::from(database::DATA_POINTERS);
    let pointer_result = pointer_map.get(&WeaponPath(_weapon_type_id as u32, _intrinsic_hash));
    if pointer_result.is_none() {
        return Err(format!(
            "No data pointers found for intrinsic hash: {}",
            _intrinsic_hash
        ));
    }
    let pointer = pointer_result.expect("Failed to get pointer");
    Ok(*pointer)
}

impl Weapon {
    pub fn generate_weapon(
        _hash: u32,
        _weapon_type_id: u8,
        _intrinsic_hash: u32,
        _ammo_type_id: u32,
        _damage_type_id: u32,
    ) -> Result<Weapon, String> {
        let data_pointer_result = get_data_pointers(_weapon_type_id, _intrinsic_hash);

        let data_pointer = data_pointer_result?;

        let range_formula: RangeFormula = database::RANGE_DATA[data_pointer.r];

        let handling_formula: HandlingFormula = database::HANDLING_DATA[data_pointer.h];

        let reload_formula: ReloadFormula = database::RELOAD_DATA[data_pointer.rl];

        let damage_mods: DamageModFormula = database::SCALAR_DATA[data_pointer.s];

        let firing_data: FiringDataFormula = database::FIRING_DATA[data_pointer.f];

        let ammo_formula: AmmoFormula = database::AMMO_DATA[data_pointer.a];

        let weapon_type = WeaponType::from(_weapon_type_id as u32);
        let ammo_type = AmmoType::from(_ammo_type_id);
        let damage_type = DamageType::from(_damage_type_id);
        let intrinsic_alias = enhanced_check(_intrinsic_hash).0;
        Ok(Weapon {
            intrinsic_hash: intrinsic_alias,
            hash: _hash,
            perks: HashMap::from([
                (
                    intrinsic_alias,
                    Perk {
                        stat_buffs: HashMap::new(),
                        enhanced: false,
                        value: 0,
                        hash: intrinsic_alias,
                        raw_hash: _intrinsic_hash,
                    },
                ),
                (
                    0,
                    Perk {
                        stat_buffs: HashMap::new(),
                        enhanced: false,
                        value: 0,
                        hash: 0,
                        raw_hash: 0,
                    },
                ),
            ]),
            stats: HashMap::new(),
            perk_value_map: HashMap::from([(intrinsic_alias, 0), (0, 0)]),
            damage_mods,
            ammo_formula,
            firing_data,
            handling_formula,
            reload_formula,
            range_formula,
            ammo_type,
            damage_type,
            weapon_type,
        })
    }
}
