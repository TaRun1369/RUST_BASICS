use rand::Rng; // Import Rng trait for gen_range
use rand::thread_rng;

fn main() {
    let arr = ["apple", "banana", "cherry", "blueberry", "orange"];
    loop {
        println!("Guess the fruit!");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim(); // Trim the guess input

        let mut rng = thread_rng();
        let random_index = rng.gen_range(0..arr.len());
        
        if guess == arr[random_index] {
            println!("You guessed it right!, Winner Winner, Crying Wiener!");
            break;
        } else {
            println!("Try again!");
        }
    }
}
