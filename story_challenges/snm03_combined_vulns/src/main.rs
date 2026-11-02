// ============================================================================
//  START HERE: READ THE README.md IN THIS PROJECT BEFORE EDITING ANYTHING.
//  It explains the scenario, which file to fix, how to test your work, and
//  the platform commands you need. This source file is not the full brief.
// ============================================================================

use snm03_combined_vulns::handle_escalation;

fn main() {
    println!("=== Snemeis Security Challenge 3: Combined Vulnerabilities ===\n");

    // Case 1: Valid input — succeeds in both vulnerable and secure versions.
    let valid = "ticket=1042;level=2;team=infra";
    println!("[INPUT] {valid}");
    match handle_escalation(valid) {
        Ok(msg) => println!("[OK]  {msg}"),
        Err(e) => println!("[ERR] {e}"),
    }

    println!();

    // Case 2: Malformed segment — will panic in the vulnerable version.
    let malformed = "ticket=1042;BADFORMAT;team=infra";
    println!("[INPUT] {malformed}");
    println!("[!] The vulnerable version will panic here. Fix it to return a controlled error.\n");
    match handle_escalation(malformed) {
        Ok(msg) => println!("[OK]  {msg}"),
        Err(e) => println!("[ERR] {e}"),
    }

    println!();

    // Case 3: Unknown field — currently ignored by the vulnerable version.
    let unknown_field = "ticket=1042;level=2;team=infra;region=eu-west";
    println!("[INPUT] {unknown_field}");
    println!("[!] The secure version should reject unexpected fields instead of silently ignoring them.\n");
    match handle_escalation(unknown_field) {
        Ok(msg) => println!("[OK]  {msg}"),
        Err(e) => println!("[ERR] {e}"),
    }
}
