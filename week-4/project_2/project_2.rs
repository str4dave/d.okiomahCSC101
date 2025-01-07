use std::io;

fn main() {
    println!("Enter experience (true/false) and age:");

    let mut experience = String::new();
    let mut age = String::new();

    io::stdin().read_line(&mut experience).expect("Failed to read input");
    io::stdin().read_line(&mut age).expect("Failed to read input");

    let experience: bool = experience.trim().parse().unwrap();
    let age: u32 = age.trim().parse().unwrap();

    let incentive = if experience {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age >= 28 {
            1_300_000
        } else {
            100_000
        }
    } else {
        100_000
    };

    println!("The incentive is: N{}", incentive);
}