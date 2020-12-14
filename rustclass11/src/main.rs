// fn main() {
//     let arrdata = [3,4,5,6,7,8];
//     println!("{}",arrdata[10] );
// }
// fn main(){
//     panic!("crash and burn");
// }
// use std::fs::File;
// fn main(){
//     let f = File::open("hello.txt");
//     print!("{:?}",f);
// }
// use std::fs::File;
// fn main(){
//     let f = File::open("hello.txt");

//     print!("{:?}",f);
// }
//class no: 13;
use std::collections::HashMap;
fn main(){
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
println!("{:#?}",scores );


}