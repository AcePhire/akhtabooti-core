use std::io;
use std::env;
use akhtabooti_core::search_directory;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let directory_path = &args[1];

    let scan_results = search_directory(directory_path).unwrap();

    for file_result in scan_results {
        println!("{:?}", file_result.filename);
        println!("{:?}", file_result.email_accounts);
        println!("{:?}", file_result.phone_numbers);
        println!("{:?}", file_result.other_piis);
    }
    Ok(())
}
