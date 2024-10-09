use std::rc::Rc;

fn main() {
    //Question 1: Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.
    let val = 5;
    let val2 = Box::new(2);
    println!("{}", val * *val2);

    //Question 2: Create a variable that holds a String
    let rc_value = String::from("RC Value");

    {
        //Create a reference counting smart pointer that points to the above String.
        let rc: Rc<String> = Rc::new(rc_value);
        
        //Print out how many references the smart pointer has.
        println!("{}", Rc::strong_count(&rc));
        //Code block
        {
            //Create another reference counting smart pointer that points to our first smart pointer
            let rc2: Rc<String> = Rc::clone(&rc);
            //Print out how many references each smart pointer has
            println!("{}", Rc::strong_count(&rc));
            println!("{}", Rc::strong_count(&rc2));
        }
        //What value is dropped here?
        //Print out how many references out first smart pointer has
        println!("{}", Rc::strong_count(&rc));

    } //What value is dropped here?
    //Comment out the line below. What do you think will happen when you try to run the program now?
    // println!("rc_value: {}", rc_value);
}
