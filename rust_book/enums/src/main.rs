use std::fmt;

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IpAddrKind::V4(a,b,c,d) => write!(f, "{} {}.{}.{}.{}","V4",a,b,c,d),
            IpAddrKind::V6(addr) => write!(f, "{} {}", "V6", addr),
        }
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing for ip kind {}", ip_kind.to_string());
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    route(home);
    route(loopback);
}
