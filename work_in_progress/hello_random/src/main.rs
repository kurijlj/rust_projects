use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Hello, {random_number}!");

}
