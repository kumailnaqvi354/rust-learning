use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess: String  = String::new();
     
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
    
    // io::stdin(): Stdin
    //     .read_line(buf: &mut guess): Result<usize, Error>
    //     .expect(msg: "failed to read line");

        println!("You guessed: {}", guess)
}
