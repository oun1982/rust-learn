fn main() {
    let x = 5;
    let x = x + 1;
    println!("x = {}",x);

    {
        let x = x * 2;
        println!("Inner x = {}", x);
    }

    println!("Outer scope: The value of x is {}", x); // 2. x ตัวนอก
}

