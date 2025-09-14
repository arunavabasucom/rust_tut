use std::thread;
use std::time::Duration;
// use std::thread::/

fn simulated_expensive_workout(intensity:u32)->u32{
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn generate_workouts(intensity:u32,random_number:u32){
    let expensive = simulated_expensive_workout(intensity); // in this case also we are calling the expensive function in evry single case 
    
    let expensive_cloususre = |num:u32|{
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };
 if intensity < 25 {
    // in this we call a really expensive function that takes 2 seconds to run so we need to limit out
    // the how many times it gonna run 
    println!(
        "Today done {} pushups",
        // simulated_expensive_workout(intensity)
        // so with the expensive varable we need to call over here 
        expensive
    );
    println!(
        "Today done {} situps",
        // simulated_expensive_workout(intensity)
        expensive
    );

 }else{
    if random_number == 3 {
        println!("take a break today ")
    }else {
        println!(
            "try to run for {} minutes",
            // simulated_expensive_workout(intensity)
            expensive
        )
    }
 }
}
pub fn main(){

}