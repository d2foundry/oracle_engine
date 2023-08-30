#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CachedBuildData {
    last_manifest_version: String,
    dim_perk_mappings: Vec<(u32, u32)>,
    procedural_intrinsic_mappings: Vec<(u32, u32)>,
    //use ordered hash map
    perk_formula_timestamps: BTreeMap<u64, u64>,
}
impl CachedBuildData {
    fn has_data(&self) -> bool {
        !self.last_manifest_version.is_empty()
            && !self.dim_perk_mappings.is_empty()
            && !self.procedural_intrinsic_mappings.is_empty()
            && !self.perk_formula_timestamps.is_empty()
    }

    fn get_timestamp(&mut self, formula: &impl UuidTimestamp) -> u64 {
        // get current unix time
        let uuid = (formula.uuid() * 10.0).to_bits();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if let std::collections::btree_map::Entry::Vacant(e) =
            self.perk_formula_timestamps.entry(uuid)
        {
            e.insert(now);
            now
        } else {
            *self.perk_formula_timestamps.get(&uuid).unwrap()
        }
    }

    fn sort(&mut self) {
        self.dim_perk_mappings.sort();
        self.procedural_intrinsic_mappings.sort();
    }
}

#[derive(Debug, Clone, Hash)]
struct WeaponPath(u32, u32);

trait UuidTimestamp {
    fn uuid(&self) -> f64;
}

fn find_uuid<T: UuidTimestamp>(vec: &Vec<T>, uuid: f64) -> Option<usize> {
    vec.iter().position(|x| x.uuid() == uuid)
}

fn json_0_float() -> Value {
    Value::Number(Number::from_f64(0.0).unwrap())
}

fn json_1_float() -> Value {
    Value::Number(Number::from_f64(1.0).unwrap())
}

