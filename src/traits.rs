use std::fmt::{Debug, Display};

//4.29
pub struct NewsArticle{
    pub author:String,
    pub headline:String,
    pub content:String
}
impl Summary for NewsArticle{
    // fn summarize(&self)->String {
    //     format!("{}, by{}",self.headline,self.author)
    // }
    // we need to implement this because it does not contain any implementation block
    fn summarize_author(&self) ->String {
        format!("{}",self.author)
    }
}
pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}
impl Summary for Tweet{
    fn summarize_author(&self) ->String {
        format!("@{}",self.username)
    }
    fn summarize(&self)->String {
        format!("{}, by {}",self.username,self.content)
    }
}
pub trait Summary {
    fn summarize_author(&self) ->String;
    fn summarize(&self)->String{
        //default implementation block
        format!("Read More...")
    }
}
/*
taking any item that implements summary
*/
pub fn notify(item:&impl Summary){
    println!("Breaking News! {}",item.summarize());
}
// it is a same as above syntactic looks different
pub fn notify_gen<T:Summary>(item:&T){
    println!("Breaking News! {}",item.summarize());
}
pub fn notify_two(item1:&(impl Summary+Display),item2:&impl Summary){
    //implementation
}
pub fn notify_two_gen<T:Summary +Display>(item1:&T,item2:&T){
    //implementation
}

fn some_function<T:Display+Clone,U:Clone+Debug>(t:&T,u:&U)->i32{
    8
}
// where clause 
fn some_function_where<T,U>(t:&T,u:&U)->i32
    where T:Display+Clone,
        U:Clone+Debug{
    8
}
// return type -> return any type that implements summary
fn return_summarize()->impl Summary{
    Tweet {
        username: String::from ("horse ebooks"),
        content: String::from("Yeah Great!"),
        reply: false,
        retweet: false
    }
}
// returing two types are not allowed we can return only one type
fn returns_summarizable(switch: bool) -> impl Summary {
    // if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        }
    // } else {
        // Tweet {
        //     username: String::from("horse_ebooks"),
        //     content: String::from(
        //         "of course, as you probably already know, people",
        //     ),
        //     reply: false,
        //     retweet: false,
        // }
    // }
}

/*
coditionally implemnt a method
*/
struct  Pair<T>{
    x:T,
    y:T
}
impl<T> Pair<T>{
    fn new(x:T,y:T)->Self{
        Self{
            x,y
        }
    }
}
impl<T:Display+PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("The largest number is x={}",self.x)
        }else {
            println!("The largest number is y={}",self.y)
        }
    }
}

// Blanket Implementations -> we can implement a trait which implements another trait
// impl<T:Display> ToString for T{
//     // snip
// }


pub fn main(){
    let tweet: Tweet = Tweet {
        username: String::from ("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };
    let article: NewsArticle = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling.")
    };
// println! ("Tweet summary: {}", tweet. summarize());
// println! ("Article summary: {}", article. summarize());

// notify(&article);
println!("{}",return_summarize().summarize());
}