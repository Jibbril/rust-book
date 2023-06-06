pub fn main() {
    println!("================ main called from challenges! ================");
    get_median();
}

fn get_median() {
    println!("--------------- get median ---------------");

    let mut arr = [1, 2, 3];
    arr.sort_unstable();

    let len = arr.len();
    let even = len % 2;

    match even {
        1 => println!("Odd!"),
        _ => println!("Even!"),
    }
}
