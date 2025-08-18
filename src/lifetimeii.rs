use std::fmt::Display;

// lifetime for structs
struct ImportantExcerpt<'a>{
    // this means the part is going to be in the before calling the ImportantExcerpt instances 
    part: &'a str
}

//lifetime collision rules 
// the compiler deterministically know the lifetime with the hwlp of lifetime collision rules
// parameter lifetime called input lifetimes 
// output lifetimes called output lifetimes 

// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters;(for multiple parameters we need to mention the lifetime)

// 3. If there are multiple input lifetime parameters, but one of them is
// &self or &mut self the lifetime of self is assigned to all output
// lifetime parameters. (it generally aply to methods )


impl<'a> ImportantExcerpt<'a>{
    // there is two input params and  one of is self so it gets it's lifetime
    fn return_part(&'a self,announcement:&str)->&'a str{
        println!("{}",announcement);
        self.part
    }    
}
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
pub fn main(){
    let novel = String::from("Call me XYZ.Some years ago...");
    let first_sen = novel.split('.').next().expect("cannot find");
    // first_sen should be in the scope before calling i 
    let i = ImportantExcerpt{
        part:first_sen
    };
}
fn main_ii(){
    // static lifetime 
    // it means the lifetime lives at the end of the program (all str have static lifetime -> it stored in the program binary)
    let s:&'static str = "hello";

}

fn long_with_an_ann<'a,T>(
    x:&'a str,
    y:&'a str,
    ann:T
)->&'a str
    where 
        T:Display
{
    println!("{}",ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}