fn main() {
    let num :u8= 255; // good practice (u8 - unsigned integer of 8 bits)
    // let num = 50; // this is also possible with easy datatype 
    // num = 198; // error as immutable
    let mut n: u8 = 255; // mutable variable
    n = 199; // this is possible
    println!("This is stored in num: {}", num); 
    println!("This is stored in n: {}", n); 


    // &str - fixed length strings  - stored in rodata - read only data
    // String - dynamic length Strings - heap allocated

    let mut string_literal : String = String::from("Hello, World!"); // dynamic length string
    string_literal.push_str("add more using this fucntion");
    println!("string is {}", string_literal);

    //mut is required as new string added to the string_literal
    let string_literal2: &str = "immutable string";



    // Tupple 
    let emo_info:(&str,u8) = ("happy", 100);
    let emp_name = emo_info.0;
    let emp_age = emo_info.1;
    println!("Employee name: {} and age: {}", emp_name, emp_age);
     
     // destructuring 
     let(name,age) = emo_info;
     print_value();
    val_print(7);

    let num1:u8 = 10;
    let num2:u8 = 20;
    let result :u8 = add(num1,num2);
    println!("Sum of {} and {} is {}", num1, num2, result);
}


fn print_value(){
    println!("This is a function");
}

fn val_print(item:u8){
    println!("This is a function with value: {}", item);
}

fn add(num1:u8,num2:u8)->u8{
    // return type should be mentioned
    return num1 + num2;
}


