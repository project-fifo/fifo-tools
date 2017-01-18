pub mod snapshots;
pub mod backups;
pub mod metadata;

use clap::{App, Arg, SubCommand};
use fmt;

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
        other =>
            println!("Sub command '{}' not implemented.", other)
    }
}
