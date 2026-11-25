use std::io::stdin;

fn admin() {
    println!("ADMINISTRATOR - Full database");
    println!("Database: globacom_dbase");
    println!("");
    println!("Customer table:");
    println!("C_id - int, primary key, not null");
    println!("C_name - varchar(50), not null");
    println!("C_age - int, not null");
    println!("C_email - varchar(50), not null");
    println!("C_mobile - varchar(20), not null");
    println!("Eid - int, not null");
    println!("Data_id - int, not null");
    println!("");
    println!("Dataplan table:");
    println!("Data_id - int, primary key, not null");
    println!("Data_size - varchar(20), not null");
    println!("Data_duration - int, not null");
    println!("Data_price - int, not null");
}

fn project_manager() {
    println!("PROJECT MANAGER - Project table");
    println!("Project table is not in this database");
    println!("Only Customer and Dataplan are in globacom_dbase");
}

fn employee() {
    println!("EMPLOYEE - Staff table");
    println!("Staff table is not in this database");
    println!("Only Customer and Dataplan are in globacom_dbase");
}

fn customer() {
    println!("CUSTOMER - Customer table");
    println!("C_id - int, primary key, not null");
    println!("C_name - varchar(50), not null");
    println!("C_age - int, not null");
    println!("C_email - varchar(50), not null");
    println!("C_mobile - varchar(20), not null");
    println!("Eid - int, not null");
    println!("Data_id - int, not null");
}

fn vendor() {
    println!("VENDOR - Dataplan table");
    println!("Data_id - int, primary key, not null");
    println!("Data_size - varchar(20), not null");
    println!("Data_duration - int, not null");
    println!("Data_price - int, not null");
}

fn main() {
    println!("Globacom Ltd");
    println!("Who are you?");
    println!("1. Administrator");
    println!("2. Project manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut choice = String::new();
    stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    if choice == "1" {
        admin();
    } else if choice == "2" {
        project_manager();
    } else if choice == "3" {
        employee();
    } else if choice == "4" {
        customer();
    } else if choice == "5" {
        vendor();
    } else {
        println!("Wrong number, enter 1 to 5");
    }
}