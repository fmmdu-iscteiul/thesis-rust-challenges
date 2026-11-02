// ============================================================================
//  START HERE: READ THE README.md IN THIS PROJECT BEFORE EDITING ANYTHING.
//  It explains the scenario, which file to fix, how to test your work, and
//  the platform commands you need. This source file is not the full brief.
// ============================================================================

use snm01_input_validation::parse_employee_id;

fn simulate_login_attempt(raw_input: &str) {
    println!("[LOGIN ATTEMPT] Raw employee ID input: {:?}", raw_input);

    let result = std::panic::catch_unwind(|| parse_employee_id(raw_input));
    match result {
        Ok(Ok(employee_id)) => {
            println!("[RESULT] Parsed employee ID: {}", employee_id);
            println!("[RESULT] OK.\n");
        }
        Ok(Err(error)) => {
            println!("[RESULT] Rejected malformed employee ID: {}", error);
            println!("[RESULT] FAIL SAFELY.\n");
        }
        Err(_) => {
            println!(
                "[RESULT] Login helper crashed while parsing untrusted input (panic detected)."
            );
            println!("[RESULT] FAIL.\n");
        }
    }
}

fn main() {
    // Keep challenge output focused on scenario messaging instead of Rust panic traces.
    std::panic::set_hook(Box::new(|_| {}));

    println!("=== Snemeis Security Challenge 1: Input Validation ===");
    println!("Scenario: employee login helper processes raw employee ID strings.");
    println!("Goal: handle malformed input safely without crashing.\n");

    simulate_login_attempt("1042");
    simulate_login_attempt(" 1042 ");
    simulate_login_attempt("abc");
    simulate_login_attempt("");
    simulate_login_attempt("12x7");
}
