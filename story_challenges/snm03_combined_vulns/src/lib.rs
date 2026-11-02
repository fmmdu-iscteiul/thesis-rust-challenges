pub struct EscalationRequest {
    pub ticket_id: u32,
    pub level: u8,
    pub team: String,
}

pub fn parse_escalation(input: &str) -> EscalationRequest {
    let mut ticket_id = None;
    let mut level = None;
    let mut team = None;

    for part in input.split(';') {
        let (key, value) = part.split_once('=').unwrap();
        match key.trim() {
            "ticket" => ticket_id = Some(value.trim().parse::<u32>().unwrap()),
            "level" => level = Some(value.trim().parse::<u8>().unwrap()),
            "team" => team = Some(value.trim().to_string()),
            _ => {}
        }
    }

    EscalationRequest {
        ticket_id: ticket_id.unwrap(),
        level: level.unwrap(),
        team: team.unwrap(),
    }
}

pub fn process_escalation(req: &EscalationRequest) -> Result<String, String> {
    if !(1..=3).contains(&req.level) {
        return Err(format!(
            "escalation level {} is out of range (allowed: 1-3)",
            req.level
        ));
    }

    Ok(format!(
        "Ticket {} escalated to level {} -> routed to team {}",
        req.ticket_id, req.level, req.team
    ))
}

pub fn handle_escalation(input: &str) -> Result<String, String> {
    let req = parse_escalation(input);
    process_escalation(&req)
}
