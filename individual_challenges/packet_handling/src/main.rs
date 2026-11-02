// ============================================================================
//  START HERE: READ THE README.md IN THIS PROJECT BEFORE EDITING ANYTHING.
//  It explains the scenario, which file to fix, how to test your work, and
//  the platform commands you need. This source file is not the full brief.
// ============================================================================

mod auth_challenge;
pub use auth_challenge::{
    process_client_packet, Session, MSG_USERAUTH_REQUEST, MSG_USERAUTH_SUCCESS,
};

#[allow(dead_code)]
fn main() {
    let mut session = Session::new(1024);
    match process_client_packet(&mut session, MSG_USERAUTH_REQUEST, 64) {
        Ok(()) => {
            println!(
                "[SETUP RESULT] normal auth request accepted, authenticated={}",
                session.authenticated
            );
        }
        Err(e) => {
            println!(
                "[SETUP RESULT] normal auth request rejected, authenticated={} ({})",
                session.authenticated, e
            );
        }
    }

    // Simulate an attacker sending USERAUTH_SUCCESS directly.
    match process_client_packet(&mut session, MSG_USERAUTH_SUCCESS, 32) {
        Ok(()) if session.authenticated => {
            println!("[ATTACK RESULT] attacker authenticated: true");
        }
        Ok(()) => {
            println!("[ATTACK RESULT] attacker authenticated: false");
        }
        Err(e) => {
            println!("[ATTACK RESULT] attacker authenticated: false ({})", e);
        }
    }
}
