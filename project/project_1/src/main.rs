use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec!["Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams",];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fay Rouz"];

    let mut file = File::create("drinks.txt").expect("create failed");
   
    file.write_all(b"lager\n").expect("could not write");
    file.write_all(lager.join("\n").as_bytes()).expect("could not write");

    

    file.write_all(b"\nstout\n").expect("could not write");
    file.write_all(stout.join("\n").as_bytes()).expect("could not write");

    file.write_all(b"\nnon_alcoholic\n").expect("could not write");
    file.write_all(non_alcoholic.join("\n").as_bytes()).expect("could not create");

    println!("data has been written!");
}