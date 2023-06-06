#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    basic_area();
    tuple_area();
    struct_area();
    debug_print_area();
    dbg_output_area();
}

fn basic_area() {
    let w = 30;
    let h = 50;

    println!("Area is: {}", area1(w, h));
}

fn area1(w: u32, h: u32) -> u32 {
    w * h
}

fn tuple_area() {
    let rect = (30, 50);
    println!("Area is: {}", area2(rect));
}

fn area2((w, h): (u32, u32)) -> u32 {
    w * h
}

fn struct_area() {
    let s = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area is: {}", area3(&s));
}

fn area3(rectangle: &Rectangle) -> u32 {
    let Rectangle { width, height } = rectangle;
    width * height
}

fn area3_alt(Rectangle { width, height }: &Rectangle) -> u32 {
    width * height
}

fn debug_print_area() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:#?}", rect);
}

fn dbg_output_area() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30*scale),
        height: 50
    };

    dbg!(&rect);
}