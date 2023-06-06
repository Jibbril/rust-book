fn main() {
    // Integers default to i32, and can be written with separators
    basic_assignment();
    mutability();
    closures();
    shadowing();
    tuples();
    arrays();
}

fn basic_assignment() {
    const X: i32 = 1_000;
    let mut x = 1001;

    println!("====================");
    println!("Should print 1000: {}",X);
    println!("Should print 1001: {}",x);
}

fn mutability() {
    let mut x = 1000;
    x = x + 100;
    println!("====================");
    println!("Should print 1101: {}", x);
}

fn closures() {
    println!("====================");
    let a = 2;
    {
        let a = 3;
        println!("{}",a);
    }
    println!("{}",a);
}

fn shadowing() {
    println!("========= shadow! ===========");
    let b = 2;
    let c = &b;
    let b = 5;
    println!("{}",b);
    println!("{}",c);
}

fn tuples() {
    println!("====================");
    let tup = (1,2,3);
    println!("tuples {}",tup.1);
}

fn arrays() {
    println!("====================");
    let arr: [&str; 3] = ["hej", "på", "dig"];
    println!("{}",arr[0]);

    let same_arr = [1; 5];
    println!("{}",same_arr[1]);
}
