fn main() {
    let t1 = temp_converter(Temperature::Celsius, 20.); // -7
    let t2 = temp_converter(Temperature::Fahrenheit, 10.); // 50
    println!("T1 is {t1}\n\
    T2 is {t2}\n");

    let result = get_fibonacci(8); //21
    println!("7th fibonacci number is {result}\n");


}

enum Temperature {
    Celsius,
    Fahrenheit
}
fn temp_converter(to: Temperature, value: f64) -> f64 {
    match to {
        Temperature::Celsius => {
            5. / 9. * (value - 32.)
        },
        Temperature::Fahrenheit => {
            (value * 9. / 5.) + 32.
        }
    }
}

fn get_fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 | 2 => 1,
        num => get_fibonacci(n - 1) + get_fibonacci(n - 2)
    }
}