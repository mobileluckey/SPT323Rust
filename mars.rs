fn main() {
    // Declare variables
    let mut fuel: i32 = 100;  // A signed 32-bit integer representing the fuel level of the spacecraft
    let mut altitude: f64 = 10000.0;  // A floating point number representing the altitude of the spacecraft
    let mut landed: bool = false;  // A boolean representing whether the spacecraft has landed
    let mut name: String = String::from("Curiosity");  // A string representing the name of the spacecraft
    let mut command: char = 'a';  // A character representing the command given by the user

    println!("Welcome to the Mars landing simulation. You are in control of the spacecraft {}.", name);

    // Loop until landed is true
    while !landed {
        println!("Fuel: {}", fuel);
        println!("Altitude: {}", altitude);
        println!("Enter a command (l to land, f to use fuel, h to check status):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        command = input.trim().parse::<char>().unwrap();

        // Check command and execute appropriate action
        match command {
            'l' => {
                if fuel < 20 {
                    println!("Not enough fuel to land. Please use more fuel.");
                } else {
                    println!("Landing...");
                    landed = true;
                }
            },
            'f' => {
                println!("How much fuel do you want to use?");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let fuel_to_use: i32 = input.trim().parse().unwrap();
                if fuel_to_use > fuel {
                    println!("Not enough fuel. Please use less fuel.");
                } else {
                    fuel -= fuel_to_use;
                    altitude += fuel_to_use as f64 * 0.1;
                }
            },
            'h' => {
                println!("Fuel: {}", fuel);
                println!("Altitude: {}", altitude);
            },
            _ => {
                println!("Invalid command. Please enter a valid command.");
            }
        }
    }

    println!("Landed successfully on Mars!");
}