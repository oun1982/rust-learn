fn main(){
    println!("This is main function");
    sec_func();
    print_number(5);
    add_number(10, 20);

    println!("This back to main function");
}

fn sec_func(){
    println!("This is second function");
}

fn print_number(x: i32){
    println!("number is {}",x);
}

fn add_number(a: i32, b: i32){
    let sum = a + b;
    println!("Nunber add = {}", sum);
}

fn five() ->i32 {
    5
}