extern crate rustc_serialize;
extern crate serde_json;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate prettytable;

mod cli;
mod cmd;
mod fmt;

fn main() {
    let matches = cli::build().get_matches();
    let opts = fmt::Opts{json: matches.is_present("json"), fields: vec![]};
    //    println!("matches = {:?}", matches);
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) =>
            cli::run(&sub, &opts)
    }
}
