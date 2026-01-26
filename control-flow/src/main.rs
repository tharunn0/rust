/*
fn main() {
    let n = 9;
    let even = if n % 2 == 0 { true } else { false };
    println!("{even}");

    let n = 4;
    let day = if n == 1 {
        "monday"
    } else if n == 2 {
        "tuesday"
    } else {
        "someday"
    };
    println!("{day}");

    //match statement
    let age = 19;
    let res = match age {
        0..=10 => "u are a kid",
        11..=19 => "u are a big kid",
        _ => "u are old as hell",
    };

    println!("{res}");

    let day = "sunday";

    match day {
        "monday" => {
            println!("its the first day baby");
        }
        "friday" => {
            println!("its the last day baby");
        }
        "saturday" | "sunday" => {
            println!("its weekday baby")
        }
        _ => {
            println!("aah hell naw");
        }
    }

    let lang = "Cpp";

    match lang {
        "Python" | "Js" => println!("{lang} is interpreted"),
        "Go" | "Rust" | "C" => println!("{lang} is compiled"),
        _ => println!("{lang} is unknown"),
    }
}
*/

// loops

//fn main() {
// loop {
//     println!("this will print for ever");
// }

// loop with break
// let mut x = 0;
// loop {
//     if x == 10 {
//         break;
//     }

//     println!("x is {x}");

//     x += 1;
// }

// let mut x = 0;
// loop {
//     if x % 2 == 0 {
//         println!("its even");
//         x += 1;
//         continue;
//     } else if x >= 10 {
//         break;
//     }
//     println!("x is {x}");
//     x += 1;
// }

// let mut y = 0;
// while y < 20 {
//     println!("y is {y}");
//     y += 1;
// }

// countdown(5);
//}

// fn countdown(sec: i32) {
//     if sec == 0 {
//         println!("Blastoff !!");
//         return;
//     }
//     println!("{sec} seconds to launch");
//     countdown(sec - 1)
// }

// mini - exercise

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number_match("blue"));

    println!("{}", factorial_recursion(5));
    println!("{}", factorial_loop(5));
}
fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        1
    } else if color == "green" {
        2
    } else if color == "blue" {
        3
    } else {
        0
    }
}
fn color_to_number_match(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_recursion(n: i32) -> i32 {
    if n == 1 {
        return n;
    }
    n * factorial_recursion(n - 1)
}
fn factorial_loop(mut n: i32) -> i32 {
    let mut a = 1;
    while n > 0 {
        a = a * n;
        n -= 1;
    }
    a
}
