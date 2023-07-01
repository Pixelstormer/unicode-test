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

    let mut iters: Vec<std::slice::Iter<char>> = vec![chars.iter()];
    let mut encoded: Vec<[u8; 4]> = vec![[0; 4]];
    let mut stdout = stdout().lock();

    loop {
        'block: {
            for (iter, value) in iters.iter_mut().zip(encoded.iter_mut()) {
                match iter.next() {
                    Some(&new_value) => {
                        value.fill(0);
                        new_value.encode_utf8(value);
                        break 'block;
                    }
                    None => {
                        *iter = chars.iter();
                        value.fill(0);
                        unsafe { iter.next().unwrap_unchecked() }.encode_utf8(value);
                    }
                }
            }
            let mut new_iter = chars.iter();
            let mut new_value = [0; 4];
            new_iter.next().unwrap().encode_utf8(&mut new_value);
            iters.push(new_iter);
            encoded.push(new_value);
        }

        for &byte in encoded.iter().flatten().filter(|&&byte| byte != 0) {
            stdout.write_all(&[byte]).unwrap();
        }
        stdout.write_all(&[b'\n']).unwrap();
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
