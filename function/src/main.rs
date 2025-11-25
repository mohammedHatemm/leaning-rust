fn main() {
    // println!("Hello, world!");
    hallow_world();
    tell_height(180);
    human_id("mohamed" , 27 , 180.0);

    let _x={
        let price :i32 = 5;
        let qty:i32 = 10;

        // no need for the return keyword in rust
        price * qty


    };

    println!("the result is {}" , _x  );

    // always must have amain function in rust
    // any the best syntax in rust is the kamal case

// add(10,20);
let y = add(10 , 20);
println!("the result is {}" , y);





let weight = 100.0;
let height = 100.0;
let bmi = calculate_bmi(weight, height);
println!("BMI: {}", bmi);

}

fn hallow_world (){
        println!("hallow rust  ");
    }

    fn tell_height (height : i32)
{
println!("my height is {}",height);
}

fn human_id (name :&str , age:u32 , height:f32  ){
    println!("my name is {} and my age is {} and my height is {}", name , age , height);

}

// function can return avalue !!!
//exprition ->>
//statmetnt -->>



// const declair_vaiable_out_side_the_main_fn:i32 ={

//     //your code here

// };
// FUNCTIOn retrun the value
fn add(a:i32 , b:i32) -> i32  {
    a+b

}
// BMI function

fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg / (height_m * height_m)
}
