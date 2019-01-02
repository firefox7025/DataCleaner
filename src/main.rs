extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate img_hash;

mod image_finders;
mod hasher;

use std::path::Path;
use std::path::PathBuf;
use clap::{Arg, App};
use std::fs;
use simplelog::*;
use std::fs::File;
use img_hash::HashType;
use rayon::prelude::*;


const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");


struct ImageHashCombo {
    hash: img_hash::ImageHash,
    path: PathBuf,
}

fn main() {
    let matches = App::new(NAME)
                          .version(VERSION)
                          .author("Alexander Montgomery <alexander.montgomery@ultimaengineering.io>")
                          .about("Sorts data into common folders via Hashes")
                          .arg(Arg::with_name("logFile")
                               .short("l")
                               .long("logFile")
                               .value_name("FILE")
                               .help("Sets a custom log file")
                               .takes_value(true))
        .arg(Arg::with_name("INPUT")
             .help("Sets the input folder to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("Sets the output folder to use")
             .required(true)
             .index(2))
        .arg(Arg::with_name("v")
              .short("v")
              .multiple(true)
              .help("Sets the margin of error for docs to be sorted together"))
        .get_matches();

    let _logfile = matches.value_of("logFile").unwrap_or("log.txt");
    let _output_path  = Path::new(matches.value_of("OUTPUT").unwrap());
    let _input_path = Path::new(matches.value_of("INPUT").unwrap());
    let _hash_type = determine_hash(&matches);
    create_log_file(&_logfile);
    info!("Arguments parsed input location {:?}, output location {:?}, hashtype {:?}", &_input_path, &_output_path, &_hash_type);
    let _output_pre_check = prep_output(_output_path);

    // more program logic goes here..
    let images = image_finders::find_images(_input_path);
    let imageHashCombos: Vec<_> = images.par_iter()
        .map(|e| ImageHashCombo { hash: hasher::hash_image(e.to_path_buf(), _hash_type), path: e.to_path_buf()})
        .collect();
    info!("Completed hashing.");
    println!("Checking the number of imageHashCombos {}", imageHashCombos.len());
}

fn determine_hash(args: &clap::ArgMatches) -> img_hash::HashType {
    match args.occurrences_of("v") {
        0 => HashType::Mean,
        1 => HashType::Gradient,
        2 => HashType::DCT,
        3 | _ => HashType::DCT,
    }
}

fn prep_output(output: &Path) -> std::io::Result<()> {
    fs::create_dir_all(output)?;
    Ok(())
}

fn create_log_file(log: &str) {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(log).unwrap()),
        ]
    ).unwrap();
}
