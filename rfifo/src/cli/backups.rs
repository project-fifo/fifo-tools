use clap::{App, Arg, SubCommand, ArgMatches};
use cmd;

pub fn build() -> App<'static, 'static> {
    SubCommand::with_name("backups")
        .about("Backup related commands")
        .subcommand(SubCommand::with_name("list")
                    .about("Lists all backups")
                    .arg(Arg::with_name("format")
                         .long("fmt")
                         .value_name("FMT")
                         .help("Fields to be shown")))
        .subcommand(SubCommand::with_name("create")
                    .about("Creates a backup")
                    .arg(Arg::with_name("COMMENT")
                         .required(true)
                         .index(1)))
}

pub fn run(matches: &ArgMatches) {
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
                    println!("Sub command '{}' not implemented for backups.", other)
            }
        }
    }
}

fn list(_app: &ArgMatches) {
    let value = cmd::run_generic("backups-list".to_string());
    println!("deserialized = {:?}", value);
}


#[derive(RustcEncodable)]
struct BackupCreateReq {
    action: String,
    comment: String
}
fn create(matches: &ArgMatches) {
    let comment = value_t!(matches, "COMMENT", String).unwrap();
    let req = BackupCreateReq{
        action:  "backup-create".to_string(),
        comment: comment
    };
    let res = cmd::run(req);
    println!("create: {:?}", res)
}
