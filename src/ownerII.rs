fn main(){
    let s = String::from("hello");
    takes_ownership(s);
    //println!("{}",s); // so the ownership is moved to `takes_ownership()` so we cann't print the s variable


    let x = 5;
    makes_copy(x);
    println!("{}",x); // so we print this because the value is copied from the x variable 
    // because int or some small variable are being copied not ownership get moved

    let sa = gives_ownership(); // now the ownership of the ss varible is moved to this s variable 
    println!("{}",sa);


    let s2 = String::from("hello");
    let s3 = takes_and_gives_back_ownership(s2); // it taking the ownership of the s2 and returing back the ownership to s3  
    println!("{}",s3)
}
fn takes_ownership(s:String){
 println!("{}",s);
}

fn makes_copy(i:i8){
    println!("{}",i);
}

// returing a string 
fn gives_ownership()-> String {
    let ss = String::from("hello");
    ss
}
fn takes_and_gives_back_ownership(a:String)->String{
    a
}