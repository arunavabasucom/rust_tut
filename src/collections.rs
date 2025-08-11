// collections are data types allocated on to the heap
// so they can grow and shrink as they neeed 

pub fn main(){
    //array 
    let a =[1,2,3];
    
    // vector
    let vec1:Vec<i32> = Vec::new();
    // we need to add mut keyword to push it into the vector
    let mut vec2:Vec<i32> = Vec::new();
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    // if it is out of scope it goint to drop it's values 
    {
    let vec3:Vec<i32> = vec![1,2,3];
    }
    // 2. accessing the datatypes 
    let v =vec![1,2,3,4,5,6];
    let third = &v[3];
    println!("{}",third);
    // array size is  pre-defined so we can get the error if we want to access the invalid array index at compile time 
    // but vec are not like that so if we want to access a invalid memory address then we get error in the 
    // runtime 
    // so we can use `get` method to use the access the index handle the use cases 
    match v.get(2) {
        Some(third)=>println!("{}",third),
        None => println!("There is no third element")
    }


}