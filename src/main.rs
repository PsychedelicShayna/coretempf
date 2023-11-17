use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::{self, read_dir, DirEntry, File, FileType, OpenOptions, ReadDir};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

mod args;
mod hwmon;

use args::*;
use hwmon::*;

use anyhow as ah;

fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    let (mode, flags) = parse_args(arguments);

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
