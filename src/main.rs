use csv::ReaderBuilder;
use csv::StringRecord;
use std::collections::HashMap;
// use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

// type Value = (usize, usize);
// type Statistic = BTreeMap<String, Value>;
type Storage = HashMap<u32, usize>;

fn main() {
    if let Some(ref path) = env::args().nth(1) {
        let mut container = Storage::new();

        let mut csv: Vec<u8> = Vec::new();
        let mut file = File::open(&path).expect("Unable to open file");
        let _ = file.read_to_end(&mut csv).expect("Unable to read {path}");

        let mut rdr = ReaderBuilder::new()
            .delimiter(b',')
            .flexible(true)
            .has_headers(true)
            .double_quote(true)
            .quoting(true)
            .from_reader(csv.as_slice());

        for result in rdr.records() {
            match result {
                Ok(record) => handle(&mut container, record),
                Err(err) => println!("{err}"),
            }
        }

        let mut v = Vec::from_iter(container);
        v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
        let total = v.iter().fold(0, |acc, value| acc + value.1);

        println!("{:>10} | {:>10}", "Second", "Amount");
        for (second, amount) in v.iter() {
            println!("{second:>10} | {amount:>10}");
        }
        println!("Total {total} bytes in {} records", v.len());
    }
}

fn handle(container: &mut Storage, records: StringRecord) {
    let fields: Vec<&str> = records.iter().collect();
    if 13 != fields.len() {
        // println!("Skiped {:?}", fields);
        return;
    }

    let length = fields[12].len();
    // let mut words = fields[12].split_whitespace();
    // let mut key = words.nth(1).expect("Key not found");
    let second = fields[2].parse::<f32>().unwrap_or_default().floor() as u32;

    container
        .entry(second)
        .and_modify(|value| {
            *value += length;
        })
        .or_insert(length);
}
