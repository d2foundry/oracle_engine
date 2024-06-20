use fnv::FnvHasher;
use ordered_float::NotNan;
use phf::{phf_map, Map as PhfMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;



/*const ID_TO_NAME: PhfMap<i32, &'static str> = phf_map! {
    6i32 => "Auto Rifle",
    31i32 => "Combat Bow",
    11i32 => "Fusion Rifle",
    23i32 => "Grenade Launcher",
    9i32=> "Hand Cannon",
    22i32 => "Linear Fusion Rifle",
    8i32=> "Machine Gun",
    13i32 => "Pulse Rifle",
    10i32 => "Rocket Launcher",
    14i32 => "Scout Rifle",
    7i32=> "Shotgun",
    12i32 => "Sniper Rifle",
    24i32 => "Submachine Gun",
    33i32 => "Glaive",
    25i32 => "Trace Rifle",
    17i32 => "Sidearm",
};*/

const NAME_TO_ID: PhfMap<&'static str, i32> = phf_map! {
    "Auto Rifle" =>        6i32,
    "Combat Bow"=>         31i32,
    "Fusion Rifle"=>       11i32,
    "Grenade Launcher" =>  23i32,
    "Hand Cannon" =>       9i32,
    "Linear Fusion Rifle"=>22i32,
    "Machine Gun"=>        8i32,
    "Pulse Rifle"=>        13i32,
    "Rocket Launcher"=>    10i32,
    "Scout Rifle"=>        14i32,
    "Shotgun"=>            7i32,
    "Sniper Rifle"=>       12i32,
    "Submachine Gun" =>    24i32,
    "Glaive"=>             33i32,
    "Trace Rifle"=>        25i32,
    "Sidearm"=>            17i32,
};

