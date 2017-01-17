use clap::{App, Arg, SubCommand, ArgMatches};
use cmd;

pub fn snapshot_cli() -> App<'static, 'static> {
    SubCommand::with_name("snapshots")
        .about("Snapshot related commands")
        .subcommand(SubCommand::with_name("list")
                    .about("Lists all snapshots"))
        .subcommand(SubCommand::with_name("create")
                    .about("Creates a snapshot")
                    .arg(Arg::with_name("COMMENT")
                         .required(true)
                         .index(1)))

        // .arg(Arg::with_name("totest")
        //      .long("thing")
        //      .help("The thing to test")
        //              .takes_value(true)
        //      .possible_values(&["thing1", "other-thing", "stuff"]))
}

pub fn run(matches: &ArgMatches) {
    println!("run: {:?}", matches);
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) => {
            let name = sub.name.as_ref();
            match name {
                "list" => {
                    list(&sub.matches)
                },
                "create" => {
                    create(&sub.matches)
                },
                other =>
                    println!("Sub command '{}' not implemented for snapshots.", other)
            }
        }
    }
}

fn list(_app: &ArgMatches) {
    let value = cmd::run_generic("zfs-list".to_string());
    println!("deserialized = {:?}", value);
    println!("action = {:?}", value.pointer("/action").unwrap());

}


#[derive(RustcEncodable)]
struct SnapshotCreateReq {
    action: String,
    comment: String
}
fn create(matches: &ArgMatches) {
    let comment = matches.args["COMMENT"].vals[1].to_string_lossy().into_owned();
    let req = SnapshotCreateReq{
        action:  "snapshot-create".to_string(),
        comment: comment
    };
    let res = cmd::run(req);
    println!("create: {:?}", res)
}
