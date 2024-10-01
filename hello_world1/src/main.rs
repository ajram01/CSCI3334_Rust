use std::io::{self, Read, Write};

struct Car {
    make: String,
    year: u32,
    color: String,
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

    print!("What's the color of you car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

    let car = Car { color, make, year };
    println!("Your cars was made by {}, and is the model year {}, and it is the color {}", car.make, car.year, car.color);

    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "Make: {}, Year: {}, Color: {}", car.make, car.year, car.color).unwrap();
    
}