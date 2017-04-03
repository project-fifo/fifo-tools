extern crate rustc_serialize;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;
#[cfg(target_os = "solaris")]
extern crate libc;

mod cli;
mod cmd;
mod fmt;
mod door;

fn main() {
    let matches = cli::build().get_matches();
    let opts = fmt::Opts{json: matches.is_present("json"), format: vec![]};
    //    println!("matches = {:?}", matches);
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) =>
            cli::run(&sub, &opts)
    }
}
