// #[derive(Debug)]
// struct Car {
//     mpg: i8,
//     color: String,
//     top_speed: i16,
// }

// #[derive(Debug)]
// struct Motorcycle {
//     mpg: i8,
//     color: String,
//     top_speed: i16,
// }

// pub trait Properties {
//     fn set_mpg(&mut self, mpg: i8);
//     fn set_color(&mut self, color: String);
//     fn set_top_speed(&mut self, top_speed: i16);
// }

// impl Properties for Car {
//     fn set_mpg(&mut self, mpg: i8) {
//         self.mpg = mpg
//     }
//     fn set_color(&mut self, color: String) {
//         self.color = color
//     }
//     fn set_top_speed(&mut self, top_speed: i16) {
//         self.top_speed = top_speed
//     }
// }

// impl Properties for Motorcycle {
//     fn set_mpg(&mut self, mpg: i8) {
//         self.mpg = mpg
//     }
//     fn set_color(&mut self, color: String) {
//         self.color = color
//     }
//     fn set_top_speed(&mut self, top_speed: i16) {
//         self.top_speed = top_speed
//     }
// }

// fn main() {
//     let mut car = Car {
//         mpg: 0,
//         color: String::from("Red"),
//         top_speed: 0,
//     };

//     let mut motorcycle = Motorcycle {
//         mpg: 0,
//         color: String::from("Green"),
//         top_speed: 0,
//     };

//     car.set_mpg(15);
//     car.set_color(String::from("blue"));
//     car.set_top_speed(250);

//     motorcycle.set_mpg(65);
//     motorcycle.set_color(String::from("black"));
//     motorcycle.set_top_speed(150);

//     println!("{:?}", car);
//     println!("{:?}", motorcycle);
// }
// answer 2
fn print<T: std::fmt::Debug>(value: T){
    println!("{:?}", value);
}

fn main() {
    print(vec![1,2,3]);
    print("String");
    print(String::from("String number two"));
    print(10);
}
