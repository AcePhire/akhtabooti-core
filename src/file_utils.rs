use std::fs::File;
use std::io::Read;
use extractous::Extractor;

fn get_file_type(file_path: &str) -> String {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return String::new(),
    };

    let mut buffer = [0; 16];
    if file.read_exact(&mut buffer).is_ok() {
        if let Some(kind) = infer::get(&buffer) {
            return kind.mime_type().to_string();
        }
    };

    String::new()
}

pub fn is_image(file_path: &str) -> bool {
    get_file_type(file_path).contains("image") 
}

pub fn is_pdf(file_path: &str) -> bool {
    get_file_type(file_path) == "application/pdf"
}

pub fn extract_text(file_path: &str) -> String {
    let extractor = Extractor::new();

    let (text, _) = extractor.extract_file_to_string(file_path).unwrap();
    text
}
