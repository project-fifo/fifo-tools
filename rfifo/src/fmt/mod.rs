use serde_json;
use serde_json::Value;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use fmt;

pub struct Field<'a> {
    pub title: &'a str,
    pub short: &'a str,
    pub default: bool,
    pub get: Box<Fn(&Value) -> String>
}

pub struct Opts<'a> {
    pub json: bool,
    pub format: Vec<&'a str>
}

pub fn print(fields: &Vec<Field>, values: &Vec<Value>, opts: &fmt::Opts) {
    if opts.json {
        let str = serde_json::to_string(&values).unwrap();
        print!("{}", str);
    } else {
        let mut table = Table::new();
        let mut hdr = Row::empty();
        for field in fields.iter() {
            if show(opts, field) {
                hdr.add_cell(Cell::new(field.title).style_spec("bc"));
            }
            //println!("field: {:?}", field.title)
        }
        table.add_row(hdr);
        for entry in values.iter() {
            let mut row = Row::empty();
            for field in fields.iter() {
                if show(opts, field) {
                    let field = (field.get)(entry);
                    row.add_cell(Cell::new(&field));
                }
            }
            table.add_row(row);
        }
        table.printstd();
    }
}

fn show(opts: &Opts, field: &Field) -> bool {
    match opts.format.iter().position(|&r| r == field.short) {
        None =>
            if opts.format.is_empty() {
                field.default
            } else {
                false
            },
        _ => true
    }
}
