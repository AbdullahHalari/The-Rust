// fn main(){
//     let x = 10;
//     let y = x;
//     println!("{},{}",x,y);

//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     let s3 = &s1;
//     let s4 = &s1;
//     println!("{},{},{},{}",s2,s2,s3,s4);
//     let s1 = String::from("hello");
// let s2 = s1;
// println!("{}, world!", s2);


// }
// fn main() {
// let s = String::from("hello"); // s comes into scope
// takes_ownership(s); // s's value moves into the function...
// // ... and so is no longer valid here
// let x = 5; // x comes into scope
// makes_copy(x);
// }
// fn takes_ownership(some_string: String) { // some_string comes into scope
// println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
// println!("{}", some_integer);
// } // Here, some_integer goes
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");    
    let s3 = takes_and_gives_back(s2);
    println!("{}",s1);    
        println!("{}",s3);        
    

}
fn gives_ownership() -> String {            
    let some_string = String::from("hello");
    some_string                             
}
fn takes_and_gives_back(a_string: String) -> String { 
    a_string  
}
