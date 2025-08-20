pub fn add_two(a:i32)->i32{
    internal_adder(2, 2)
}
// it is private function 
fn internal_adder(a:i32,b:i32)->i32{
 a+b
}
// we can test out the private functions 
#[test]
fn internal(){
    assert_eq!(4,internal_adder(4, 6))
}