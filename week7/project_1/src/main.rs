use std::io;

fn main() {
    // Define the roles and specializations using vectors
    let office_administrator = vec![
        "Intern", "Administrator", "Senior_Administrator", "Office_Manager", "Director", "CEO",
    ];
    let academic = vec![
        "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean",
    ];
    let lawyer = vec![
        "Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2",
        "Senior Associate 3-4", "Partner",
    ];
    let teacher = vec![
        "Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal",
        "Principal",
    ];

    // Input: Profession
    println!("What profession are you in? (e.g., office_administrator, academic, lawyer, teacher):");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let profession = input1.trim();

    // Input: Specialization
    println!("What exactly do you specialize in? (e.g., Intern, Research Assistant, etc.):");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let specialization = input2.trim();

    // Input: Years of experience
    println!("How many years of experience do you have?");
    let mut input3 = String::new();
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read input");
    let years_of_experience: i32 = input3.trim().parse().expect("Please enter a valid number");

    // Determine APS level using if-else statements
    if profession == "office_administrator" && office_administrator.contains(&specialization) {
        if specialization == "Intern" && years_of_experience >= 1 && years_of_experience <= 2 {
            println!("Based on your input, your APS level is: APS 1-2");
        } else if specialization == "Administrator" && years_of_experience >= 3 && years_of_experience <= 5 {
            println!("Based on your input, your APS level is: APS 3-5");
        } else if specialization == "Senior_Administrator" && years_of_experience >= 5 && years_of_experience <= 8 {
            println!("Based on your input, your APS level is: APS 5-8");
        } else if specialization == "Office_Manager" && years_of_experience >= 8 && years_of_experience <= 10 {
            println!("Based on your input, your APS level is: EL1 8-10");
        } else if specialization == "Director" && years_of_experience >= 10 && years_of_experience <= 13 {
            println!("Based on your input, your APS level is: EL2 10-13");
        } else if specialization == "CEO" {
            println!("Based on your input, your APS level is: SES");
        } else {
            println!("Invalid years of experience for the given specialization.");
        }
    } else if profession == "academic" && academic.contains(&specialization) {
        if specialization == "Research Assistant" && years_of_experience >= 3 && years_of_experience <= 5 {
            println!("Based on your input, your APS level is: APS 3-5");
        } else if specialization == "PhD Candidate" && years_of_experience >= 5 && years_of_experience <= 8 {
            println!("Based on your input, your APS level is: APS 5-8");
        } else if specialization == "Post-Doc Researcher" && years_of_experience >= 8 && years_of_experience <= 10 {
            println!("Based on your input, your APS level is: EL1 8-10");
        } else if specialization == "Senior Lecturer" && years_of_experience >= 10 && years_of_experience <= 13 {
            println!("Based on your input, your APS level is: EL2 10-13");
        } else if specialization == "Dean" {
            println!("Based on your input, your APS level is: SES");
        } else {
            println!("Invalid years of experience for the given specialization.");
        }
    } else if profession == "lawyer" && lawyer.contains(&specialization) {
        if specialization == "Paralegal" && years_of_experience >= 3 && years_of_experience <= 5 {
            println!("Based on your input, your APS level is: APS 3-5");
        } else if specialization == "Associate" && years_of_experience >= 5 && years_of_experience <= 8 {
            println!("Based on your input, your APS level is: APS 5-8");
        } else if specialization == "Senior Associate 1-2" && years_of_experience >= 8 && years_of_experience <= 10 {
            println!("Based on your input, your APS level is: EL1 8-10");
        } else if specialization == "Senior Associate 3-4" && years_of_experience >= 10 && years_of_experience <= 13 {
            println!("Based on your input, your APS level is: EL2 10-13");
        } else if specialization == "Partner" {
            println!("Based on your input, your APS level is: SES");
        } else {
            println!("Invalid years of experience for the given specialization.");
        }
    } else if profession == "teacher" && teacher.contains(&specialization) {
        if specialization == "Placement" && years_of_experience >= 1 && years_of_experience <= 2 {
            println!("Based on your input, your APS level is: APS 1-2");
        } else if specialization == "Classroom Teacher" && years_of_experience >= 3 && years_of_experience <= 5 {
            println!("Based on your input, your APS level is: APS 3-5");
        } else if specialization == "Snr Teacher" && years_of_experience >= 5 && years_of_experience <= 8 {
            println!("Based on your input, your APS level is: APS 5-8");
        } else if specialization == "Leading Teacher" && years_of_experience >= 8 && years_of_experience <= 10 {
            println!("Based on your input, your APS level is: EL1 8-10");
        } else if specialization == "Deputy Principal" && years_of_experience >= 10 && years_of_experience <= 13 {
            println!("Based on your input, your APS level is: EL2 10-13");
        } else if specialization == "Principal" {
            println!("Based on your input, your APS level is: SES");
        } else {
            println!("Invalid years of experience for the given specialization.");
        }
    } else {
        println!("Invalid profession or specialization entered.");
    }
}