mod debug;
mod hwmon;

use debug::*;
use hwmon::*;

pub enum AuxiliaryOptions {
    Cores(Vec<CoreNumber>),
    WithUnitGlyph,
    Celcius,
    Farenheit,
    Kelvin,
}

pub enum ExclusiveOptions {
    List,
    Min,
    Max,
    Average,
    Median,
    Help,
}

mod segments {
    use crate::hwmon::*;
    use anyhow as ah;

    pub fn raw_core(core_temp: &HwmCoreTemp, core_number: CoreNumber) -> ah::Result<String> {
        let temp = core_temp.read_core(core_number)?;
        Ok(format!("{}", temp))
    }

    pub fn raw_average(core_temp: &HwmCoreTemp) -> ah::Result<String> {
        let avg = core_temp.read_average();
        Ok(format!("{}", avg))
    }

    pub fn raw_median(core_temp: &HwmCoreTemp) -> ah::Result<String> {
        let median = core_temp.read_median();
        Ok(format!("{}", median))
    }

    pub fn raw_calc_min(core_temp: &HwmCoreTemp) -> ah::Result<String> {
        let (_, min) = core_temp.calculate_min()?;
        Ok(format!("{}", min))
    }

    pub fn raw_calc_max(core_temp: &HwmCoreTemp) -> ah::Result<String> {
        let (_, max) = core_temp.calculate_max()?;
        Ok(format!("{}", max))
    }

    pub fn raw_max(core_temp: &HwmCoreTemp, core_n: CoreNumber) -> ah::Result<String> {
        let max = core_temp.read_max(core_n)?;
        Ok(format!("Core {} Max: {}", core_n, max))
    }



}

// pub fn print_segment_raw(core_temp: &CoreTemp, core_number: CoreNumber) {
//     // let temp = core_temp.read_temp_input(core_number).unwrap_or(0);
//     // println!("{}", temp);
//     todo!();
// }

pub fn print_temp(main_mode: ExclusiveOptions, options: Vec<AuxiliaryOptions>) {
    //
    // let hwmons: Vec<HwmonDir> = match HwmonDir::find_from_sys() {
    //     Ok(hwmons) => {
    //         let core_temp_hwmons = hwmons.iter().filter(|hwmon| hwmon.name == "coretemp");
    //
    //         let mut core_temp: Option<HwmonDir> = None;
    //
    //         for hwmon in core_temp_hwmons {
    //             if let HwmType::CoreTemp(_) = hwmon.sensors {
    //                 core_temp = Some(hwmon.clone());
    //                 break;
    //             }
    //         }
    //
    //         let core_temp = match core_temp {
    //             Some(core_temp) => core_temp,
    //             None => {
    //                 eprintln!("Error: no coretemp hwmon found");
    //                 std::process::exit(1);
    //             }
    //         };
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // };
}

// pub fn parse_args(args: Vec<String>) -> (Mode, Vec<Flag>) {
// todo!()
// let mut display_options: Vec<DisplayOptions> = Vec::new();
//
// let mut options: Vec<OutOpts> = Vec::new();
//
// for (arg, narg) in args.iter().zip((1..=args.len()).map(|i| args.get(i))) {
//     if mode.is_none() {
//         let arg = arg.as_str();
//
//         match arg {
//             "-a" | "--all" => mode = Some(Mode::All),
//             "-m" | "--max" => mode = Some(Mode::Max),
//             "-n" | "--min" => mode = Some(Mode::Min),
//             "-l" | "--list" => mode = Some(Mode::List),
//             "-h" | "--help" => mode = Some(Mode::Help),
//             "-v" | "--verbose" => flags.push(Flag::Verbose),
//             "-d" | "--debug" => flags.push(Flag::Debug),
//             _ => {
//                 eprintln!("Error: unrecognized argument {:?}", arg);
//                 std::process::exit(1);
//             }
//         }
//
//         match (arg, narg) {
//             ("-c" | "--core", Some(next_argument)) => {
//                 let core_number = match next_argument.parse::<CoreNumber>() {
//                     Ok(core_number) => core_number,
//                     Err(error) => {
//                         eprintln!(
//                             "Error: invalid core number {:?}, {:?}",
//                             next_argument, error
//                         );
//
//                         std::process::exit(1);
//                     }
//                 };
//
//                 mode = Some(Mode::Core(core_number));
//             }
//             _ => (),
//         }
//     }
// }
//
// }

fn main() {
    // let arguments = std::env::args().collect::<Vec<String>>();
    // let (_mode, _flags) = parse_args(arguments);

    // let hwmons = match HwmonDir::find_from_sys() {
    //     Ok(hwmons) => hwmons,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         std::process::exit(1);
    //     }
    // };
    //
    // println!("{:?}", hwmons);
    //
    // for hwmon in hwmons {
    //     if let HwmType::CoreTemp(core_temp) = hwmon.sensors {
    //         let cores = core_temp.0;
    //
    //         for i in (1..=cores) {
    //             if let Ok(temp) = core_temp.read_temp_input(i) {
    //                 println!("Core {}: {}", i, temp);
    //             } else {
    //                 println!("Core {}: N/A", i);
    //             }
    //         }
    //     }
    // }

    // match mode {
    //     Mode::All => todo!(),
    //     Mode::Max => todo!(),
    //     Mode::Min => todo!(),
    //     Mode::List => todo!(),
    //     Mode::Core(n) => todo!(),
    //     Mode::Version => todo!(),
    //     Mode::Help => todo!(),
    // }
}
