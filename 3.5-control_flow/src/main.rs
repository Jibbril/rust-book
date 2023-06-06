fn main() {
    standard_if_else();
    let_if();

    try_loop();
    loop_with_return();

    try_while();

    try_for();
    conditional_for();

    let temp = convert_temperature(22.0, "C");
    println!("Temperature is: {}", temp);

    let n = 15;
    let fib = fibonacci(n);
    println!("Fibnr for argument {} was {}",n,fib);
    
    christmas_song();
}

fn let_if() {
    let number = if true { 3 } else { 5 };
    println!("{}", number);

    // Doesn't work since returned types from if are not the same
    // let number = if true { 3 } else { "six" };
    // println!("{}",number);
}

fn standard_if_else() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was true");
    }
}

fn try_loop() {
    let mut i = 0;
    loop {
        i += 1;
        println!("Loop nr: {}", i);
        if i >= 10 {
            break;
        }
    }
}

fn loop_with_return() {
    let mut i = 0;

    let result = loop {
        i += 1;
        if i >= 10 {
            break i * 5;
        };
    };
    println!("Result is: {}", result);
}

fn try_while() {
    let mut number = 3;

    while number > 0 {
        println!("Countdown: {}", number);
        number -= 1;
    }
    println!("Liftoff!");
}

fn try_for() {
    let arr = [1, 2, 3, 4, 5];
    for num in arr {
        println!("The value is: {}", num);
    }
}

fn conditional_for() {
    for number in (1..4).rev() {
        println!("Countdown: {}", number);
    }
    println!("Liftoff!");
}

fn convert_temperature(val: f32, temp: &str) -> f32 {
    match temp {
        "C" => (val * 9.0 / 5.0) + 32.0,
        "F" => (val - 32.0) * 5.0 / 9.0,
        _   => 0.0
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut i = 0;
    let mut m = n;
    let mut prev_sum = 0;
    let mut new_sum = 0;
    let mut temp_sum;    
    while m > 0 {
        if i == 1 { new_sum = 1 }; 
        i += 1;
        temp_sum = prev_sum + new_sum;
        prev_sum = new_sum;
        new_sum = temp_sum; 
        m -= 1;
    }
    return new_sum;
}

fn christmas_song() {
    let lyrics = [
        "On the first day of Christmas
        My good friends brought to me
        A song and a Christmas tree",
        "On the second day of Christmas
        My good friends brought to me
        Two candy canes
        And a song for the Christmas tree",
        "On the third day of Christmas
        My good friends brought to me
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the fourth day of Christmas
        My good friends brought to me
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the fifth day of Christmas
        My good friends brought to me
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the sixth day of Christmas
        My good friends brought to me
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the seventh day of Christmas
        My good friends brought to me
        Candles a-glowing
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the eighth day of Christmas
        My good friends brought to me
        Gold and silver tinsel
        Candles a-glowing
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the ninth day of Christmas
        My good friends brought to me
        A guardian angel
        Gold and silver tinsel
        Candles a-glowing. 
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the tenth day of Christmas
        My good friends brought to me
        Some mistletoe
        A guardian angel
        Gold and silver tinsel
        Candles a-glowing
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the 11th day of Christmas
        My good friends brought to me
        Gifts for one and all
        Some mistletoe
        A guardian angel
        Gold and silver tinsel
        Candles a-glowing
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree",
        "On the 12th day of Christmas
        My good friends brought to me
        All their good wishes
        Gifts for one and all
        Some mistletoe
        A guardian angel
        Gold and silver tinsel
        Candles a-glowing
        Little silver bells
        A shining star
        Four colored lights
        Three boughs of holly
        Two candy canes
        And a song for the Christmas tree"
    ];

    for verse in lyrics {
        println!("{}",verse);
        println!("")
    }
}