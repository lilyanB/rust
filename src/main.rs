use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Extract numbers from the file
    let numbers: Vec<u32> = contents
        .split_whitespace()
        .filter_map(|x| x.parse().ok()) // Ignore les erreurs de conversion
        .collect();

    // Create two columns from the numbers
    let first_column: Vec<u32> = numbers.iter().step_by(2).copied().collect(); // Colonne impaire (1ère colonne)
    let second_column: Vec<u32> = numbers.iter().skip(1).step_by(2).copied().collect(); // Colonne paire (2ème colonne)

    // Order the columns
    let mut sorted_first_column = first_column.clone();
    let mut sorted_second_column = second_column.clone();
    sorted_first_column.sort();
    sorted_second_column.sort();

    // Calculate the difference between the two columns
    let mut result = 0;
    for (i, value) in sorted_first_column.iter().enumerate() {
        for (j, value2) in sorted_second_column.iter().enumerate() {
            if i == j {
                let diff = (*value as isize - *value2 as isize).abs();
                result += diff;
            }
        }
    }

    println!("Total result: {:?}", result);

    Ok(())
}
