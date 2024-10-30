// use std::io; 
// use rand::Rng;
// use std::cmp::Ordering;

//the rand::Rng  the rand is an object to access the library functions.

// struct  User{
//     active: bool, 
//     username: String, 
//     email: String
// }

// fn build_user(email: String, username: String) -> User {
//     User{
//         active: true, 
//          email, 
//         username,
//     }
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//to make it more readable . use the tuple in side there. so from the dimension 
//get the dimension.0 , and dimention.1; 
// fn area(dimension: (u32, u32)) -> u32{
//     dimension.0 * dimension.1
// }

//another way using reference 
// struct Rectangle {
//     height: u32, 
//     width: u32
// }

//Storing values directly inside the enums for more concise 
//it also Store different typeof data  one is string which i use 
//diffrent type of data 
enum IpAddrKind {
    // V4(String),
    V4(u8, u8,u8, u8),
    V6(String),
}

// struct Circle {
//     width: u32, 
//     height: u32
// }
//why use reference because we want to use its field without taking the ownership
// fn area(dimension: &Rectangle) -> u32 {
//     dimension.height * dimension.width
// }

//Method Syntax 
// impl Rectangle{
//     fn area(&self) -> u32{
//         self.height * self.width
//     }
// }

// impl Rectangle {
//     fn square(size: u32)-> Self{
//         Self{
//             width: size, 
//             height: size
//         }
//     }
//}

// impl Circle {
//     fn sc(&self) -> u32{
//         self.height * self.width
//     }
// }

// struct IpAddress{
//     kind: IpAddrKind, 
//     address: String
// }

//multiple struct under one message 
// enum Message {
//     Quit, // no store data 
//     Move {x: i32, y: i32}, //annoymous struct bcuz no name is found 
//     Write (String), 
//     ChangeColor (i32, i32, i32)
// }
// impl Message {
//     fn call(&self) {
//         match  self {
//             Message::Quit => println!("Quite Struct"), 
//             Message::Write(text) => println!("Text from Write {}", text), 
//             Message::Move { x, y } => println!("x value is {} y Value is {}", x ,y), 
//             _ => println!("default")
//         }
//     }

//     // let m = Message::Write(String::from(""));
// }

// fn route(ip_kind: IpAddrKind){}

enum Option<T>{
    Some(T), 
    None
}

