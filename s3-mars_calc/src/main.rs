use std::io;

fn main() {
    let mut user_input_weight = String::new();

    println!("Please enter your weight in kg:");
    io::stdin().read_line(&mut user_input_weight).unwrap();
    user_input_weight = user_input_weight.trim().to_string();

    let weight: f32 = user_input_weight.parse().unwrap();

    let mut mars_weight = weight;
    println!("Weight on mars: {:?}kg", calculate_weight_on_mars(mars_weight)); 
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
