use anyhow as ah;
use std::collections::HashMap;
use std::fs;

use std::path::{PathBuf};

const SYSFS_HWMON: &str = "/sys/class/hwmon";

#[derive(Debug, Clone, Copy)]
enum Identifier {
    Core(u64),
    Package,
}

struct CoreSensor {
    label_id: Identifier,
    temp_id: u64,
    temp_label: PathBuf,
    temp_input: PathBuf,
    temp_crit: PathBuf,
    temp_crit_alarm: PathBuf,
}

impl CoreSensor {
    fn read_label(&self) -> ah::Result<String> {
        let label = fs::read_to_string(&self.temp_label)?;
        Ok(label.trim().to_string())
    }

    fn read_input(&self) -> ah::Result<u64> {
        let input = fs::read_to_string(&self.temp_input)?;
        Ok(input.trim().parse::<u64>()?)
    }

    fn read_crit(&self) -> ah::Result<u64> {
        let crit = fs::read_to_string(&self.temp_crit)?;
        Ok(crit.trim().parse::<u64>()?)
    }

    fn read_crit_alarm(&self) -> ah::Result<u64> {
        let crit_alarm = fs::read_to_string(&self.temp_crit_alarm)?;
        Ok(crit_alarm.trim().parse::<u64>()?)
    }
}

pub struct CoreTemp {
    package: CoreSensor,
    cores: HashMap<u64, CoreSensor>,
}

impl CoreTemp {
    pub fn get_package(&self) -> ah::Result<u64> {
        self.package.read_input()
    }

    pub fn get_cores(&self) -> Vec<u64> {
        self.cores.keys().copied().collect()
    }

    pub fn get_count(&self) -> usize {
        self.cores.len()
    }

    pub fn get_temp(&self, core: u64) -> ah::Result<u64> {
        let core = self
            .cores
            .get(&core)
            .ok_or(ah::anyhow!("Core {} does not exist", core))?;

        core.read_input()
    }

    pub fn get_critical(&self, core: u64) -> ah::Result<u64> {
        let core = self
            .cores
            .get(&core)
            .ok_or(ah::anyhow!("Core {} does not exist", core))?;

        core.read_crit()
    }

    pub fn get_critical_alarm(&self, core: u64) -> ah::Result<u64> {
        let core = self
            .cores
            .get(&core)
            .ok_or(ah::anyhow!("Core {} does not exist", core))?;

        core.read_crit_alarm()
    }

    pub fn get_temps_for(&self, cores: &Vec<u64>) -> ah::Result<Vec<u64>> {
        let mut temps = Vec::new();
        for core_n in cores {
            temps.push(self.get_temp(*core_n)?);
        }
        Ok(temps)
    }

    pub fn get_average(&self) -> ah::Result<u64> {
        let mut sum: u64 = 0;

        for (_, core) in &self.cores {
            sum += core.read_input()?;
        }

        Ok(sum / self.cores.len() as u64)
    }

    pub fn get_median(&self) -> ah::Result<u64> {
        let mut temperatures: Vec<u64> = Vec::with_capacity(self.cores.len());

        for (_, core) in &self.cores {
            let temperature = core.read_input()?;
            temperatures.push(temperature);
        }

        temperatures.sort();

        let median = if temperatures.len() % 2 == 0 {
            let center = temperatures.len() / 2;

            (temperatures[center] + temperatures[center + 1]) / 2
        } else {
            temperatures[temperatures.len() / 2]
        };

        Ok(median)
    }

    pub fn get_min(&self) -> ah::Result<u64> {
        let mut min = u64::MAX;

        for (_, core) in &self.cores {
            let temp = core.read_input()?;

            if temp < min {
                min = temp;
            }
        }

        Ok(min)
    }

    pub fn get_max(&self) -> ah::Result<u64> {
        let mut max = u64::MIN;

        for (_, core) in &self.cores {
            let temp = core.read_input()?;

            if temp > max {
                max = temp;
            }
        }

        Ok(max)
    }

    pub fn try_new() -> ah::Result<CoreTemp> {
        let hwmon_dirs = fs::read_dir(SYSFS_HWMON)?;

        let mut package: Option<CoreSensor> = None;
        let mut cores = HashMap::<u64, CoreSensor>::new();

        for dir in hwmon_dirs {
            let dir = match dir {
                Ok(dir) => dir,
                Err(_) => continue,
            };

            let dir_path = dir.path();
            let name_file = dir_path.join("name");

            // Skip directories with no name file.
            if !name_file.exists() {
                continue;
            }

            let name = match fs::read_to_string(&name_file) {
                Ok(name) => name,
                Err(_) => continue,
            };

            // Skip directories without a name file containing "coretemp"
            if name.trim() != "coretemp" {
                continue;
            }

            let entries = fs::read_dir(&dir_path)?;
            let mut temp_ids_seen = Vec::<u64>::new();

            // Try to collect all tempN_xyz files for unque N, into the map,
            // where xyz is label, input, crit, crit_alarm. Do this by looking
            // for tempN_label files, and using its N to infer the paths of
            // input, crit, and crit_alarm files.

            for entry in entries.flatten() {
                let entry_path = entry.path();

                let entry_name = match entry_path.file_name() {
                    Some(name) => match name.to_str() {
                        Some(as_str) => as_str,
                        None => continue,
                    },
                    None => continue,
                };

                let candidate = entry_name.starts_with("temp")
                    && entry_name.ends_with("_label")
                    && entry_name.len() > 10;

                if !candidate {
                    continue;
                }

                // Range start is inclusive, range end is exclusive, so - 6.
                let temp_id = &entry_name[4..entry_name.len() - 6];

                if !temp_id.chars().all(|c| c.is_ascii_digit()) {
                    continue;
                }

                let temp_id = match temp_id.parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => continue,
                };

                if temp_ids_seen.contains(&temp_id) {
                    continue;
                }

                let label = fs::read_to_string(&entry_path)?.trim().to_lowercase();

                let label_id: Identifier;

                if label.starts_with("package") {
                    label_id = Identifier::Package;
                } else if label.starts_with("core") {
                    let core_n = match label
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<u64>()
                    {
                        Ok(n) => n,
                        Err(_) => continue,
                    };

                    label_id = Identifier::Core(core_n);
                } else {
                    continue;
                }

                let temp_label = entry_path.clone();

                let temp_input = entry_path
                    .clone()
                    .with_file_name(format!("temp{}_input", temp_id));

                let temp_crit = entry_path
                    .clone()
                    .with_file_name(format!("temp{}_crit", temp_id));

                let temp_crit_alarm = entry_path
                    .clone()
                    .with_file_name(format!("temp{}_crit_alarm", temp_id));

                if !temp_input.exists() || !temp_crit.exists() || !temp_crit_alarm.exists() {
                    continue;
                }

                let sensor = CoreSensor {
                    label_id,
                    temp_id,
                    temp_label,
                    temp_input,
                    temp_crit,
                    temp_crit_alarm,
                };

                match label_id {
                    Identifier::Core(core_n) => {
                        if cores.contains_key(&core_n) {
                            continue;
                        }

                        cores.insert(core_n, sensor);
                    }

                    Identifier::Package => {
                        if package.is_some() {
                            continue;
                        }

                        package = Some(sensor);
                    }
                }

                temp_ids_seen.push(temp_id);
            }
        }

        match package {
            Some(package) => Ok(CoreTemp { package, cores }),
            None => Err(ah::anyhow!("No package sensor found")),
        }

    }
}
