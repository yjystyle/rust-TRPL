use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    // rust에서 변수는 기본적으로 immutable
    // mut라는 예약어를 통해 mutable하게 만들어야 한다.
    // let apples = 5; immutable
    // let mut bananas = 5; mutable
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}