const INTRINSIC_MAP: PhfMap<u32, &'static [&'static str]> = phf_map! {
901u32 => &["High-Impact Frame"],
902u32 => &["VEIST Rapid-Fire", "Rapid-Fire Frame", "Rapid-Fire Glaive"],
903u32 => &["Adaptive Frame", "Adaptive Glaive", "Adaptive Burst"],
904u32 => &["Aggressive Frame", "Aggressive Glaive", "Aggressive Burst"],
905u32 => &["Lightweight Frame", "MIDA Synergy"],
906u32 => &["Precision Frame", "Häkke Precision Frame", "Pinpoint Slug Frame"],
907u32 => &["Double Fire", "Heavy Burst"],
908u32 => &["Wave Frame", "Compressed Wave Frame"],
911u32 => &["Legacy PR-55 Frame"],
912u32 => &["Support Frame"],
913u32 => &["Area Denial Frame"],
914u32 => &["Rocket-Assisted Frame"],
915u32 => &["Shot Package"]
};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = FnvHasher::default();
    t.hash(&mut s);
    s.finish()
}
trait PartialHash {
    fn partial_hash<H: Hasher>(self, state: &mut H);
}
impl PartialHash for f64 {
    fn partial_hash<H: Hasher>(self, state: &mut H) {
        NotNan::new(self).unwrap().hash(state);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CachedBuildData {
    last_manifest_version: String,
    dim_perk_mappings: Vec<(u32, u32)>,
    procedural_intrinsic_mappings: Vec<(u32, u32)>,

    perk_timestamps: BTreeMap<u64, u64>,
    #[serde(skip_serializing, default)]
    current_timestamps: HashSet<u64>,
}

impl CachedBuildData {
    fn has_data(&self) -> bool {
        !self.last_manifest_version.is_empty()
            && !self.dim_perk_mappings.is_empty()
            && !self.procedural_intrinsic_mappings.is_empty()
            && !self.perk_timestamps.is_empty()
    }

    fn sort(&mut self) {
        self.dim_perk_mappings.sort();
        self.procedural_intrinsic_mappings.sort();
    }

    fn clean_timestamps(&mut self) {
        for key in self
            .perk_timestamps
            .clone()
            .keys()
            .filter(|x| !self.current_timestamps.contains(x))
        {
            self.perk_timestamps.remove(key);
        }
    }

    fn get_timestamp(&mut self, formula: &impl Hash) -> u64 {
        // get current unix time
        let timestamps = &mut self.perk_timestamps;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash = calculate_hash(&formula);
        self.current_timestamps.insert(hash);

        timestamps.get(&hash).cloned().unwrap_or_else(|| {
            timestamps.insert(hash, now);
            now
        })
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct WeaponPath(u32, u32);

fn find_uuid<T: Hash>(vec: &[T], uuid: &T) -> Option<usize> {
    vec.iter()
        .position(|x| calculate_hash(&x) == calculate_hash(uuid))
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
        "#[doc=r#\"{}\"#]\n#[allow(dead_code,clippy::approx_constant)]\npub const {}: {} = {};",
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

    cached_data.clean_timestamps();
    cached_data.sort();
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

//fn set_data(val: Value, weapon_def: Value, weapon_hash: u32, weapon_id: String) {]

fn construct_weapon_formulas(formula_file: &mut File, cached: &mut CachedBuildData) {
    let jdata_path = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("build_resources/weapon_formulas.json");

    let jdata: WeaponFormulaJson =
        serde_json::from_str(&std::fs::read_to_string(jdata_path).unwrap()).unwrap();

    let mut handling_data: Vec<HandlingFormula> = Vec::new();
    let mut range_data: Vec<RangeFormula> = Vec::new();
    let mut reload_data: Vec<ReloadFormula> = Vec::new();
    let mut ammo_data: Vec<AmmoFormula> = Vec::new();
    let mut firing_data: Vec<FiringData> = Vec::new();
    let mut scalar_data: Vec<DamageMods> = Vec::new();

    let mut updated_weapon_defs: Vec<(WeaponPath, DataPointers)> = Vec::new();

    for (weapon_family, val) in jdata.types {
        for (weapon_hash, weapon_def) in val.intrinsics {
            let mut data = DataPointers::default();
            //add error handling
            let cat: Category = *val.cat.get(&weapon_def.cat).unwrap();
            let mag: AmmoFormula = *val.mag_prof.get(&weapon_def.mag_prof).unwrap();
            let fam: FiringData = *val.sub_fam.get(&weapon_def.sub_fam).unwrap();

            let mut reload: ReloadFormula = cat.reload;

            let index_option = find_uuid(&reload_data, &reload);
            if let Some(index) = index_option {
                data.rl = index;
            } else {
                data.rl = reload_data.len();
                reload.timestamp = cached.get_timestamp(&reload);
                reload_data.push(reload);
            }

            let mut range: RangeFormula = cat.range;
            let index_option = find_uuid(&range_data, &range);
            if let Some(index) = index_option {
                data.r = index;
            } else {
                data.r = range_data.len();
                range.timestamp = cached.get_timestamp(&range);
                range_data.push(range);
            }

            let mut handling: HandlingFormula = cat.handling;

            let index_option = find_uuid(&handling_data, &handling);
            if let Some(index) = index_option {
                data.h = index;
            } else {
                data.h = handling_data.len();
                handling.timestamp = cached.get_timestamp(&handling);
                handling_data.push(handling);
            }

            let mut scalar: DamageMods = cat.combatant_scalars;
            scalar.pve = weapon_def.pve;

            let index_option = find_uuid(&scalar_data, &scalar);
            if let Some(index) = index_option {
                data.s = index;
            } else {
                data.s = scalar_data.len();
                scalar.timestamp = cached.get_timestamp(&scalar);
                scalar_data.push(scalar);
            }

            let mut ammo: AmmoFormula = mag;

            let index_option = find_uuid(&ammo_data, &ammo);

            if let Some(index) = index_option {
                data.a = index
            } else {
                data.a = ammo_data.len();
                ammo.timestamp = cached.get_timestamp(&ammo);
                ammo_data.push(ammo);
            }

            let mut firing: FiringData = fam;
            let index_option = find_uuid(&firing_data, &firing);
            if let Some(index) = index_option {
                data.f = index;
            } else {
                data.f = firing_data.len();
                firing.timestamp = cached.get_timestamp(&firing);
                firing_data.push(firing);
            }

            updated_weapon_defs.push((
                WeaponPath(
                    *NAME_TO_ID.get(&weapon_family).unwrap() as u32,
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
    let has_internet = if let Ok(ping) = ping {
        ping.status().is_success()
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
                    for id in INTRINSIC_MAP.keys() {
                        let names = INTRINSIC_MAP.get(id).unwrap();
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

//these types reflect whats in src/types/rs_types.rs
#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct StatQuadraticFormula {
    #[serde(default)]
    pub evpp: f64,
    pub vpp: f64,
    pub offset: f64,
}

impl Hash for StatQuadraticFormula {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.evpp.partial_hash(state);
        self.vpp.partial_hash(state);
        self.offset.partial_hash(state);
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct DamageMods {
    #[serde(default)]
    pub pve: f64,
    pub minor: f64,
    pub elite: f64,
    pub miniboss: f64,
    pub champion: f64,
    pub boss: f64,
    pub vehicle: f64,
    #[serde(default)]
    pub timestamp: u64,
}

impl Hash for DamageMods {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pve.partial_hash(state);
        self.minor.partial_hash(state);
        self.elite.partial_hash(state);
        self.miniboss.partial_hash(state);
        self.champion.partial_hash(state);
        self.boss.partial_hash(state);
        self.vehicle.partial_hash(state);
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(try_from = "RangeJson")]
pub struct RangeFormula {
    pub start: StatQuadraticFormula,
    pub end: StatQuadraticFormula,
    pub floor_percent: f64,
    #[serde(default)]
    pub fusion: bool,
    #[serde(default)]
    pub timestamp: u64,
}

impl Hash for RangeFormula {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
        self.floor_percent.partial_hash(state);
        self.fusion.hash(state);
    }
}

impl From<RangeJson> for RangeFormula {
    fn from(value: RangeJson) -> Self {
        let start = StatQuadraticFormula {
            vpp: value.vpp_start,
            offset: value.offset_start,
            ..Default::default()
        };
        let end = StatQuadraticFormula {
            vpp: value.vpp_end,
            offset: value.offset_end,
            ..Default::default()
        };
        RangeFormula {
            start,
            end,
            floor_percent: value.floor_percent,
            fusion: value.fusion.unwrap_or_default(),
            timestamp: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct ReloadFormula {
    #[serde(flatten)]
    pub reload_data: StatQuadraticFormula,
    #[serde(default)]
    pub ammo_percent: f64,
    #[serde(default)]
    pub timestamp: u64,
}

impl Hash for ReloadFormula {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.reload_data.hash(state);
        self.ammo_percent.partial_hash(state);
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct HandlingFormula {
    pub ready: StatQuadraticFormula,
    pub stow: StatQuadraticFormula,
    pub ads: StatQuadraticFormula,
    #[serde(default)]
    pub timestamp: u64,
}

impl Hash for HandlingFormula {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ready.hash(state);
        self.stow.hash(state);
        self.ads.hash(state);
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
pub struct AmmoFormula {
    pub mag: StatQuadraticFormula,
    #[serde(default)]
    pub round_to: i32,
    #[serde(default)]
    pub reserve_id: u32,
    #[serde(default)]
    pub timestamp: u64,
}

impl Hash for AmmoFormula {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mag.hash(state);
        self.round_to.hash(state);
        self.reserve_id.hash(state);
    }
}

fn default_i32_1() -> i32 {
    1
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default)]
#[serde(from = "SubFamJson")]
pub struct FiringData {
    pub damage: f64,
    pub crit_mult: f64,
    pub pve_damage: f64,
    pub pve_crit_mult: f64,
    pub burst_delay: f64,
    pub inner_burst_delay: f64,
    #[serde(default)]
    pub burst_size: i32,
    #[serde(default)]
    pub one_ammo: bool,
    #[serde(default)]
    pub charge: bool,
    #[serde(default)]
    pub timestamp: u64,
}

impl From<SubFamJson> for FiringData {
    fn from(value: SubFamJson) -> Self {
        FiringData {
            damage: value.damage,
            crit_mult: (value.crit_mult) / 51.0 + 1.5,
            pve_damage: value.pve_damage,
            pve_crit_mult: (value.pve_crit_mult) / 51.0 + 1.5,
            burst_delay: value.burst_delay / 30.0,
            inner_burst_delay: value.inner_burst_delay / 30.0,
            burst_size: value.burst_size,
            one_ammo: value.one_ammo.unwrap_or_default(),
            charge: value.charge.unwrap_or_default(),
            timestamp: 0,
        }
    }
}

impl Hash for FiringData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.damage.partial_hash(state);
        self.crit_mult.partial_hash(state);
        self.burst_delay.partial_hash(state);
        self.inner_burst_delay.partial_hash(state);
        self.burst_size.hash(state);
        self.one_ammo.hash(state);
        self.charge.hash(state);
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct DataPointers {
    h: usize,
    r: usize,
    rl: usize,
    s: usize,
    f: usize,
    a: usize,
}

#[derive(Clone, Deserialize)]
struct WeaponFormulaJson {
    #[serde(flatten)]
    types: HashMap<String, WeaponFormula>,
}

#[derive(Clone, Deserialize)]
struct WeaponFormula {
    #[serde(flatten)]
    intrinsics: HashMap<String, WeaponIntrinsic>,
    cat: HashMap<String, Category>,
    #[serde(rename = "subFam")]
    sub_fam: HashMap<String, FiringData>,
    #[serde(rename = "magProf")]
    mag_prof: HashMap<String, AmmoFormula>,
}

const fn default_pve() -> f64 {
    1.0
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WeaponIntrinsic {
    cat: String,
    sub_fam: String,
    mag_prof: String,
    #[serde(default = "default_pve")]
    pve: f64,
}

#[derive(Clone, Copy, Deserialize)]
struct Category {
    range: RangeFormula,
    reload: ReloadFormula,
    handling: HandlingFormula,
    combatant_scalars: DamageMods,
}

#[derive(Clone, Copy, Deserialize)]
struct RangeJson {
    vpp_start: f64,
    offset_start: f64,
    vpp_end: f64,
    offset_end: f64,
    floor_percent: f64,
    fusion: Option<bool>,
}

#[derive(Clone, Copy, Deserialize)]
struct SubFamJson {
    damage: f64,
    crit_mult: f64,
    pve_damage: f64,
    pve_crit_mult: f64,
    burst_delay: f64,
    #[serde(default = "default_i32_1")]
    burst_size: i32,
    inner_burst_delay: f64,
    one_ammo: Option<bool>,
    charge: Option<bool>,
}
