fn main(){
    let mut guess:i8 = "42".parse().expect("Not a number");
    guess = 20;
    println!("guess = {}", guess);

    let small_address = 57u8;
    println!("Small number {}", small_address);

    let big_number = 1_000_000i64;
    println!("Big number {}", big_number);

}