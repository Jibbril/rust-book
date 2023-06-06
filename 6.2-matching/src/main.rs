enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska
}

fn main() {
    show_alabama();
    matching_option();
    catch_all();
}

fn show_alabama() {
    let a = Coin::Quarter(State::Alabama);
    let value = value_in_cents(a);
    println!("Value is: {}",value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny     => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel    => 5,
        Coin::Dime      => 10,
        Coin::Quarter(state)   => {
            println!("State quarter from {:?}",state);
            25
        }
    }
}

fn matching_option() {
    let a = plus_one(Some(3));
    let b = plus_one(None);

    println!("a: {:?}",a);
    println!("b: {:?}",b);
}

fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        Some(i) => Some(i + 1),
        None    => None
    }
}

#[derive(Debug)]
enum MyEnum {
    First,
    Second,
    Third
}

fn catch_all() {
    let a = catch_all_myenum(MyEnum::Third);
    println!("{:?}",a);
}

fn catch_all_myenum(e: MyEnum) -> MyEnum {
    match e {
        MyEnum::First   => MyEnum::Second,
        _               => e
    }
}
