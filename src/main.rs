use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!(":::::: GUESS THE NUMBER!! ::::::");

    //Generation of random number between 1 and 100 (higher limit is exclusive)
    let random_number = rand::thread_rng().gen_range(1, 101);
    
    let mut num_of_attempts: i16 = 0;
    let mut number_match = false;
    let mut wrong_inputs: i8 = 5;

    println!("Please enter a number between 1 and 100");    

    while !number_match {
        let mut entered_number = String::with_capacity(1);

        io::stdin().read_line(&mut entered_number)
            .expect("Please enter a number between 1 and 100");

        //Match against Result object, to evaluate if asking for number again, if it is invalid
        let entered_number: i32 = match entered_number.trim().parse() {
            Result::Ok(num) => num,
            Result::Err(_) => {
                wrong_inputs = wrong_inputs - 1;
                if wrong_inputs > 0 {
                    println!("Enter a valid number!! {} attempts left.", wrong_inputs);
                    continue;
                } else {
                    println!("You don't wanna play, let's finish this >:-( ");
                    return;
                }
            }
        };

        match entered_number.cmp(&random_number) {
            Ordering::Less => println!("\"{}\" is lesser than random number", entered_number),
            Ordering::Equal => {
                number_match = true;
                println!("Congratulations! You guessed! {} is the number", entered_number);
            },
            Ordering::Greater => println!("\"{}\" is greater than random number", entered_number)
        }

        num_of_attempts = num_of_attempts + 1;
    }

    println!("It took you {} attempts to guess the number", num_of_attempts);
}