fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    _text.push_str("!");
    let textik = _text.trim_start();
    if let Some(word_1) = textik.find(char::is_whitespace) {
        return &textik[0..word_1];
    } else {
        return textik.strip_suffix('!').unwrap_or(textik);
    }
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
