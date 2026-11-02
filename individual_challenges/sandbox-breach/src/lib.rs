pub mod sandbox;

pub use sandbox::{AccessError, Sandbox};

pub fn read_from_guest(
    sandbox: &Sandbox,
    offset32: u32,
    len: usize,
) -> Result<Vec<u8>, AccessError> {
    if len == 0 {
        return Err(AccessError::InvalidLength);
    }

    if len > sandbox.guest_len() {
        return Err(AccessError::OutOfBounds);
    }

    let computed = sandbox.guest_base().wrapping_add(offset32 as usize);

    unsafe {
        let ptr = sandbox.host_memory().as_ptr().add(computed);
        let slice = std::slice::from_raw_parts(ptr, len);
        Ok(slice.to_vec())
    }
}

pub fn write_to_guest(
    sandbox: &mut Sandbox,
    offset32: u32,
    data: &[u8],
) -> Result<(), AccessError> {
    if data.is_empty() {
        return Err(AccessError::InvalidLength);
    }

    if data.len() > sandbox.guest_len() {
        return Err(AccessError::OutOfBounds);
    }

    let computed = sandbox.guest_base().wrapping_add(offset32 as usize);

    unsafe {
        let dst = sandbox.host_memory_mut().as_mut_ptr().add(computed);
        std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
    }

    Ok(())
}
