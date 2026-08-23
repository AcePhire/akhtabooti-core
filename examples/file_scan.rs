use std::io;
use std::env;
use akhtabooti_core::search_file;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let scan_result = search_file(file_path).unwrap();

    println!("{:?}", scan_result.filename);
    println!("{:?}", scan_result.email_accounts);
    println!("{:?}", scan_result.phone_numbers);
    println!("{:?}", scan_result.other_piis);
    Ok(())
}
