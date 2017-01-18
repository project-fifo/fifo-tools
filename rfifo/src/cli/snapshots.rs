use clap::{App, Arg, SubCommand, ArgMatches};
use serde_json::Value;
use cmd;
use fmt;

pub fn build() -> App<'static, 'static> {
    SubCommand::with_name("snapshots")
        .about("Snapshot related commands")
        .subcommand(SubCommand::with_name("list")
                    .about("Lists all snapshots")
                    .arg(Arg::with_name("format")
                         .long("fmt")
                         .value_name("FMT")
                         .help("Fields to be shown")))
        .subcommand(SubCommand::with_name("create")
                    .about("Creates a snapshot")
                    .arg(Arg::with_name("comment")
                         .value_name("COMMENT")
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
                    println!("Sub command '{}' not implemented for snapshots.", other)
            }
        }
    }
}

fn list(_app: &ArgMatches) {
    let fields =  vec![
        fmt::Field{
            title: "UUID",
            short: "uuid",
            default: true,
            get: Box::new(|x| { x.lookup("uuid").unwrap().as_str().unwrap().to_string() })
        },
        fmt::Field{
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
                }})

        },
        fmt::Field{
            title: "Comment",
            short: "comment",
            default: true,
            get: Box::new(|x| { x.lookup("comment").unwrap().as_str().unwrap().to_string() })
        }
    ];
    let value = cmd::run_generic("snapshot-list".to_string());
    let obj = value.as_object().unwrap();
    let mut vec = Vec::new();
    for (uuid, data) in obj {
        let mut element = data.as_object().unwrap().clone();
        element.insert("uuid".to_string(), Value::String(uuid.to_string()));
        vec.push(Value::Object(element));
    }
    fmt::print(&fields, &vec);
}


#[derive(RustcEncodable)]
struct SnapshotCreateReq {
    action: String,
    comment: String
}

fn create(matches: &ArgMatches) {
    let comment = value_t!(matches, "comment", String).unwrap();
    let req = SnapshotCreateReq{
        action:  "snapshot-create".to_string(),
        comment: comment
    };
    let res = cmd::run(req);

    println!("create: {:?}", res)
}
