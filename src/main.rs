use std::{
    env::args,
    io::{stdout, Write},
};

use rand::prelude::{thread_rng, SliceRandom};
use unicode_intervals::UnicodeCategory;

fn main() {
    let intervals = unicode_intervals::query()
        .exclude_categories(UnicodeCategory::C | UnicodeCategory::Lo)
        .intervals()
        .unwrap();

    let chars = intervals
        .iter()
        .flat_map(|(left, right)| *left..=*right)
        .map(|i| char::from_u32(i).unwrap())
        .collect::<Vec<_>>();

    match args().last() {
        Some(string) if string == "print" => print(&chars),
        Some(string) if string == "iterate" => iterate(&chars),
        Some(string) if string == "random" => random(&chars),
        _ => {
            println!(
                "Working with {} chars (Enter 'print' to print)",
                chars.len()
            );
            println!(
                "Enter 'iterate' to iterate over chars in order or 'random' to output random chars"
            );
        }
    }
}

fn print(chars: &[char]) {
    println!("{}", String::from_iter(chars.iter()));
}

fn iterate(chars: &[char]) {
    assert!(!chars.is_empty(), "Chars must not be empty");

    let mut iters: Vec<std::iter::Peekable<std::slice::Iter<char>>> = vec![chars.iter().peekable()];

    let mut string = String::new();

    loop {
        'block: {
            for i in &mut iters {
                i.next();
                match i.peek() {
                    None => *i = chars.iter().peekable(),
                    Some(_) => break 'block,
                }
            }
            iters.push(chars.iter().peekable());
        }

        string.clear();
        for i in iters.iter_mut().rev() {
            string.push(**i.peek().unwrap());
        }

        println!("{string}");
    }
}

fn random(chars: &[char]) {
    assert!(!chars.is_empty(), "Chars must not be empty");

    let mut rng = thread_rng();
    let mut buffer = [0u8; 4];
    let mut stdout = stdout().lock();

    loop {
        let value = unsafe { chars.choose(&mut rng).unwrap_unchecked() };
        let value_encoded = value.encode_utf8(&mut buffer);
        stdout.write_all(value_encoded.as_bytes()).unwrap();
    }
}
