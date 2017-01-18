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

pub fn print(fields: &Vec<Field>, values: &Vec<Value>) {
    let mut table = Table::new();
    let mut hdr = Row::empty();
    for field in fields.iter() {
        if field.default {
            hdr.add_cell(Cell::new(field.title));
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
