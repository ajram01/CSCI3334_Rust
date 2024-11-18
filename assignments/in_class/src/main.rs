// struct ComputingDevice{
//     cpu: String,
//     ram: u16,
// }

// impl ComputingDevice{
//     fn new(cpu: String, ram: u16) -> ComputingDevice{
//         Self{
//             cpu: cpu,
//             ram: ram
//         }
//     }

//     fn computation(&self){
//         println!("The computation 16 / 2 = 8 was performed by my computer with a cpu: {}, and {}gb of ram", self.cpu, self.ram);
//     }
// }

// fn main() {
//     let my_computer: ComputingDevice = ComputingDevice::new(
//         "AMD".to_string(),
//         16
//     );

//     my_computer.computation();
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("exam.txt");
//     println!("{:?}", f);
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("exam.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error),
//         },
//     };
// }

// #[derive(Debug)]
// struct Person<T> {
//     saying:saying
// }
// impl<U,T> Person<U,T> {
//     fn new(name:U, age:T) -> Person<T> {
//         Person{
//             saying:saying,
//         }
//     }

//     fn talk() {
//         println!("{}", saying);
//     }
    
// }

// struct Parrot {
//     saying:saying,
// }

// impl Parrot  {
//     fn talk() {

//     }
// }

// fn main(){

//     // let p = Person::new("John", 5);
//     // let p = Person::new("777".to_string());

//     // println!("{:?}", p);

//     pub trait talkative for Person{
//         fn talk(&self) -> String;
//     }
//     pub trait talkative for Parrot{
//         fn talk(&self) -> String;
//     } 


//     let me = Person::new("Hello!");
//     let bird = Parrot::new("Hi!");

//     let new_vec = vec!(me, bird);

//     for i in new_vec{
//         println!(i);
//     }

// }

struct Bitcoin{
    api_address:String,
    file_name:String,
};

struct Etherium{
    api_address:String,
    file_name:String,
};

struct SP500{
    api_address:String,
    file_name:String,
};

pub trait Pricing{
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self) -> f32;
}

impl Pricing for Bitcoin{
    fn fetch_price(&self) -> f32{
        return 32.0;
    }
    fn save_to_file(&self) -> f32{
        println!("saved to {:?}", self.file_name);
    }
}

struct BTCPriceAPI{
    Bitcoin: cost,
}

fn main(){
    let btc_api = "some random api address";
    let btc_txt = "btc_prices.json".to_string();
    let b = Bitcoin{api_address:btc_api, file_name:btc_txt};

}

// no asynchronus calls whatsoever