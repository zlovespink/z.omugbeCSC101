//define the brand and price of the laptops
struct laptop {
    brand:String, price:u32
}

//logic to calculate the total cost of laptops
fn main() {
    let hp = laptop{
        brand:String::from("hp"),
        price:650_000,
};

let ibm = laptop {
    brand:String::from("ibm"),
    price:755_000,
};

let toshiba = laptop{
    brand:String::from("toshiba"),
    price:550_000,
};

let dell = laptop{
    brand:String::from("dell"),
    price:850_000,
};

let quantity:u32 = 3;

let total_hp = hp.price * quantity;
let total_ibm = ibm.price * quantity;
let total_toshiba = toshiba.price * quantity;
let total_dell =dell.price * quantity;

let total_cost = total_hp + total_ibm + total_toshiba + total_dell;


println!(" the total cost of  {} hp laptops is N{}", quantity, total_hp);
println!("the total cost of {} ibm laptops is N{}" , quantity, total_ibm);
println!("the total cost of {} toshiba laptops is N{}", quantity, total_toshiba);
println!("the total cost of {} dell laptops is N{}", quantity, total_dell);

println!("Therefore, your total cost for all four laptops is N{}. Thank you for shopping at Alaba International Market!", total_cost);

}