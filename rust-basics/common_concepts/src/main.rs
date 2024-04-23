fn main() {
    let mut x: i32 = 5;
    println!("{:?}", x);
    x = 6;

    // constant variables
    const MOUNT_COUNT: i32 = 50;
    println!("{MOUNT_COUNT}");

    // tuples
    let tup = ("Tarun Kumar", 21);
    let (name, age) = tup;

    println!("name is {name} and age is {age}");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    my_function();

    let ans = add(10, 10);
    println!("The sum of two number is {ans}");

    // control flow
    let number: i32 = 5;

    if number == 5 {
        println!("Number is correct");
    } else {
        println!("Number is incorrect");
    }

    // loops
    for elem in arr {
        println!("{elem}");
    }
}

fn my_function() {
    println!("Hello from another function");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
