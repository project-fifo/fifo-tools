use serde_json;
use serde_json::Value;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;


pub struct Field {
    pub title: &'static str,
    pub short: &'static str,
    pub default: bool,
    pub get: Box<Fn(&Value) -> String>
}

pub struct Opts {
    pub json: bool,
    pub fields: Vec<&'static str>
}

pub fn print(fields: &Vec<Field>, values: &Vec<Value>, opts: &Opts) {
    if opts.json {
        let str = serde_json::to_string(&values).unwrap();
        print!("{}", str);
    } else {
        let mut table = Table::new();
        let mut hdr = Row::empty();
        for field in fields.iter() {
            if field.default {
                hdr.add_cell(Cell::new(field.title).style_spec("bc"));
            }
            //println!("field: {:?}", field.title)
        }
        table.add_row(hdr);
        for entry in values.iter() {
            let mut row = Row::empty();
            for field in fields.iter() {
                if field.default {
                    let field = (field.get)(entry);
                    row.add_cell(Cell::new(&field));
                }
            }
            table.add_row(row);
        }
        table.printstd();
    }
}
