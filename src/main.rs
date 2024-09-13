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
    pub enum Message {
        User,
        Shipment,
        Id,
        Id2,
        Name,
        Position,
        ShipmentId,
        Quantity,
        Contents,
        Table,
        Menu,
    }
}

mod company {

    use ::fltk::table::Table;
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

    pub fn generate_table_size(db: Connection, option: String, table: Table) {
        let table_contents = db.execute(&format!("SELECT * FROM {}", option), ());
        println!("{:?}", table_contents);
    }
}

use colored::Colorize;
use company::*;
use fltk::{
    app,
    button::Button,
    frame::Frame,
    group::Flex,
    input::Input,
    menu::{Choice, MenuBar},
    output::Output,
    prelude::*,
    table::Table,
    window::Window,
};
use gui::*;
use rusqlite::{self, Connection};
use std::{env::args, io::Bytes, os::unix::fs::FileExt};

fn main() {
    let app = app::App::default();
    let mut window = Window::new(1000, 1000, 1000, 1000, "Sqlite Interface");

    let mut flex = Flex::default()
        .with_size(900, 900)
        .center_of_parent()
        .column();

    flex.set_spacing(20);

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

    let user_output = Flex::default()
        .with_size(400, 200)
        .column()
        .with_label("Data Base Query");
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

    let shipment_output = Flex::default()
        .with_size(400, 200)
        .column()
        .with_label("Data Base Query");
    let mut shipment_out = Output::default();
    let mut shipment_submit = Button::new(50, 100, 100, 200, "Submit");

    shipment_output.end();

    let menu_flex = Flex::default().with_size(50, 50).center_of_parent().row();

    let mut table_menu = MenuBar::default()
        .with_label("Select Data to Display")
        .center_of_parent()
        .with_size(50, 50);
    table_menu.add_choice("Personel|Shipments");

    menu_flex.end();

    let mut table = Table::default()
        .with_size(600, 600)
        .center_of_parent()
        .with_label("Data");
    table.set_rows(50);
    table.set_cols(50);

    table.end();

    flex.end();

    window.end();
    window.show();

    //open db here?

    let (send, recieve) = app::channel::<Message>();

    id1.emit(send, Message::Id);
    name.emit(send, Message::Name);
    position.emit(send, Message::Position);

    id2.emit(send, Message::Id);
    shipment_id.emit(send, Message::ShipmentId);
    quantity.emit(send, Message::Quantity);
    contents.emit(send, Message::Contents);

    user_submit.emit(send, Message::User);
    shipment_submit.emit(send, Message::Shipment);

    table_menu.emit(send, Message::Menu);
    table.emit(send, Message::Table);

    while app.wait() {
        let path = "./company.sqlite3";
        let db = Connection::open(path).expect("Could not open database");
        if let Some(user_message) = recieve.recv() {
            match user_message {
                Message::Id => user_out.set_value(&format!(
                    "INSERT INTO personel VALUES ({}, {}, {})",
                    &id1.value(),
                    &name.value(),
                    &position.value()
                )),
                Message::Name => user_out.set_value(&format!(
                    "INSERT INTO personel VALUES ({}, {}, {})",
                    &id1.value(),
                    &name.value(),
                    &position.value()
                )),

                Message::Position => user_out.set_value(&format!(
                    "INSERT INTO personel VALUES ({}, {}, {})",
                    &id1.value(),
                    &name.value(),
                    &position.value()
                )),

                Message::Id2 => shipment_out.set_value(&format!(
                    "INSERT INTO shippments VALUES ({}, {}, {}, {})",
                    &id2.value(),
                    &shipment_id.value(),
                    &quantity.value(),
                    &contents.value()
                )),

                Message::ShipmentId => shipment_out.set_value(&format!(
                    "INSERT INTO shippments VALUES ({}, {}, {}, {})",
                    &id2.value(),
                    &shipment_id.value(),
                    &quantity.value(),
                    &contents.value()
                )),

                Message::Quantity => shipment_out.set_value(&format!(
                    "INSERT INTO shippments VALUES ({}, {}, {}, {})",
                    &id2.value(),
                    &shipment_id.value(),
                    &quantity.value(),
                    &contents.value()
                )),

                Message::Contents => shipment_out.set_value(&format!(
                    "INSERT INTO shippments VALUES ({}, {}, {}, {})",
                    &id2.value(),
                    &shipment_id.value(),
                    &quantity.value(),
                    &contents.value()
                )),

                Message::User => {
                    let personel = generate_personel(
                        &id1.value().parse().unwrap(),
                        &name.value(),
                        &position.value(),
                    );
                    let personel_entry = personel.personel_entry(db);
                    assert!(personel_entry == 1, "Database entry failed");
                }

                Message::Shipment => {
                    let shipment = generate_shipment(
                        &id2.value().parse().unwrap(),
                        &shipment_id.value().parse().unwrap(),
                        &contents.value(),
                        &quantity.value(),
                    );
                    let shipment_entry = shipment.shipment_enty(db);
                    assert!(shipment_entry == (1, 1), "Database entry failed");
                }

                Message::Table => {}
                Message::Menu => {}
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
