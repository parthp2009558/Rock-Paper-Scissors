extern crate rand;
use rand::Rng; // Import the Rng trait for generating random numbers
/// Rust Final Project : Rock Paper Scissors Game
/// Attributes: Computer will guss a random choice and it will be matched against users input
/// 
/// Author (Parth Patel)
///  Date  (December 19)

fn main() {
    // Generate a random number between 0 and 2
    let computer_choice = rand::thread_rng().gen_range(0, 3);

    // Convert the computer's choice to a string
    let computer_choice = match computer_choice {
        0 => "rock",
        1 => "paper",
        2 => "scissors",
        _ => unreachable!(),
    };

    // Print the prompt and read the user's input
    println!("Choose rock, paper, or scissors: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    // Determine the winner of the round
    if input == computer_choice {
        println!("It's a tie!");
    } else if (input == "rock" && computer_choice == "scissors") ||
        (input == "paper" && computer_choice == "rock") ||
        (input == "scissors" && computer_choice == "paper")
    {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}
