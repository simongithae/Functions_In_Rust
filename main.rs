//---------------------____________________------------------- FUNCTIONS & USER INPUT ---------------__________________---------------->
//Prelude 
use std::io;
//--------------------_____________________-------------------- FUNCTIONS ---------------------______________________----------------->
//Simple function
fn greetings(){
    println!("hello there partner");
}
//Function with perimeters 
fn say_hi(name: &str, intro: &str, no: u8){
       println!("Hi {} and {} you have {} days left",name, intro,no);
}
//Return types 
//type 1
fn five() -> u32 {
    return 5;
}
//type 2
fn six() -> u32 {
    6
}
//application
fn mult(a: u8, b: u8) -> u8 {
    let multiple = a*b;
     return multiple;
}
fn sum(a: u8, b: u8) -> u8 {
    let add = a + b;
     add
}

//-------------------------______________________------------------ USER INPUT --------------------__________________-------------->
//Function that takes in the name and age of the user and displays them
fn user_details(){
    println!("Please enter your name ");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name);
    println!("Please enter your age ");
    let mut user_age = String::new();
    io::stdin().read_line(&mut user_age);
    println!("Your name is {} and you are {}yrs old", user_name,user_age);
}





fn main() {
    //Invoking a function
    greetings();
    
    //Assigning the return value to a variable 
    let nofive = five();
    println!("{}",nofive);

    
    //Invoking a function and passing arguments
    say_hi("Jimmy","Laura",5);
    let sumof = sum(10,20);
    let multiply = mult(10,20);

    println!("sum is {}",sumof);
    println!("multiple is {}",multiply);

    user_details();
    
    
}
