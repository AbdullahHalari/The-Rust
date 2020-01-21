fn main(){
    let x : (i32,i32,i32)=(23,43,54);
    let s = (x.0+x.1+x.2)/2;
    println!("{}",s);
    let t = (s*(s-x.0)*(s-x.1)*(s-x.2)) as f64;
    let final_value = t.sqrt();
    println!("the result is{} ",final_value);
}
// fn main(){
//     let x = Triangle{a:23,b:43,c:54};
//    println!("{}",x.calculate_value());
// }
// #[derive(Debug)]
// struct Triangle {
//     a : i32,
//     b :i32,
//     c: i32,
    
// }
// impl Triangle{      // implement
//     fn calculate_value(self)->f64{

//         let s = (self.a + self.b + self.c)/2;
//          let t = s*(s-self.a)*(s-self.b)*(s-self.c);
//           let final_value = (t as f64).sqrt();
//           final_value
         
  //  }
//}
// fn main(){
//     let x : (i32,i32,i32)=(23,43,54);
//     let s = (x.0+x.1+x.2)/2;
//     println!("{}",s);
//     let t = (s*(s-x.0)*(s-x.1)*(s-x.2)) as f64;
//     let final_value = t.sqrt();
//     println!("the result is{} ",final_value);
// }
// fn main(){
//     let x = Rectangle{l:23,w:43};
//    println!("{}",x.calculate_value());
// }
// #[derive(Debug)]
// struct Rectangle {
//     l : i64,
//     w :i64,
    
// }
// impl Rectangle{      // implement
//     fn calculate_value(self)->i64{

//         let s = (self.l * self.w );
//           s
         
//     }
// }
// mod vehicle;
// use vehicle::vehicle_01;
// fn main(){
    
// }