use std::io;
fn main() {
    println!("Temperature Converter!");

    println!(" 1: To convert F to C");
    println!(" 2: To convert C to F");

    let mut choise = String::new();

    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to read line");

    let choise: i32 = choise.trim().parse().expect("Please Enter a number");

    println!("choise : {}", choise);

    println!("Please enter temperature: ",);

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = temp.trim().parse().expect("Please enter a number");

    if choise == 1 {
        let temp :f32= (temp - 32.0) / 1.8;
        println!("{} C", temp);
    } else if choise == 2 {
        let temp:f32 = (temp * 1.8) + 32.0;
        println!("{} F", temp);
    } else {
        println!("Please enetr a vaild input!");
    }
  
}

