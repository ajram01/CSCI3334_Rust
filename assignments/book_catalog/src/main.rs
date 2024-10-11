use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();

    for i in books{

        writeln!(file, "{:?}, {:?}, {:?}", i.title, i.author, i.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the line result
        let parts: Vec<&str> = line.split(',').collect(); // Split into a Vec<&str>
        if parts.len() == 3 {
            let title = parts[0].trim().to_string(); // Use trim to remove extra spaces
            let author = parts[1].trim().to_string();
            let year = parts[2].trim().parse::<u16>().unwrap_or(0); // Handle year parsing
            
            books.push(Book { title, author, year });
        }
    }

    books // Return the books vector
}


fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}