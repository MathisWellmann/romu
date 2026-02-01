//! Implements traits from the `rand` crate to provide compatibility with the broader ecosystem.

use core::convert::Infallible;

use crate::Rng;

impl rand_core::TryRng for Rng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.u64())
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.fill_bytes(dst);
        Ok(())
    }
}
