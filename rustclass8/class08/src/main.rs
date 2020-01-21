//My_Lib
// extern crate my_lib as my_lib;
// fn main() {
//    my_lib::print_text();

// }
// //www.tiny.cc/IOTGS1
//IOT_IO
//use iot_io::read_inputs as io;//:read inputs;
// fn main(){
//     println!("Enter the name");
//     let name = iot_io::read_inputs::read();
//     println!("{}",name);
// }
extern crate student;
use student::student_register as mod_s;
//use teacher::teacher_register as mod_t;
fn main(){
    let student_01 = mod_s::Student::new("Arsalan".to_string(),"marsalan485@gmail.com".to_string(),25,true);
    //println!("{:?}",student_01);
    student_01.get_student();

   // let teacher_01 = mod_t::Teacher::new("Arsalan".to_string(),"marsalan485@gmail.com".to_string(),25,true);
    //println!("{:?}",student_01);
    //teacher_01.get_teacher();
}