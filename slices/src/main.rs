// fn main() {
//     let fullname = String::from("Cristiano Ronaldo");

//     let first_name = &fullname[0..9];
//     println!("{first_name}");

//     let second_name = &fullname[10..];
//     println!("{second_name}");

//     let player = "Sachin Tendulkar";

//     let f_name = &player[0..6];
//     println!("{f_name}");
// }

// fn main() {
//     let em = "🚀📈🤝🔥";
//     println!("{em}");
//     println!("{}", em.len());

//     let n = "Left";
//     let mut st = n.to_string();
//     st.push_str(" and Right");
//     println!("{st}");
// }

// fn main() {
//     let mut arr: [i32; 9] = [1, 3, 5, 7, 9, 2, 4, 6, 0];
//     println!("{arr:?}");

//     {
//         let sl = &mut arr[..5];
//         sl[2] = 999;
//         println!("{sl:?}");
//     }

//     arr[2] = 999;
//     println!("{:?} ", arr);
// }

fn main() {
    // Define a `cereals` array with 5 heap Strings
    //   - Cookie Crisp
    //   - Cinnamon Toast Crunch
    //   - Frosted Flakes
    //   - Cocoa Puffs
    //   - Captain Crunch

    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captian Crunch"),
    ];

    // Declare a `first_two` variable that extracts a slice
    // of the first two cereals. Print the slice.

    let first_two = &cereals[..2];
    println!("{first_two:?}");

    // Declare a `mid_three` variable that extracts a slice
    // of the middle three cereals (Cinnamon Toast Crunch
    // up to and including Cocoa Puffs). Print the slice.

    let mid_three = &cereals[1..4];
    println!("{:?}", mid_three);

    // Declare a `last_three` variable that extracts a slice
    // of the last three cereals. Print the slice.

    let last_three = &mut cereals[2..];
    println!("{:?}", last_three);

    // Using the `last_three` slice, target the last element
    // ("Captain Crunch") and replace it with "Lucky Charms".
    // Print the complete `cereals` array.

    last_three[2] = String::from("Lucky Charms");
    println!("{cereals:?}");

    // Declare a `cookie_crisp` variable that references the
    // "Cookie Crisp" String.

    let cookie_crisp = &cereals[0];

    // Declare a `cookie` variable that extracts a slice of
    // the text "Cookie" from the String. Print the slice.

    let cookie: &str = &cookie_crisp[..6];
    println!("{}", cookie);

    // Declare a `cocoa_puffs` variable. Make it a reference
    // to the "Cocoa Puffs" String (in other words, a &String).

    let cocoa_puffs = &cereals[3];

    // Declare a `puffs` variable that extracts a slice of
    // the text "Puffs" from the String. Print the slice.

    let puffs = &cocoa_puffs[5..];
    println!("{puffs:?}");
}
