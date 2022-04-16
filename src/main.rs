use std::io::stdin;

fn main() {
    // ! 는 매크로란 의미다. 따라서, println 은 지금 함수가 아니라 매크로다.
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
