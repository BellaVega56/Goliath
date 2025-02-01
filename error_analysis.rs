pub fn parse_prover_output(output: &str) -> String {
    let mut parsed_output = String::new();

    if output.contains("error") {
        parsed_output.push_str("Errors found in the Move Prover output:\n");
        for line in output.lines() {
            if line.contains("error") {
                parsed_output.push_str(&format!("{}\n", line));
            }
        }
    } else {
        parsed_output.push_str("No errors found in the Move Prover output.\n");
    }

    parsed_output
} 