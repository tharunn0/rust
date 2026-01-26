// fn main() {
//     println!("Hello, world!");

//     print_name("Tharun");

//     print_name_and_age("Tharun", 20);

//     println!("{}", is_negative_num(-1));

//     println!("{:?}", prod_sum(5, 2));

//     let multi = 5;

//     let x = {
//         let num = 9.1;
//         num * multi as f64
//     };

//     println!("{x}");
// }

// fn prod_sum(a: i32, b: i32) -> (i32, i32) {
//     return (a * b, a + b);
// }

// fn is_negative_num(n: i32) -> bool {
//     return n < 0;
// }

// fn print_name(name: &str) {
//     println!("Hai, my name is {name}");
// }

// fn print_name_and_age(name: &str, age: i16) {
//     println!("Name is {name} and age is {age}");
// }
//

// mini - exercise

fn main() {
    apply_to_jobs(3, "SRE");

    println!("{}", is_even(2));

    println!("{:?}", alphabets("Bananas"));
}

fn apply_to_jobs(num: i32, title: &str) {
    println!("Im applying to {num} {title} jobs !")
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn alphabets(s: &str) -> (bool, bool) {
    (s.contains("a"), s.contains("z"))
}
