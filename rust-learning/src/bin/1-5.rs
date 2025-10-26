fn main(){
    
    println!("--------- How to access TUPLE---------");
    let tub:(i32, f64,u8,&str) = (500,3.14,2,"Pongsakon");

    let _tub2 = (25,1.11,2,"beagle"); //unuse variable

    // println!("tub = {}", tub);

    let (w,x,y,z) = tub;
    println!("valut in tub first method = {} {} {} {} ",w,x,y,z);

    let first_var = tub.0;
    let second_var = tub.1;
    let third_var = tub.2;
    let fouth_var = tub.3;

    println!("valut in tub second method = {} {} {} {} ",first_var,second_var,third_var,fouth_var);

    println!("valut in tub third method = {:?} ",tub);

    

    println!("--------- How to access ARRAY---------");
    let a = [1,2,3,4,5];

    let name = ["Pongsakon","Jerapon","Dad","Mom"];
    println!("Array a = {:?}", a);
    println!("Array name {}",name[0]);

    let dup_arr = [9; 10];
    println!("Print 9 amount 10 time {:?}",dup_arr);

}