#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::mem::{size_of, transmute};

use crate::{generate_seed, split_mix_64, SeedSource};

/// Implements `RomuTrio` with 512-bit width.
pub struct RngWide {
    x: __m512i,
    y: __m512i,
    z: __m512i,
    seed_source: SeedSource,
}

impl Default for RngWide {
    fn default() -> Self {
        unsafe {
            let mut rng = Self {
                x: _mm512_setzero_si512(),
                y: _mm512_setzero_si512(),
                z: _mm512_setzero_si512(),
                seed_source: SeedSource::Fixed,
            };
            rng.seed();
            rng
        }
    }
}

impl RngWide {
    /// Creates a new [`RngWide`] with a seed from the best available randomness source.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new [`RngWide`] from the given eight 64-bit seeds.
    ///
    /// The seeds should be from a high randomness source.
    pub const fn from_seed_with_64bit(seeds: [u64; 8]) -> Self {
        let lane0 = split_mix_64(seeds[0]);
        let lane1 = split_mix_64(seeds[1]);
        let lane2 = split_mix_64(seeds[2]);
        let lane3 = split_mix_64(seeds[3]);
        let lane4 = split_mix_64(seeds[4]);
        let lane5 = split_mix_64(seeds[5]);
        let lane6 = split_mix_64(seeds[6]);
        let lane7 = split_mix_64(seeds[7]);

        assert!(size_of::<__m512i>() == size_of::<[u64; 8]>());
        unsafe {
            Self {
                x: transmute::<[u64; 8], __m512i>([
                    lane0[0], lane1[0], lane2[0], lane3[0], lane4[0], lane5[0], lane6[0], lane7[0],
                ]),
                y: transmute::<[u64; 8], __m512i>([
                    lane0[1], lane1[1], lane2[1], lane3[1], lane4[1], lane5[1], lane6[1], lane7[1],
                ]),
                z: transmute::<[u64; 8], __m512i>([
                    lane0[2], lane1[2], lane2[2], lane3[2], lane4[2], lane5[2], lane6[2], lane7[2],
                ]),
                seed_source: SeedSource::User,
            }
        }
    }

    /// Creates a new [`RngWide`] from the given eight 192-bit seeds.
    ///
    /// The seeds should be from a high randomness source.
    ///
    /// If the seeds are of low quality, user should call [`RngWide::mix()`] to improve the quality of the
    /// first couple of random numbers.
    ///
    /// # Notice
    /// The variables must be seeded such that at least one bit of state is non-zero.
    pub const fn from_seed_with_192bit(seeds: [[u64; 3]; 8]) -> Self {
        assert!(size_of::<__m512i>() == size_of::<[u64; 8]>());
        unsafe {
            Self {
                x: transmute::<[u64; 8], __m512i>([
                    seeds[0][0],
                    seeds[1][0],
                    seeds[2][0],
                    seeds[3][0],
                    seeds[4][0],
                    seeds[5][0],
                    seeds[6][0],
                    seeds[7][0],
                ]),
                y: transmute::<[u64; 8], __m512i>([
                    seeds[0][1],
                    seeds[1][1],
                    seeds[2][1],
                    seeds[3][1],
                    seeds[4][1],
                    seeds[5][1],
                    seeds[6][1],
                    seeds[7][1],
                ]),
                z: transmute::<[u64; 8], __m512i>([
                    seeds[0][2],
                    seeds[1][2],
                    seeds[2][2],
                    seeds[3][2],
                    seeds[4][2],
                    seeds[5][2],
                    seeds[6][2],
                    seeds[7][2],
                ]),
                seed_source: SeedSource::User,
            }
        }
    }

    /// Mixes the states, which should improve the quality of the random numbers.
    ///
    /// Should be called when having (re-)seeded the generator with a fixed value of low randomness.
    pub fn mix(&mut self) {
        (0..10).for_each(|_| {
            self.next();
        });
    }

    /// Re-seeds the [`RngWide`] from the best available randomness source.
    pub fn seed(&mut self) {
        let mut memory_address = self as *const _ as u64;
        let mut seed_source = SeedSource::Fixed;

        let mut x = [0u64; 8];
        let mut y = [0u64; 8];
        let mut z = [0u64; 8];

        x.iter_mut()
            .zip(y.iter_mut())
            .zip(z.iter_mut())
            .for_each(|((x, y), z)| {
                let (lane, source) = generate_seed(memory_address);

                *x = lane[0];
                *y = lane[1];
                *z = lane[2];

                seed_source = source;
                memory_address = memory_address.wrapping_add(1);
            });

        assert!(size_of::<__m512i>() == size_of::<[u64; 8]>());
        unsafe {
            self.x = transmute::<[u64; 8], __m512i>(x);
            self.y = transmute::<[u64; 8], __m512i>(y);
            self.z = transmute::<[u64; 8], __m512i>(z);
        }

        self.seed_source = seed_source;
    }

    /// The actual wide `RomuTrio` algorithm.
    ///
    /// Great for general purpose work, including huge jobs.
    /// Est. capacity = 2^75 bytes. State size = 192 bits.
    ///
    /// Copyright 2020 Mark A. Overton
    /// Licensed under Apache-2.0.
    #[inline(always)]
    fn next(&mut self) -> [u64; 8] {
        let xp = self.x;
        let yp = self.y;
        let zp = self.z;

        unsafe {
            // 0xD3833E804F4C574B
            let high_mul = _mm512_set1_epi64(3548593792);
            let low_mul = _mm512_set1_epi64(1330403147);

            let zp_high = _mm512_mul_epu32(zp, high_mul);
            let zp_high_shift = _mm512_srli_epi64::<32>(zp);
            let zp_mid = _mm512_mul_epu32(zp_high_shift, low_mul);
            let zp_mid_high = _mm512_add_epi64(zp_mid, zp_high);
            let zp_mid_high = _mm512_slli_epi64::<32>(zp_mid_high);
            let zp_low = _mm512_mul_epu32(low_mul, zp);
            self.x = _mm512_add_epi64(zp_low, zp_mid_high);

            let ty = _mm512_sub_epi64(yp, xp);
            let srl = _mm512_srli_epi64::<52>(ty);
            let sll = _mm512_slli_epi64::<12>(ty);
            self.y = _mm512_or_si512(sll, srl);

            let tz = _mm512_sub_epi64(zp, yp);
            let srl = _mm512_srli_epi64::<20>(tz);
            let sll = _mm512_slli_epi64::<44>(tz);
            self.z = _mm512_or_si512(sll, srl);

            assert!(size_of::<__m512i>() == size_of::<[u64; 8]>());
            transmute(xp)
        }
    }

    /// Generates eight random u64 values.
    #[inline(always)]
    pub fn u64x8(&mut self) -> [u64; 8] {
        self.next()
    }

    /// Fills a mutable `[u8]` slice with random values.
    pub fn fill_bytes(&mut self, slice: &mut [u8]) {
        const CHUNK_SIZE: usize = u64::BITS as usize;

        let mut chunks = slice.chunks_exact_mut(CHUNK_SIZE);
        for chunk in &mut chunks {
            let data: [u8; CHUNK_SIZE] = unsafe { transmute(self.next()) };
            chunk.copy_from_slice(&data);
        }

        assert!(size_of::<[u8; CHUNK_SIZE]>() == size_of::<[u64; 8]>());
        let data: [u8; CHUNK_SIZE] = unsafe { transmute(self.next()) };
        chunks
            .into_remainder()
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = data[i]);
    }
}
