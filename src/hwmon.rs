use std::collections::HashMap;
use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};

const HWMON_DIR: &str = "/sys/class/hwmon";
const HWMON_PREFIX: &str = "hwmon";
const HWMON_NAME_FILE: &str = "name";
const HWMON_CORE_TEMP_NAME: &str = "coretemp";

const CORE_TEMP_PREFIX: &str = "temp";
const CORE_TEMP_SUFFIXES: [&str; 5] = ["_input", "_max", "_crit", "_label", "_crit_alarm"];

pub type CoreNumber = u64;

use anyhow as ah;

#[derive(Debug, Clone, Copy)]
pub enum CoreTempDataKind {
    Input,
    Max,
    Crit,
    Label,
    CritAlarm,
}

impl CoreTempDataKind {
    pub fn from_path(path: &PathBuf) -> Option<(CoreNumber, CoreTempDataKind)> {
        let file_name = path.file_name().unwrap().to_str().unwrap();

        if !file_name.starts_with(CORE_TEMP_PREFIX) {
            return None;
        }

        let (_, xs) = match file_name.split_once(CORE_TEMP_PREFIX) {
            Some((prefix, xs)) if prefix == CORE_TEMP_PREFIX => {
                (prefix.to_string(), xs.to_string())
            }
            _ => return None,
        };

        let (id, suffix) = match xs.split_once('_') {
            Some((id, suffix)) if CORE_TEMP_SUFFIXES.contains(&suffix) => {
                (id.to_string(), suffix.to_string())
            }
            _ => return None,
        };

        let core_number = match id.parse::<CoreNumber>() {
            Ok(id_num) => id_num,
            _ => return None,
        };

        let kind = match suffix.as_str() {
            "input" => CoreTempDataKind::Input,
            "max" => CoreTempDataKind::Max,
            "crit" => CoreTempDataKind::Crit,
            "label" => CoreTempDataKind::Label,
            "crit_alarm" => CoreTempDataKind::CritAlarm,
            _ => return None,
        };

        return Some((core_number, kind));
    }
}

#[derive(Debug, Clone)]
pub struct TempSensorFiles {
    input: PathBuf,
    max: PathBuf,
    crit: PathBuf,
    label: PathBuf,
    crit_alarm: PathBuf,
}

#[derive(Debug, Clone)]
pub struct HwmCoreTemp(usize, HashMap<CoreNumber, TempSensorFiles>);

impl HwmCoreTemp {
    pub fn from_dir(path: &PathBuf) -> ah::Result<HwmCoreTemp> {
        let mut cores: HwmCoreTemp = HwmCoreTemp(0, HashMap::new());
        let mut data: HashMap<CoreNumber, Vec<(PathBuf, CoreTempDataKind)>> = HashMap::new();

        for entry in path.read_dir()? {
            let file_path = match entry.map(|e| e.path()) {
                Ok(path) if path.is_file() => path,
                _ => continue,
            };

            let kind = CoreTempDataKind::from_path(&file_path);

            let (core_number, kind) = match kind {
                Some((n, k)) => (n, k),
                None => continue,
            };

            data.entry(core_number)
                .or_insert_with_key(|_| vec![(file_path.clone(), kind)])
                .push((file_path, kind));
        }

        for (core_number, kinds) in data {
            let mut input = None;
            let mut max = None;
            let mut crit = None;
            let mut label = None;
            let mut crit_alarm = None;

            for kind in kinds {
                match kind {
                    (p, CoreTempDataKind::Input) => input = Some(p),
                    (p, CoreTempDataKind::Max) => max = Some(p),
                    (p, CoreTempDataKind::Crit) => crit = Some(p),
                    (p, CoreTempDataKind::Label) => label = Some(p),
                    (p, CoreTempDataKind::CritAlarm) => crit_alarm = Some(p),
                }
            }

            match (&input, &max, &crit, &label, &crit_alarm) {
                (Some(ip), Some(mp), Some(cp), Some(lp), Some(ap)) => {
                    cores.1.insert(
                        core_number,
                        TempSensorFiles {
                            input: ip.to_path_buf(),
                            max: mp.to_path_buf(),
                            crit: cp.to_path_buf(),
                            label: lp.to_path_buf(),
                            crit_alarm: ap.to_path_buf(),
                        },
                    );
                }
                _ => {
                    println!("Missing data for core {}", core_number);

                    println!(
                        "input: {:?},  max: {:?},  crit: {:?},  label: {:?}, crit_alarm: {:?}",
                        input, max, crit, label, crit_alarm
                    );
                }
            }
        }

        cores.0 = cores.1.len();
        Ok(cores)
    }
}

