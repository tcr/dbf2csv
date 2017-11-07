extern crate dbf;
extern crate csv;

use std::error::Error;
use csv::Writer;

use dbf::*;

use std::path::Path;
use std::env::args;

pub fn main() {
    example().expect("Error encountered");
}

fn example() -> Result<(), Box<Error>> {
    let filename = args().nth(1).expect("input missing: dbf2csv <input.dbf> <output.csv>");
    let out = args().nth(2).expect("output missing: dbf2csv <input.dbf> <output.csv>");

    let mut dbffile = DbfFile::open_file(&Path::new(&filename));
    let mut wtr = Writer::from_path(out)?;
    
    let headers = dbffile.headers().clone();
    for (_, hdr) in headers.iter().enumerate() {
        // let field_type = match hdr.field_type {
        //     FieldType::Character => "String",
        //     FieldType::Numeric => {
        //         if hdr.decimal_count == 0 {
        //             "Integer"
        //         } else {
        //             "Double"
        //         }
        //     }
        // };

        wtr.write_field(hdr.name.to_string())?;
    }
    wtr.write_record(None::<&[u8]>)?;

    for rec_id in 0..dbffile.num_records() {
        // print!("\n");
        // println!("Record: {}", rec_id);
        let rec = dbffile.record(rec_id).unwrap();
        for header in headers.iter() {
            if rec[&header.name].to_string() != "(NULL)" {
                wtr.write_field(rec[&header.name].to_string())?;
            } else {
                wtr.write_field(&[])?;        
            }
            // println!("{}: {}", header.name, rec[&header.name]);
        }
        wtr.write_record(None::<&[u8]>)?;
    }

    wtr.flush()?;
    Ok(())
}
