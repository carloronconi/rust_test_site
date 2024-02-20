use std::io;

pub fn run() {
    println!("Input a string of text:");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    println!("Input the world number you want to extract:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .unwrap();

    let num = num.lines().nth(0).unwrap().parse().unwrap();
    let word = find_nth_word(num, &text);

    match word {
        Some(w) => println!("The {num}'th word is {w}"),
        None => println!("The {num}'th word doesn't exist!"),
    }

}

fn find_nth_word(n: usize, text: &str) -> Option<&str> {

    let mut word_count: (usize, usize) = (0, 0);
    for (i, &item) in text.as_bytes().iter().enumerate() {
        if item == b' ' {
            match word_count {
                (num, idx) if num == n => return Some(&text[idx..i]),
                _ => { word_count.0 += 1; word_count.1 = i + 1},
            }
        }
    }

    if word_count.0 == n { return Some(&text[word_count.1..]) }

    None
}
