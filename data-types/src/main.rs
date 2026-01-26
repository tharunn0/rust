// fn main() {
//     let isval: bool = true;

//     println!("{isval}");

//     println!("Hai \n bro");

//     println!("The path is home\\docs");

//     println!("He said \"no \" ");

//     let x: i64 = 399;

//     let z = x as f32;

//     let y: i32 = x.try_into().expect("failed bro");

//     println!("{y}{z:.4}");

//     let b = 'B';

//     let c: char = 'C';
// }

fn main() {
    // let mut teams = ["arsenal", "city", "chelsea", "liverpool"];

    // teams[1] = "PSG";

    // println!("{:?}", teams);

    // let mut winner = teams[3];

    // println!("The winner of the season is {winner}");

    // winner = teams[1];

    // println!("The winner of spanish league is {winner}");

    // let ages: [i32; 5] = [23, 34, 53, 65, 12];

    // dbg!({ "{:?}" }, ages);

    // println!("{:#?}", ages);

    // tuples

    // let data = ("Tharun", 20, true);

    // let (name, age, isadult) = data;

    // println!("Name: {name}, Age :{age}, Is_adult: {isadult}");

    // Generics
    //

    // let numbers: std::ops::Range<i8> = 1..10;

    // for num in numbers {
    //     print!("{num}, ");
    // }
    // println!()

    // mini excercises

    let leet: i32 = 1_3_3_7;
    println!("leet: {leet}");

    let newleet = leet as i16;
    println!("new leet: {newleet}");

    let flt = 1.23456;
    println!("with 3 precision: {:.3}", flt);

    let with_milk: bool = true;
    let with_sugar = false;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    println!("coffee is my type ? : {is_my_type_of_coffee}");

    let is_acceptable_coffee: bool = with_milk || with_sugar;
    println!("is acceptable coffee ? : {is_acceptable_coffee}");

    let arr: [i8; 4] = [2, 3, 4, 5];
    println!("{arr:?}");

    let tup = (leet, flt, with_sugar, arr);
    println!(" tuple is  : {tup:#?}");
}
