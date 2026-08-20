
use std::io;
use std::env;
use akhtabooti_core::file_utils::ocr_rec;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let text = ocr_rec(file_path).unwrap();

    println!("{}", text);

    Ok(())
}
