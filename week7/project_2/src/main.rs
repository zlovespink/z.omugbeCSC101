use std::io;

fn main() {
    println!("How many developers are you interviewing? :");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let no_of_developers:i32 = input1.trim().parse().expect("not a valid number");


    let mut developers = Vec::new();

for i in 1..= no_of_developers {
    println!("Developer's details please: {}", i);

    println!("What is your name? : {}", i);

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("not a valid input");
    let name_of_developer = input1.trim();

    println!("How many years of experience? {}", i);
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let years:i32 = input1.trim().parse().expect("not a valid integer");

    developers.push((name_of_developer.to_string(),years));

}


let mut highest_experience = 0;
let mut selected_developer = String::new();

for developer in developers {
   if developer.1 > highest_experience{
    highest_experience = developer.1;
    selected_developer = developer.0;
   }
}

println!("The developer whom has the highest programming experience is {} with {} years", selected_developer, highest_experience);
}