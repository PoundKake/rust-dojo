fn main() {
    // This will error
    // let x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");

    // This will not
    let mut y = 5;
    println!("The value of y is {y}");
    y = 6;
    println!("The value of y is {y}");

    // Example of a cont declaration
    const HOURS: u32 = 60;
    const SECONDS: u32 = 60;
    const THREE_HOURS_IN_SECODNS: u32 = HOURS * SECONDS * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECODNS}s")
}
