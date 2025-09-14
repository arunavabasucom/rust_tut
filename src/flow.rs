pub fn main(){
    // control flow
    
    // IF ELSE STATEMENT
    // let number = 8;
    // // we need to explicit about the condition 
    // if number < 10 {
    //     println!("first condition was true");
    // }else if number < 22 {
    //     println!("second conditon was true");
    // }
    // else {
    //     println!("condition was false");
    // }
    
    // OTHER TYPE
    // let condition = true;
    // let number = if condition {5} else {6};

    // // LOOP
    loop{
        println!("Hello");
        break; // for breaking the loop
    }

    // let counter = 0;
    // loop{
    //     counter += 1;
    //     if counter == 10{
    //         // break counter;
    //     }
    // }
    let mut number =1;
    while number != 0 {
        println!("{}",number);
        number -= 1;
    }
    // for loop
    let a = [10,20,30,40];
    for ele in a.iter(){
        println!("{}",ele);
    }
    // last number is exclusive 
     for ele in 1..4{
        println!("{}",ele);
    }
    // line comment
    /*
    Block Comment g
    */

}
