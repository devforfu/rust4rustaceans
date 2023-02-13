use interfaces::refs::{Name, PhoneBook, PhoneNumber};
use std::io::{stdin, BufRead};

fn main() -> std::io::Result<()> {
    let mut book = PhoneBook::default();
    for (name, phone) in read_names()?.into_iter().flat_map(parse) {
        book.add(name, phone);
    }
    println!("Book size: {}", book.len());
    Ok(())
}

fn read_names() -> std::io::Result<Vec<String>> {
    let mut names = Vec::new();
    let mut buf = String::new();
    while let Ok(n) = stdin().lock().read_line(&mut buf) {
        buf = buf.trim().to_string();
        if n == 0 || buf == "EOF" {
            break;
        }
        names.push(buf);
        buf = String::new();
    }
    Ok(names)
}

fn parse(entry: String) -> Option<(Name, PhoneNumber)> {
    let mut split = entry.split(':');
    if let Some(name) = split.next() {
        if let Some(phone) = split.next() {
            if let Ok(phone) = phone.parse::<usize>() {
                return Some((name.to_string(), phone));
            }
        }
    }
    None
}
