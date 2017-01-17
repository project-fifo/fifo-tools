use clap::{App, SubCommand};
pub fn metadata_cli() -> App<'static, 'static> {
    SubCommand::with_name("metadata")
        .about("Snapshot related commands")
        .subcommand(SubCommand::with_name("list")
                    .about("Lists metadata"))
        .subcommand(SubCommand::with_name("create")
                    .about("Creates snapshot"))

        // .arg(Arg::with_name("totest")
        //      .long("thing")
        //      .help("The thing to test")
        //              .takes_value(true)
        //      .possible_values(&["thing1", "other-thing", "stuff"]))
}
