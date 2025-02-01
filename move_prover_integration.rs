use std::process::Command;
use std::fs;

pub fn run_move_prover(file_path: &str) -> String {
    // Prefix with an underscore to suppress the warning
    let _code = fs::read_to_string(file_path).expect("Unable to read file");

    // Placeholder logic to run the Move Prover
    // Execute the Move Prover command and capture the output
    let output = Command::new("move-prover")
        .arg(file_path)
        .output()
        .expect("Failed to execute Move Prover");

    String::from_utf8_lossy(&output.stdout).to_string()
} 