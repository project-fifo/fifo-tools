use std::process;
use clap::{App, Arg, SubCommand, ArgMatches};
use serde_json;
use serde_json::Value;
use cmd;
use fmt;
use std::io;
use std::io::Write;

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
                    // TODO
                    // --delete
                    // --parent
                    .arg(Arg::with_name("comment")
                         .value_name("COMMENT")
                         .required(true)
                         .index(1)))
}

pub fn run(matches: &ArgMatches, opts: &fmt::Opts) {
    match matches.subcommand {
        None =>
            println!("help"),
        Some(ref sub) => {
            let name = sub.name.as_ref();
            match name {
                "list" => {
                    list(&sub.matches, opts)
                },
                "create" => {
                    create(&sub.matches)
                },
                other => {
                    writeln!(io::stderr(), "Sub command '{}' not implemented for backups.", other).unwrap();
                    process::exit(1);
                }
            }
        }
    }
}

fn list(matches: &ArgMatches, opts: &fmt::Opts) {
    let fields =  vec![
        fmt::Field{
            title: "UUID",
            short: "uuid",
            default: true,
            get: Box::new(|x| { x.lookup("uuid").unwrap().as_str().unwrap().to_string() })
        },
        fmt::Field{
            title: "Parent",
            short: "parent",
            default: true,
            get: Box::new(|x| {
                match x.lookup("parent") {
                    None => "-".to_string(),
                    value => {
                        value
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string()
                    }
                }})
        },
        fmt::Field{
            title: "Local",
            short: "local",
            default: false,
            get: Box::new(|x| {
                match x.lookup("local") {
                    None => "-".to_string(),
                    value => {
                        value
                            .unwrap()
                            .as_bool()
                            .unwrap()
                            .to_string()
                    }
                }})
        },        fmt::Field{
            title: "Timestamp",
            short: "timestamp",
            default: true,
            get: Box::new(|x| { x.lookup("timestamp").unwrap().as_i64().unwrap().to_string() })
        },
        fmt::Field{
            title: "Size",
            short: "size",
            default: true,
            get: Box::new(|x| {
                match x.lookup("size") {
                    None => "0".to_string(),
                    value => value
                        .unwrap()
                        .as_i64()
                        .unwrap()
                        .to_string()
                }
            })
        },
        fmt::Field{
            title: "State",
            short: "state",
            default: true,
            get: Box::new(|x| { x.lookup("state").unwrap().as_str().unwrap().to_string() })
        },
        fmt::Field{
            title: "Comment",
            short: "comment",
            default: true,
            get: Box::new(|x| { x.lookup("comment").unwrap().as_str().unwrap().to_string() })
        }
    ];
    let value = cmd::run_generic("backup-list".to_string());
    let obj = value.as_object().unwrap();
    let mut vec = Vec::new();
    for (uuid, data) in obj {
        let mut element = data.as_object().unwrap().clone();
        element.insert("uuid".to_string(), Value::String(uuid.to_string()));
        vec.push(Value::Object(element));
    }
    if matches.is_present("format") {
        let fmt_str: String = value_t!(matches, "format", String).unwrap();
        let format: Vec<&str> = fmt_str.split(',').collect();
        let opts_w_fmt = fmt::Opts{
            json: opts.json,
            format: format
        };
        fmt::print(&fields, &vec, &opts_w_fmt);
    } else {
        fmt::print(&fields, &vec, &opts);
    }
}

#[derive(RustcEncodable)]
struct BackupCreateReq {
    action: String,
    comment: String
}

fn create(matches: &ArgMatches) {
    let comment = value_t!(matches, "comment", String).unwrap();
    let req = BackupCreateReq{
        action:  "backup-create".to_string(),
        comment: comment
    };
    let res = cmd::run(req);
    print!("{}", serde_json::to_string(&res).unwrap());
}
