mod r#move;
#[derive(Debug)]
struct Rec{
    height:u32,
    width:u32
}
struct Guess{
    value:i32
}
impl Guess {
    pub fn new(value:i32)->Guess{
        // if value < 1 || value >100{
        //     panic!("Pls put correct value");
        // }
        if value < 1 {
            panic!("Pls put correct value");
        }else if value > 100{
            panic!("pls put value")
        }
        Guess { value }
    }
}
impl Rec {
    fn can_hold(&self,rec:&Rec)->bool{
        self.height > rec.height && self.width > rec.width
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn add_two(a:u8)->u8{
    a+2
}
pub fn greetings(name:&str)->String{
    format!("{}",name)
}
#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn larger_hold_smaller(){
         let rec1 = Rec{
            height:8,
            width:7
         };
         let rec2 = Rec{
            height:5,
            width:4
        };
        // assert!(rec1.can_hold(&rec2))
        assert_eq!(rec1.can_hold(&rec2),true);
    }
    #[test]
    fn smaller_hold_larger(){
         let rec1 = Rec{
            height:8,
            width:7
         };
         let rec2 = Rec{
            height:5,
            width:4
        };
        // we can put the expected values at right side or left side 
        assert!(!rec2.can_hold(&rec1));
        // assert_eq!(rec2.can_hold(&rec1),false);
    }
    #[test]
    fn add_twos(){
        assert_eq!(4,add_two(2));
        assert_ne!(6,add_two(2));
    }
     #[test]
    fn check_gretings(){
        let res = greetings("carol");
        assert!(res.contains("carol"));
        // assert!(res.contains("ce"),"Greeting cannot comtain name value was`{}`",res);

    }
    #[test]
    // #[should_panic] //this thing assert that the cofde inside the body should panicked
    // we make this more presize 
    #[should_panic(expected="pls put value")]
    fn guess_check(){
        Guess::new(200);
    }
    #[test]
    fn it_works()->Result<(),String>{
        if 2+3 == 4{
            Ok(());
        }{
            Err(String::from("two plus two does not equal to four"))
        }
    }

    // #[test] // this functions are called tests 
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
    // #[test]
    // fn falling_tests(){
    //     panic!("Make this test fail")
    // }
}
