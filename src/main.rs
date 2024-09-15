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

    use ::fltk::{draw, table::Table};
    use fltk::enums;
    use rusqlite::{self, Connection, Error, Statement};

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
                                "INSERT INTO contents (shipment_id, quantity, contents) VALUES ({}, '{}', '{}')",
                                &self.shipment_id,
                                &self.quantity,
                                &self.contents
                            ),
                            (),
                        )
                        .unwrap();
            (shipments_entry, contents_entry)
        }
    }

    #[derive(Debug)]
    pub struct PersonelRow {
        pub id: i32,
        pub name: String,
        pub position: String,
    }

    #[derive(Debug)]
    pub struct ShipmentRow {
        pub id: i32,
        pub shipment_id: i32,
    }

    #[derive(Debug)]
    pub struct ContentsRow {
        pub shipment_id: i32,
        pub quantity: String,
        pub contents: String,
    }

    pub fn generate_personel_table(db: Connection, sort: i32) -> Vec<PersonelRow> {
        fn make_table(mut table_query: Statement<'_>) -> Vec<PersonelRow> {
            let table_iter = table_query
                .query_map([], |row| {
                    Ok(PersonelRow {
                        id: row.get(0).unwrap(),
                        name: row.get(1).unwrap(),
                        position: row.get(2).unwrap(),
                    })
                })
                .unwrap();
            let mut table = Vec::new();
            for row in table_iter {
                table.push(row.unwrap());
            }
            table
        }
        if sort == 0 {
            let table_query = db
                .prepare(&format!("SELECT * FROM personel ORDER BY id"))
                .unwrap();
            println!("{:?}", table_query);
            make_table(table_query)
        } else if sort == 1 {
            let table_query = db
                .prepare(&format!("SELECT * FROM personel ORDER BY name"))
                .unwrap();
            make_table(table_query)
        } else if sort == 2 {
            let table_query = db
                .prepare(&format!("SELECT * FROM personel ORDER BY position"))
                .unwrap();
            make_table(table_query)
        } else {
            let table_query = db.prepare(&format!("SELECT * FROM personel")).unwrap();
            make_table(table_query)
        }
    }

    pub fn generate_shipment_table(db: Connection, sort: i32) -> Vec<ShipmentRow> {
        fn make_table(mut table_query: Statement<'_>) -> Vec<ShipmentRow> {
            let table_iter = table_query
                .query_map([], |row| {
                    Ok(ShipmentRow {
                        id: row.get(0).unwrap(),
                        shipment_id: row.get(1).unwrap(),
                    })
                })
                .unwrap();
            let mut table = Vec::new();
            for row in table_iter {
                table.push(row.unwrap());
            }
            table
        }
        if sort == 0 {
            let table_query = db
                .prepare(&format!("SELECT * FROM shippments ORDER BY id"))
                .unwrap();
            make_table(table_query)
        } else if sort == 4 {
            let table_query = db
                .prepare(&format!("SELECT * FROM shippments ORDER BY shipment_id"))
                .unwrap();
            make_table(table_query)
        } else {
            let table_query = db.prepare(&format!("SELECT * FROM shippments")).unwrap();
            make_table(table_query)
        }
    }

    pub fn generate_contents_table(db: Connection, sort: i32) -> Vec<ContentsRow> {
        fn make_table(mut table_query: Statement<'_>) -> Vec<ContentsRow> {
            let table_iter = table_query
                .query_map([], |row| {
                    Ok(ContentsRow {
                        shipment_id: row.get(0).unwrap(),
                        quantity: row.get(1).unwrap(),
                        contents: row.get(2).unwrap(),
                    })
                })
                .unwrap();
            let mut table = Vec::new();
            for row in table_iter {
                table.push(row.unwrap());
            }
            table
        }

        if sort == 4 {
            let table_query = db
                .prepare("SELECT * FROM contents ORDER BY shipment_id")
                .unwrap();
            make_table(table_query)
        } else if sort == 5 {
            let table_query = db
                .prepare(&format!("SELECT * FROM contents ORDER BY quantity"))
                .unwrap();
            make_table(table_query)
        } else if sort == 6 {
            let table_query = db
                .prepare(&format!("SELECT * FROM contents ORDER BY contents"))
                .unwrap();
            make_table(table_query)
        } else {
            let table_query = db.prepare("SELECT * FROM contents").unwrap();
            make_table(table_query)
        }
    }

    pub fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
        draw::push_clip(x, y, w, h);
        draw::draw_box(
            enums::FrameType::ThinUpBox,
            x,
            y,
            w,
            h,
            enums::Color::FrameDefault,
        );
        draw::set_draw_color(enums::Color::Black);
        draw::set_font(enums::Font::Helvetica, 14);
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
        draw::pop_clip();
    }

    pub fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
        let xyh = 50;
        let width = 50;
        draw::push_clip(x, y, w, h);
        if selected {
            draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
        } else {
            draw::set_draw_color(enums::Color::White);
        }
        draw::draw_rectf(x, y, w, h);
        draw::set_draw_color(enums::Color::Gray0);
        draw::set_font(enums::Font::Helvetica, 14);
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
        draw::draw_rect(x, y, w, h);
        draw::pop_clip();
    }

    pub fn matricize_personel(data: Vec<PersonelRow>) -> Vec<Vec<String>> {
        let mut primary_vec: Vec<Vec<String>> = Vec::new();

        for row in data {
            let mut secondary_vec: Vec<String> = Vec::new();
            let string = format!("{},{},{}", row.id, row.name, row.position);
            let ssplit = string.split(",");
            for element in ssplit {
                secondary_vec.push(element.trim().to_string());
            }
            primary_vec.push(secondary_vec);
        }
        println!("{:?}", primary_vec);
        primary_vec
    }

    pub fn matricize_shipment(data: Vec<ShipmentRow>) -> Vec<Vec<String>> {
        let mut primary_vec: Vec<Vec<String>> = Vec::new();

        for row in data {
            let mut secondary_vec: Vec<String> = Vec::new();
            let string = format!("{},{}", row.id, row.shipment_id);
            let ssplit = string.split(",");
            for element in ssplit {
                secondary_vec.push(element.trim().to_string());
            }
            primary_vec.push(secondary_vec);
        }
        println!("{:?}", primary_vec);
        primary_vec
    }

    pub fn matricize_contents(data: Vec<ContentsRow>) -> Vec<Vec<String>> {
        let mut primary_vec: Vec<Vec<String>> = Vec::new();

        for row in data {
            let mut secondary_vec: Vec<String> = Vec::new();
            let string = format!("{},{},{}", row.shipment_id, row.quantity, row.contents);
            let ssplit = string.split(",");
            for element in ssplit {
                secondary_vec.push(element.trim().to_string());
            }
            primary_vec.push(secondary_vec);
        }
        println!("{:?}", primary_vec);
        primary_vec
    }
}

