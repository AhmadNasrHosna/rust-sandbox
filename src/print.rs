pub fn run() {
    // Print to console
    println!("Hello from print.rs file. :))))");

    // Basic formatting
    println!("My name is {}, and I'm {} years old", "Ahmad", "27");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}.",
        "Ahmad", "Egypt", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}.",
        name = "Ahmad",
        activity = "Chess"
    );

    // Placeholer traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debu trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10)
}
