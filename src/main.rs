#[macro_use]
mod debug;

mod temps;
use temps::*;

mod units;
use units::*;

mod help;
use help::exit_with_usage;

use std::collections::HashMap;

use anyhow as ah;

fn parse_args() -> Vec<(String, Vec<String>)> {
    let args: Vec<String> = std::env::args().collect();
    let args_with_index: Vec<(usize, String)> = args.into_iter().enumerate().collect();

    let (key_args, val_args): (Vec<(usize, String)>, Vec<(usize, String)>) =
        args_with_index.into_iter().partition(|(_, arg)| {
            (arg.starts_with("--")
                && arg
                    .strip_prefix("--")
                    .is_some_and(|s| !s.contains("--") && s.len() == arg.len() - 2))
                || ((arg.len() == 2 || arg.len() == 3) && arg.starts_with('-'))
        });

    let mut argument_pairs: Vec<(String, Vec<String>)> = Vec::with_capacity(key_args.len());

    let _argument_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut key_args_iter = key_args.into_iter().peekable();

    while let Some((karg_idx, karg)) = key_args_iter.next() {
        let next_karg_idx = key_args_iter.peek().map(|(i, _)| i);

        let karg_values = val_args
            .iter()
            .filter_map(|(varg_idx, varg)| {
                let before_next_karg = if let Some(next_karg_idx) = next_karg_idx {
                    varg_idx < next_karg_idx
                } else {
                    // No next karg index, remaining vargs belong to this karg.
                    true
                };

                // Must be after the current karg's index.
                let after_this_karg = varg_idx > &karg_idx;

                (after_this_karg && before_next_karg).then_some(varg.into())
            })
            .collect();

        argument_pairs.push((karg, karg_values));
        // argument_map.insert(karg.clone()into(), karg_values);
    }

    argument_pairs
}

fn process_segments(ct: &CoreTemp, argm: Vec<(String, Vec<String>)>) -> ah::Result<String> {
    struct FormatSettings {
        pub include_glyph: bool,
        pub base_unit: Unit,
        pub target_unit: Option<Unit>,
    }

    let mut fmts = FormatSettings {
        include_glyph: false,
        base_unit: Unit::Celcius,
        target_unit: None,
    };

    fn print_temp(temp: Option<u64>, settings: &FormatSettings) -> String {
        let temp = match temp {
            Some(temp) => temp,
            None => return "N/A".into(),
        };

        let mut final_value: f64 = (temp as f64) / 1000.0;

        if let Some(target_unit) = &settings.target_unit {
            final_value = settings.base_unit.convert_to(target_unit, final_value);
        }

        let glyph = if settings.include_glyph {
            settings.base_unit.to_str_glyph()
        } else {
            ""
        };

        format!("{:.2}{}", final_value, glyph)
    }

    let mut output = String::new();

    for (key, values) in argm {
        // Arguments that require no values.
        match key.as_str() {
            "--glyph" | "-g" => output += fmts.base_unit.to_str_glyph(),
            "--avg" | "-av" => {
                let average = ct.get_average()?;
                output += &print_temp(Some(average), &fmts)
            }
            "--median" | "-md" => {
                let median = ct.get_median()?;
                output += &print_temp(Some(median), &fmts)
            }
            "--newline" | "-nl" | "-\\n" | "-cr" => {
                output += "\n";
            }
            "--strings" | "-s" => output += &values.join(" ").to_string(),
            "--temp-min" | "-tm" => {
                let min = ct.get_min()?;
                output += &print_temp(Some(min), &fmts)
            }
            "--temp-max" | "-tx" => {
                let max = ct.get_max()?;
                output += &print_temp(Some(max), &fmts)
            }
            "--temp-package" | "-tp" => {
                let package = ct.get_package()?;
                output += &print_temp(Some(package), &fmts)
            }
            "--core-count" | "-cc" => {
                let core_count = ct.get_count();
                output += &format!("{}", core_count).to_string()
            }

            _ => (),
        }

        // Arguments after first are arguments that require at least 1 value.
        // If there is no first value, skip the argument.
        let first = match values.first() {
            Some(first) => first,
            None => continue,
        };

        match (key.as_str(), first.as_str()) {
            ("--base-unit" | "-bu", first) => {
                if let Some(unit) = Unit::from_str(first) {
                    fmts.base_unit = unit;
                }
            }

            ("--target-unit" | "-tu", first) => {
                if let Some(unit) = Unit::from_str(first) {
                    fmts.target_unit = Some(unit);
                }
            }

            ("--use-glyph" | "-ug", "true" | "yes" | "y" | "on") => {
                fmts.include_glyph = true;
            }

            ("--use-glyph" | "-ug", "false" | "no" | "n" | "off") => {
                fmts.include_glyph = false;
            }

            ("--temp" | "-t", first) => {
                let cores: Vec<u64> = if matches!(first, "all" | "*") {
                    (0..ct.get_count() as u64).collect()
                } else {
                    values
                        .iter()
                        .filter_map(|core| core.parse::<u64>().ok())
                        .collect()
                };

                for (i, core) in cores.iter().enumerate() {
                    let temp = print_temp(ct.get_temp(*core).ok(), &fmts);

                    if i != cores.len() - 1 {
                        output += &format!("{}, ", temp);
                    } else {
                        output += &temp.to_string();
                    }
                }
            }
            ("--core-critical" | "-cC", _) => {
                let cores: Vec<u64> = values
                    .iter()
                    .filter_map(|core| core.parse::<u64>().ok())
                    .collect();

                for (i, core) in cores.iter().enumerate() {
                    let crit = print_temp(ct.get_critical(*core).ok(), &fmts);

                    if i != cores.len() - 1 {
                        output += &format!("{}, ", crit);
                    } else {
                        output += &crit.to_string();
                    }
                }
            }

            ("--core-alarm" | "-ca", _) => {
                let cores: Vec<u64> = values
                    .iter()
                    .filter_map(|core| core.parse::<u64>().ok())
                    .collect();

                for (i, core) in cores.iter().enumerate() {
                    let alarm = match ct.get_critical_alarm(*core) {
                        Ok(alarm) => match alarm {
                            0 => "false".to_string(),
                            1 => "true".to_string(),
                            _ => "N/A/".to_string(),
                        },

                        Err(_) => "N/A".to_string(),
                    };

                    if i != cores.len() - 1 {
                        output += &format!("Core {}: {}, ", i + 1, alarm);
                    } else {
                        output += &format!("Core {}: {}", i + 1, alarm);
                    }
                }
            }

            _ => (),
        }
    }

    Ok(output)
}

fn main() {
    let arguments = parse_args();

    if arguments.is_empty() {
        exit_with_usage(1);
    }

    if arguments
        .iter()
        .any(|(key, _)| matches!(key.as_str(), "--help" | "-h"))
    {
        exit_with_usage(0)
    }

    let core_temp = match CoreTemp::try_new() {
        Ok(core_temp) => core_temp,
        Err(e) => {
            eprintln!("Hwmon error: {}", e);
            std::process::exit(1);
        }
    };

    let output = match process_segments(&core_temp, arguments) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Segment parser error: {}", e);
            std::process::exit(1);
        }
    };

    println!("{}", output);
}
