use crate::text_util::detect_keywords;

mod text_util;

fn main() {
    let regex = text_util::get_regexes("definitions.json").unwrap();

    let email_piis = text_util::detect_emails(
        regex.clone(),
        "acephire27@gmail.com hello hello no there is google batman@firefox.com something something",
    );

    let phone_piis = text_util::detect_phone_numbers(
        regex.clone(),
        "0790841445 asdfhj asdjhg hello zeor zero 9123 fhas haa 0795798540",
    );

    // println!("{:?}", email_piis);
    // println!("{:?}", phone_piis);
    //
    let detection = detect_keywords(regex, "Jordan Passpurt");
    println!("{:?}", detection)
}