#[derive(Debug, Clone)]
pub enum HwmType {
    CoreTemp(HwmCoreTemp),
}

#[derive(Debug, Clone)]
pub struct HwmonDir {
    name: String,
    path: PathBuf,
    hwmon_id: u64,
    sensors: HwmType,
}

impl HwmonDir {
    pub fn find_from_sys() -> ah::Result<Vec<HwmonDir>> {
        Self::find_from_dir(HWMON_DIR)
    }

    pub fn find_from_dir(dir_path: &str) -> ah::Result<Vec<HwmonDir>> {
        let mut hwmons: Vec<HwmonDir> = Vec::new();
        let hwmon_dir = Path::new(dir_path);

        for entry in read_dir(hwmon_dir).unwrap() {
            let entry = if let Ok(some_entry) = entry {
                some_entry
            } else {
                println!("Failed to get entry \"{:?}\" in \"{:?}\"", entry, hwmon_dir);
                continue;
            };

            let entry_path = entry.path();

            if entry_path.is_dir() {
                let dir_name = match entry_path.file_name() {
                    Some(name) => name,
                    None => {
                        println!(
                            "No .file_name() for entry path \"{:?}\", of entry \"{:?}\"",
                            entry_path, entry
                        );

                        continue;
                    }
                };

                let dir_name = match dir_name.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        println!(
                            "Invalid UTF-8 \"{:?}\" for entry path \"{:?}\", of entry \"{:?}\"",
                            dir_name, entry_path, entry
                        );

                        continue;
                    }
                };

                let hwmon_id: u64 = if let Some(remaining) = dir_name.strip_prefix(HWMON_PREFIX) {
                    if !remaining.chars().all(char::is_numeric) {
                        println!(
                            "Invalid hwmon directory name \"{:?}\". Expected numeric suffix",
                            dir_name
                        );

                        continue;
                    }

                    match remaining.parse::<u64>() {
                        Ok(id) => id,
                        Err(e) => {
                            println!(
                                "Failed to parse \"{:?}\" as u64 for hwmon \"{:?}\"",
                                remaining, dir_name
                            );

                            continue;
                        }
                    }
                } else {
                    println!("Invalid hwmon directory name \"{:?}\"", dir_name);
                    continue;
                };

                let name_file = entry_path.join(HWMON_NAME_FILE);

                if !name_file.exists() {
                    println!("Missing name file for hwmon \"{:?}\"", dir_name);
                    continue;
                }

                let name = match fs::read_to_string(&name_file) {
                    Ok(name) if name == HWMON_CORE_TEMP_NAME => {
                        let core_temp_sensors = HwmCoreTemp::from_dir(&entry_path)?;

                        hwmons.push(HwmonDir {
                            name,
                            path: entry_path,
                            hwmon_id,
                            sensors: HwmType::CoreTemp(core_temp_sensors),
                        });
                    }

                    Ok(s) => {
                        println!(
                            "Ignoring uncrecognized hwmon/name \"{:?}\", from \"{:?}\"",
                            s, dir_name
                        );
                        continue;
                    }

                    Err(e) => {
                        println!(
                            "Failed to read name file \"{:?}\" for hwmon \"{:?}\"",
                            &name_file, dir_name
                        );

                        continue;
                    }
                };
            }
        }

        Ok(hwmons)
    }
}
