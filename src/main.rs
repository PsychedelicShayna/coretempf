









mod args;
mod hwmon;

use args::*;
use hwmon::*;



fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    let (_mode, _flags) = parse_args(arguments);

    let hwmons = match HwmonDir::find_from_sys() {
        Ok(hwmons) => hwmons,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("{:?}", hwmons);

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
