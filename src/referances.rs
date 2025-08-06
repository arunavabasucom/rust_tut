// referances in rust does not take ownership of a value 
// referances are immutable by default

pub fn main(){
    let s1 = String::from("Hello");
    let len = cal_len(&s1); // the &s (referances means the memory address)-> &s1 -> pointing to the actual value
    println!("Lenght of the string is {}",len)

}
fn cal_len(s:&String)->usize{
    // s.push_str("oops!!");
    let len =s.len();
    len
}

// to modify the value in referce 
fn main_ii(){
    let mut s1 = String::from("Hello");
    change(&mut s1); //pasing mutable referance
}

fn change(some_string: &mut String){
    some_string.push_str(", World");
}

pub fn main_iii(){
    // if we remove the mut keyword the we can use ref multiple time -> only readable 
    let mut s = String::from("Hello");
    // we cann't decalre more than one mutable refs -> to prevent one pointer to update at the sane read or write 
    // but we can have more than one immutable refs 
    // we can not have mutable ref if any other immutable ref exsists
    let r1 = &mut s;
    
    // let r2 = &mut s;
    // println!("{}{}",r1,r2);

}


pub fn main_iv(){

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}{}",r1,r2);

    // when the r1 and r2 scope ends we can create the mutables refs 
    let r3 =  &mut s;
    println!("{}",r3);// we can get r3 

}


// dangle 

// fn main_v(){
//  let ref_to_nothing = dangle();
// }
// fn dangle()->&String{
//     let s = String::from("hello");
//     &s// we cann't really return the value because the scope ends after function complition so the value is dropped from the db

// }