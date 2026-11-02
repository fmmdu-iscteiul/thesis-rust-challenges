// ============================================================================
//  START HERE: READ THE README.md IN THIS PROJECT BEFORE EDITING ANYTHING.
//  It explains the scenario, which file to fix, how to test your work, and
//  the platform commands you need. This source file is not the full brief.
// ============================================================================

use snm02_improper_handling::{export_ticket_record, Ticket};

fn main() {
    println!("=== Snemeis Security Challenge 2: Improper Handling ===");

    let valid_ticket = Ticket {
        id: 1001,
        subject: "Cannot access VPN".to_string(),
        requester_email: Some("analyst@snemeis.example".to_string()),
    };

    match export_ticket_record(&valid_ticket) {
        Ok(row) => println!("[OK] Valid export row: {row}"),
        Err(err) => println!("[WARN] Valid record unexpectedly failed: {err}"),
    }

    let missing_required_field_ticket = Ticket {
        id: 1002,
        subject: "Billing question".to_string(),
        requester_email: None,
    };

    let malformed_requester_email_ticket = Ticket {
        id: 1003,
        subject: "Security review follow-up".to_string(),
        requester_email: Some("analyst-at-snemeis.example".to_string()),
    };

    // Capture panic behavior to display challenge status clearly.
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let probe = std::panic::catch_unwind(|| export_ticket_record(&missing_required_field_ticket));
    let malformed_probe = export_ticket_record(&malformed_requester_email_ticket);
    std::panic::set_hook(previous_hook);

    println!();
    println!("--- Challenge Status Probe ---");
    match probe {
        Err(_) => {
            println!("Status: VULNERABLE");
            println!("Reason: missing required field still triggers a panic.");
            println!("Goal: return a safe Result::Err instead of panicking.");
        }
        Ok(Err(err)) => {
            println!("Status: SOLVED");
            println!("Reason: missing required field is handled safely.");
            println!("Observed safe error: {err}");
        }
        Ok(Ok(row)) => {
            println!("Status: NEEDS REVIEW");
            println!("Reason: missing required field unexpectedly exported: {row}");
            println!("Goal: reject missing required field safely.");
        }
    }

    println!();
    match malformed_probe {
        Ok(row) => {
            println!("Malformed email probe: VULNERABLE");
            println!("Reason: malformed requester_email was exported: {row}");
            println!("Goal: reject malformed requester_email safely.");
        }
        Err(err) => {
            println!("Malformed email probe: SOLVED");
            println!("Reason: malformed requester_email is handled safely.");
            println!("Observed safe error: {err}");
        }
    }
}
