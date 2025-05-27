fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 1;

    {
        let y = y * 2;
        println!("The value of y inside the scope is: {y}");
    }
    println!("The value of y outside the scope is: {y}");


    let tup: (i32,_,_) = (-500, 6.4, 1);

    let k = tup.1;
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("Or the calue of y is : {k}");

}
