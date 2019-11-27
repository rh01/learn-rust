#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let _home_ = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let _some_string = Some("a string");

    // 这里的None需要指定类型
    let _absent_number: Option<i32> = None;

    println!("Hello, world!");
    println!("some_number is {:?}", some_number)

    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);
    //cannot add `std::option::Option<i8>` to `i8`
    //let sum = x + y;
    //println!("sum is {}", sum)
}
