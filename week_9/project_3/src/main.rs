use std::fs::File;
use std::io::Write;

fn main() {
    // First, we define the three separate datasets as vectors
    let commissioners: Vec<&str> = vec![
        "Aigbogun Alamba Dautu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries: Vec<&str> = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones: Vec<&str> = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Then,we create and open the output file
    let mut file = File::create("merged_ministers.txt").expect("Failed to create the output file");

    // Header line for the merged file
    let header = "Commissioner | Ministry | Zone\n";
    file.write_all(header.as_bytes()).expect("Failed to write header to file");

    // Merge datasets by index row-by-row
    for i in 0..commissioners.len() {
        let line = format!(
            "{} | {} | {}\n",
            commissioners[i], ministries[i], zones[i]);
        
        file.write_all(line.as_bytes()).expect("Failed to write record to file");
    }

    println!("Success: Data successfully merged into 'merged_ministers.txt'");
}
