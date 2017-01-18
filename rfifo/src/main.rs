extern crate rustc_serialize;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;

mod cli;
mod cmd;

fn main() {
    let matches = cli::build().get_matches();
//    println!("matches = {:?}", matches);
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) =>
            cli::run(&sub)
    }
}
