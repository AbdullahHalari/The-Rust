// #[derive(Debug)]
// struct car {
//     name : String,
//     colour : String,
//     cc : i32,
//     model : i32,
//     types : String,

 //}
// let car_01 = car {
//     name : "mehran".to_string(),
//     colour : "white".to_string(),
//     cc : "800".to_string(),
//     model : "2016".to_string(),
//     types : "manual".to_string(),
//   }

// fn main() {
//     let car_01 = car {
//     name : "mehran".to_string(),
//     colour : "white".to_string(),
//     cc : 800,
//     model : 2016,
//     types : "manual".to_string(),
//   };
//     println!("{:#?}",car_01);
    
// }
// #[derive(Debug)]
// struct Student{
//     first_name: String,
//     last_name: String,
//     age: i8,
//     single: bool,
// 	height: f32, 
// }

// fn main() {
//     let mut student_01 = Student{
//         single: true,
//         first_name: "Faheem".to_string(),
//         last_name: "Uz Zaman".to_string(),
//         age: 24,
//         height: 5.8,
//     };
//         let student_02 = Student{
//         single: true,
//         first_name: "Zain".to_string(),
//         last_name: String::from("Ul Abdin"),
//         age: 25,
//         height: 5.9,
//     };
//     student_01.first_name = "Fahim".to_string();
//     println!("{:?}", student_01);
//     println!("{}", student_01.first_name);
// }
#[derive(Debug)]
struct Student{
    first_name: String,
    last_name: String,
    age: i8,
    single: bool,
	height: f32, 
}

fn main() {
let mut student_01 = create_student("Fahim".to_string(),"Uz Zaman".to_string(),24,true,5.8);
student_01.first_name ="Faheem".to_string();
println!("{:?}", student_01);
}
fn create_student(f_n:String,l_n:String,age:i8,s:bool,h:f32)-> Student{
    Student{
        first_name: f_n,
        last_name: l_n,
        age: age,
        single: s,
        height: h,
    }
}

// #[derive(Debug)]
// struct student{
//   NAME: String,
//   FATHER_NAME: String,
//   AGE: i32,
//   CLASS: i32,
// }
// fn main(){
//       let mut student_01=student{
//       NAME: String::from("abdullah"),
//       FATHER_NAME: String::from("m.ibrahim"),
//       AGE: 18,
//       CLASS: 12,
//       };
//       student_01.NAME = String::from("ABDULLAH HALARI");
//       println!("{:#?}",student_01 )
// }