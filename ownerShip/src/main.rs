// fn main() {
//     // println!("Hello, world!");
//     let s1 = String::from("Rust");
//     //now the owner of the sting is s2 not s1 if i want to print s1 there is an error
//     let s2 = s1;
//     println!( s2 = {}", s2);
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);

// }
// fn calculate_length(s:&String) -> usize {
//     s.len()
// }


// when owner goes out of scope the Value will be dropped

fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}
fn print_lost(s:&String ){
    println!("the string is lost{}" , s1);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

//

