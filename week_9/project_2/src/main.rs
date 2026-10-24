use std::fs::File;
use std::io::Write;

fn main() {
    // We create a vector using vec! to hold all our student info.
    // Each student is represented by a tuple: (Name, Matric Number, Department, Level)
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shania Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
    ];

    // --- STEP 1: DISPLAY THE DETAILS ON THE SCREEN ---
    println!("--- STUDENT MANAGEMENT INFORMATION SYSTEM ---");
    // We print the table header (the numbers like :<20 just add neat spaces so it looks like a clean table)
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{}", "-".repeat(60));

    // We loop through our vector to show each student on the screen
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.0, student.1, student.2, student.3);
    }

    // --- STEP 2: SAVE THE DETAILS TO A FILE ---
    // We create a new file named student_records.txt
    let mut file = File::create("student_records.txt").expect("Oops, could not create the file!");

    // We create the header string for our file
    let file_header = format!("{:<20} {:<15} {:<15} {:<5}\n", "Student Name", "Matric. Number", "Department", "Level");
    let line_separator = format!("{}\n", "-".repeat(60));

    // We use write_all instead of writeln! to save the header lines
    file.write_all(file_header.as_bytes()).expect("Failed to write header");
    file.write_all(line_separator.as_bytes()).expect("Failed to write line");

    // We loop through the vector again and use write_all to save each student row
    for student in &students {
        let row = format!(
            "{:<20} {:<15} {:<15} {:<5}\n",
            student.0, student.1, student.2, student.3);
        file.write_all(row.as_bytes()).expect("Failed to write student data");
    }

    println!("\nSuccess! All student data has been saved to 'student_records.txt'.");
}
