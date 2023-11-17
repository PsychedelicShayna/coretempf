use crate::hwmon::*;
use anyhow as ah;

pub enum Mode {
    All,
    Max,
    Min,
    List,
    Core(CoreNumber),
    Version,
    Help,
}

pub enum Flag {
    Verbose,
    Debug,
}

pub fn parse_args(args: Vec<String>) -> (Mode, Vec<Flag>) {
    let mut mode = Mode::All;
    let mut flags: Vec<String> = Vec::new();

    let mut mode: Option<Mode> = None;

    // ("-V" | "--verbose", _) => flags.push(Flag::Verbose),
    // ("-D" | "--debug", _) => flags.push(Flag::Debug),

    for (arg, narg) in args.iter().zip((1..=args.len()).map(|i| args.get(i))) {
        if let None = mode {
            let arg = arg.as_str();

            match arg {
                "-a" | "--all" => mode = Some(Mode::All),
                "-m" | "--max" => mode = Some(Mode::Max),
                "-n" | "--min" => mode = Some(Mode::Min),
                "-l" | "--list" => mode = Some(Mode::List),
                "-v" | "--version" => mode = Some(Mode::Version),
                "-h" | "--help" => mode = Some(Mode::Help),
                _ => (),
            }

            match (arg, narg) {
                ("-c" | "--core", Some(next_argument)) => {
                    // let core_list = next_argument
                    //     .split(",")
                    //     .map(|s| {
                    //         s.parse::<u32>()
                    //             .map_err(|e| ah::anyhow!("{}", e))
                    //             .and_then(|i| {
                    //                 core_exists(i)
                    //                     .then_some(i)
                    //                     .ok_or_else(|| ah::anyhow!("Invalid core: {}", i))
                    //             })
                    //     })
                    //     .collect::<Vec<Result<u32, Error>>>();

                    // for result in results {
                    //     if let (Err((core, err))) = result {
                    //         eprintln!("Invalid core: {} ({})", core, err);
                    //     }
                    //
                    //     eprintln!("Invalid core: {} ({})", core, err);
                    // }

                    todo!()
                }
                _ => (),
            }
        }
    }

    todo!();
}
