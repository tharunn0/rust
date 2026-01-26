fn main() {
    let mut age: i16 = 20;
    let name = "tharun";
    let is_adult: bool;

    if age >= 18 {
        is_adult = true;
    } else {
        is_adult = false
    }

    let mut id = 99;

    {
        id = 22;
    }

    println!("id is {}", id);

    println!(
        "I am {1} and im {0} years old im an adult {is_adult}",
        name, age
    );

    age = 23;
    println!("{}", age)
}
