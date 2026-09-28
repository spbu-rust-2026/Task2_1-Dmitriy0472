fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    _text.push_str(" !");
    if let Some(word_1) = _text.find(' ') {
        return &_text[0..word_1 + 1];
    } else {
        return "";
    }
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
