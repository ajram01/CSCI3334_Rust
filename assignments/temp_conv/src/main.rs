const FREEZE_WATER_FAHRENHEIT: f64 = 32.0;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64{
    return (fahrenheit - 32.0) * (5.0/9.0);
}

fn celsius_to_fahrenheit(celsius: f64) -> f64{
    return celsius * (9.0/5.0) + 32.0;
}

fn main(){
    let temperature: f64 = 45.0;

    let result: f64 = fahrenheit_to_celsius(temperature);

    println!("{} degrees F converts to {} degrees C", temperature, result);

    let mut count :f64 = 0.0;

    loop{
        count += 1.0;
        let new_temp = temperature + count;

        let result = fahrenheit_to_celsius(new_temp);
        println!("{} degrees F converts to {} degrees C", new_temp, result);

        if count == 5.0{
            break;
        }
    }

}