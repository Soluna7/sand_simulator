use rand;

const SIZE: usize = 32;

fn main() {
    let my_array: [[[f64; SIZE]; SIZE]; SIZE] = rand::random();
    println!("{:?}", my_array);
}