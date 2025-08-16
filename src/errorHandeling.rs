/*
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#listing-2-6
https://doc.rust-lang.org/book/ch11-01-writing-tests.html?highlight=impl%20Guess#checking-for-panics-with-should_panic

most of the to handle out the the errors we gonna use the error propagation and proper match handeling to do that 
panic! macro generally -> we can use where the recovery is not possible from the error and in other functions 
like unwrap,except , test or prototype function where the we just wnat to test out the code in that case we can use 
the panic! macro
*/
use std::error::Error as Errors;
use std::fs::{self,File};
use std::io::{self,Error,ErrorKind,Read};
use std::net::IpAddr;
// pub fn main(){
//     panic!("crash and burn");
    
// }
/*
thread 'main' panicked at src/errorHandeling.rs:17:9:
Do not pass 22!
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: rust_tut::errorHandeling::c
             at ./src/errorHandeling.rs:17:9
   3: rust_tut::errorHandeling::b
             at ./src/errorHandeling.rs:13:5
   4: rust_tut::errorHandeling::a
             at ./src/errorHandeling.rs:10:5
   5: rust_tut::errorHandeling::main
             at ./src/errorHandeling.rs:6:5
   6: rust_tut::main
             at ./src/main.rs:89:5
   7: core::ops::function::FnOnce::call_once
             at /private/tmp/rust-20250515-8123-mdgm7y/rustc-1.87.0-src/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

*/
pub fn main(){
    // a();

    /*
    // in cases where we can handlw the error gracfully and want to handle it gracefully then
    // like the optional enum -> Ok(success) and error(holds generic errors)
    // it is brough into the scope by default 
    enum  Result<T,E> {
        OK(T),
        Err(E)
    }
    */
    let f = File::open("Hello.txt");
    let f = match f {
        Ok(file)=>file,
        // Err(error)=>panic!("problem opening the file{:?}",error)
         Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem ceating the file {:?}",e)
            },
            other_error=>{
               panic!("Problem opening the file{:?}",other_error)
            }
         }
    };
    // using clousures 
    let f: File = File::open("hello.txt").unwrap_or_else(|error: Error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error: Error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // unwrap does the same thing to handle the things it gonna get the file if all things are ok 
    // gonna panic if there is an error 
    let file = File::open("Hello.txt").unwrap();
    // also we have except method where we can pass the message that we want to pass to the error message 
    let files = File::open("Hello.txt").expect("error opening the file");


}
fn a(){
    b();
}
fn b(){
    c(20);
}
fn c(num:i32){
    if(num == 22){
        // if we do not know how to handle out the error 
        // we can use panic macro to exit out of the program 
        panic!("Do not pass 22!") 
    }
}

/* 
Error propagaton -> in some cases we want to handle the error in the function caller side so that caller can decide
what to do with the error no going to handle in the function side 
*/ 
fn read_username_from_file() -> Result<String, io::Error> {
    //let mut f= File::open("hello.txt")?;


    // let mut f: File = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
   //f.read_to_string(&mut s)?;
    //Ok(s)

    // more simplified using channing
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)

    // using built in method 
    fs::read_to_string("hello.txt")
}
// Box<dyn Error> -> any type of error
fn main_ii()->Result<(),Box<dyn Errors>>{
    // in this case the ip is hard coded so it would not fail but if it is coming from a dynamic source thwn we want to 
    // use match expression and other ways to handle the cases properly 
    let home:IpAddr ="127.0.0.1".parse().unwrap();
    
    
    let f = File::open("hello.txt")?;
    Ok(())

}

