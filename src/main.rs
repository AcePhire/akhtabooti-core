mod file_utils;
mod text_utils;
mod akhtabooti;

use serde_json;
use std::io;

fn main() -> Result<(), io::Error> {
    let piis = akhtabooti::search_directory("pii/");
    for i in piis {
        let json = serde_json::to_string(&i)?;
        
        println!("{}", json);
    }

    Ok(())
}
