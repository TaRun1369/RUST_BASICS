fn main() {

    // // here stack is used so this will work fine
    // let a = 5;
    // let b = a;
    // println!("a = {}, b = {}", a, b);

    // but for heap memory no data can be copied - owership concept
    
    // ERROR below 
    let s1 = String::from("hello"); // str1 is owner of hello
    let s2 = s1; // transfer of ownership 
    // but both can't be owner (rule 1 and 2) so str2 is new owner of hello
    //  println!("s1 = {}, s2 = {}", s1, s2); // this will give error

     println!("s2 = {}",s2); // this will work fine
}
