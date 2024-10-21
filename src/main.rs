// use std::io; 
// use rand::Rng;
// use std::cmp::Ordering;

//the rand::Rng  the rand is an object to access the library functions.


fn main() {

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