fn is_even(number: i32) -> bool{
    let result = number % 2;

    if result == 0{
        return true;
    } else{
        return false;
    }
}

fn main() {
    let number_array = [30, 13, 27, 40, 39, 28, 49, 96, 28, 23];
    let mut sum :i32 = 0;
    let mut counter :usize = 9;
    let mut greatest :i32 = 0;

    for iterator in number_array{
        //println!("{}", is_even(iterator));
        if iterator % 3 == 0 && iterator % 5 == 0{
            println!("FizzBuzz");
        }else{
            if iterator % 3 == 0{
                println!("Fizz");
            }else if iterator % 5 == 0{
                println!("Buzz");
            } else{
                println!("{}", is_even(iterator));
            }
        }
    }

    while counter != 0{
        sum += number_array[counter];
        counter -= 1;
    }
    println!("Sum of the array: {}", sum);

    for idx in 0..number_array.len(){
        if greatest < number_array[idx]{
            greatest = number_array[idx];
        }
    }
    println!("Greatest number in the array: {}", greatest);

}
