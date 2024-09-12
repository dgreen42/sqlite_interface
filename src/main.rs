// to build you will need the lsqlite3 Lua library
//// lurarocks install lsqlite3
// to install lqlite3 you will also need the base and dev librarys for sqlite3, for some reasone
// they are not included when you install sqlite3 cause they are C headers? anyways...
//// apt install libsqlite3-0 libsqlite3-dev
/*
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
        stdin().read_line(&mut id).unwrap();
        let id = id.trim();
        println!("{}: {:?}", "User ID entered: ".green(), &id);

        println!("{}", "Enter Shipment ID".cyan());
        let mut shipment_id = String::new();
        stdin().read_line(&mut shipment_id).unwrap();
        let shipment_id = shipment_id.trim();
        println!("{}: {:?}", "Shipment ID entered: ".green(), &shipment_id);

        println!("{}", "Enter Contents".cyan());
        let mut contents = String::new();
        stdin().read_line(&mut contents).unwrap();
        let contents = contents.trim().to_string();
        println!("{}: {:?}", "Contents: ".green(), &contents);

        println!("{}", "Enter Quantity".cyan());
        let mut quantity = String::new();
        stdin().read_line(&mut quantity).unwrap();
        let quantity = quantity.trim().to_string();
        println!("{}: {:?}", "Quantity: ".green(), &quantity);

        Shipments {
            id: id.trim().parse::<i32>().unwrap(),
            shipment_id: shipment_id.parse::<i32>().unwrap(),
            contents: contents,
            quantity: quantity,
        }
    }

    pub fn get_personel_info() -> Personel {
        println!("{}", "Enter Personel ID".cyan());
        let mut id = String::new();
        stdin().read_line(&mut id).unwrap();
        let id = id.trim();
        println!("{}: {:?}", "Personel ID entered: ".green(), &id);

        println!("{}", "Enter Name".cyan());
        let mut name = String::new();
        stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();
        println!("{}: {:?}", "Name: ".green(), &name);

        println!("{}", "Enter Position".cyan());
        let mut position = String::new();
        stdin().read_line(&mut position).unwrap();
        let position = position.trim().to_string();
        println!("{}: {:?}", "Position: ".green(), &position);

        Personel {
            id: id.trim().parse::<i32>().unwrap(),
            name: name,
            position: position,
        }
    }
}

*/

mod gui {

    #[derive(Debug, Clone, Copy)]
    pub enum Submission {
        User,
        Shipment,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum UserMessage {
        Id,
        Name,
        Position,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum ShipMessage {
        Id,
        ShipmentId,
        Quantity,
        Contents,
    }
}

mod company {

    use rusqlite::{self, Connection};

    #[derive(Clone)]
    pub struct Shipment {
        pub id: i32,
        pub shipment_id: i32,
        pub contents: String,
        pub quantity: String,
    }

    #[derive(Clone)]
    pub struct Personel {
        pub id: i32,
        pub name: String,
        pub position: String,
    }

    pub fn generate_shipment(
        id: &i32,
        shipment_id: &i32,
        contents: &str,
        quantity: &str,
    ) -> Shipment {
        Shipment {
            id: *id,
            shipment_id: *shipment_id,
            contents: contents.to_string(),
            quantity: quantity.to_string(),
        }
    }

    pub fn generate_personel(id: &i32, name: &str, position: &str) -> Personel {
        Personel {
            id: *id,
            name: name.to_string(),
            position: position.to_string(),
        }
    }

    impl Personel {
        pub fn personel_entry(&self, db: Connection) -> usize {
            let personel_entry = db
                .execute(
                    &format!(
                        "INSERT INTO personel (id, name, position) VALUES ({}, '{}', '{}')",
                        &self.id, &self.name, &self.position,
                    ),
                    (),
                )
                .unwrap();
            personel_entry
        }
    }

