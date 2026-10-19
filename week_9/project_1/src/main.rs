use std::fs::File;
use std::io::Write;

fn main() {
    //First, we create vectors containing the drink names for each category
    let lager = vec![
        "33 Export", 
        "Desperados", 
        "Goldberg", 
        "Gulder", 
        "Heineken", 
        "Star"];
    
    let stout = vec![
        "Legend", 
        "Turbo King", 
        "Williams"
        "-"];
    
    let non_alcoholic = vec![
        "Maltina", 
        "Amstel Malta", 
        "Malta Gold", 
        "Fayrouz"];

    // 2. Create a new text file to save the data
    let mut file = File::create("nigerian_breweries.txt").expect("create failed");

    //  Here, we write the Lager category heading and its items
    file.write_all(b"--- LAGER ---\n").expect("write failed");
    for drink in lager {
        let line = format!("{}\n", drink);
        file.write_all(line.as_bytes()).expect("write failed");
    }
    file.write_all(b"\n").expect("write failed"); 

    //  Then, write the Stout category heading and its items
    file.write_all(b"--- STOUT ---\n").expect("write failed");
    for drink in stout {
        let line = format!("{}\n", drink);
        file.write_all(line.as_bytes()).expect("write failed");
    }
    file.write_all(b"\n").expect("write failed");

    // Lastly, write the Non-Alcoholic category heading and its items
    file.write_all(b"--- NON-ALCOHOLIC ---\n").expect("write failed");
    for drink in non_alcoholic {
        let line = format!("{}\n", drink);
        file.write_all(line.as_bytes()).expect("write failed");

}
    println!("Success! File 'nigerian_breweries.txt' has been created.");
    
}
