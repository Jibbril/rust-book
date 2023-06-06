/**
 * Main takeaways:
 * 
 * 1: The & symbol creates a reference to a given variable. This reference can
 * then be passed around and used  in new contexts. When the reference is 
 * removed at the end of context, the original object will not disappear. 
 * 
 * 2: The process of using references is called borrowing.
 * 
 * 3: You can make references mutable, see "mutable_reference()"
 * 
 * 4: You can make several immutable references at once, but only one mutable.
 * You can however make a mutable reference after an immutable one if there 
 * are no occurances of the immutable reference below the assignment of the
 * mutable one. See "scopes()" for examples.
 */
fn main() {
    references_example();
    mutable_reference();
    scopes();
}


fn references_example() {
    let s1 = String::from("Hello");
    let len = length(&s1);
    println!("The length of {} is {}",s1,len);
}

fn length(s: &String) -> usize {
    s.len()
}

fn mutable_reference() {
    let mut s = String::from("Hejsan");
    change(&mut s);
    println!("'Hejsan' became '{}'",s);
}

fn change(s: &mut String) {
    s.push_str(" svejsan!");
}

fn scopes() {
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}",r1,r2);

    let r3 = &mut s;
    r3.push_str(", world!");
    println!("r3: {}",r3);
}