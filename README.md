# akhtabooti-core

A Rust library for detecting PIIs in documents, PDFs, and images.

It extracts text from a file (using OCR for images), then runs it against a set of detection rules defined in `definitions.json`.

## What it detects

- Email addresses
- Phone numbers (Jordanian and Syrian mobile formats)
- Jordanian national IDs
- Jordanian passports
- Syrian refugee documents (work permits, service documents, UNHCR documents, etc.)

## How it works

1. **Text extraction**: images are passed through PaddleOCR (`ocr-rs`), and everything else (PDFs, Office docs, text files) through [extractous](https://github.com/yobix-ai/extractous).
2. **Detection**: the text is checked against the rules in `definitions.json`, using either regex or keyword matching. Keyword matching is fuzzy and normalizes Arabic text.

## Models

The OCR models come from the [ocr-rs repo (`next` branch)](https://github.com/zibo-chen/ocr-rs/tree/next) and need to be in a `models/` directory at the crate root. From the crate root, run:

```sh
mkdir -p models && cd models
for f in PP-OCRv6_small_det.mnn PP-OCRv6_small_rec.mnn ppocr_keys_v6_small.txt \
         arabic_PP-OCRv5_mobile_rec_infer.mnn ppocr_keys_arabic.txt; do
  curl -LO "https://raw.githubusercontent.com/zibo-chen/ocr-rs/next/models/$f"
done
```

## Usage

```rust
use akhtabooti_core::{search_file, search_directory};

let result = search_file("path/to/file.pdf")?;
println!("{:?}", result.email_accounts);
println!("{:?}", result.phone_numbers);
println!("{:?}", result.other_piis);

let results = search_directory("path/to/dir")?;
```

Model and definition paths are currently relative, so run from the crate root.

## Examples

```sh
cargo run --example file_scan -- path/to/file
cargo run --example directory_scan -- path/to/dir
cargo run --example debug_ocr -- path/to/image.png
```

## Detection rules

Rules live in `definitions.json`. Each definition has a list of rules, and a definition matches if **any** rule matches. Each rule is a list of conditions, and a rule matches only if **all** of its conditions match.

A condition is either a regex or a list of keywords. A keyword list matches if **any** one of its keywords appears in the text.

```json
"Jordanian Passport": {
  "region": "Jordan",
  "rules": [
    [{ "regex": "P<JOR[A-Z<]{30,39}" }],
    [{ "keywords": ["Passport"] }, { "keywords": ["JOR", "Jordan"] }]
  ]
}
```

Keywords of 3 characters or fewer require an exact match. Longer keywords are fuzzy-matched (Jaro-Winkler ≥ 90).

## License

Apache-2.0
