use std::io::{self, BufReader, BufRead, Write};
use std::fs::File;

struct Car {
    make: String,
    year: u32,
    color: String,
}

fn read_file_line_by_line(){
    let file = File::open("user_info.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("Whats the make of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    print!("Whats the year of your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();
    buffer.clear();

    print!("What's the color of you car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

    let car = Car { color, make, year };
    println!("Your car was made by {}, and is the model year {}, and it is the color {}", car.make, car.year, car.color);

    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "Make: {}, Year: {}, Color: {}", car.make, car.year, car.color).unwrap();

    
}

fn main() {
    reading_from_console();
    read_file_line_by_line();
}