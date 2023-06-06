fn main() {
    println!("Hello, world!");

    another_function(5);
    statements_vs_expressions();
    return_values();
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn statements_vs_expressions() {
    // Statements are instructions that perform some action and do not return
    // a value. Expressions evaluate to a resulting value. Let’s look at some
    // examples.

    // Statement example
    let x = 5;

    // Statements don't return values and thus cannot be used in other
    // statements. The code below breaks
    // let a = (let b = 2);
    // let a = b = 2;

    // Blocks are expressions so the below works
    let a = {
        let x = 3;
        x + 1
    };
    println!("{}", a);
}

fn return_values() {
    println!("{}", return_5())
}

fn return_5() -> i32 {
    5
}
