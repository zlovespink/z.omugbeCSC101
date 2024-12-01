use std::io;

fn main() {
    // Display options to the user
    println!("These are the shapes you are to choose from to calculate the area or volume: ");
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");

    // Ask the user for their shape choice
    println!("What shape would you like to calculate? (Enter the name of the shape): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice_of_shape = input.trim().to_lowercase();

    // Call the appropriate function based on user choice
    match choice_of_shape.as_str() {
        "trapezium" => calculate_trapezium_area(),
        "rhombus" => calculate_rhombus_area(),
        "parallelogram" => calculate_parallelogram_area(),
        "cube" => calculate_cube_area(),
        "cylinder" => calculate_cylinder_volume(),
        _ => println!("Invalid shape selected."),
    }
}

fn calculate_trapezium_area() {
    // Get the height and base lengths from the user
    println!("Enter the height of the trapezium: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let height: f32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the first base of the trapezium: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let base1: f32 = input2.trim().parse().expect("Please enter a valid number");

    println!("Enter the second base of the trapezium: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let base2: f32 = input3.trim().parse().expect("Please enter a valid number");

    // Calculate the area of the trapezium
    let area_of_trap: f32 = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {}", area_of_trap);
}

fn calculate_rhombus_area() {
    println!("Enter the first diagonal of the rhombus: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let diagonal1: f32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the second diagonal of the rhombus: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let diagonal2: f32 = input2.trim().parse().expect("Please enter a valid number");

    // Calculate the area of the rhombus
    let area_of_rhombus = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {}", area_of_rhombus);
}

fn calculate_parallelogram_area() {
    println!("Enter the base of the parallelogram: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let base: f32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the altitude of the parallelogram: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let altitude: f32 = input2.trim().parse().expect("Please enter a valid number");

    // Calculate the area of the parallelogram
    let area_of_parallelogram = base * altitude;
    println!("The area of the parallelogram is: {}", area_of_parallelogram);
}

fn calculate_cube_area() {
    println!("Enter the side length of the cube: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let side: f32 = input.trim().parse().expect("Please enter a valid number");

    // Calculate the surface area of the cube
    let area_of_cube = 6.0 * side * side;
    println!("The surface area of the cube is: {}", area_of_cube);
}

fn calculate_cylinder_volume() {
    println!("Enter the radius of the cylinder: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let radius: f32 = input1.trim().parse().expect("Please enter a valid number");

    println!("Enter the height of the cylinder: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let height: f32 = input2.trim().parse().expect("Please enter a valid number");

    // Calculate the volume of the cylinder
    let volume_of_cylinder = std::f32::consts::PI * radius * radius * height;
    println!("The volume of the cylinder is: {}", volume_of_cylinder);
}
