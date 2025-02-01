use regex::Regex;

pub fn generate_spec_template(code: &str) -> String {
    let mut spec_template = String::new();

    // Add module-level specifications at the beginning
    spec_template.push_str("\x1b[1;34m// Module Specifications\x1b[0m\n");
    spec_template.push_str("\x1b[1;30m// ----------------------------------------\x1b[0m\n");
    spec_template.push_str("spec module {\n");
    spec_template.push_str("    invariant total_supply >= 0;\n");
    spec_template.push_str("    invariant forall addr: address: exists<Capabilities>(addr) ==> addr == resource_account;\n");
    spec_template.push_str("}\n");
    spec_template.push_str("\x1b[1;30m// ----------------------------------------\x1b[0m\n\n");

    let function_regex = Regex::new(r"(?m)^\s*(public(\s*\(\s*(friend|package)\s*\))?\s+|entry\s+|native\s+)*fun\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*(<[^>]*>)?\s*\(([^)]*)\)").unwrap();

    for cap in function_regex.captures_iter(code) {
        let function_name = &cap[4];
        let params_raw = &cap[6];
        let line_number = cap.get(0).map_or(0, |m| m.start());

        let params = process_params(params_raw, function_name);

        spec_template.push_str(&format!(
            "\x1b[1;34m// Function: {}\x1b[0m\n", function_name
        ));
        spec_template.push_str(&format!("\x1b[1;30m// Line: {}\x1b[0m\n", line_number));
        spec_template.push_str("\x1b[1;30m// ----------------------------------------\x1b[0m\n");
        
        spec_template.push_str(&format!("spec {}({}) {{\n", function_name, params));
        generate_function_specs(&mut spec_template, function_name, &params);
        spec_template.push_str("}\n");
        spec_template.push_str("\x1b[1;30m// ----------------------------------------\x1b[0m\n\n");
    }

    if spec_template.is_empty() {
        spec_template.push_str("// No functions found to generate specs.\n");
    }

    spec_template
}

fn process_params(params_raw: &str, function_name: &str) -> String {
    params_raw.split(',')
        .filter(|p| !p.trim().is_empty())
        .map(|param| {
            let parts: Vec<&str> = param.trim().split(':').collect();
            let type_name = parts.last().unwrap_or(&"").trim();

            let name = if parts.len() >= 2 {
                parts[0].trim().to_string()
            } else {
                match type_name {
                    t if t.contains("0x1::coin::Coin<MOD>") => "coin",
                    t if t.contains("0x1::fungible_asset::FungibleAsset") => "fa",
                    t if t.contains("signer") => "signer",
                    t if t.contains("vector<address>") => "addresses",
                    t if t.contains("u64") => match function_name {
                        "mint" | "mint_fa" | "reconcile" => "amount",
                        "burn_from" => "burn_amount",
                        _ => "value"
                    },
                    t if t.contains("address") => "addr",
                    _ => "value"
                }.to_string()
            };

            format!("{}: {}", name, type_name)
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn generate_function_specs(spec_template: &mut String, function_name: &str, _params: &str) {
    match function_name {
        "burn" => {
            spec_template.push_str("    \x1b[1;36m// Module must be initialized\x1b[0m\n");
            spec_template.push_str("    requires initialized();\n");
            spec_template.push_str("    \x1b[1;36m// Burn capability must exist\x1b[0m\n");
            spec_template.push_str("    requires exists<Capabilities>(resource_account_address());\n");
            spec_template.push_str("    \x1b[1;36m// Supply changes\x1b[0m\n");
            spec_template.push_str("    ensures coin.value > 0 ==> total_supply == old(total_supply) - coin.value;\n");
            spec_template.push_str("    ensures coin.value == 0 ==> total_supply == old(total_supply);\n");
        },
        "burn_from" => {
            spec_template.push_str("    \x1b[1;36m// Authorization required\x1b[0m\n");
            spec_template.push_str("    requires is_authorized(signer);\n");
            spec_template.push_str("    \x1b[1;36m// Burn capability must exist\x1b[0m\n");
            spec_template.push_str("    requires exists<Capabilities>(resource_account_address());\n");
            spec_template.push_str("    \x1b[1;36m// Amount checks\x1b[0m\n");
            spec_template.push_str("    requires burn_amount > 0;\n");
            spec_template.push_str("    requires coin_store[addr].value >= burn_amount;\n");
            spec_template.push_str("    ensures total_supply == old(total_supply) - burn_amount;\n");
        },
        "initialize" => {
            spec_template.push_str("    \x1b[1;36m// Can only initialize once\x1b[0m\n");
            spec_template.push_str("    requires !initialized();\n");
            spec_template.push_str("    \x1b[1;36m// Post-conditions\x1b[0m\n");
            spec_template.push_str("    ensures initialized();\n");
            spec_template.push_str("    ensures exists<Capabilities>(resource_account_address());\n");
            spec_template.push_str("    ensures total_supply == 0;\n");
        },
        "mint" | "mint_fa" => {
            spec_template.push_str("    \x1b[1;36m// Module must be initialized\x1b[0m\n");
            spec_template.push_str("    requires initialized();\n");
            spec_template.push_str("    \x1b[1;36m// Mint capability must exist\x1b[0m\n");
            spec_template.push_str("    requires exists<Capabilities>(resource_account_address());\n");
            spec_template.push_str("    \x1b[1;36m// Amount checks\x1b[0m\n");
            spec_template.push_str("    requires amount > 0;\n");
            spec_template.push_str("    ensures result.value == amount;\n");
            spec_template.push_str("    ensures total_supply == old(total_supply) + amount;\n");
        },
        name if name.contains("freeze") => {
            spec_template.push_str("    \x1b[1;36m// Authorization required\x1b[0m\n");
            spec_template.push_str("    requires is_authorized(signer);\n");
            spec_template.push_str("    \x1b[1;36m// Freeze capability must exist\x1b[0m\n");
            spec_template.push_str("    requires exists<Capabilities>(resource_account_address());\n");
            spec_template.push_str("    \x1b[1;36m// Post-conditions\x1b[0m\n");
            spec_template.push_str("    ensures forall addr in addresses: is_frozen(addr);\n");
        },
        "initialized" => {
            spec_template.push_str("    \x1b[1;36m// Check capabilities existence\x1b[0m\n");
            spec_template.push_str("    ensures result == exists<Capabilities>(resource_account_address());\n");
        },
        "metadata" => {
            spec_template.push_str("    \x1b[1;36m// Module must be initialized\x1b[0m\n");
            spec_template.push_str("    requires initialized();\n");
            spec_template.push_str("    \x1b[1;36m// Metadata must exist\x1b[0m\n");
            spec_template.push_str("    ensures exists<0x1::fungible_asset::Metadata>(result);\n");
        },
        "reconcile" => {
            spec_template.push_str("    \x1b[1;36m// Authorization required\x1b[0m\n");
            spec_template.push_str("    requires is_authorized(signer);\n");
            spec_template.push_str("    \x1b[1;36m// Module must be initialized\x1b[0m\n");
            spec_template.push_str("    requires initialized();\n");
            spec_template.push_str("    \x1b[1;36m// Balance changes\x1b[0m\n");
            spec_template.push_str("    ensures balance(signer_address()) == old(balance(signer_address())) + amount;\n");
        },
        _ => {
            spec_template.push_str("    \x1b[1;36m// Module must be initialized\x1b[0m\n");
            spec_template.push_str("    requires initialized();\n");
        }
    }
} 