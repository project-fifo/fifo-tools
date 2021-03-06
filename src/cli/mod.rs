use std::process;
use clap::{App, Arg, SubCommand};
use fmt;

pub mod snapshots;
pub mod backups;
pub mod metadata;
pub mod cluster;
pub mod stack;
pub mod zfs;
use std::io;
use std::io::Write;


pub fn build() -> App<'static, 'static> {
    App::new("fifo")
        .about("FiFo zone tools")
    //        .version(crate_version!())
        .arg(Arg::with_name("json")
             .short("j")
             .long("json")
             .help("Return results as json"))
        .subcommand(snapshots::build())
        .subcommand(backups::build())
        .subcommand(metadata::build())
        .subcommand(cluster::build())
        .subcommand(stack::build())
        .subcommand(zfs::build())
}

pub fn run(cmd: &SubCommand, opts: &fmt::Opts) {
    let name = cmd.name.as_ref();
    match name {
        "snapshots" => {
            snapshots::run(&cmd.matches, opts)
        },
        "backups" => {
            backups::run(&cmd.matches, opts)
        },
        "metadata" => {
            metadata::run(&cmd.matches, opts)
        },
        "cluster" => {
            cluster::run(&cmd.matches, opts)
        },
        "stack" => {
            stack::run(&cmd.matches, opts)
        },
        "zfs" => {
            zfs::run(&cmd.matches, opts)
        },
        other => {
            writeln!(io::stderr(), "Sub command '{}' not implemented.", other).unwrap();
            process::exit(1);
        }

    }
}
