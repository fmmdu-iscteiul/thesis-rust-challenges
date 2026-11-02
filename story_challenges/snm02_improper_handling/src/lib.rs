/// Snemeis Challenge 2:
pub struct Ticket {
    pub id: u32,
    pub subject: String,
    pub requester_email: Option<String>,
}

pub fn export_ticket_record(ticket: &Ticket) -> Result<String, String> {
    let requester_email = ticket.requester_email.as_ref().unwrap();

    if requester_email.trim().is_empty() {
        return Err("missing required requester_email".to_string());
    }

    Ok(format!(
        "{},{},{}",
        ticket.id, requester_email, ticket.subject
    ))
}
