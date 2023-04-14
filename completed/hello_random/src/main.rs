use rand::Rng;

fn main() {
    let app_name = "hello_random";
    let random_number = rand::thread_rng().gen_range(1..=100);
    let message = format!("{app_name}: Hello, {random_number}!");

    // Print message to stdout
    println!("{message}");

    // Pass message to a function
    take_ownership(&message);

    println!("{message}");

}

fn take_ownership(msg: &String) {
    println!("{msg}");

}
