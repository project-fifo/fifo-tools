pub mod snapshots;
pub mod backups;
pub mod metadata;

use clap::{App, Arg, SubCommand};
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

pub fn run(cmd: &SubCommand) {
    let name = cmd.name.as_ref();
    match name {
        "snapshots" => {
            snapshots::run(&cmd.matches)
        },
        "backups" => {
            backups::run(&cmd.matches)
        },
        "metadata" => {
            metadata::run(&cmd.matches)
        },
        other =>
            println!("Sub command '{}' not implemented.", other)
    }
}
