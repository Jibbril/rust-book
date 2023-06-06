#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, s: u32) {
        self.height = self.height * s;
        self.width = self.width * s;
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        self.area() > r.area()
    }

    fn square(size:u32) -> Rectangle {
        construct_rectangle(size, size)
    }
}

fn main() {
    use_methods();
    can_hold_example();
    associated_functions();
}

fn use_methods() {
    let mut rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("Area is {}",rect.area());
    rect.scale(2);
    println!("Area is {}",rect.area());
}

fn can_hold_example() {
    let rect1 = construct_rectangle(30, 50);
    let rect2 = construct_rectangle(30, 50);
    println!("Can rect2 hold rect1? {}",rect2.can_hold(&rect1));
}

fn construct_rectangle(width:u32, height: u32) -> Rectangle {
    Rectangle {
        width,
        height
    }
}

fn associated_functions() {
    // We can create general (think 'static' from java) methods by leaving out
    // the &self argument in the impl block.
    let rect = Rectangle::square(30);
}