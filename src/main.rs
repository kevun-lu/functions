fn main() {
    println!("Hello World!");

    another_function();

    return_five();

    let x = plus_one(10);

    println!("The value of x is {x}");

}

fn another_function() {
    println!("Hello function!");

    parameters(10);
}

// use of parameters
fn parameters(x: i32) { 
    println!("The value of x is: {x}");

    print_labeled_measurements(3, 'D');
}
// use of 2 parameters
fn print_labeled_measurements(x: i32, unit_label: char) {
    println!("The print labeled measurement is {x}{unit_label}");
}

// return value for function
fn five() -> i32 {
    5
}

fn return_five() {
    let x = five();

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


