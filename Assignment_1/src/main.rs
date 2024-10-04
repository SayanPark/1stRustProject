// Answer 1
// fn main() {
//     let val1:u8 = 5;
//     let val2:u8 = 2;
//     let ans:u8 = val1 % val2;
    
//     println!("{}", ans);
//     }
// Answer 2
// fn main() {
//     let mut vec = vec![2, 4, 6, 8, 10];
//     println!("{:?}", vec);

//     vec.pop();
//     vec.push(12);
//     println!("{:?}", vec);
// }
// Answer 3
// fn concat_string(input: String) -> String {
//     let result = input + " World";
//     result
//     }
    
//     fn main() {
//     let my_string = String::from("Hello");
//     let result = concat_string(my_string);
//     println!("{}", result);
//     }   
// Answer 4
fn control_flow(val: i32) {
    if val == 1 {
        println!("The value is one");
        } else if val > 50 {
            println!("The value is greater than 50");
        } else if val < 25 {
            println!("The value is less than 25");
        } else if val > 25 && val <= 50 {
            println!("The value is greater than 25 but less than 50");
        }
}
    
fn main() {
    control_flow(1);
    control_flow(60);
    control_flow(20);
    control_flow(30);
}