fn main() {

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let s = Message::call(&Message::Quit);

    //let localHost = IpAddrKind::V4(String::from("127.0.0.1")); this one for string
    // let localHost = IpAddrKind::V4(120, 0, 0, 1); 



    // let localHost = IpAddress{
    //     kind: IpAddrKind::V4, 
    //     address: String::from("127.0.0.1")
    // };




    // let rect1 = Rectangle{
    //     height : 12, 
    //     width : 4
    // }; 

    // let s = Rectangle::square(3);

    // println!("Rectangle are is {}",s.height )

    // let rectangle = Rectangle {
    //     height: 20, 
    //     width: 5
    // };

    //here we create a tuple to make it more readable 
    // let rect = (30,40);

    // let height = 32; 
    // let width = 4;

    // println!("The area of the rectangle is {} pixels", area(width, height))
    //here we also create the reference to rectangle.
    // println!("The area of the rectangle is {} pixels", area(&rectangle))
    // println!("The area of the rectangle is {} pixels", area(rect))

//    let g = build_user(String::from(" Email "), String::from("Waleed UserNAme"));
//     let user1 = User{
//         active : true, 
//         email: String::from("emaill"), 
//         username: String::from("username")
//     };

//     let user2 = User{
//         active: false, 
//         email: String::from("EmailUser2"), 
//         ..user1
//     };


//     println!("{}", g.username)

    // let user1 = User{
    //     active: true, 
    //     email: String::from("Waleed@gmail.com"),
    //     username: String::from("Waleed_ahmad")
    // };

    // let g = user1.email;
    // let s = user1.username;
    
    // print!("{g}");
    // let a = [12,32,3,42];
    // let slice = &a[1..4];
    // println!("{:?}", slice);
    //sliceing without taking ownership 
    // let mut arr = [1,443,2,123,123,53,9,34];
    // let slices = &mut arr[0..5];
    // print!("{:?}", slices);
    // let mut g = "fdf";
    // g = "asd";
    // println!("{g}");
    // let mut s = String::from("Hello World");
    // let word = first_word(&s);
    // print!("{word}");
    // s.clear();
        // println!("{}",g);
    


    // s.push_str(" hello ,World ");

    // println!("{s}")

    // let arr = [12,434,53,532];
    //using for loop  iter use to give iteration of every element 
    // for e in arr.iter(){
    //     println!("item {e}")
    // }
    // for e in 1..4{
    //     println!("{e}");
    // }

    // let mut  index = 0; 
    // while  index < arr.len()  {
    //     println!("Items {}", arr[index]);

    //     index +=1;
    // }

    // let mut count = 4; 
    // while  count != 0 {
    //     println!("count {count}");

    //     count -= 1;
    // }

    // let mut count = 0; 

    // 'counting_up : loop {
    //     println!("count is {count}"); 
        
    //     let mut remaining = 10; 
        
    //     loop {
    //         println!("remainig is {remaining}"); 
    //         if remaining == 9{
    //             break;
    //         } 
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;

    //     }
    //     count +=1;
    // }    

    // println!("End of count {count}");

    // let mut counter = 0; 

    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter + 1 ;
    //     }
    // };

    // println!("Result is {}", result);

    // let x = 20;
    // if x == 20 {
    //     print!("true");
    // }
    // let y = {
    //     let x = 2;
    //     x+1
    // };

    //     println!("Print the value of y is {}", y);

        // multiple_parameters_function("Waleed", 23);
        // another_function(13);

    //array in rust
    // let array = [1,2,3,4,5,6,7,8,9,10];
    // let mut index = String::new();
    // io::stdin().read_line(&mut index).expect("error message for input");
    // let index: usize = index.trim().parse().expect("element error");
    // let findi = array[index];
    // print!("Element found at index: {index}, element: {findi}")

    //array 
    // let array = [12, 34, 30,040, 80, 1,11];
    // let array:[i32; 6] = [12, 34, 30,040, 80, 1,11];
    // let first = array[0];
    // print!("{first}");

    //destructuring tuples
    // let tuples: (i32, &str, u64) = (12, "waleed", 43);
    //also using to access item of tuples using index 
   // let five_hundred = tuples.0;
    // let six_point_four = tuples.1;
    // let (x,y,z) = tuples;
    // println!("{}");
    //     println!("{x}, {y}, {z}");

    // let f: f64 = 2.5;
    // println!("{f}");

    //Shadowing 
    // let x = 5; 
    // let x = x +1; 

    // {
    //     let x = x *2; 
    //     println!("Inner scope function {}", x);
    // }

    // println!("outer scope value of x {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();

    //if remove the let from spaces then it gives error because of the mismatch datatypes. the spaces data type is &str.
        // print!("{spaces}");
//    let x: i32 = 1; 
//    let _y: i32;
//    assert_eq!(x, 1);
//     print!("Success");


//mutable variable 
// let mut x = 4;
// x += 1;
// assert_eq!(x,5 );
// println!("Success");

//Scope Range 
// let x: i32  = 10; 
// let y: i32 = 5; 

// { 
//     //  inner Scope 
// // let y: i32 = 5;  we cant use this variable outside this scope so we declare it inside the outerscope .

// print!("the value of x {}  and y is {} ", x,y);

// // Use the {} this to concatenate the variables … so first will be x then it will be y and so on.
// }
// print!("the value of x is {} and y is {}", x, y);

//function calling e.g 
// define_x();

// println!("Guees the number"); 

// let sect_num = rand::thread_rng().gen_range(1..=40);

// println!("SEcret number {sect_num}");

// println!("Enter number"); 
// //:: use to access function enums etc.. 
// let mut numb1 = String::new();

// io::stdin().read_line(&mut numb1).expect("Enter Correct number 1");

// println!("Enter Operator");
// //&mut the & keyword used to reference and borrow without taking ownership
// let mut operator = String::new();
// io::stdin().read_line( &mut operator).expect("Enter Correct operator");

// println!("Enter number 2");

// let mut numb2 = String::new();
// io::stdin().read_line(&mut numb2).expect("Enter Correct number2");

// let  numb1: u32 = numb1.trim().parse().expect("error Converting number 1");
// let numb2: u32 = numb2.trim().parse().expect("error converting number 2");
// let operator  = operator.trim();
// let result: u32;

// match operator {
//     "+" => {
//         result = numb1 + numb2;
//         println!("Sum is {}", result);
//     }, 
//     "-" => {
//         result = numb1 - numb2;
//         println!("Subtract is {}", result);
//     }, 
//     "*" => {
//         result = numb1 * numb2;
//         println!("Multiplication is {}", result);
//     }, 
//     "/" => {
//         result = numb1 / numb2;
//         println!("Division is {}", result);
//     }, 

//     _ => println!("Unknow")
// }

// println!("Gueesed Number {}", guess);

//converting string to int ... using unsinged integer. 


// match  guess.cmp(&sect_num) {
//     Ordering::Less => println!("too snmall"),
//     Ordering::Greater => println!("too Big"), 
//     Ordering::Equal => {
//         println!("You win");
//     }, 
//     _=> println!("Error")
// }

}

// fn define_x(){
//     let x: &str = "hello";
//str annotaion represent string
//     println!("{} , World", x );
// }

//parameters passing or argument passing in function 
// fn another_function(x: i32){
//     println!("the value is {x}");
// }

//functions with multiple parameters 
// fn multiple_parameters_function(label: &str, digit: i32){
//     println!("values are for label {label} and the digit is {digit}")
// }

//function return types 
//fn sum_number(x:i32, y:i32) -> i32{
//two types of return type 
//simple use return 123; 
//second .. 5 that's it 

//}


//what happens if we reference toward invalid data 
// function attempts to return a reference to a local variable that is 
// dropped when the function exits, which is not allowed in Rust. You can 
// either return the owned value directly or return a reference that points 
// to data with a longer lifetime. This strict borrowing and ownership system 
// is one of the key features of Rust that helps prevent memory safety issues 
// like dangling
// fn dangle() -> &String {
//     let s = String::from("hello");
//      &s
// }

//SLices 
// A slice is a reference to a contiguous portion of a collection, such as an array or a vector.
// Slices are used when you want to access a subset of an array or string without needing to copy or own the entire collection.
// The syntax for creating a slice involves using a range syntax with the collection, such as &array[start..end].
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for(i, &items) in bytes.iter().enumerate(){
//         if items == b' '{
//             return  i;
//         } 
//     }
//     s.len()
// }