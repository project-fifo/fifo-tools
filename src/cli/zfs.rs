use std::process;
use clap::{App, ArgMatches, SubCommand};
//use serde_json;
//use serde_json::Value;
//use cmd;
use fmt;

pub fn build() -> App<'static, 'static> {
    SubCommand::with_name("zfs")
        .about("Snapshot related commands")
        // TODO
        .subcommand(SubCommand::with_name("list")
                    .about("Lists ZFS data"))
}

pub fn run(matches: &ArgMatches, opts: &fmt::Opts) {
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) => {
            let name = sub.name.as_ref();
            match name {
                "list" =>
                    list(matches, opts),
                other => {
                    println!("Sub command '{}' not implemented for zfs.", other);
                    process::exit(1);
                }
            }
        }
    }
}

fn list(_app: &ArgMatches, _opts: &fmt::Opts) {
}
