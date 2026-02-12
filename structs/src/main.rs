// #[derive(Debug)]
// #[allow(dead_code)]
// struct Prof {
//     name: String,
//     age: i32,
//     domain: String,
//     alive: bool,
// }
// #[derive(Debug)]
// #[allow(dead_code)]
// struct Car {
//     manufacturer: String,
//     model: String,
//     price: i32,
// }

// fn new_car(manufacturer: String, model: String, price: i32) -> Car {
//     Car {
//         manufacturer,
//         model,
//         price,
//     }
// }

// fn main() {
//     let p1 = Prof {
//         name: String::from("Ron"),
//         age: 20,
//         alive: true,
//         domain: String::from("Go"),
//     };

//     let status = if p1.alive { "alive" } else { "dead" };

//     println!("{:#?}", p1);
//     println!(
//         "Name is {}, age is {}, domain is {}, and is {}",
//         p1.name, p1.age, p1.domain, status,
//     );

//     println!("{}", p1.name);

//     let bmw = new_car(String::from("BMW"), String::from("M5"), 5000);
//     println!("{:?}", bmw);
// }

// #[derive(Debug)]
// struct Laptop {
//     brand: String,
//     model: String,
//     is_gaming: bool,
//     price: u32,
// }

// impl Laptop {
//     fn new(brand: String, model: String, is_gaming: bool, price: u32) -> Self {
//         Laptop {
//             brand,
//             model,
//             is_gaming,
//             price,
//         }
//     }

//     fn printdata(&self) {
//         println!(
//             "Brand: {}, Model: {}, Is Gaming: {}, Price: {}",
//             self.brand, self.model, self.is_gaming, self.price,
//         )
//     }

//     fn updateprice(&mut self, n: u32) {
//         self.price = self.price + n;
//     }
// }

// fn main() {
//     let mut mylap = Laptop {
//         brand: String::from("Asus"),
//         is_gaming: false,
//         model: String::from("Rog"),
//         price: 1_00_000,
//     };

//     mylap.printdata();

//     println!("{:?}", mylap);

//     mylap.updateprice(1000);

//     println!("{:?}", mylap);

//     println!("================");

//     let lap = Laptop::new(String::from("lenovo"), String::from("legion"), true, 50000);
//     println!("{:?}", lap);
// }

// #[derive(Debug)]
// struct Profile {
//     name: String,
//     age: u8,
//     domain: String,
//     is_employed: bool,
// }

// // builder pattern
// impl Profile {
//     fn new(name: String, age: u8, domain: String, is_employed: bool) -> Self {
//         Profile {
//             name,
//             age,
//             domain,
//             is_employed,
//         }
//     }

//     fn up_age(&mut self, age: u8) -> &mut Self {
//         self.age = age;
//         self
//     }
//     fn up_empstatus(&mut self, f: bool) -> &mut Self {
//         self.is_employed = f;
//         self
//     }
// }

// fn main() {
//     let mut me = Profile::new(String::from("tharun"), 20, String::from("Go"), false);

//     println!("{:?}", me);

//     me.up_age(50).up_empstatus(true);

//     println!("{:?}", me);
// }

// ====================================================================================
// mini - exercise

#[derive(Debug)]
struct Flight {
    org: String,
    dest: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(org: String, dest: String, price: f64, passengers: u32) -> Self {
        Flight {
            org,
            dest,
            price,
            passengers,
        }
    }

    fn change_dest(&mut self, new_dest: String) -> &mut Self {
        self.dest = new_dest;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price = self.price * 1.20;
        self
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.org, self.dest);
    }
}
fn main() {
    let mut f1 = Flight::new("delhi".to_string(), "dubai".to_string(), 155.50, 122);
    f1.itinerary();
    f1.change_dest("london".to_string()).increase_price();

    println!("{:?}", f1);
}
