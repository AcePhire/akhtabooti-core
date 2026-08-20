use std::fs::File;
use std::io::Read;
use extractous::Extractor;
use ocr_rs::OcrEngine;

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

pub fn ocr_rec(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let DET_MODEL: String = "models/PP-OCRv6_small_det.mnn".to_string();
    let REC_MODEL: String = "models/PP-OCRv6_small_rec.mnn".to_string();
    let KEYS: String = "models/ppocr_keys_v6_small.txt".to_string();

    let engine = OcrEngine::new(
        DET_MODEL,
        REC_MODEL,
        KEYS,
        None,
    )?;

    let image = image::open(file_path)?;
    let results = engine.recognize(&image)?;

    let mut text: String = "".to_string();
    for item in results {
        text.push_str(&item.text);
    }

    Ok(text.to_string())
}

pub fn extract_text(file_path: &str) -> String {
    if is_image(file_path) {
        return ocr_rec(file_path).unwrap();
    }

    let extractor = Extractor::new();

    let (text, _) = extractor.extract_file_to_string(file_path).unwrap();
    text
}
