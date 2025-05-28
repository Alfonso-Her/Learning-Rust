fn main() {
    // print_labeled_measurement(5, 'h');
    // let x = (let y = 3);
    let y = { // This block is an expression that evaluates to 4 the is assign in the statement of this line
        let x = 3;
        let x = x + 1;// If you place a ; here it will be converted to a statment and don't return the 4
        println!("{}",x);
        x
    };
    // println!("{}",x); x is not declared in this scope
    println!("The value of y: {}",y);
    println!("The value of y plus one is: {}", plus_one(y));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

}

fn plus_one(x : i32) -> i32{
    x + 1 
}