//these types reflect whats in src/types/rs_types.rs
#[derive(Debug, Clone, Serialize, Default)]
pub struct StatQuadraticFormula {
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}
impl From<&Map<String, Value>> for StatQuadraticFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        StatQuadraticFormula {
            evpp: _val
                .get("evpp")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
            vpp: _val
                .get("vpp")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
            offset: _val
                .get("offset")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
        }
    }
}
impl UuidTimestamp for StatQuadraticFormula {
    fn uuid(&self) -> f64 {
        (self.evpp - 11.0) * 97293.0 + self.vpp * 84892.0 + self.offset * 3321.0
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct DamageMods {
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
    pub timestamp: u64,
}
impl From<&Map<String, Value>> for DamageMods {
    fn from(_val: &Map<String, Value>) -> Self {
        DamageMods {
            pve: 1.0,
            minor: _val
                .get("minor")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            elite: _val
                .get("elite")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            miniboss: _val
                .get("miniboss")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            champion: _val
                .get("champion")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            boss: _val
                .get("boss")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            vehicle: _val
                .get("vehicle")
                .unwrap_or(&json_1_float())
                .as_f64()
                .unwrap_or(1_f64),
            timestamp: 0,
        }
    }
}
impl DamageMods {
    pub fn add_pve_mult(self, mult: f64) -> Self {
        // not super memory efficient but it works
        DamageMods {
            pve: mult,
            minor: self.minor,
            elite: self.elite,
            miniboss: self.miniboss,
            champion: self.champion,
            boss: self.boss,
            vehicle: self.vehicle,
            timestamp: 0,
        }
    }
}
impl UuidTimestamp for DamageMods {
    fn uuid(&self) -> f64 {
        (self.pve - 12.0) * 6729.0
            + self.minor * 18342.0
            + self.elite * 88831.0
            + self.miniboss * 544.0
            + self.champion * 995.0
            + self.boss * 392.0
            + self.vehicle * 3223.0
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct RangeFormula {
    pub start: StatQuadraticFormula,
    pub end: StatQuadraticFormula,
    pub floor_percent: f64,
    pub fusion: bool,
    pub timestamp: u64,
}
impl From<&Map<String, Value>> for RangeFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        RangeFormula {
            start: StatQuadraticFormula {
                evpp: 0.0,
                vpp: _val["vpp_start"].as_f64().unwrap_or_default(),
                offset: _val["offset_start"].as_f64().unwrap_or_default(),
            },
            end: StatQuadraticFormula {
                evpp: 0.0,
                vpp: _val["vpp_end"].as_f64().unwrap_or_default(),
                offset: _val["offset_end"].as_f64().unwrap_or_default(),
            },
            floor_percent: _val["floor_percent"].as_f64().unwrap_or_default(),
            fusion: _val
                .get("fusion")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap_or(false),
            timestamp: 0,
        }
    }
}
impl UuidTimestamp for RangeFormula {
    fn uuid(&self) -> f64 {
        (self.start.uuid() + 17.0) * 920.0
            + self.end.uuid()
            + self.floor_percent * 92.0
            + (self.fusion as u32) as f64 * 88.0
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ReloadFormula {
    pub reload_data: StatQuadraticFormula,
    pub ammo_percent: f64,
    pub timestamp: u64,
}
impl From<&Map<String, Value>> for ReloadFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        ReloadFormula {
            reload_data: StatQuadraticFormula::from(&_val.clone()),
            ammo_percent: _val
                .get("ammo_percent")
                .unwrap_or(&json_0_float())
                .as_f64()
                .unwrap_or_default(),
            timestamp: 0,
        }
    }
}
impl UuidTimestamp for ReloadFormula {
    fn uuid(&self) -> f64 {
        (self.reload_data.uuid() + 29.5) * 72.0 + (self.ammo_percent + 3.0) * 12.0
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct HandlingFormula {
    pub ready: StatQuadraticFormula,
    pub stow: StatQuadraticFormula,
    pub ads: StatQuadraticFormula,
    pub timestamp: u64,
}
impl From<&Map<String, Value>> for HandlingFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        HandlingFormula {
            ready: StatQuadraticFormula::from(_val["ready"].as_object().unwrap_or(&Map::new())),
            stow: StatQuadraticFormula::from(_val["stow"].as_object().unwrap_or(&Map::new())),
            ads: StatQuadraticFormula::from(_val["ads"].as_object().unwrap_or(&Map::new())),
            timestamp: 0,
        }
    }
}
impl UuidTimestamp for HandlingFormula {
    fn uuid(&self) -> f64 {
        self.ready.uuid() * 79.0 + self.stow.uuid() / 2.0 + self.ads.uuid() * 79.9
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct AmmoFormula {
    pub mag: StatQuadraticFormula,
    pub round_to: i32,
    pub reserve_id: u32,
    pub timestamp: u64,
}
impl From<&Map<String, Value>> for AmmoFormula {
    fn from(_val: &Map<String, Value>) -> Self {
        AmmoFormula {
            mag: StatQuadraticFormula::from(_val["mag"].as_object().unwrap_or(&Map::new())),
            round_to: _val
                .get("round_to")
                .unwrap_or(&Value::Null)
                .as_i64()
                .unwrap_or_default() as i32,
            reserve_id: _val
                .get("reserve_id")
                .unwrap_or(&Value::Null)
                .as_u64()
                .unwrap_or_default() as u32,
            timestamp: 0,
        }
    }
}
impl UuidTimestamp for AmmoFormula {
    fn uuid(&self) -> f64 {
        self.mag.uuid() * 99.0 + self.round_to as f64 * 6723.3 + self.reserve_id as f64 * 5299.2
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct FiringData {
    pub damage: f64,
    pub crit_mult: f64,
    pub burst_delay: f64,
    pub inner_burst_delay: f64,
    pub burst_size: i32,
    pub one_ammo: bool,
    pub charge: bool,
    pub timestamp: u64,
}
impl From<&Map<String, Value>> for FiringData {
    fn from(_val: &Map<String, Value>) -> Self {
        FiringData {
            damage: _val["damage"].as_f64().unwrap_or_default(),
            crit_mult: _val["crit_mult"].as_f64().unwrap_or_default(),
            burst_delay: _val["burst_delay"].as_f64().unwrap_or_default(),
            inner_burst_delay: _val["inner_burst_delay"].as_f64().unwrap_or_default(),
            burst_size: _val["burst_size"].as_i64().unwrap_or_default() as i32,
            one_ammo: _val
                .get("one_ammo")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap_or(false),
            charge: _val
                .get("charge")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap_or(false),
            timestamp: 0,
        }
    }
}
impl UuidTimestamp for FiringData {
    fn uuid(&self) -> f64 {
        (self.damage * 821.88
            + self.crit_mult * 388.1
            + self.burst_delay * 9999.7
            + self.inner_burst_delay * 7234.9
            + self.burst_size as f64 * 999.3
            + (self.one_ammo as u32) as f64 * 16655.5
            + (self.charge as u32) as f64 * 7388.9)
            * 10.0
    }
}

#[derive(Debug, Clone, Default)]
struct DataPointers {
    h: usize,
    r: usize,
    rl: usize,
    s: usize,
    f: usize,
    a: usize,
}

fn write_variable(
    writer: &mut std::fs::File,
    name: &str,
    datatype: &str,
    value: String,
    doc: &str,
) {
    let res = writeln!(
        writer,
        "#[doc=r#\"{}\"#]\n#[allow(dead_code)]\n#[allow(clippy::approx_constant)]\npub const {}: {} = {};",
        doc, name, datatype, value
    );
    if res.is_err() {
        println!("cargo:warning=error writing variable");
    }
}

fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);

    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let built_dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
    let formula_dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("formulas.rs");

    built::write_built_file_with_opts(&opts, src.as_ref(), &built_dst)
        .expect("Failed to acquire build-time information");

    let mut formula_file = std::fs::File::create(formula_dst).unwrap();

    //write imports in file
    let res = writeln!(
        formula_file,
        "use crate::types::rs_types::{{StatQuadraticFormula, RangeFormula, HandlingFormula, ReloadFormula, DamageMods, AmmoFormula, DataPointers, FiringData, WeaponPath}};");
    if res.is_err() {
        panic!("cargo:warning=error writing imports");
    }

    let build_cache_path = std::path::Path::new("./build_resources/cached_build.ron");
    let mut cached_data: CachedBuildData;
    //if "./build_resources/cached_build.ron" exists
    if !build_cache_path.exists() {
        println!("cargo:warning=no cached build file found");
        cached_data = CachedBuildData::default();
    } else {
        let file_res = std::fs::File::open(build_cache_path);
        if file_res.is_err() {
            println!(
                "cargo:warning=error opening cached build file: {}",
                file_res.err().unwrap()
            );
            cached_data = CachedBuildData::default();
        } else {
            let file = file_res.unwrap();
            let res = ron::de::from_reader(file);
            if res.is_err() {
                panic!("cargo:warning=error reading cached build file");
            } else {
                cached_data = res.unwrap();
            }
        }
    }

    construct_enhance_perk_mapping(&mut formula_file, &mut cached_data);
    construct_weapon_formulas(&mut formula_file, &mut cached_data);

    cached_data.sort();

    //check if being run by rust-analyzer
    let is_rust_analyzer = std::env::var("IS_RA");
    if is_rust_analyzer.is_ok() && is_rust_analyzer.unwrap() == "true" {
        println!("cargo:warning=running in rust-analyzer");
        return;
    }

    let file_res = std::fs::File::create("./build_resources/cached_build.ron");
    if file_res.is_err() {
        println!("cargo:warning=error writing cached build file");
    } else {
        let mut file = file_res.unwrap();
        // let serializer = ron::ser::Serializer::new(file, None);
        let res = ron::ser::to_string_pretty(&cached_data, ron::ser::PrettyConfig::default());
        if res.is_err() {
            panic!("cargo:warning=error initializing ron serializer");
        } else {
            #[allow(unused_variables)]
            let cd: CachedBuildData = ron::de::from_str(&res.clone().unwrap())
                .expect("cargo:warning=error deserializing");
        }
        file.write_all(res.unwrap().as_bytes())
            .expect("cargo:warning=error writing cached build file");
    }
}

fn construct_weapon_formulas(formula_file: &mut File, cached: &mut CachedBuildData) {
    let id_to_name = HashMap::from([
        (6, "Auto Rifle"),
        (31, "Combat Bow"),
        (11, "Fusion Rifle"),
        (23, "Grenade Launcher"),
        (9, "Hand Cannon"),
        (22, "Linear Fusion Rifle"),
        (8, "Machine Gun"),
        (13, "Pulse Rifle"),
        (10, "Rocket Launcher"),
        (14, "Scout Rifle"),
        (7, "Shotgun"),
        (12, "Sniper Rifle"),
        (24, "Submachine Gun"),
        (33, "Glaive"),
        (25, "Trace Rifle"),
        (17, "Sidearm"),
    ]);

    //get current directory
    let jdata_path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("build_resources/weapon_formulas.json");
    let mut jdata: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(jdata_path).unwrap()).unwrap();
    // remove "COMMENTS" from jdata
    let res = jdata.get("COMMENTS");
    if res.is_none() {
        println!("cargo:warning=comments not found");
        return;
    } else {
        jdata.as_object_mut().unwrap().remove("COMMENTS");
    }

    let mut new_jdata: Value = Value::Object(Map::new());
    let index_data: Value = jdata["INDEX"].take();
    for (key, value) in index_data.as_object().unwrap() {
        let tmp_data_get = jdata[value.as_str().unwrap()].take();
        let mut tmp_data_set = tmp_data_get.as_object().unwrap().clone();
        for (key, value) in tmp_data_get.as_object().unwrap() {
            if key.parse::<u32>().is_ok() {
                let mut tmp_value = value.as_object().unwrap().clone();
                tmp_value.remove("name");
                tmp_data_set[key] = Value::Object(tmp_value);
            }
        }
        new_jdata[key] = Value::Object(tmp_data_set);
    }

    let mut handling_data: Vec<HandlingFormula> = vec![HandlingFormula::default()];
    let mut range_data: Vec<RangeFormula> = vec![RangeFormula::default()];
    let mut reload_data: Vec<ReloadFormula> = vec![ReloadFormula::default()];
    let mut ammo_data: Vec<AmmoFormula> = vec![AmmoFormula::default()];
    let mut firing_data: Vec<FiringData> = vec![FiringData::default()];
    let mut scalar_data: Vec<DamageMods> = vec![DamageMods::default()];

    let mut updated_weapon_defs: Vec<(WeaponPath, DataPointers)> = Vec::new();
    for (weapon_id, inner_values) in new_jdata.as_object().unwrap() {
        for (weapon_hash, weapon_def) in inner_values.as_object().unwrap() {
            let mut data = DataPointers::default();
            //check if weapon_hash is a valid u32
            if weapon_hash.parse::<u32>().is_err() {
                continue;
            }
            // make a closure to parse the json value and return the rrors that occured
            let mut set_data = |val: Value| -> Result<(), Vec<String>> {
                let mut err_list: Vec<String> = Vec::new();
                let cat: Map<String, Value>;
                if val.get("cat").is_some() && weapon_def.get("cat").is_some() {
                    if weapon_def.get("cat") == Some(&Value::Null) {
                        err_list.push(format!(
                            "category not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        cat = Map::new();
                    } else {
                        cat = val["cat"][weapon_def["cat"].as_str().unwrap()]
                            .as_object()
                            .unwrap()
                            .clone();
                    }
                } else {
                    err_list.push(format!(
                        "category not found for {} :> {}",
                        id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                        weapon_hash
                    ));
                    cat = Map::new();
                }
                let mag: Map<String, Value>;
                if val.get("magProf").is_some() && weapon_def.get("magProf").is_some() {
                    if weapon_def.get("magProf") == Some(&Value::Null) {
                        err_list.push(format!(
                            "magazine profile not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        mag = Map::new();
                    } else {
                        mag = val["magProf"][weapon_def["magProf"].as_str().unwrap()]
                            .as_object()
                            .unwrap()
                            .clone();
                    }
                } else {
                    err_list.push(format!(
                        "magazine profile not found for {} :> {}",
                        id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                        weapon_hash
                    ));
                    mag = Map::new();
                }
                let fam: Map<String, Value>;
                if val.get("subFam").is_some() && weapon_def.get("subFam").is_some() {
                    if weapon_def.get("subFam") == Some(&Value::Null) {
                        err_list.push(format!(
                            "subfamily not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        fam = Map::new();
                    } else {
                        fam = val["subFam"][weapon_def["subFam"].as_str().unwrap()]
                            .as_object()
                            .unwrap()
                            .clone();
                    }
                } else {
                    err_list.push(format!(
                        "subfamily not found for {} :> {}",
                        id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                        weapon_hash
                    ));
                    fam = Map::new();
                }
                let empty_object = Value::Object(Map::new());

                let mut reload: ReloadFormula = cat
                    .get("reload")
                    .unwrap_or_else(|| {
                        err_list.push(format!(
                            "reload not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        &empty_object
                    })
                    .as_object()
                    .unwrap()
                    .into();
                let index_option = find_uuid(&reload_data, reload.uuid());
                if index_option.is_some() {
                    data.rl = index_option.unwrap();
                } else {
                    data.rl = reload_data.len();
                    reload.timestamp = cached.get_timestamp(&reload);
                    reload_data.push(reload);
                }

                let mut range: RangeFormula = cat
                    .get("range")
                    .unwrap_or_else(|| {
                        err_list.push(format!(
                            "range not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        &empty_object
                    })
                    .as_object()
                    .unwrap()
                    .into();
                let index_option = find_uuid(&range_data, range.uuid());
                if index_option.is_some() {
                    data.r = index_option.unwrap();
                } else {
                    data.r = range_data.len();
                    range.timestamp = cached.get_timestamp(&range);
                    range_data.push(range);
                }

                let mut handling: HandlingFormula = cat
                    .get("handling")
                    .unwrap_or_else(|| {
                        err_list.push(format!(
                            "handling not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        &empty_object
                    })
                    .as_object()
                    .unwrap()
                    .into();
                let index_option = find_uuid(&handling_data, handling.uuid());
                if index_option.is_some() {
                    data.h = index_option.unwrap();
                } else {
                    data.h = handling_data.len();
                    handling.timestamp = cached.get_timestamp(&handling);
                    handling_data.push(handling);
                }

                let b_scalar: DamageMods = cat
                    .get("combatant_scalars")
                    .unwrap_or_else(|| {
                        err_list.push(format!(
                            "combatant_scalars not found for {} :> {}",
                            id_to_name.get(&weapon_id.parse::<i32>().unwrap()).unwrap(),
                            weapon_hash
                        ));
                        &empty_object
                    })
                    .as_object()
                    .unwrap()
                    .into();
                let mut scalar = b_scalar.add_pve_mult(
                    weapon_def
                        .get("pve")
                        .unwrap_or(&json_1_float())
                        .as_f64()
                        .unwrap_or(1.0),
                );
                let index_option = find_uuid(&scalar_data, scalar.uuid());
                if index_option.is_some() {
                    data.s = index_option.unwrap();
                } else {
                    data.s = scalar_data.len();
                    scalar.timestamp = cached.get_timestamp(&scalar);
                    scalar_data.push(scalar);
                }

                let mut ammo: AmmoFormula = (&mag).into();
                let index_option = find_uuid(&ammo_data, ammo.uuid());
                if index_option.is_some() {
                    data.a = index_option.unwrap();
                } else {
                    data.a = ammo_data.len();
                    ammo.timestamp = cached.get_timestamp(&ammo);
                    ammo_data.push(ammo);
                }

                let mut firing: FiringData = (&fam).into();
                firing.burst_delay *= 1.0 / 30.0;
                firing.inner_burst_delay *= 1.0 / 30.0;
                firing.crit_mult = 1.5 + (firing.crit_mult / 51.0);
                let index_option = find_uuid(&firing_data, firing.uuid());
                if index_option.is_some() {
                    data.f = index_option.unwrap();
                } else {
                    data.f = firing_data.len();
                    firing.timestamp = cached.get_timestamp(&firing);
                    firing_data.push(firing);
                }

                if !err_list.is_empty() {
                    return Err(err_list);
                }
                Ok(())
            };
            let set_data_res = set_data(inner_values.clone());
            if set_data_res.is_err() {
                println!("cargo:warning={:?}", set_data_res.unwrap_err());
            }
            updated_weapon_defs.push((
                WeaponPath(
                    weapon_id.parse::<u32>().unwrap(),
                    weapon_hash.parse::<u32>().unwrap(),
                ),
                data,
            ));
        }
    }

    write_variable(
        formula_file,
        "DATA_POINTERS",
        &format!(
            "[(WeaponPath, DataPointers); {}]",
            updated_weapon_defs.len()
        ),
        format!("{:?}", updated_weapon_defs),
        "Hashmapping for weapon intrinsic hash to data pointers",
    );
    write_variable(
        formula_file,
        "RANGE_DATA",
        &format!("[RangeFormula; {}]", range_data.len()),
        format!("{:?}", range_data),
        "Array of range formulas",
    );
    write_variable(
        formula_file,
        "HANDLING_DATA",
        &format!("[HandlingFormula; {}]", handling_data.len()),
        format!("{:?}", handling_data),
        "Array of handling formulas",
    );
    write_variable(
        formula_file,
        "RELOAD_DATA",
        &format!("[ReloadFormula; {}]", reload_data.len()),
        format!("{:?}", reload_data),
        "Array of reload formulas",
    );
    write_variable(
        formula_file,
        "SCALAR_DATA",
        &format!("[DamageMods; {}]", scalar_data.len()),
        format!("{:?}", scalar_data),
        "Array of combatant scalar formulas",
    );
    write_variable(
        formula_file,
        "FIRING_DATA",
        &format!("[FiringData; {}]", firing_data.len()),
        format!("{:?}", firing_data),
        "Array of firing data formulas",
    );
    write_variable(
        formula_file,
        "AMMO_DATA",
        &format!("[AmmoFormula; {}]", ammo_data.len()),
        format!("{:?}", ammo_data),
        "Array of ammo formulas",
    );
}

fn construct_enhance_perk_mapping(formula_file: &mut File, cached: &mut CachedBuildData) {
    let ping = reqwest::blocking::get("https://www.bungie.net");
    let has_internet = if ping.is_ok() {
        ping.unwrap().status() == reqwest::StatusCode::OK
    } else {
        false
    };

    if !has_internet {
        println!("cargo:warning=no internet connection");
    }

    let mut perk_mappings: Vec<(u32, u32)> = Vec::new();
    if has_internet {
        let json_file = reqwest::blocking::get(
            "https://raw.githubusercontent.com/DestinyItemManager/d2-additional-info/master/output/trait-to-enhanced-trait.json");
        if json_file.is_ok() {
            let json_file = json_file.unwrap();
            let dct = json_file.json::<HashMap<String, u32>>();
            if dct.is_ok() {
                let dct = dct.unwrap();
                for i in dct {
                    perk_mappings.push((i.1, i.0.parse::<u32>().unwrap()));
                }
                cached.dim_perk_mappings = perk_mappings.clone();
            } else {
                println!("cargo:warning=dim enhanced mapping could not be parsed");
                return;
            }
        } else {
            println!("cargo:warning=dim enhanced mapping not found");
            return;
        }
    } else if cached.has_data() {
        println!("cargo:warning=using cached dim enhanced mapping");
        let mut dim_mappings = cached.dim_perk_mappings.clone();
        perk_mappings.append(&mut dim_mappings);
    } else {
        panic!("cargo:warning=no cached dim enhanced mapping found");
    }

    if has_internet {
        let mut manifest_secured: bool;
        let manifest_raw =
            reqwest::blocking::get("https://www.bungie.net/Platform/Destiny2/Manifest/");
        let manifest_text: String;
        if manifest_raw.is_ok() {
            manifest_text = manifest_raw.unwrap().text().unwrap();
            manifest_secured = true;
        } else {
            manifest_text = String::from("");
            println!("cargo:warning=bungie manifest raw error");
            manifest_secured = false;
        }
        let manifest_json: Value;
        if manifest_secured {
            manifest_json = serde_json::from_str(&manifest_text).unwrap();
            if manifest_json["ErrorCode"].as_i64().unwrap_or_default() as i32 != 1_i32 {
                println!("cargo:warning=bungie manifest error code");
                manifest_secured = false;
            }
        } else {
            println!("cargo:warning=bungie manifest error");
            manifest_json = Value::Null;
        }

        if !manifest_secured && cached.has_data() {
            println!("cargo:warning=using cached manifest");
            let mut cached_manifest_mappings = cached.procedural_intrinsic_mappings.clone();
            perk_mappings.append(&mut cached_manifest_mappings);
        } else if !manifest_secured {
            panic!("cargo:warning=bungie manifest error, cached manifest not found");
        }

        if manifest_secured {
            if manifest_json["Response"]["version"] == cached.last_manifest_version {
                let mut cached_manifest_mappings = cached.procedural_intrinsic_mappings.clone();
                perk_mappings.append(&mut cached_manifest_mappings);
            } else {
                cached.last_manifest_version = manifest_json["Response"]["version"]
                    .as_str()
                    .unwrap()
                    .to_owned();
                let intrinsic_map: HashMap<u32, Vec<&str>> = HashMap::from([
                    (901, vec!["High-Impact Frame"]),
                    (902, vec!["VEIST Rapid-Fire", "Rapid-Fire Frame"]),
                    (903, vec!["Adaptive Frame", "Adaptive Glaive"]),
                    (904, vec!["Aggressive Frame", "Aggressive Glaive"]),
                    (905, vec!["Lightweight Frame", "MIDA Synergy"]),
                    (906, vec!["Precision Frame", "Häkke Precision Frame"]),
                    (907, vec!["Double Fire"]),
                    (908, vec!["Wave Frame"]),
                    (911, vec!["Legacy PR-55 Frame"]),
                ]);
                let content_paths = manifest_json["Response"]["jsonWorldComponentContentPaths"]
                    ["en"]
                    .as_object()
                    .unwrap();
                let item_data_raw = reqwest::blocking::get(format!(
                    "https://www.bungie.net{}",
                    content_paths["DestinyInventoryItemDefinition"]
                        .as_str()
                        .unwrap()
                ));
                println!("cargo:warning=downloaded new manifest");
                cached.procedural_intrinsic_mappings.clear();
                let item_data_json: Value =
                    serde_json::from_str(&item_data_raw.unwrap().text().unwrap()).unwrap();
                for (key, value) in item_data_json.as_object().unwrap() {
                    let hash = key.parse::<u32>().unwrap();
                    //does value have a key called itemTypeDisplayName?
                    if !value
                        .as_object()
                        .unwrap()
                        .contains_key("itemTypeDisplayName")
                    {
                        continue;
                    }
                    if !value["itemTypeDisplayName"]
                        .as_str()
                        .unwrap()
                        .contains("Intrinsic")
                    {
                        continue;
                    }
                    let name = value["displayProperties"]["name"].as_str().unwrap();
                    for (id, names) in intrinsic_map.iter() {
                        if names.contains(&name) {
                            perk_mappings.push((hash, *id));
                            cached.procedural_intrinsic_mappings.push((hash, *id));
                        }
                    }
                }
            }
        }
    } else {
        let mut cached_manifest_mappings = cached.procedural_intrinsic_mappings.clone();
        perk_mappings.append(&mut cached_manifest_mappings);
    }
    write_variable(
        formula_file,
        "ENHANCE_PERK_MAPPING",
        &format!("[(u32, u32); {}]", perk_mappings.len()),
        format!("{:?}", perk_mappings),
        "Mapping of enhanced perks and intrinsics to their base perk/intrinsic",
    );
}
