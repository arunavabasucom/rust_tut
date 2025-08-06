// insted of taking ownership it does not take ownership 
// and insted of pointing to the whole collection we can point it out some of the part of it  
fn main(){
    let mut s = String::from("hello world");
    let hello = &s[0..5]; // if it first of the then we can simply [...5]
    let world = &s[6..11];// if it last of the string then we can use [6...]
    

}
