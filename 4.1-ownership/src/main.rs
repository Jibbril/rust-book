/**
 * Main takeaways:
 * 
 * 1: Rust has a special memory management system based around closures. All 
 * data saved to the heap is automatically removed at the end of scope. 
 * 
 * 2: To ensure freeing of data is done correctly, assigning a variable to
 * another actually invalidates the previous variable (see "invalidation()").
 * 
 * 3: Providing a variable pointing towards the heap into a function removes
 * it from the current scope (see "dinner()").
 * 
 * 4: The term for a variable assigned with a value is owner. So in 
 * let a = 2; a is the owner of the value 2. 
 */
fn main() {
    dinner();
    modify_string();
    invalidation();
    try_clone();
}

fn dinner() {
    let s = String::from("This will be eaten.");
    let t = eat(s);
    // Doesn't work since s has been passed into eat().
    // println!("{}",s);
    println!("Food left: {}", t);
}

fn eat(food: String) -> String {
    return food;
}

fn modify_string() {
    let mut s = String::from("Hey");
    s.push_str(", world!");
    println!("{}",s);
}

fn invalidation() {
    let s1 = String::from("Hejsan");
    let s2 = s1;
    // Doesn't work because the first value is invalidated by the move.
    // println!("{}",s1);
}

fn try_clone() {
    let a = String::from("hej");
    let b = a.clone();
    println!("a: {}, b: {}",a,b);
}