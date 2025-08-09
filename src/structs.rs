struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool
}

#[derive(Debug)]
struct Rec{
    height:u32,
    width:u32
}

// methods using implementation block
impl Rec {
    fn area(&self)->u32{
        self.height*self.width
    }
    fn can_hold(&self,rec:&Rec)->bool{
        self.height > rec.height && self.width > rec.width
    }
}
// struct can have multiple implmentation block 
// we can differenciate the methods and the associated function by . and ::
impl Rec {
    fn squre(size:u32)->Rec{
        Rec { height: size, width: size }
    }
}
pub fn main(){
    // to modify any value of this instance we can add mut keyword infront of that 
    // it going to make entrire struct mutable not a single field
    let mut user1= User{
        username:String::from("user1"),
        email:String::from("user1@user.com"),
        sign_in_count:1 ,
        active:true
    };
    let name = user1.username;
    user1.username = String::from("user");
    let user2 = build_user(
        String::from("user2@user.com"), String::from("user2")
    );
    // taking part of another user
    let user3 = User{
        username:String::from("user3"),
        email:String::from("user3@user.com"),
        ..user2// take other two fields from the user2
    };

    // tuples struct -> without name fields (it is useful when we are passing different types types but passing type of data)
    struct  Color(i32,i32,i32);
    struct Point(i32,i32,i32);
    main_ii();

}
fn build_user(email:String , username:String)->User{
    User{
        // username:username,
        // email:email,
        username,
        email,
        active:true,
        sign_in_count:1
    }
}


// use of tuple structs 
fn main_ii(){
    // let height = 30;
    // let width = 50;
    // println!(
    //     "The area is {}",area( width,height)
    // )

    // let rect = (30,50);
    // println!(
    //     "The area is {}",area(rect)
    // )

    let rec1 = Rec{
        height:60,
        width:30
    };
    let rec2 = Rec{
        height:20,
        width:10
    };
    println!("rec {:#?}",rec1); // primitive type implemnt the display trait by default 
    // but in custom datatype we need to addup this trait and 
    // also the debug trait it is going to print out the debug info in the console
    println!(
        "The area is {}",area(&rec1)
    );
    // using method
    println!(
        "The area is {}",rec1.area()
    );
    println!(
        "{}",rec2.can_hold(&rec1)
    );
    println!(
        "{}",rec1.can_hold(&rec2)
    );
    let rec3 = Rec::squre(20);
    


}

// fn area(width:u32,height:u32)->u32{
//     width *height
// }

// fn area(dimention:(u32,u32))->u32{
//     dimention.0*dimention.1
// }

// we are just need to read out the values no need of owning that out 
fn area(Rec:&Rec)->u32{
    Rec.height *Rec.width
}