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
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECODNS}s");

    // Example of shadowing variables
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of x in the inner scope is: {z}");
    }
    println!("The value of x is: {z}");

    // Example of type changes between shadowing variables and mut
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");

    // This will give compile errors
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
