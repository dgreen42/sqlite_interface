// to build you will need the lsqlite3 Lua library
//// lurarocks install lsqlite3
// to install lqlite3 you will also need the base and dev librarys for sqlite3, for some reasone
// they are not included when you install sqlite3 cause they are C headers? anyways...
//// apt install libsqlite3-0 libsqlite3-dev

struct Personel {
    id: i32,
    name: String,
    position: String,
}

struct Shipments {
    id: i32,
    shipment_id: i32,
}

struct Contents {
    shipment_id: i32,
    conents: String,
    quantity: String,
}

use rusqlite::{self, Connection};
use std::env::args;

fn main() {
    let option = args().nth(1).expect("Please enter option");
    if option == "--help" {
        println!("Here is the manual");
        std::process::exit(3);
    }

    let id = args().nth(2).expect("Enter Persons ID");

    if option == "-add_shipment" {
        let shipment_id = args().nth(3).expect("Enter the Shipment ID");
        let contents = args().nth(4).expect("Enter Shipment Contents");
        let quantity = args().nth(5).expect("Enter quantity of shipment");

        let current_shipment = Shipments {
            id: id.trim().parse::<i32>().unwrap(),
            shipment_id: shipment_id.trim().parse::<i32>().unwrap(),
        };

        let current_contents = Contents {
            shipment_id: shipment_id.trim().parse::<i32>().unwrap(),
            conents: contents,
            quantity: quantity,
        };

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
                    &current_contents.shipment_id,
                    &current_contents.conents,
                    &current_contents.quantity
                ),
                (),
            )
            .unwrap();

        println!("{:?}", shipments_entry);
        println!("{:?}", contents_entry);
    }
    if option == "-add_user" {
        let name = args().nth(3).expect("Enter Name of New Personel");
        let position = args().nth(4).expect("Enter Current Position");

        let path = "./company.sqlite3";
        let db = Connection::open(path).expect("Could not open database");

        let current_person = Personel {
            id: id.trim().parse::<i32>().unwrap(),
            name: name,
            position: position,
        };

        let tester = &format!(
            "INSERT INTO personel (id, name, position) VALUES ({}, {}, {})",
            &current_person.id, &current_person.name, &current_person.position,
        );
        println!("{:?}", tester);

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
