use std::os::unix::io::AsRawFd;
use std::fs::File;
use std::io::Read;
use std::io;
use extractous::Extractor;
use ocr_rs::OcrEngine;

fn with_stdout_silenced<T>(f: impl FnOnce() -> T) -> T {
    let devnull = File::create("/dev/null").unwrap();
    let old_stdout = unsafe { libc::dup(1) };
    unsafe { libc::dup2(devnull.as_raw_fd(), 1) };

    let result = f();

    unsafe { libc::fflush(std::ptr::null_mut()) };

    unsafe { libc::dup2(old_stdout, 1) };
    unsafe { libc::close(old_stdout) };
    result
}

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
    let det_model = "models/PP-OCRv6_small_det.mnn".to_string();
    let rec_model = "models/PP-OCRv6_small_rec.mnn".to_string();
    let keys = "models/ppocr_keys_v6_small.txt".to_string();

    let engine = with_stdout_silenced(|| {
        OcrEngine::new(det_model, rec_model, keys, None)
   })?;

    let image = image::open(file_path)?;
    let results = engine.recognize(&image)?;

    let mut text = String::new();
    for item in results {
        text.push_str(&item.text);
        text.push('\n');
    }

    Ok(text)
}

pub fn extract_text(file_path: &str) -> Result<String, io::Error> {
    if is_image(file_path) {
        let text = ocr_rec(file_path).unwrap();
        return Ok(text);
    }

    let extractor = Extractor::new();

    match extractor.extract_file_to_string(file_path) {
        Ok((text, _)) => {
            return Ok(text)
        },
        Err(_err) => {
            return Ok("".to_string())
        }
    };
}
