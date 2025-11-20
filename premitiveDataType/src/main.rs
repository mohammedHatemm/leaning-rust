fn main() {

    ///this is the premitiv data type in rust
    println!("Hello, world!");
    //int -> i16 , i32 ,64i ,i128 -> singed integers
    // int -> u16 , u32 , u64 , u128 -> unsigned integers
    let number = 100;
    println!("{}", number);
    let my_age_is:f32 = 28.6;
    //float -> 32f , 64f
    println!("my age is: {}"  , my_age_is );
    //bollean -> true , false
    let is_shown:bool = true;
    println!("is shown: {}" , is_shown);
    // the chcaracter -> char
    let letter :char = 'm';
    let letter2 = "a";
    println!("letter: {}" , letter);
    println!("the second letter is : {} " , letter2);
}
