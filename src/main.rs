use core::panic;
use std::env;
use std::io::prelude::*;
use std::ops::Index;
use std::path::Path;
use std::{fs::File, io};
// A silly program that transforms strings and numbers into binary blobs
enum Value {
    Int(i32),
    Float(f32),
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("NOT ENOUGH PARAMETERS!\ndablob makeblob - create blob\ndablob deblob - read blob")
    }
    if args.len() > 2 {
        panic!("TOO MANY PARAMETERS! YOU ARE GREEDY!")
    }
    if args.index(1).to_string() == String::from("deblob") {
        deconstruct_blob()
    }
    if args.index(1).to_string() == String::from("makeblob") {
        construct_blob()
    }
}

fn deconstruct_blob() {
    println!("Enter a thing to be deblobbed:");
    let mut to_be_deblobbed = String::new();

    match io::stdin().read_line(&mut to_be_deblobbed) {
        Ok(_) => {
            println!("{} will be deblobbed", to_be_deblobbed.trim());
        }
        Err(e) => println!("error: {e}"),
    }
    let blobbed_trim = to_be_deblobbed.trim();
    let path = Path::new(blobbed_trim);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(v) => v,
    };
    
    if blobbed_trim == "text.blob" {
        let mut s = Vec::new();
        match file.read(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("opened {display}"),
        }
        let text = match std::str::from_utf8(&s) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };
        println!("{text}");
        
    } else { 
        let mut s: [u8; 4] = [0; 4];
        match file.read(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => println!("opened {display}"),
        }
        if blobbed_trim == "int.blob" {
            let val = i32::from_be_bytes(s);
            println!("{val}");
        } else if blobbed_trim == "float.blob" {
            let val = f32::from_be_bytes(s);
            println!("{val}");
        }
    }
}

fn construct_blob() {
    println!("Enter a thing to be blobbed:");
    let mut to_be_blobbed = String::new();

    match io::stdin().read_line(&mut to_be_blobbed) {
        Ok(v) => {
            println!("{v} bytes worth of blob");
            println!("{} will be blobbed", to_be_blobbed.trim());
        }
        Err(e) => println!("error: {e}"),
    }

    match will_it_blob(&to_be_blobbed) {
        Some(Value::Int(i)) => match make_blob("int", i.to_be_bytes().to_vec()) {
            Ok(_) => println!("Blobbed Integer"),
            Err(e) => panic!("{e}"),
        },
        Some(Value::Float(f)) => match make_blob("float", f.to_be_bytes().to_vec()) {
            Ok(_) => println!("Blobbed Float"),
            Err(e) => panic!("{e}"),
        },
        None => match make_blob("text", to_be_blobbed.as_bytes().to_vec()) {
            Ok(_) => println!("Blobbed Text"),
            Err(e) => panic!("{e}"),
        },
    }
}

fn will_it_blob(s: &String) -> Option<Value> {
    if let Ok(i) = s.trim().parse::<i32>() {
        Some(Value::Int(i))
    } else if let Ok(f) = s.trim().parse::<f32>() {
        Some(Value::Float(f))
    } else {
        None
    }
}

fn make_blob(blob_kind: &str, data: Vec<u8>) -> io::Result<()> {
    let mut f = File::create(format!("{blob_kind}.blob"))?;
    f.write_all(&data)?;
    Ok(())
}
