enum IpAddrKind {
    V4,
    V6
}

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
    XzibitEnum(IpAddrKind) // Enum as argument of enum
}

struct Ipv4Addr { /* content */ }
struct Ipv6Addr { /* content */ }

// Possible to use structs as argument of enums
enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32)
}

// Can implement methods on enums
impl Message {
    fn call(&self) {
        // Logic
    }
}

fn main() {
    create_variant();
    enum_function();
    create_ip_address();
    options();
}

fn create_variant() {
    let first = IpAddrKind::V4;
    let second = IpAddrKind::V6;
}

fn enum_function() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {
    // Accepts both V4 and V6
}

fn create_ip_address() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn options() {
    let some_number = Some(5);
    let some_string = Some("string");
    let absent_number: Option<i32> = None;
}