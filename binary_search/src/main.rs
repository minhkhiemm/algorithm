use std::io;
use std::str::FromStr;

fn binary_search(input: &[u8], item: &u8) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = input.len() - 1;
    while low <= high {
        let mid: usize = low + high;
        let guess = input[mid];
        if guess == *item {
            return Option::from(mid);
        }
        if guess > *item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return None;
}

fn main() {
    println!("please enter input slice");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut input: Vec<u8> = input
        .split(",")
        .filter_map(|word| u8::from_str(word).ok())
        .collect();
    input.sort();

    println!("please enter guess number");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");

    println!("guess: {}", guess.as_str());
    match u8::from_str(guess.as_str().trim()) {
        Ok(guess) => {
            match binary_search(&input, &guess) {
                None => println!("your item is not in list."),
                Some(position) => println!("your item is in position: {}", position + 1),
            };
        }
        Err(e) => println!("error: {}", e),
    };
}
