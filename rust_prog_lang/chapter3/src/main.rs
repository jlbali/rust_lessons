use std::char::MAX;

fn main() {
    // Tuples
    let t: (i32, f64, u8) = (500, 6.4, 2);
    let x = t.0;
    println!("Primer valor de la tupla vale {}", x);
    let (x,y,z) = t; // deestructuring.
    // Arrays.
    let a = [1,2,3,4,5];
    let first = a[0];
    // Functions.
    another_function();
    printing_val(23);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
    println!("2 plus 1 is {}", add_one(2));
    // If
    let number = 3;
    if number < 5 {
        println!("Number is small");
    } else {
        println!("Number is big");
    }

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else {
        println!("Number is not divisible by 3 or 4.");
    }
    let x = if number < 5 {
        "small"
    } else {
        "big"
    };
    println!("x value is {}", x);
    // Loops.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);
    // whiles.
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    // for
    let a = [10,20,30,40,50];
    for element in a.iter() {
        println!("{}", element);
    }
    // Exercises
    println!("5th fibonacci: {}", fibonacci(5));
}

fn another_function() {
    println!("Another function");
}

fn printing_val(x: i32) {
    println!("The value of x is {}", x);
}

fn add_one(x:i32) -> i32 {
    x + 1
}



fn fibonacci(n:i32) -> i32 {
    if n == 0 {
        return 1;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}