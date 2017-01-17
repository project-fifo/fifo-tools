pub mod snapshots;
pub mod metadata;

use clap::{App, Arg, SubCommand};
pub fn build() -> App<'static, 'static> {
    App::new("fifo")
        .arg(Arg::with_name("json")
             .short("j")
             .long("json")
             .help("Return results as json"))
        .subcommand(snapshots::snapshot_cli())
        .subcommand(metadata::metadata_cli())
}

pub fn run(cmd: &SubCommand) {
    println!("subcommand = {:?}", cmd);
    let name = cmd.name.as_ref();
    match name {
        "snapshots" => {
            snapshots::run(&cmd.matches)
        },
        other =>
            println!("Sub command '{}' not implemented.", other)
    }
}
