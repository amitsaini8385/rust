use std::io;

fn main() {
    println!("Temperature convertor");

    loop {
        println!("Please enter the corrosponding number to select convertor");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("q. quit the program");

        let mut convertor: String = String::new();

        io::stdin()
            .read_line(&mut convertor)
            .expect("Failed to read");

        println!("You Entered {convertor}");
        let convertor = convertor.trim();

        if convertor == "1" {
            println!("Enter the tempearture in Fahrenheit");
            let mut temp: String = String::new();

            io::stdin().read_line(&mut temp).expect("Failed to read");
            let ftc: f64 = temp.trim().parse().expect("Failed to parse");
            let result: f64 = (ftc - 32.0) * (5.0 / 9.0);
            println!("The temprature in Celsius is {result}");
            break;
        } else if convertor == "2" {
            println!("Enter the tempearture in Celsius");
            let mut temp: String = String::new();

            io::stdin().read_line(&mut temp).expect("Failed to read");
            let ftc: f64 = temp.trim().parse().unwrap();
            let result: f64 = ftc * (9.0 / 5.0) + 32.0;
            println!("The temprature in Fahrenheit is {result}");
            break;
        } else if convertor == "q" {
            break;
        } else {
            println!("Please choose between given values");
        }
    }
}
