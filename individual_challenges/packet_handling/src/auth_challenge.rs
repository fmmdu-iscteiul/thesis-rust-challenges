pub const MSG_USERAUTH_REQUEST: u8 = 50;
pub const MSG_USERAUTH_SUCCESS: u8 = 52;
const FRAME_HEADER_SIZE: u32 = 5;

#[derive(Debug, Clone)]
pub struct Session {
    pub authenticated: bool,
    pub max_packet_size: u32,
}

impl Session {
    pub fn new(max_packet_size: u32) -> Self {
        Self {
            authenticated: false,
            max_packet_size,
        }
    }
}

pub fn process_client_packet(
    session: &mut Session,
    msg_type: u8,
    payload_len: u32,
) -> Result<(), String> {
    let frame_size = (payload_len as u16).wrapping_add(FRAME_HEADER_SIZE as u16) as u32;

    if frame_size > session.max_packet_size {
        return Err(format!(
            "frame size {} exceeds max {}",
            frame_size, session.max_packet_size
        ));
    }

    if msg_type == MSG_USERAUTH_SUCCESS {
        session.authenticated = true;
    }

    Ok(())
}
