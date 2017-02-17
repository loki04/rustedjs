extern crate librustedjs;
use librustedjs::js_run;

use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;

extern crate getopts;
use getopts::Options;

// Basic usage information
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE [FILE ...]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_arguments() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // List of the command line options
    opts.optflag("h", "help", "Print this help screen.");
    opts.optflag("t", "test", "Execute basic tests.");

    // Creates matches for command line options
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Check if any command line option presents
    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0);
    }

    let files: Vec<String>;

    if matches.opt_present("t") {
        // Execute the basic tests
        files = vec![
            String::from("tests/example/test1.js"),
            String::from("tests/example/test2.js")];
    } else if !matches.free.is_empty() {
        // Use command line arguments
        files = matches.free;
    } else {
        // No javascript source has been added
        print_usage(&program, opts);
        process::exit(1);
    }
    files
}

// Load all command line arguments as a javascript text file
fn load_sources(files: Vec<String>) -> String {
    let mut javascript_source = String::new();
    for arg in &files {
        let mut file = match File::open(arg) {
            Err(why) => panic!("Couldn't open '{}': {}", arg, why.description()),
            Ok(file) => file,
        };
        let mut next_source = String::new();
        match file.read_to_string(&mut next_source) {
            Err(why) => panic!("Couldn't read '{}': {}", arg, why.description()),
            Ok(_) => javascript_source += &next_source,
        };
    }
    javascript_source
}

// Main entry point
fn main() {
    let files: Vec<String> = parse_arguments();

    let javascript_source = load_sources(files);

    js_run(javascript_source);
}
