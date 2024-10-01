const freezeWaterFahrenheit: f64 = 32

fn fahrenheit_to_celsius(fahrenheit: f64){
    return ((fahrenheit - 32) * (5/9))
}

fn celsius_to_fahrenheit(celsius: f64){
    return (celsius * (9/5) + 32)
}

fn main(){
    let temperature = 32

    println!("{}, degrees F converts to {} degrees Celsius" , temperature, fahrenheit_to_celsius(temperature))

}