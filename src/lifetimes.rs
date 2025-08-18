// generic lifetime annotations 
// it create generix realations between references
// it means the smallerlife time of the variable will be lifetime of all the others 
// the parctice is to use lowercase letter followed by abcd

// &i32 -> a referance 
// &'a i32 -> a referance with an explicit lifetime 
// &'a mut i32 -> a mutable referance with a explicit lifetime
fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
fn longest_x<'a>(x:&'a str,y:&str)->&'a str{
    x
}
// the lifetime is going to tied with the parameters because can not return a referance that created on the function 
// fn longest_y<'a>(x:&'a str,y:&'a str)->&'a str{
//     let res = String::from("Hello");
//     // res.as_str(); //we can not get the referance because when the  function ends the res referance will drop 
// }

// we can do this because it going to transfer the ownership
fn longest_z<'a>(x:&str,y:&str)->String{
    let res = String::from("Hello");
    res
}
pub fn main(){
 let s1 = String::from("abcd");
 let s2 = String::from("xyz");
 let res = longest(s1.as_str(), s2.as_str());
 
 // now borrow checker knows that the lifetime of the res going to be same as smallest of all lifetimes
 println!("{}",res);
}


pub fn main_ii(){
    let x = 5;
    let r = &x;
    println!("{}",r); // now x lives till the en=d of the program 
}
pub fn main_iii(){
    // let r;
    // dangling referances 
    //when the scope ends the  x will drop but it has a referance to r so we cannot get r 
    // rust knows this stuff in the compile time because it rus the borrow checker
    // to do so , r lives for the whole program but for x it drops after the scope 
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}",r);
}
pub fn main_iv(){
 let s1 = String::from("abcd");
 let res;
 {
    let s2 = String::from("xyz");
    //  res = longest(s1.as_str(), s2.as_str());
    // now we are caring about the x lifetime because it only returns a not dependent on the s2
    res = longest_x(s1.as_str(), s2.as_str());
    // res lifetime is same as s2's lifetime
    // println!("{}",res);
    // after the scope ends the s2 is dropped res going to point to a dangling referances 
} 
    println!("{}",res);

}