    impl Shipment {
        pub fn shipment_enty(&self, db: Connection) -> (usize, usize) {
            let shipments_entry = db
                .execute(
                    &format!(
                        "INSERT INTO shippments (id, shipment_id) VALUES ({}, {})",
                        self.id, self.shipment_id,
                    ),
                    (),
                )
                .unwrap();

            let contents_entry = db
                        .execute(
                            &format!(
                                "INSERT INTO contents (shipment_id, contents, quantity) VALUES ({}, '{}', '{}')",
                                &self.shipment_id,
                                &self.contents,
                                &self.quantity
                            ),
                            (),
                        )
                        .unwrap();
            (shipments_entry, contents_entry)
        }
    }
}

use colored::Colorize;
use company::*;
use fltk::{
    app, button::Button, frame::Frame, group::Flex, input::Input, output::Output, prelude::*,
    window::Window,
};
use gui::*;
use rusqlite::{self, Connection};
use std::{env::args, io::Bytes, os::unix::fs::FileExt};

fn main() {
    let app = app::App::default();
    let mut window = Window::new(1000, 1000, 1000, 1000, "Sqlite Interface");

    let mut flex = Flex::default()
        .with_size(900, 500)
        .center_of_parent()
        .column();

    flex.set_spacing(100);

    let add_user_box = Flex::default()
        .with_size(400, 30)
        .center_of_parent()
        .row()
        .with_label("Add User");
    let mut id1 = Input::default().with_label("ID");
    let _space = Frame::new(100, 100, 100, 30, "");

    // Make the id inputs only int

    let mut name = Input::default().with_label("Name");
    let _space = Frame::new(100, 100, 100, 30, "");
    let mut position = Input::default().with_label("Position");
    let _space = Frame::new(100, 100, 100, 30, "");

    add_user_box.end();

    let user_output = Flex::default().with_size(400, 200).column();
    let mut user_out = Output::default();
    let mut user_submit = Button::new(50, 100, 100, 200, "Submit");
    user_output.end();

    let add_shipment_box = Flex::default()
        .with_size(400, 30)
        .center_of_parent()
        .row()
        .with_label("Add Shipment");
    let mut id2 = Input::default().with_label("ID");
    let _space = Frame::new(50, 50, 50, 30, "");
    let mut shipment_id = Input::default().with_label("Shipment ID");
    let _space = Frame::new(50, 50, 50, 30, "");
    let mut quantity = Input::default().with_label("Quantity");
    let _space = Frame::new(50, 50, 50, 30, "");
    let mut contents = Input::default().with_label("Contents");
    let _space = Frame::new(50, 50, 50, 30, "");

    add_shipment_box.end();

    let shipment_output = Flex::default().with_size(400, 200).column();
    let mut shipment_out = Output::default();
    let mut shipment_submit = Button::new(50, 100, 100, 200, "Submit");
    shipment_output.end();

    flex.end();

    window.end();
    window.show();

    //open db here?

    let (us, ur) = app::channel::<UserMessage>();
    let (ss, sr) = app::channel::<ShipMessage>();
    let (sub_send, sub_rec) = app::channel::<Submission>();

    id1.emit(us.clone(), UserMessage::Id);
    name.emit(us.clone(), UserMessage::Name);
    position.emit(us, UserMessage::Position);

    id2.emit(ss, ShipMessage::Id);
    shipment_id.emit(ss, ShipMessage::ShipmentId);
    quantity.emit(ss, ShipMessage::Quantity);
    contents.emit(ss, ShipMessage::Contents);

    user_submit.emit(sub_send, Submission::User);
    shipment_submit.emit(sub_send, Submission::Shipment);

    while app.wait() {
        match ur.recv().unwrap() | sr.recv().unwrap() | sub_rec.recv().unwrap() {
            UserMessage::Id | UserMessage::Name | UserMessage::Position => {
                if let Some(user_message) = ur.recv() {
                    match user_message {
                        UserMessage::Id => user_out.set_value(&format!(
                            "{} {} {}",
                            &id1.value(),
                            &name.value(),
                            &position.value()
                        )),
                        UserMessage::Name => user_out.set_value(&format!(
                            "{} {} {}",
                            &id1.value(),
                            &name.value(),
                            &position.value()
                        )),

                        UserMessage::Position => user_out.set_value(&format!(
                            "{} {} {}",
                            &id1.value(),
                            &name.value(),
                            &position.value()
                        )),
                    }
                }
            }

            ShipMessage::Id
            | ShipMessage::Contents
            | ShipMessage::Quantity
            | ShipMessage::Contents => {
                if let Some(shipment_message) = sr.recv() {
                    match shipment_message {
                        ShipMessage::Id => shipment_out.set_value(&format!(
                            "{} {} {} {}",
                            &id1.value(),
                            &shipment_id.value(),
                            &quantity.value(),
                            &contents.value()
                        )),

                        ShipMessage::ShipmentId => shipment_out.set_value(&format!(
                            "{} {} {} {}",
                            &id1.value(),
                            &shipment_id.value(),
                            &quantity.value(),
                            &contents.value()
                        )),

                        ShipMessage::Quantity => shipment_out.set_value(&format!(
                            "{} {} {} {}",
                            &id1.value(),
                            &shipment_id.value(),
                            &quantity.value(),
                            &contents.value()
                        )),

                        ShipMessage::Contents => shipment_out.set_value(&format!(
                            "{} {} {} {}",
                            &id1.value(),
                            &shipment_id.value(),
                            &quantity.value(),
                            &contents.value()
                        )),
                    }
                }
            }

            Submission::Shipment | Submission::User => {
                if let Some(submission_message) = sub_rec.recv() {
                    let path = "./company.sqlite3";
                    let db = Connection::open(path).expect("Could not open database");
                    match submission_message {
                        Submission::User => {
                            let personel = generate_personel(
                                &id1.value().parse().unwrap(),
                                &name.value(),
                                &position.value(),
                            );
                            let personel_entry = personel.personel_entry(db);
                            assert!(personel_entry == 0, "Database entry failed");
                        }
                        Submission::Shipment => {
                            let shipment = generate_shipment(
                                &id2.value().parse().unwrap(),
                                &shipment_id.value().parse().unwrap(),
                                &contents.value(),
                                &quantity.value(),
                            );
                            let shipment_entry = shipment.shipment_enty(db);
                            assert!(shipment_entry == (0, 0), "Database entry failed");
                        }
                    }
                }
            }
        }
    }

    app.run().unwrap();

    /*
        let option = args().nth(1).expect(&"Please enter option".red());
        if option == "--help" {
            println!("Here is the manual");
            std::process::exit(3);
        }

        if option == "-add_shipment" {
            let current_shipment = get_shipment_info();
            println!(
                "{} {}, {}, {}, {}",
                "Shipment Entry: ".yellow(),
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

            if shipments_entry == 1 && contents_entry == 1 {
                println!("{}", "Entry Success".green());
            } else {
                println!("{}", "Shipment Entry Failed".red());
            }
        }
        if option == "-add_user" {
            let current_person = get_personel_info();

            println!(
                "{} {}, {}, {}",
                "Personel Entry: ".yellow(),
                current_person.id,
                current_person.name,
                current_person.position
            );

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

            if personel_entry == 1 {
                println!("{}", "Entry Success".green());
            } else {
                println!("{}", "Shipment Entry Failed".red());
            }
        }
    */
}
