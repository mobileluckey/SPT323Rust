//add a library so we can input from the command line
use std::io;

// add our main function, this is the entry point into our program.
fn main() {
    //Print a title to the console
    println!("\n\n\t ***Name that Movie Quiz***\n\n");

    //Create a string to hold the user answer.
    //let is used to create a variable.
    //mut is used to make a variable mutable(changable)
    //user_answer is the name of the variable
    let mut user_answer = String::new();

    //Print a movie quote to the console.
    print!("\"I've got a bad feeling about this...\" - is from what Movie?\n");

    //Get the user input and store it in the user_answer variable.
    io::stdin().read_line(&mut user_answer).expect("Failed to read line.");

    //Trim the user input removing any white space.
    user_answer = user_answer.trim().to_string();

    //Make the user input all lower case.
    user_answer = user_answer.to_lowercase();

    // Check the user input against the correct answer.
    if user_answer == "star wars" {
        // The user got the correct answer.
        println!("You are correct!  A Jedi you are!");
    }else {
        // The user got the answer wrong.
        println!("You are incorrect!  A Jedi you are not!");

    }
    // End the program and say goodbye.
    println!("\n\n\tThank you for using my Movie quiz!  Goodbye!!\n\n");
}