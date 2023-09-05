use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // This new function creates a new, empty string.
        let mut guess = String::new();

        // Now we’ll call the stdin function from the io module, which will allow us to handle user input.
        // The stdin function returns an instance of std::io::Stdin, which is a type that represents
        // a handle to the standard input for your terminal.
        io::stdin()
            .read_line(&mut guess) // calls the read_line method on the standard input handle to get input from the user
                                                            // The full job of read_line is to take whatever the user types into standard input
                                                            // and append that into a string (without overwriting its contents). The string argument
                                                            // needs to be mutable so the method can change the string’s content.

                                                            // The & indicates that this argument is a reference, which gives you a way to let
                                                            // multiple parts of your code access one piece of data without needing to copy that
                                                            // data into memory multiple times.

                                                            // Like variables, references are immutable by default. Hence, you need to write
                                                            // &mut guess rather than &guess to make it mutable.

            .expect("Failed to read line"); // read_line() puts whatever the user enters into the string we pass to it, but it also returns
                                                // a Result value. Result is an enumeration, often called an enum, which is a type that can be in
                                                // one of multiple possible states. We call each possible state a variant.

                                                // Result’s variants are Ok and Err. The Ok variant indicates the operation was successful, and inside
                                                // Ok is the successfully generated value. The Err variant means the operation failed, and Err contains
                                                // information about how or why the operation failed.

                                                // If this instance of Result is an Err value, expect will cause the program to crash and display the
                                                // message that you passed as an argument to expect. If this instance of Result is an Ok value, expect
                                                // will take the return value that Ok is holding and return just that value to you so you can use it.
                                                // In this case, that value is the number of bytes in the user’s input.

        // Shadowing lets us reuse the guess variable name rather than forcing
        // us to create two unique variables, such as guess_str and guess.
        // This feature is often used when you want to convert a value from one type to another type.

        //The trim method on a String instance will eliminate any whitespace at the beginning and end,
        // which we must do to be able to compare the string to the u32, which can only contain numerical data.

        // The parse method on strings converts a string to another type. Here, we use it to convert from a string to a number.
        // We need to tell Rust the exact number type we want by using let guess: u32.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it! You win!");
                break;
            }
        }
    }
}
