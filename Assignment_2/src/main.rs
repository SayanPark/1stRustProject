//Answer 1
// fn ans(mut val: Vec<i32>) -> bool{
//     let first_value = val.first().unwrap_or(&0);
//     let result = *first_value == 1;
//     val.push(15);
//     println!("{:?}", val);
//     result
//     }
 
// fn main() {
//     let values = vec![1, 3, 5, 7];
//     let answer = ans(values);
 
//     println!("Is the first value 1? {}", answer);
// }
//Answer 2
fn add_two(x: i8) -> i8 {
    x + 2
}
 
fn main() {
    let num = 5;
    let answer = add_two(num);
    println!("the answer is {}", answer);
}