use serde_json;
use serde_json::Value;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use fmt;
use std::io;
use std::process;
use std::io::Write;

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


pub fn print_value(values: &Value) {
    let str = serde_json::to_string(&values).unwrap();
    print!("{}\n", str);
 }

pub fn print_values(values: &Vec<Value>) {
    let str = serde_json::to_string(&values).unwrap();
    print!("{}\n", str);
}
pub fn print(fields: &Vec<Field>, values: &Vec<Value>, opts: &fmt::Opts) {
    let display_fields = format_fields(fields, opts);
    if opts.json {
        print_values(values)
    } else {
        let mut table = Table::new();
        let mut hdr = Row::empty();
        for field in display_fields.iter() {
            if show(opts, field) {
                hdr.add_cell(Cell::new(field.title).style_spec("bc"));
            }
            //println!("field: {:?}", field.title)
        }
        table.add_row(hdr);
        for entry in values.iter() {
            let mut row = Row::empty();
            for field in display_fields.iter() {
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

fn format_fields<'a>(fields: &'a Vec<Field<'a>>, opts: &fmt::Opts) -> Vec<&'a Field<'a>> {
    if opts.format.is_empty() {
        fields.iter().filter(|f| f.default).collect()
    } else {
        let mut result: Vec<&Field> = vec![];
        for &n in opts.format.iter() {
            match fields.iter().position(|f| f.short == n) {
                None => {
                    writeln!(io::stderr(), "Error: field {} doesn't exist!", n).unwrap();
                    process::exit(1)
                }
                pos =>
                    result.push(&fields[pos.unwrap()])
            }
        }
        result
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
