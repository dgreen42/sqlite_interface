// to build you will need the lsqlite3 Lua library
//// lurarocks install lsqlite3
// to install lqlite3 you will also need the base and dev librarys for sqlite3, for some reasone
// they are not included when you install sqlite3 cause they are C headers? anyways...
//// apt install libsqlite3-0 libsqlite3-dev

mod company {
    use colored::Colorize;
    use std::io::stdin;

    pub struct Shipments {
        pub id: i32,
        pub shipment_id: i32,
        pub contents: String,
        pub quantity: String,
    }

    pub struct Personel {
        pub id: i32,
        pub name: String,
        pub position: String,
    }

    pub fn get_shipment_info() -> Shipments {
        println!("{}", "Enter Personel ID".cyan());
        let mut id = String::new();
        _ = stdin().read_line(&mut id).unwrap().to_string().trim();
        println!("{}: {:?}", "User ID entered: ".green(), &id);

        println!("{}", "Enter Shipment ID".cyan());
        let mut shipment_id = String::new();
        _ = stdin()
            .read_line(&mut shipment_id)
            .unwrap()
            .to_string()
            .trim();
        println!("{}: {:?}", "Shipment ID entered: ".green(), &shipment_id);

        println!("{}", "Enter Contents".cyan());
        let mut contents = String::new();
        _ = stdin().read_line(&mut contents).unwrap().to_string().trim();
        println!("{}: {:?}", "Contents: ".green(), &contents);

        println!("{}", "Enter Quantity".cyan());
        let mut quantity = String::new();
        _ = stdin().read_line(&mut quantity).unwrap().to_string().trim();
        println!("{}: {:?}", "Quantity: ".green(), &quantity);

        Shipments {
            id: id.trim().parse::<i32>().unwrap(),
            shipment_id: shipment_id.trim().parse::<i32>().unwrap(),
            contents: contents,
            quantity: quantity,
        }
    }

    pub fn get_personel_info() -> Personel {
        println!("{}", "Enter Personel ID".cyan());
        let mut id = String::new();
        _ = stdin().read_line(&mut id).unwrap().to_string().trim();
        println!("{}: {:?}", "Personel ID entered: ".green(), &id);

        println!("{}", "Enter Name".cyan());
        let mut name = String::new();
        _ = stdin().read_line(&mut name).unwrap().to_string().trim();
        println!("{}: {:?}", "Name: ".green(), &name);

        println!("{}", "Enter Position".cyan());
        let mut position = String::new();
        _ = stdin().read_line(&mut position).unwrap().to_string().trim();
        println!("{}: {:?}", "Position: ".green(), &position);

        Personel {
            id: id.trim().parse::<i32>().unwrap(),
            name: name,
            position: position,
        }
    }
}

use company::*;
use rusqlite::{self, Connection};
use std::env::args;

fn main() {
    let option = args().nth(1).expect("Please enter option");
    if option == "--help" {
        println!("Here is the manual");
        std::process::exit(3);
    }

    if option == "-add_shipment" {
        let current_shipment = get_shipment_info();
        println!(
            "{},{},{},{}",
            current_shipment.id,
            current_shipment.shipment_id,
            current_shipment.contents,
            current_shipment.quantity
        );

        let path = "./company.sqlite3";
        let db = Connection::open(path).expect("Could not open database");

        // decided to use format! instead of putting the parameters in with params! and using th 1?
        // format that the library uses. Either way fors to format the query. May change it later
        let shipments_entry = db
            .execute(
                &format!(
                    "INSERT INTO shippments (id, shipment_id) VALUES ({}, {})",
                    current_shipment.id, current_shipment.shipment_id,
                ),
                (),
            )
            .unwrap();

        let contents_entry = db
                    .execute(
                        &format!(
                            "INSERT INTO contents (shipment_id, contents, quantity) VALUES ({}, '{}', '{}')",
                            &current_shipment.shipment_id,
                            &current_shipment.contents,
                            &current_shipment.quantity
                        ),
                        (),
                    )
                    .unwrap();

        println!("{:?}", shipments_entry);
        println!("{:?}", contents_entry);
    }
    if option == "-add_user" {
        let current_person = get_personel_info();

        let path = "./company.sqlite3";
        let db = Connection::open(path).expect("Could not open database");

        let personel_entry = db
            .execute(
                &format!(
                    "INSERT INTO personel (id, name, position) VALUES ({}, '{}', '{}')",
                    &current_person.id, &current_person.name, &current_person.position,
                ),
                (),
            )
            .unwrap();

        println!("{:?}", personel_entry);
    }
}
