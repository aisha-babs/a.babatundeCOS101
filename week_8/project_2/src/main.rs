fn main() {
    // First, we create a vector containing tuples of candidates: (name, years_of_experience)
    // This uses both "vector" and "tuple" compound data types as requested.
    let candidates = vec![
        ("Chidi", 3),
        ("Amara", 8),
        ("Tunde", 5),
        ("Zainab", 12),
        ("Olumide", 6),
    ];

    // Then, initialize variables to keep track of the highest experience found so far
    let mut highest_name = "";
    let mut highest_years = 0;

    // Walk through the vector with a loop
    for candidate in &candidates {
        // Accessing tuple elements: .0 is the name, .1 is the years of experience
        if candidate.1 > highest_years {
            highest_years = candidate.1;
            highest_name = candidate.0;
        }
    }

    //  Lastly, print out the final result for EY Nigeria
    println!("--- EY Nigeria Interview Results ---");
    println!("The candidate with the highest programming experience is: {}", highest_name);
    println!("Years of experience: {}", highest_years);
}
