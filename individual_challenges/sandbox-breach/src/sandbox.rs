#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessError {
    OutOfBounds,
    InvalidLength,
}

/// Represents a simplified process memory layout:
/// - a protected host-only prefix
/// - a guest-accessible linear memory region
/// - a protected host-only suffix
///
/// The serious-game challenge is to ensure reads/writes stay strictly inside
/// the guest region.
pub struct Sandbox {
    host_memory: Vec<u8>,
    guest_base: usize,
    guest_len: usize,
}

impl Sandbox {
    pub fn new() -> Self {
        let prefix = b"HOST_SECRET:";
        let guest = vec![0u8; 64];
        let suffix = b":HOST_TRAILER";

        let mut host_memory = Vec::new();
        host_memory.extend_from_slice(prefix);
        let guest_base = host_memory.len();
        host_memory.extend_from_slice(&guest);
        let guest_len = guest.len();
        host_memory.extend_from_slice(suffix);

        Self {
            host_memory,
            guest_base,
            guest_len,
        }
    }

    pub fn guest_len(&self) -> usize {
        self.guest_len
    }

    pub fn host_prefix(&self) -> &[u8] {
        &self.host_memory[..self.guest_base]
    }

    pub fn host_suffix(&self) -> &[u8] {
        &self.host_memory[self.guest_base + self.guest_len..]
    }

    pub fn guest_slice(&self) -> &[u8] {
        &self.host_memory[self.guest_base..self.guest_base + self.guest_len]
    }

    pub fn guest_slice_mut(&mut self) -> &mut [u8] {
        let start = self.guest_base;
        let end = self.guest_base + self.guest_len;
        &mut self.host_memory[start..end]
    }

    pub fn seed_guest(&mut self, content: &[u8]) {
        let guest = self.guest_slice_mut();
        let n = content.len().min(guest.len());
        guest[..n].copy_from_slice(&content[..n]);
    }

    pub fn guest_base(&self) -> usize {
        self.guest_base
    }

    pub fn host_memory(&self) -> &[u8] {
        &self.host_memory
    }

    pub fn host_memory_mut(&mut self) -> &mut [u8] {
        &mut self.host_memory
    }
}
