const THREE_HOUR_IN_SECONDS: i32 = 60*60*3;
fn main(){
    let x =5;
    let x1_address = &x;
    println!("x first address {:p}", x1_address);
    println!("x second address value from *x1_address {}", *x1_address);
    println!("x first is value {}", x);

    assert_eq!(x,5);

    let x = 6;
    let x2_address = &x;
    println!("x second address {:p}", x2_address);
    println!("x second address value from *x2_address {}", *x2_address);
    println!("x second is value {}", x);

    assert_eq!(x,6);

    let user_input = " 10 ";
    println!("user first = {}", user_input);

    let user_input = user_input.trim();
    println!("user second = {}", user_input);

    let user_input:i32 = user_input.parse().unwrap();
    println!("user third = {}", user_input);

}