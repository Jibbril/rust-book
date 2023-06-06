pub fn main() {
    println!("================ main called from vectors! ================");
    init_vectors();
    update_vectors();
    reading_vectors();
    iterate_vectors();
    enum_vectors();
}

fn init_vectors() {
    let _v1: Vec<i32> = Vec::new();
    let _v2 =  vec![1,2,3];
}

fn update_vectors() {
    println!("--------------- Vector updates ---------------");
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    println!("{:?}",v);
    v.pop();
    println!("{:?}",v);
    v.clear();
    println!("{:?}",v);
}

fn reading_vectors() {
    println!("--------------- Reading vectors ---------------");
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2]; // Causes program to panic if index doesn't exist
    println!("The third element is {}",third);

    match v.get(2) { // Optional so fine even if index does not exist
        Some(third) => println!("The third element is {}",third),
        None => println!("There is no third element")
    }
}

fn iterate_vectors() {
    println!("--------------- Iterating vectors ---------------");
    let mut v = vec![1,2,3];

    for i in &v {
        println!("{}",i);
    }

    for i in &mut v {
        *i += 5;
        println!("{}",i);
    }
}

struct Multiple(i32,String);
enum Number {
    Int(i32),
    Float(f32)
}
fn enum_vectors() {
    println!("--------------- Enum vectors ---------------");
    let _v1 = vec![
        Multiple(1,String::from("One")), 
        Multiple(2,String::from("Two"))
    ];

    let _v2 = vec![Number::Int(1), Number::Float(1.0)];
}