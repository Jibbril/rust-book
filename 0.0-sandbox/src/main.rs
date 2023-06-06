

const MODIFIERS: [char;3] = ['I', 'X', 'C'];
const MODIFIABLE: [char;6] = ['V','X','L','C','D','M'];
const NUM_MAP: HashMap<char,usize> = HashMap::from([
    ('I', 1),
    ('V', 5),
    ('X', 10),
    ('L', 50),
    ('C', 100),
    ('D', 500),
    ('M', 1000)
]);

use std::str::Chars;
use std::iter::Peekable; 
use std::collections::HashMap;

fn main() {
    problem();
}

fn problem() {
    let s = String::from("MCMXCIV");
    let res = roman_to_int(s);
    println!("{}",res);
}

pub fn roman_to_int(s:String) -> i32 {
    let mut sum = 0;
    for (i,c) in s.chars().enumerate() {
        if (i < s.len() - 1 && NUM_MAP[&c] )
    }

}

pub fn roman_to_int_first(s: String) -> i32 {
    let mut iter = s.chars().peekable();
    let mut sum = 0;
    let mut i = 0;
    while i < s.len() {
        println!("==========================");
        println!("Sum: {}",sum);
        println!("Iter: {:?}",iter);

        if let Some(c) = iter.next()  {
            let (is_modifier,val) =  handle_modifiable(&c,&mut iter);
            println!("Found val: {}",val);
            if is_modifier { i += 1; }
            sum += val;
            i += 1;
        } else { break }
    }
    sum
}

fn handle_modifiable(c: &char, iter: &mut Peekable<Chars>) -> (bool,i32) {
    if MODIFIERS.contains(&c) {
        if let Some(next) = iter.peek() {
            if MODIFIABLE.contains(&next) {
                if (is_modifiable(&c,&next)) {
                    resolve_modifiable(&c,&iter.next().unwrap())
                } else {
                    (false,resolve(&c))
                }
            } else {
                (false,resolve(&c))
            }
        } else { (false, resolve(&c)) }
    } else {
        (false,resolve(&c))
    }
}

fn is_modifiable(c: &char, next: &char) -> bool {
    match c {
        'I' => {
            match next {
                'V' => true,
                'X' => true,
                _   => false
            }
        },
        'X' => {
            match next {
                'L' => true,
                'C' => true,
                _   => false
            }
        },
        'C' => {
            match next {
                'D' => true,
                'M' => true,
                _   => false
            }
        },
        _   => false
    }
}

fn resolve_modifiable(c: &char, next: &char) -> (bool,i32) {
    match c {
        'I' => {
            match next {
                'V' => (true,4),
                'X' => (true,9),
                _   => (false,1)
            }
        },
        'X' => {
            match next {
                'L' => (true,40),
                'C' => (true,90),
                _   => (false,10)
            }
        },
        'C' => {
            match next {
                'D' => (true,400),
                'M' => (true,900),
                _   => (false,100)
            }
        },
        _   => (false,0)
    }
}

fn resolve(c: &char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _   => 0
    }
}