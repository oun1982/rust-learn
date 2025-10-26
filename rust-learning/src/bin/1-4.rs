fn main(){
    let mut guess:i8 = "42".parse().expect("Not a number");
    guess = 20;
    println!("guess = {}", guess);

    let small_address = 57u8;
    println!("Small number {}", small_address);

    let big_number = 1_000_000i64;
    println!("Big number {}", big_number);

    let a = 2.0;
    println!("a = {}", a);

    let b = 3.14f32;
    println!("b = {}",b);

    let t = true;
    let f:bool = false;
    println!("t = {} and f = {}",t,f);

    let c = 'A';
    let d = 'z';

    let thai_char = 'ก';
    let heart = '😻';

    println!("Char are {} {} {} {} ",c,d,thai_char, heart);

    let test_usize: u32 = 20;
    let test_usize64 = 30u64;

    let test_size32 = 30;
    let test_size64 = 30i64;

}