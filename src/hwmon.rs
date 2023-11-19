use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};

use crate::debug;

const HWMON_DIR: &str = "/sys/class/hwmon";
const HWMON_PREFIX: &str = "hwmon";
const HWMON_NAME_FILE: &str = "name";
const HWMON_CORE_TEMP_NAME: &str = "coretemp";

const CORE_TEMP_PREFIX: &str = "temp";
const CORE_TEMP_SUFFIXES: [&str; 5] = ["input", "max", "crit", "label", "crit_alarm"];

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
    pub fn from_path(path: &Path) -> Option<(CoreNumber, CoreTempDataKind)> {
        let file_name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name_utf8) => name_utf8,
                None => {
                    debug!("Invalid UTF-8 file name: {:?}", name);
                    return None;
                }
            },
            None => {
                debug!("Cannot get filename for file path {:?}", path);
                return None;
            }
        };

        if !file_name.starts_with(CORE_TEMP_PREFIX) {
            debug!(
                "Wrong prefix for \"{:?}\", expecting \"{:?}\"",
                file_name, CORE_TEMP_PREFIX
            );

            return None;
        }

        let xs = match file_name.split_once(CORE_TEMP_PREFIX) {
            Some((prefix, xs)) if prefix.is_empty() => xs.to_string(),
            _ => {
                debug!(
                    "Unable to split by prefix \"{:?}\", for file \"{:?}\"",
                    CORE_TEMP_PREFIX, file_name
                );
                return None;
            }
        };

        let (id, suffix) = match xs.split_once('_') {
            Some((id, suffix)) if CORE_TEMP_SUFFIXES.contains(&suffix) => {
                (id.to_string(), suffix.to_string())
            }
            _ => {
                return {
                    debug!(
                        "Wrong suffix for \"{:?}\", expecting one of \"{:?}\"",
                        path, CORE_TEMP_SUFFIXES
                    );
                    None
                }
            }
        };

        let core_number = match id.parse::<CoreNumber>() {
            Ok(id_num) => id_num,
            Err(e) => {
                debug!("Invalid core number: \"{:?}\" - err \"{:}\"", id, e);
                return None;
            }
        };

        let kind = match suffix.as_str() {
            "input" => CoreTempDataKind::Input,
            "max" => CoreTempDataKind::Max,
            "crit" => CoreTempDataKind::Crit,
            "label" => CoreTempDataKind::Label,
            "crit_alarm" => CoreTempDataKind::CritAlarm,
            _ => {
                debug!("Unknown suffix \"{:?}\", cannot map to data kind.", suffix);
                return None;
            }
        };

        Some((core_number, kind))
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
pub struct HwmCoreTemp(pub u64, pub HashMap<CoreNumber, TempSensorFiles>);

impl HwmCoreTemp {
    pub fn read_median(&self) -> f64 {
        let mut core_temps: Vec<f64> = self
            .read_all()
            .iter()
            .map(|(core_n, &core_t)| core_t)
            .collect();

        core_temps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let count = core_temps.len();
        let middle = count / 2;

        let median: f64;

        if count % 2 == 0 && middle < count {
            median = (core_temps[middle - 1] + core_temps[middle]) / 2.0;
        } else {
            median = core_temps[middle];
        }

        median
    }

    pub fn read_average(&self) -> f64 {
        let core_temperatures = self.read_all();
        let core_count: f64 = core_temperatures.len() as f64;

        let core_temp_sum: f64 = core_temperatures
            .iter()
            .map(|(core_n, &core_t)| core_t)
            .sum();

        core_temp_sum / core_count
    }

    pub fn read_max(&self, core_n: CoreNumber) -> ah::Result<f64> {
        let sensor_files = match self.1.get(&core_n) {
            Some(files) => files,
            None => return Err(ah::anyhow!("No sensor files for core {}", core_n)),
        };

        let string = fs::read_to_string(&sensor_files.max)?;
        let string_trim = string.trim();

        let max: f64 = match string_trim.parse::<f64>() {
            Ok(float) => float,
            Err(e) => {
                return Err(ah::anyhow!(
                    "Failed to parse \"{:?}\" as f64 for core_n {} - err \"{}\", file content: {:?}",
                    core_n,
                    core_n,
                    e,
                    string
                ))
            }
        };

        Ok(max / 1000.0)
    }

