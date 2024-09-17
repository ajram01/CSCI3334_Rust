
fn check_guess(guess: i32, secret: i32) -> i32{
    if guess == secret{
        return 1;
    } else if guess > secret{
        return 0;
    } else{
        return -1;
    }
}


fn main() {
    let secret_num = 5;
    let guess_choice: [i32; 4] = [1, 7, 4, 5];
    let mut amount_guesses = 0;

    loop{
        if check_guess(guess_choice[amount_guesses], secret_num) == 1{
            println!("Correct!");
            amount_guesses += 1;
            break;
        } else{
            if check_guess(guess_choice[amount_guesses], secret_num) == 0{
                println!("Too high!");
            }else{
                println!("Too low!")
            }
        }
        amount_guesses += 1;
    }
    println!("It took this many guesses: {}", amount_guesses);
}
