// an enum has many variants
enum IpAddrKind {
    V4,
    V6,
}

// associated pieces of data for each variant in an enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home1 = IpAddrKind::V4;
    let loopback1 = IpAddrKind::V6;

    let home2 = IpAddr::V4(127, 0, 0, 1);
    let loopback2 = IpAddr::V6(String::from("::1"));
}

enum Message {
    Quit,                       // no data association
    Move { x: i32, y: i32 },    // has named fields, like a struct does
    Write(String),              // includes a single String, like a tuple dict WriteMessage(String)
    ChangeColor(i32, i32, i32), // includes 3 i32 integers, like a tuple dict ChangeColorMessage(i32, i32, i32)
}

// enums can also implement methods
impl Message {
    fn call(&self) {
        // method body defined here
    }
}

fn main2() {
    let m = Message::Write(String::from("hello world"));
    m.call();
}

fn main3() {
    let some_number = Some(5); // type inference from value passed to Some()
    let some_color = Some("red");

    let absent_number: Option<i32> = None;
}
