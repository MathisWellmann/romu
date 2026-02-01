// AVX-512 (highest priority)
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx512f"
))]
mod avx512;

// AVX2 (when no AVX-512)
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx512f"),
    target_feature = "avx2"
))]
mod avx2;

// SSE2 (when no AVX-512 and no AVX2)
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx512f"),
    not(target_feature = "avx2"),
    target_feature = "sse2"
))]
mod sse2;

// Fallback (no SIMD)
#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(
        target_feature = "sse2",
        target_feature = "avx2",
        target_feature = "avx512f"
    ),
)))]
mod fallback;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx512f"),
    target_feature = "avx2"
))]
pub use avx2::RngWide;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx512f"
))]
pub use avx512::RngWide;
#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    any(
        target_feature = "sse2",
        target_feature = "avx2",
        target_feature = "avx512f"
    ),
)))]
pub use fallback::RngWide;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    not(target_feature = "avx512f"),
    not(target_feature = "avx2"),
    target_feature = "sse2"
))]
pub use sse2::RngWide;
