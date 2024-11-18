// fn add(x:i32,y:i32) -> i32{
//     x+y
// }

// fn multiply(x:i32, y:i32) -> i32{
//     x*y
// }

// struct Calculator{
//     addition: fn(i32,i32) -> i32,
//     multiplication: fn(i32,i32) -> i32,
// }

// impl Calculator{
//     fn new(addition_behavior:fn(i32,i32) -> i32, multiplication_behavior:fn(i32,i32) -> i32) -> Self {
//         Calculator{
//             addition:addition_behavior,
//             multiplication:multiplication_behavior
//         }
//     }
//     fn add(&self, x:i32, y:i32) -> i32{
//         (self.addition)(x,y)
//     }
//     fn multiply(&self, x:i32, y:i32) -> i32{
//         (self.multiplication)(x,y)
//     }
// }

// fn main() {

//     let c = Calculator::new(add, multiply);
//     let result = c.add(5,10);
//     let mult_result = c.multiply(5,10);
//     println!("{:?}, {:?}", result, mult_result);

// }


fn box_polymorphism() {
    use core::fmt::Debug;
    
    trait Animal: Debug {
        fn sound(&self) -> String;
    }
    
    #[derive(Debug)]
    struct Dog;
    
    impl Animal for Dog {
        fn sound(&self) -> String {
            "Woof woof".to_string()
        }
    }
    
    #[derive(Debug)]
    struct Cat;
    
    impl Animal for Cat {
        fn sound(&self) -> String {
            "Meow meow".to_string()
        }
    }
    
    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    
    zoo.push(Box::new(Dog{}));
    zoo.push(Box::new(Cat{}));
    
    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }
}

fn main(){
    box_polymorphism();
}
