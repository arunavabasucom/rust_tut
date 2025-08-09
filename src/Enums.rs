use std::os::macos::raw::stat;

// we can also store we to mention in parenthesis
// enum  IPAddressKind {
//     V4(String),
//     V6(String)
// }
enum  IPAddressKind {
    V4(u8,u8,u8,u8),
    V6(String)
}
struct IPAdrr{
    kind:IPAddressKind,
    address:String
}
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}
impl Message {
    fn some_fn(){
        println!("Hello World");
    }
}

// optional Enum 
// enum Option<T> {
//     Some(T),
//     None
// }

// match expressions
#[derive(Debug)]
enum State {
    WB,
    DL,
    MA,
    AP

} 
enum Coin{
    Penny,
    Nikel,
    Dime,
    Qurter(State)
}
fn value_in_cents(coin:Coin)->u8{
    match coin {
        Coin::Penny => {
            println!("Luckey Penny");
            1
        }
        Coin::Dime => 5,
        Coin::Nikel =>10,
        Coin::Qurter(state   )=>{
            println!("State qurter from {:?}!",state);
            25
        }
    }
}
fn plus_one(x:Option<i32>)->Option<i32>{
 // in match expression we need match all of the possible varients
 match x {
     None =>None,
     Some(i)=>Some(i+1)
 }
}
pub fn main(){
    // let four = IPAddressKind::V4;
    // let six =IPAddressKind::V6;
    // let localhost = IPAdrr{
    //     kind:IPAddressKind::V4,
    //     address:String::from("127.0.0.1");
    // };

    // let localhost = IPAddressKind::V4(String::from("127.0.0.1"));
    // let localhost = IPAddressKind::V4(127,0,0,1);
    // let some_number = Some(5);
    // let some_string = Some("a string");
    // let absent:Option<i32> = None;

    let x:i8 = 5;
    let y:Option<i8> =Some(5); //optional enum -> some value, none
    // let sun = x+y;// we can not really add this 
    // to add up this we need handle all of the cases 
    let sum = x+y.unwrap_or(0);
    value_in_cents(Coin::Qurter(State::WB));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);
    match some_value{
        Some(3)=>println!("three"),
        // we want to takle out the some(3), other values going to takle out by the 
        // this function 
        _ =>()

    }

    // if else syntax 
    // it was less verbose than the match expressions
    if let Some(3) = some_value{
        println!("three");
    }

}
