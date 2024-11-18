
// fn track_changes() {
//     let mut tracker = 0;
//     let mut update = || {
//         // Your implementation here
//         tracker += 1;
//         println!("{}", tracker);
//     };

//     update();
//     update();
// }


// fn main() {
//     let operation = |a: i32, b:i32| a * b;

//     println!("Result: {}", operation(10,5));

//     track_changes();
// }


// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     // Your implementation here
//     vec.into_iter().map(f).collect()
// }

// fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     let mut result = Vec::new();
//     for x in vec {
//         result.push(f(x)); // Apply the closure
//     }
//     result
// }

// fn main() {
//     let numbers = vec![1, 2, 3];
//     let numbers_for = vec![1, 2, 3];

//     let doubled = process_vector(numbers.clone(), |x| {
//         // Implement: multiply each number by 2
//         x * 2
//     });

//     let doubled_for = process_vector_with_for_loop(numbers.clone(), |x| {
//         x * 2
//     });

//     let replaced = process_vector(numbers, |x| {
//         // Implement: if number > 2, replace with 0, else keep number
//         let mut num = x;
//         if num > 2{
//             num = 2;
//         }
//         num
//     });

//     let replaced_for = process_vector_with_for_loop(numbers_for, |x| {

//         let mut num2 = x;

//         if num2 > 2{
//             num2 = 2;
//         }

//         num2

//     });

//     println!("Doubled: {:?}", doubled);
//     println!("Doubled for: {:?}", doubled_for);
//     println!("Replaced: {:?}", replaced);
//     println!("Replaced for: {:?}", replaced_for);
// }



use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache{
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.to_string()
            }
            None => {
                let v = (self.computation)();
                self.value = Some(v.clone());
                v
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}



