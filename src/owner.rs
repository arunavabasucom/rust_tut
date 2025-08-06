// accessing data in the stack is faster than in the heap
// alocating memory in the heap is much more slower than stack
// stack frame size caculated at compile time
// when a function get excuting all the local variables get dropped in the stack

pub fn main(){
    // ownership rules 
    // 1. each value in rust has a variable that's called it's owner .
    // 2. There can only be one owner at a time.
    // 3. when the owner goes out of scope the values get dropped  
    // fn a(){
    //     let x = "hello";
    //     let y = 22;
    //     b();
    // }
    // fn b(){
    //     let x = String::from("world"); // dymanic in size

    // }

    // scope 
    {
        // string literal 
        let sl ="Hello"; // it allocated in stack(sting literal)
        let s = String::from("hello");// it is in the heap dymaic in size
        // alocate memory on the heap in cpp generally we use new keyword
        // but in rust the value is allocated on the heap automattically but when the scope ends the var 
        // gets dropped
        // do stuff with s 
    }// scope ends 


    let x = 5;
    let y =x; //copy -> rust trait genearlly copy the simple data type and it is stored in the stack

    let s1 = String::from("Hello");
    // let s2 = s1;//move
    let s2= s1.clone();//actully get copied


    // we can not access the value of s1 because it is moved to s2

    println!("{}",s1);

}