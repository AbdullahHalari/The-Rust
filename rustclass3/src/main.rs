fn main(){
    println!("1233");
    println!("the sum is:{}",addtwonumber(45,30)); // function calling
    println!("the square is: {}",
    square(25));
    hello_world();
    print_name("halari".to_string());
   println!("the square is:{}",calculate_square());
   let mut input_01 = String::new();
    io::stdin().read_line(&mut input_01);
    let input_01:i32 = input_01.trim().parse().unwrap();
    let mut input_02 = String::new();
    io::stdin().read_line(&mut input_02);
    let input_02:i32 = input_02.trim().parse().unwrap();
    println!("The greater input number is {}", check_maxV2(input_01, input_02));

    let mix_data = take_full_name();
    println!("{:?}",mix_data); 

    let cal_result = arthimatic_operations(input_01, input_02);
    println!("The Sum is: {}",cal_result.0);
    println!("The Sub is: {}",cal_result.1);
    println!("The Mul is: {}",cal_result.2);
    println!("The Div is: {}",cal_result.3);

    let cal_sq_cb = square_and_cube(input_01);
    println!("The number {} square is {} and cube is {}",
    input_01,cal_sq_cb.0,cal_sq_cb.1);

let arrNum = [345,567,678,234];
println!("the max number is {}",largest(&arrNum));
let arrNum02 = [345,567,678,234];
println!("the max number is {}",largest(&arrNum02));
 }
// addtwonum function name
//(x:i32...) function parameter
//->i32 indicates return type

fn addtwonumber(x:i32,y:i32)->i32{
    let mut sum: i32;                  // all use
    sum = x+y;
    sum
}
fn square(x:i32)->i32{            //all
    x*x
}
fn hello_world(){                        // return or input not available
    println!("hello_world");
}
 fn print_name(name:String){
     println!("the name is: {}",name);
 }
use std::io;
fn calculate_square()->i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input:i32 = input.trim().parse().unwrap();
    input*input
}
fn check_max(x:i32,y:i32)->String{
    if x>y{
       "The number 01 input is greater".to_string()
    }
    else{
        "The number 02 input is greater".to_string()
    }
}
fn check_maxV2(x:i32,y:i32)->i32{
    if x>y{
        x
    }                                   //chech_maxV2
    else{
        y
    }
}


 fn take_full_name()->(String,String){
     println!("Enter First Name");
    let mut input_01 = String::new();
    io::stdin().read_line(&mut input_01);
    let input_01 = input_01.trim().to_string();

    println!("Enter Last Name");
    let mut input_02 = String::new();
    io::stdin().read_line(&mut input_02);
    let input_02 = input_02.trim().to_string();

    (input_01,input_02)
 }
     

fn arthimatic_operations(x:i32,y:i32)->(i32,i32,i32,i32){
    (x+y,x-y,x*y,x/y)
}
fn square_and_cube (x:i32)->(i32,i32){
    (x*x,x*x*x)
}

 fn largest(arrNum:&[i32])->i32{
     let mut max = arrNum[0];
     for x in (1..arrNum.len()) 
     {
         if max < arrNum[x]{

         max = arrNum[x]
         }
     }
     max
 }