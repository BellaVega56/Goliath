mod code_analysis;
mod error_analysis;
mod move_prover_integration;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: goliath <Move file path>");
        return;
    }

    let file_path = &args[1];

    // Run the Move Prover
    let prover_output = move_prover_integration::run_move_prover(file_path);

    // Parse the prover output
    let parsed_output = error_analysis::parse_prover_output(&prover_output);
    println!("Prover Output:\n{}", parsed_output);

    // Generate a spec template
    let code = std::fs::read_to_string(file_path).expect("Unable to read file");
    let spec_template = code_analysis::generate_spec_template(&code);
    println!("Spec Template:\n{}", spec_template);
} 