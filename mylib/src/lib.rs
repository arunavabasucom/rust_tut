mod libII;
mod front_of_houses;
// mod front_of_house{
//     mod hosting{
//         fn add_to_waitlist(){}
//         fn seat_at_table(){}
//     }
//     mod serving{
//         fn take_order(){}
//         fn serve_order(){}
//         fn take_payment(){}
//     }
// }

mod front_of_house{
    // by deafault the child module is private from the parent module 
    // we need to addup pub keyword 
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
}
pub fn eat_at_res(){
    //absolute path 
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path ``
    front_of_house::hosting::add_to_waitlist();
}


fn serve_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();//with the super keyword we allows us to call from the parent functions 
    }
    fn cook_order(){}
}

mod back_of_resturant{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String
    }
    impl Breakfast {
        pub fn summer(toast:&str)->Breakfast{
            Breakfast { 
                toast: String::from(toast) ,
                seasonal_fruit: String::from("peaches") 
            }
        }
    }
}

pub fn eat_at_resturant(){
    let mut meal = back_of_resturant::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); //by default the values are also private we need to add up pub to get access them 

    // let meal2 = back_of_resturant::Breakfast{
    //     toast:String::from("Wheat"),
    //     seasonal_fruit:String::from("Peaches") // we can not create a breakfast struct beacause of the fields are private 

    // };
}

mod back_of_lake{
    pub enum  Appetizer {
        Soup,
        Salad
    }
}
pub fn eat_at_lake(){
    let order1 = back_of_lake::Appetizer::Soup;
    let order2 = back_of_lake::Appetizer::Salad;

}
// using use keyword
mod front_of_lake{
    pub mod hostingg{
        pub fn add_to_waitlist(){}
    }
}
// with the help of the use keyword we can take any function or anything to the scope of thf file
use crate::front_of_lake::hostingg; // also we can be more specific what we need to bring into the scope
// EXCEPTION - if we bring the module in scope it has the same name then we can may get some conflicts 
// also we use self to specify the crate
// pub fn eat_at_front_res()[
//     hostingg::add_to_waitlist();
// ]



// mod libII{
//     // metion the code 
// }

// insted of that we can just metion -> mod libII;
// it will  bring the content of the modules automatically