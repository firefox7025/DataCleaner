extern crate clap;
use clap::{Arg, App};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Image Sorter")
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

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let logfile = matches.value_of("config").unwrap_or("log.txt");
    println!("Value for config: {}", logfile);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}
