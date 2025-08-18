mod flow;
mod owner;
mod ownerII;
mod referances;
mod slices;
mod guessing_game;
mod hello_world;
mod structs;
mod Enums;
mod lib;
mod collections;
mod errorHandeling;
mod generics;
mod traits;
mod lifetimes;
mod lifetimeii;
// https://doc.rust-lang.org/book/ch03-02-data-types.html?highlight=data#data-types
// fn main() {
    // normally variables in rust are immutable
    // but with the help of mut keyword we can change the mutability of the variable
    // let mut x = 5;
    // println!("The value of x is:{}",x);
    // x = 6;
    // println!("The value of x is:{}",x);

    // const keyword we declare constant this value never got changed
    // we can not add mutability to the constant
    // const SUBSCRIBER_COUNT:u32 = 100_000;
    
    // SHADOWING 
    // the first x variable is shadowed by the second x variable 
    // this way we preserve immutability 
    // stil we can mutate the variable
    // let  x = 5;
    // let x = "six";
    
// }
// fn main(){
    // SCALER DATATYPES 
    //integers
    /*
    signed -> + & - (2^{bit-1}-1)
    unsigned -> + (2^{bit}-1)
    */
    // let a = 98_222;//Decimal 
    // let b = 0xff;//Hex
    // let c = 0o77;//Octal
    // let d = 0b1111_0000;//Binary
    // let e = b'A';//binary
    // let f =255;

    //floating point numbers
    // let f:f32 = 2.0;
    // let f:f64 = 3.0;

//     //addition 
//     let sum = 5+10;
//     //substraction
//     let sub = 95.5-4.3;
//     //product
//     let product = 4*30;
//     //quotient
//     let quo = 56.7/32.2;
//     //remainder
//     let remaimder = 43%5;
//     //booleans
//     let t = true ;
//     let f = false;
//     //characters
//     let c = 'z';
//     let z ='🧘' ;
// }
fn main(){
    // compound types
    // tuple 
//     let tup=("Hello world",100_000);
//    let (name ,sub_count) = tup;
//    let sub_count = tup.1;

//    // array 
//    let error_codes = [200,404,500]; // array sized are fixed if we dynamicallt want to chnage it we need to use a vector
//    let not_found = error_codes[1];

//     let byte = [0;8]; // create a array of 8 values and set every value to zero
    // my_func(11,12);
    // flow::main();
    // owner::main();
    // referances::main_iv();
    // slices::main();
    // structs::main();
    // Enums::main();
    // collections::main();
    // errorHandeling::main();
    // generics::main();
    // traits::main();
    // lifetimes::main();
    lifetimeii::main();
}
// fn my_func(x:i8,y:i8)->i8{
//     println!("Hello from my_func");
//     // let sum = x+y;
//     // return sum;
//     // sum
//     x+y
// }