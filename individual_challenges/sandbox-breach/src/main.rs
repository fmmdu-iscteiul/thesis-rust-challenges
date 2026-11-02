// ============================================================================
//  START HERE: READ THE README.md IN THIS PROJECT BEFORE EDITING ANYTHING.
//  It explains the scenario, which file to fix, how to test your work, and
//  the platform commands you need. This source file is not the full brief.
// ============================================================================

use serious_game_wasmtime_sandbox::{read_from_guest, write_to_guest, Sandbox};

fn main() {
    let mut sandbox = Sandbox::new();
    sandbox.seed_guest(b"HELLO_GUEST_MEMORY");

    println!("Running sandbox access demo...");
    println!("Guest length: {}", sandbox.guest_len());

    match read_from_guest(&sandbox, 0, 5) {
        Ok(bytes) => println!(
            "[safe] Read succeeded: {:?}",
            String::from_utf8_lossy(&bytes)
        ),
        Err(e) => println!("[safe] Read blocked: {:?}", e),
    }

    match read_from_guest(&sandbox, 63, 2) {
        Ok(bytes) => println!(
            "[cross-boundary read attempt] Read succeeded: {:?}",
            String::from_utf8_lossy(&bytes)
        ),
        Err(e) => println!("[cross-boundary read attempt] Read blocked: {:?}", e),
    }

    match write_to_guest(&mut sandbox, 62, b"WXYZ") {
        Ok(()) => println!("[cross-boundary write attempt] Write succeeded"),
        Err(e) => println!("[cross-boundary write attempt] Write blocked: {:?}", e),
    }

    println!(
        "Host suffix after write attempt: {:?}",
        String::from_utf8_lossy(sandbox.host_suffix())
    );
    println!("If reads/writes outside guest memory succeed, the sandbox is vulnerable.");
}
