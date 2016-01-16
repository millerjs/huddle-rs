#![allow(dead_code)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;
extern crate ncurses;
extern crate getopts;
extern crate rustc_serialize;

mod letters;
mod term;
mod parser;
mod colors;

use term::*;
use ncurses::*;
use parser::load_config_file;
use std::env;

fn main() {

    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify path to config");
    }
    let mut canvas = load_config_file(args[1].clone());
    timeout(canvas.delay);
    while getch() != 'q' as i32 {
        canvas.refresh();
    }
    println!("Exited cleanly.")

}
