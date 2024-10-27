fn main(){
    let principal: f64 = 520_000_000.0; // Principal amount (Naira)
    let rate: f64 = 0.10; // Interest rate (10% per annum)
    let years: u32 = 5; // Number of years

    // Calculate the amount after the specified years
    let amount = principal * (1.0 + rate).powi(years as i32);

    // Calculate the compound interest
    let compound_interest = amount - principal;

    // Print the results
    println!("Principal: N{:.2}", principal);
    println!("Amount after {} years: N{:.2}", years, amount);
    println!("Compound Interest after {} years: N{:.2}", years, compound_interest);
}

