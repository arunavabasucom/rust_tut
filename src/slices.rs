// insted of taking ownership it does not take ownership 
// and insted of pointing to the whole collection we can point it out some of the part of it 
// str -> unsize string type(string slice) and String -> growable string UTF-8 that heap allocated 
pub fn main(){
    let mut s = String::from("hello world");
    let hello = &s[0..5]; // if it first of the then we can simply [...5]
    let world = &s[6..11];// if it last of the string then we can use [6...]
    first_word(&s);
    let a = [1,2,3,4,5];
    let slice = &a[0..2];
}

fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    // for b in bytes{
    //     println!("bytes {}",b);
    // }

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}