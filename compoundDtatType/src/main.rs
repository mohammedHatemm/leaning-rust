fn main() {

    // compound data type
    // arrays , tuples , slices , strings (slice string)
    //


    let numbers: [i32 ; 10] = [1,2,3,4,5,6,7,8,9,10];
    // println!("{}" , numbers); -> wrong
    println!("{:?}",numbers);
    // let mix = [1 , "mohmed " , true]; -> wrong

    // ALL THE array
    println!("{:?}",numbers);
    //one element
    println!("{}",numbers[0]);
    // tupels

    let human = ("mohamed" , 28 , true );
    let human_2 : (String , i32 , bool)= ("mohamed".to_string() , 28 , true );
    println!("{:?}",human);
    println!("{}",human.0);
    println!("{}",human.1);
    println!("{}",human.2);


        println!("{:?}",human_2);
        println!("{}",human_2.0);
        println!("{}",human_2.1);
        println!("{}",human_2.2);
    // println!("{)


    let mix_tuples = ("mahmoud" , 21 , [1,2,3,4] , true);
    println!("{:?}",mix_tuples);
    println!("{}",mix_tuples.0);
    println!("{:?}",mix_tuples.1);
    println!("{:?}",mix_tuples.2);
    println!("{}",mix_tuples.3);
}
