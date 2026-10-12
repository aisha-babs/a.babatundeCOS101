fn main() {
    
    // Create an empty vector 
    let mut _level_checker : Vec<String> = Vec::new();
    
    println!("WELCOME TO THE APS LEVEL CHECKER");
    
    //Use a macro to write the vector containing the positions for each professions
    let levels = vec!["APS 1-2","APS 3-5", "APS 5-8","EL1 8-10", "EL2 10-13", "SES",];
    let public_servant =  vec!["APS 1-2","APS 3-5", "APS 5-8","EL1 8-10", "EL2 10-13", "SES",];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director","CEO",];
    let academic = vec!["-","Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean",];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner",];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal",];

    // Ask the user to enter their profession
    println!("Choose a profession:");
    println!("1. Public Servant");
    println!("2. Office Admin");
    println!("3. Academic");
    println!("4. Lawyer");
    println!("5. Teacher");
    
    let mut profession = String::new();
    std::io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim();
    
    println!("Enter the position:");
    let mut position = String::new();
    std::io::stdin().read_line(&mut position).expect("Failed to read input");
    let position = position.trim();

    // Use match to select the correct profession
    let positions = match profession {
        "1" => &public_servant,
        "2" => &office_admin,
        "3" => &academic,
        "4" => &lawyer,
        "5" => &teacher,
        _ => {
            println!("Invalid profession selected.");
            return;
        }
    };

     // Find the position in the selected vector
    match positions.iter().position(|&x| x == position) {
        Some(index) => {
            println!("The staff level is: {}", levels[index]);
        }

        None => {
            println!("Position not found. Please check your input.");
        }
    }
}













