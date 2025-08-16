// using generics -> generally we add up the generic type after the function<T> to add as generic function
// we triats to limit that the type must be compared and copied 
fn get_largest<T: PartialOrd + Copy>(number_list:Vec<T>) -> T{
    let mut largest = number_list[0];
    for num in number_list{
        if num > largest{
            largest = num;
        }
    }
    largest
}
//using in structs 
struct Point<T>{
    x:T,
    y:T
}
// impl<T> Point<T> {
//     fn x(&self)->&T{
//         &self.x
//     }
// }


impl<U> Point<U> {
    fn x(&self)->&U{
        &self.x
    }
}
impl Point<f64> {
    fn y(&self)-> f64{
        self.y
    }
}
// we can have multiple generic type defined <T,U>
struct PointII<T,U>{
    x:T,
    y:U
}
impl<T,U> PointII<T,U>{
    fn mixup<V,W>(self,other:PointII<V,W>) ->PointII<T,W>{
        PointII{
            x:self.x,
            y:other.y
        }
    }
}

// using in enum 
enum Option<T>{
    Some(T),
    None
}
enum Result<T,E>{
    Ok(T),
    Err(E)
}
pub fn main(){
    let numbers = vec![1,2,3,4];
    let chars = vec!['k','l','m','n'];
    let largest = get_largest(chars);
    println!("{}",largest);

    let p1 = Point{x:10,y:20};
    p1.x();
    let p2 = Point{x:10.0,y:20.0};
    p2.y();

    let p3 = PointII{x:10,y:10.0};

    let pp1 = PointII{x:10,y:10.4};
    let pp2 = PointII{x:"Hello",y:"c"};

    let pp3 = pp1.mixup(pp2);
    println!("p3.x = {}, p3.y ={}",pp3.x,pp3.y);


    /*
    performance 
    in the option enum the it gonna convert the option enum seperate for two -> i32,f64 after compilation 
    */
    let integer = Option::Some(5);
    let float = Option::Some(5.4); 

}
