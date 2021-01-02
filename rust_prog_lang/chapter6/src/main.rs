
enum IpAddrKind {
    V4,
    V6,
}


struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

enum NewIpAddr{
    V4(String),
    V6(String),
}

enum NewIpAddr2{
    V4(u8, u8, u8, u8),
    V6(String),
}


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

enum Coin{
  Penny,
  Nickel,
  Dime,
  Quarter  
}

enum Coin2{
    Penny,
    Nickel,
    Dime,
    Quarter(String)  
  }
  


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = NewIpAddr::V4(String::from("127.0.0.1"));
    let loopback = NewIpAddr::V6(String::from("::1"));

    let home = NewIpAddr2::V4(127,0,0,1);
    let loopback = NewIpAddr2::V6(String::from("::1"));

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let
    let some_u8_value: Option<u8> = Some(5);
    if let Some(3) = some_u8_value {
        println!("Three!");
    }

    let coin = Coin2::Quarter(String::from("Alabama"));
    if let Coin2::Quarter(state) = coin {
        println!("Moneda de {}", state);
    }

}


fn value(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

