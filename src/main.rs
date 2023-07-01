use std::env::args;

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
        Some(string) if string == "iterate" => iterate(&chars),
        Some(string) if string == "random" => random(&chars, 64),
        _ => println!(
            "Enter 'iterate' to iterate over chars in order or 'random' to output random chars"
        ),
    }
}

fn iterate(chars: &[char]) {
    let mut iters: Vec<std::iter::Peekable<std::slice::Iter<char>>> = vec![chars.iter().peekable()];

    let mut string = String::new();

    loop {
        'block: {
            for i in iters.iter_mut() {
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

fn random(chars: &[char], len: usize) {
    let mut string = String::with_capacity(len);
    let mut rng = thread_rng();

    loop {
        let iter = chars.choose_multiple(&mut rng, len);
        string.clear();
        string.extend(iter);
        print!("{string}");
    }
}
