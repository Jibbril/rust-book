/**
 * Main takeaways: 
 * 
 * 1: String slices are references to parts of a String. The data structure 
 * essentially consist of length (number of slots) and a pointer pointing 
 * towards an array in memory. 
 * 
 * 2: "danger()" explains a very interesing error scenario where we modify the
 * basis for a function result making the result incorrect. Slicing solves this. 
 * 
 * 3: Slices are more general than the full class. For example in first_word()
 * using &str for argument type is better than using &String because it can 
 * now accept both full Strings and slices. This is due to a full String 
 * being considered the same as a string sliced from start to end of the String. 
 */
fn main() {
    let mut s = String::from("hej");
    let len = first_word_unsafe(&s);

    s.push_str(" då");

    println!("'{}' has length: {}",s,len);

    slice();
    ranges();
}

fn ranges() {
    let s = String::from("Hello");

    let  slice1 = &s[..2]; // Same as &s[0..2]
    let  slice2 = &s[2..]; // Same as &s[2..s.len()]

    println!("{}",slice1);
    println!("{}",slice2);
}

fn slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}",hello);
}

fn first_word_unsafe(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_safe(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn danger() {
    let mut s = String::from("hello world");

    let len = first_word_unsafe(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}