use colored::Colorize;
use company::*;
use fltk::{
    app,
    button::Button,
    draw, enums,
    frame::Frame,
    group::Flex,
    input::Input,
    menu::{Choice, MenuBar},
    output::Output,
    prelude::*,
    table::{self, Table},
    window::Window,
};
pub use gui::*;
use rusqlite::{self, Connection};
use std::{env::args, io::Bytes, os::unix::fs::FileExt};

fn main() {
    let app = app::App::default();
    let mut window = Window::new(500, 700, 1000, 800, "Sqlite Interface");

    let flex = Flex::default()
        .with_size(900, 700)
        .column()
        .center_of_parent();

    let mut flex_db_entry = Flex::default().with_size(800, 300).column();

    flex_db_entry.set_spacing(50);

    let add_user_box = Flex::default()
        .with_size(800, 200)
        .row()
        .with_label("Add User");
    let mut id1 = Input::default().with_label("ID");
    let _space = Frame::default();

    // Make the id inputs only int

    let mut name = Input::default().with_label("Name");
    let _space = Frame::default();
    let mut position = Input::default().with_label("Position");
    let _space = Frame::default();

    add_user_box.end();

    let user_output = Flex::default()
        .with_size(800, 100)
        .column()
        .with_label("Data Base Query");
    let mut user_out = Output::default();
    let mut user_submit = Button::new(50, 100, 100, 200, "Submit");

    user_output.end();

    let add_shipment_box = Flex::default()
        .with_size(800, 200)
        .row()
        .with_label("Add Shipment");
    let mut id2 = Input::default().with_label("ID");
    let _space = Frame::default();
    let mut shipment_id = Input::default().with_label("Shipment ID");
    let _space = Frame::default();
    let mut quantity = Input::default().with_label("Quantity");
    let _space = Frame::default();
    let mut contents = Input::default().with_label("Contents");
    let _space = Frame::default();

    add_shipment_box.end();

    let shipment_output = Flex::default()
        .with_size(800, 100)
        .column()
        .with_label("Data Base Query");
    let mut shipment_out = Output::default();
    let mut shipment_submit = Button::new(50, 100, 100, 200, "Submit");

    shipment_output.end();

    flex_db_entry.end();

    let flex_data = Flex::default()
        .with_size(800, 300)
        .column()
        .below_of(&flex_db_entry, 20);

    let menu_flex = Flex::new(100, 100, 800, 100, "")
        .center_of_parent()
        .column();

    let sub_title_flex = Flex::new(100, 100, 800, 100, "").center_of_parent().row();

    let _sub_title1 = Frame::new(100, 100, 100, 100, "Select Data to Display");
    let _sub_title2 = Frame::new(100, 100, 100, 100, "Sort By");

    sub_title_flex.end();

    let sub_menu_flex = Flex::new(100, 100, 800, 100, "").center_of_parent().row();

    let mut table_menu = MenuBar::new(100, 100, 800, 100, "");
    table_menu.add_choice("Personel|Shipments|Contents");
    let mut sort_menu = MenuBar::new(100, 100, 800, 100, "").with_align(enums::Align::Right);
    sort_menu.add_choice("ID|Name|Position|Shipment ID|Quantity|Contents");

    sub_menu_flex.end();

    menu_flex.end();

    let mut table = Table::default().with_size(800, 200);

    table.end();

    flex_data.end();

    flex.end();

    window.make_resizable(true);
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

                Message::Menu => {}

                Message::Table => {
                    let sort_opt = sort_menu.value();
                    println!("{}", sort_opt);
                    match table_menu.value().to_string().as_ref() {
                        // 0 = personel
                        // 1 = shipments
                        // 2 = contents
                        "0" => {
                            let data = generate_personel_table(db, sort_opt);
                            table.set_rows(data.len().to_string().parse::<i32>().unwrap());
                            table.set_cols(3);
                            let mat_data = matricize_personel(data);
                            let headers = Vec::from(["ID", "Name", "Position"]);
                            table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                                table::TableContext::StartPage => {
                                    draw::set_font(enums::Font::Helvetica, 14)
                                }
                                table::TableContext::ColHeader => {
                                    draw_header(&format!("{}", col as u8 as char), x, y, w, h)
                                }
                                table::TableContext::RowHeader => {
                                    draw_header(&format!("{}", headers[row as usize]), x, y, w, h)
                                }
                                table::TableContext::Cell => {
                                    draw_data(
                                        &format!("{:?}", mat_data[row as usize][col as usize]),
                                        x,
                                        y,
                                        w,
                                        h,
                                        t.is_selected(row, col),
                                    );
                                    println!("{} {} {} {}", x, y, w, h);
                                }
                                _ => (),
                            });
                        }
                        "1" => {
                            let data = generate_shipment_table(db, sort_opt);
                            table.set_rows(data.len().to_string().parse::<i32>().unwrap());
                            table.set_cols(2);
                            let mat_data = matricize_shipment(data);
                            table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                                table::TableContext::StartPage => {
                                    draw::set_font(enums::Font::Helvetica, 14)
                                }
                                table::TableContext::ColHeader => draw_header(
                                    &format!("{}", (col + 65) as u8 as char),
                                    x,
                                    y,
                                    w,
                                    h,
                                ),
                                table::TableContext::RowHeader => {
                                    draw_header(&format!("{}", row + 1), x, y, w, h)
                                }
                                table::TableContext::Cell => draw_data(
                                    &format!("{:?}", mat_data[row as usize][col as usize]),
                                    x,
                                    y,
                                    w,
                                    h,
                                    t.is_selected(row, col),
                                ),
                                _ => (),
                            });
                        }
                        "2" => {
                            let data = generate_contents_table(db, sort_opt);
                            table.set_rows(data.len().to_string().parse::<i32>().unwrap());
                            table.set_cols(3);
                            let mat_data = matricize_contents(data);
                            table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                                table::TableContext::StartPage => {
                                    draw::set_font(enums::Font::Helvetica, 14)
                                }
                                table::TableContext::ColHeader => draw_header(
                                    &format!("{}", (col + 65) as u8 as char),
                                    x,
                                    y,
                                    w,
                                    h,
                                ),
                                table::TableContext::RowHeader => {
                                    draw_header(&format!("{}", row + 1), x, y, w, h)
                                }
                                table::TableContext::Cell => draw_data(
                                    &format!("{:?}", mat_data[row as usize][col as usize]),
                                    x,
                                    y,
                                    w,
                                    h,
                                    t.is_selected(row, col),
                                ),
                                _ => (),
                            });
                        }
                        _ => (),
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
