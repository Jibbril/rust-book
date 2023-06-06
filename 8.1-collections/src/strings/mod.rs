
pub fn main() {
    println!("================ main called from strings! ================");
    create_empty();
    create_string();   
    append_string();
    append_char();
    string_format();
    iterating_strings();
}

fn create_empty() {
    let _s = String::new();
}

fn create_string() {
    println!("--------------- String init ---------------");
    let s1 = "hej".to_string();
    let s2 = String::from("då");

    println!("s1: {}, s2: {}",s1,s2);
}

fn append_string() {
    println!("--------------- String append ---------------");
    let mut s = "Hej".to_string();
    s.push_str(" då");
    println!("Append string gives: {}",s);
}

fn append_char() {
    println!("--------------- Char append ---------------");
    let mut s = "Hej".to_string();
    s.push('!');
    println!("Append char gives: {}",s);
}

fn string_format() {
    println!("--------------- String format ---------------");
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Format gave: {}",s);
}

fn iterating_strings() {
    println!("--------------- Iterate strings ---------------");
    let s = "halloj".to_string();

    // chars give you the characters that make up the String
    for c in s.chars() {
        println!("{}",c);
    }

    // bytes give you the bytes that the represent the characters. Depending 
    // on language/letter a single char may be represented as many bytes. 
    for b in s.bytes() {
        println!("{}",b);
    }
}