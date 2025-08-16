// collections are data types allocated on to the heap
// so they can grow and shrink as they neeed 
use unicode_segmentation::UnicodeSegmentation;
//hashmap 
use std::collections::HashMap;
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
    let mut v =vec![1,2,3,4,5,6];
    let third = &v[3];
    // v.push(6);// in vectorw we can not take immutable and mutable refs at the same time 
    println!("{}",third);
    // array size is  pre-defined so we can get the error if we want to access the invalid array index at compile time 
    // but vec are not like that so if we want to access a invalid memory address then we get error in the 
    // runtime 
    // so we can use `get` method to use the access the index handle the use cases 
    match v.get(2) {
        Some(third)=>println!("{}",third),
        None => println!("There is no third element")
    }
    // iteration in vectors

    for i in &v{
        println!("{}",i);
    }
    // iteration();
    // str_referencing();
    // hashmaps();
    hashmap_example();
    
    
}
fn iteration(){
    let mut v =vec![1,2,3,4,5,6];
    //normally when we iterate we take a immutable refs 
    // but we can take mutable refs also 
    for i in &mut v{
        *i += 50;
        // * dereferance operator with this geting the underline value of the 
    }
    for i in &v{
        println!("{}",i);
    }
}
fn main_ii(){
    //suppose we want the the vector going to reprsent the different types of datatypes in that case 
    // we use enum to do that 
    enum SpredSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row= vec![
        SpredSheetCell::Int(3),
        SpredSheetCell::Float(4.15),
        SpredSheetCell::Text(String::from("Hello World"))
    ];
    //the main catch is that we need to match out the type when we want to 
    // figure out the which type of the data it is using match expression 
    match &row[1]{
        // if it is an integer print out the 
        SpredSheetCell::Int(i)=>println!("{}",i),
        _=>println!("Not a Integer")
    }
}
fn strings(){
    //Strings stored as collections of UTF-8 encoded bytes
    // in ASCII each charcter can represent as 1 byte(and only 7 bits used to represent the charcters means it can be used 
    // to represnt 128 unique characters) but UTF-8 it can be represnt(it is backward complatable with ASCII)
    // in UTF-8(1,2,3,4 bytes)

    let s1 = String::new();
    let s2 = "initial contents"; // string literal
    let s3 = s2.to_string(); // string literal to string 
    let s4 = String::from("initial commit");

    // let 
    let mut s = String::from("foo");
    s.push_str("bar"); // we gonna pass string literal not the string because not gonna take the ownership of the string
    s.push('!');

    let ss1 = String::from("Hello, ");
    let ss2 = String::from("World!");
    // moving owmership of ss1 to ss and then we are adding ss2 all characters to s3
    //let ss = ss1 + &ss2; 
    //without taking ownership of the string using format macro
    let sss = format!("{}{}",ss1,ss2);

}
fn str_referencing(){
    let s = String::from("Hello");
    let r = String::from("Здравствуйте"); 
    /*
    hello in russian -> in the english word each characters is in one byte but in russian 
    it is two bytes to by referencing with interger it will give half of the russian word which going to be 
    weired 
    */
    // let h1 = s[0]; // we cann't index the string as a integer 

    let hello: String = String::from("नमस्ते");
    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for b in "नमस्ते".bytes(){
        println!("{}",b);
    }
    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']
    for b in "नमस्ते".chars(){
        println!("{}",b);
    }

    // Grapheme clusters -> it is not inculded in the std library of rust(we need to use unicode-segmentation to do that)
    // ["न", "म", "स्", "ते"]
    for b in "नमस्ते".graphemes(true){
        println!("{}",b);
    }    
}
fn hashmaps(){
    // hashmaps used to store key-value pairs
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10); 
    scores.insert(yellow,50);
    //we are not passing the referance so it will move the ownership to hashmap
    // println!("{}",blue);
    let score = scores.get(&String::from("Hello"));//the get method takes a referance of the key 
    for (key ,value) in &scores{
        println!("{}{}",key,value);
    }

    let mut scored = HashMap::new();
    scored.insert(String::from("Blue"), 10);
    scored.insert(String::from("Blue"), 20); // it will update the value 10->20

    scored.entry(String::from("Yellow")).or_insert(10);
    /*
    it will this key holdes a value or not then it will insert the value 
    if there is no value there if there is already a value inserted then we gonna insert the value
    */
    scored.entry(String::from("Yellow")).or_insert(20); 
}
fn hashmap_example(){
    let text = "Hello World Wonderful World";
    let mut map = HashMap::new();
    print!("hashmap");
    for word in text.split_whitespace(){
        /*
        it will check that the value already there is not if 
        1. yes -> it will do nothing 
        2. no -> it will insert it value and give a mutable referance 
        */
        let count = map.entry(word).or_insert(0);
        println!("{}",count);
        // dereferance it 
        *count += 1;     
    }
    println!("{:?}",map);
}