    pub fn calculate_max(&self) -> ah::Result<(CoreNumber, f64)> {
        let all = self.read_all();

        let max = all
            .iter()
            .max_by(|&a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
            .ok_or_else(|| ah::anyhow!("Failed to get maximum core temperature for cores: {:?}, because max_by returned None.", all))
            .and_then(|(&core_n, &core_t)| Ok((core_n, core_t)))?;

        Ok(max)
    }

    pub fn calculate_min(&self) -> ah::Result<(CoreNumber, f64)> {
        let all = self.read_all();

        let min = all
            .iter()
            .min_by(|&a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
            .ok_or_else(|| ah::anyhow!("Failed to get the minimum core temperature because min_by returned None, for cores {:?}", all))
            .and_then(|(&core_n, &core_t)| Ok((core_n, core_t)))?;

        Ok(min)
    }

    pub fn read_all(&self) -> HashMap<CoreNumber, f64> {
        self.1
            .iter()
            .filter_map(|(&core_n, core_f)| match self.read_core(core_n) {
                Ok(core_t) => Some((core_n, core_t)),
                Err(e) => {
                    debug!("Error reading temperature for core {}: {:?}", core_n, e);
                    None
                }
            })
            .collect::<HashMap<CoreNumber, f64>>()
    }

    pub fn read_core(&self, core_number: CoreNumber) -> ah::Result<f64> {
        let sensor_files = match self.1.get(&core_number) {
            Some(files) => files,
            None => return Err(ah::anyhow!("No sensor files for core {}", core_number)),
        };

        let input = fs::read_to_string(&sensor_files.input)?;
        let input = input.trim();

        let input = match input.parse::<f64>() {
            Ok(input) => input,
            Err(e) => {
                return Err(ah::anyhow!(
                    "Failed to parse \"{:?}\" as f64 for core {} - err \"{}\"",
                    input,
                    core_number,
                    e
                ))
            }
        };

        let input = input / 1000.0;

        Ok(input)
    }

    pub fn from_dir(path: &Path) -> ah::Result<HwmCoreTemp> {
        let mut cores: HwmCoreTemp = HwmCoreTemp(0, HashMap::new());
        let mut data: HashMap<CoreNumber, Vec<(PathBuf, CoreTempDataKind)>> = HashMap::new();

        for entry in path.read_dir()? {
            let file_path = match entry {
                Ok(entry) if entry.path().is_file() => entry.path(),
                Ok(entry) => {
                    debug!("Skipping non-file entry: {:?}", entry);
                    continue;
                }
                _ => {
                    debug!("Unkonwn error reading entry: {:?}", entry);
                    continue;
                }
            };

            let kind = CoreTempDataKind::from_path(&file_path);

            let (core_number, kind) = match kind {
                Some((n, k)) => (n, k),
                None => {
                    debug!("Skipping unrecognized file: {:?}", file_path);
                    continue;
                }
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
                    debug!("Missing data for core {}", core_number);

                    debug!(
                        "input: {:?},  max: {:?},  crit: {:?},  label: {:?}, crit_alarm: {:?}",
                        input, max, crit, label, crit_alarm
                    );
                }
            }
        }

        cores.0 = cores.1.len() as u64;
        Ok(cores)
    }
}

#[derive(Debug, Clone)]
pub enum HwmType {
    CoreTemp(HwmCoreTemp),
}

#[derive(Debug, Clone)]
pub struct HwmonDir {
    pub name: String,
    pub path: PathBuf,
    pub hwmon_id: u64,
    pub sensors: HwmType,
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
                debug!("Failed to get entry \"{:?}\" in \"{:?}\"", entry, hwmon_dir);
                continue;
            };

            let entry_path = entry.path();

            if entry_path.is_dir() {
                let dir_name = match entry_path.file_name() {
                    Some(name) => name,
                    None => {
                        debug!(
                            "No .file_name() for entry path \"{:?}\", of entry \"{:?}\"",
                            entry_path, entry
                        );

                        continue;
                    }
                };

                let dir_name = match dir_name.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        debug!(
                            "Invalid UTF-8 \"{:?}\" for entry path \"{:?}\", of entry \"{:?}\"",
                            dir_name, entry_path, entry
                        );

                        continue;
                    }
                };

                let hwmon_id: u64 = if let Some(remaining) = dir_name.strip_prefix(HWMON_PREFIX) {
                    if !remaining.chars().all(char::is_numeric) {
                        debug!(
                            "Invalid hwmon directory name \"{:?}\". Expected numeric suffix",
                            dir_name
                        );

                        continue;
                    }

                    match remaining.parse::<u64>() {
                        Ok(id) => id,
                        Err(_e) => {
                            debug!(
                                "Failed to parse \"{:?}\" as u64 for hwmon \"{:?}\"",
                                remaining, dir_name
                            );

                            continue;
                        }
                    }
                } else {
                    debug!("Invalid hwmon directory name \"{:?}\"", dir_name);
                    continue;
                };

                let name_file = entry_path.join(HWMON_NAME_FILE);

                if !name_file.exists() {
                    debug!("Missing name file for hwmon \"{:?}\"", dir_name);
                    continue;
                }

                match fs::read_to_string(&name_file) {
                    Ok(name) if name.trim() == HWMON_CORE_TEMP_NAME => {
                        let core_temp_sensors = HwmCoreTemp::from_dir(&entry_path)?;

                        hwmons.push(HwmonDir {
                            name: name.trim().into(),
                            path: entry_path,
                            hwmon_id,
                            sensors: HwmType::CoreTemp(core_temp_sensors),
                        });
                    }

                    Ok(s) => {
                        debug!(
                            "Ignoring uncrecognized hwmon/name \"{:?}\", from \"{:?}\"",
                            s, dir_name
                        );
                        continue;
                    }

                    Err(_e) => {
                        debug!(
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
