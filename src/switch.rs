//! Implementation for Skyline-rs
use crate::Error;
use skyline::nn;

pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    // Prevent overflow of u32
    for chunk in dest.chunks_mut(u32::max_value() as usize) {
        unsafe { nn::os::GenerateRandomBytes(chunk.as_mut_ptr() as _, chunk.len() as _); }
    }
    Ok(())
}