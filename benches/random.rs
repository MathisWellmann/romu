use std::hint::black_box;

use criterion::{criterion_main, BenchmarkId, Criterion, Throughput};
use romu::{Rng, RngWide};

pub fn scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar");

    let rng = Rng::new();

    group.bench_function(BenchmarkId::new("u8", ""), |b| {
        b.iter(|| {
            black_box(rng.u8());
        })
    });

    group.bench_function(BenchmarkId::new("u16", ""), |b| {
        b.iter(|| {
            black_box(rng.u16());
        })
    });

    group.bench_function(BenchmarkId::new("u32", ""), |b| {
        b.iter(|| {
            black_box(rng.u32());
        })
    });

    group.bench_function(BenchmarkId::new("u64", ""), |b| {
        b.iter(|| {
            black_box(rng.u64());
        })
    });

    group.bench_function(BenchmarkId::new("usize", ""), |b| {
        b.iter(|| {
            black_box(rng.usize());
        })
    });

    group.bench_function(BenchmarkId::new("i8", ""), |b| {
        b.iter(|| {
            black_box(rng.i8());
        })
    });

    group.bench_function(BenchmarkId::new("i16", ""), |b| {
        b.iter(|| {
            black_box(rng.i16());
        })
    });

    group.bench_function(BenchmarkId::new("i32", ""), |b| {
        b.iter(|| {
            black_box(rng.i32());
        })
    });

    group.bench_function(BenchmarkId::new("i64", ""), |b| {
        b.iter(|| {
            black_box(rng.i64());
        })
    });

    group.bench_function(BenchmarkId::new("isize", ""), |b| {
        b.iter(|| {
            black_box(rng.isize());
        })
    });

    group.bench_function(BenchmarkId::new("f32", ""), |b| {
        b.iter(|| {
            black_box(rng.f32());
        })
    });

    group.bench_function(BenchmarkId::new("f64", ""), |b| {
        b.iter(|| {
            black_box(rng.f64());
        })
    });

    group.bench_function(BenchmarkId::new("bool", ""), |b| {
        b.iter(|| {
            black_box(rng.bool());
        })
    });

    group.finish();
}

pub fn mod_u(c: &mut Criterion) {
    let mut group = c.benchmark_group("mod");

    let rng = Rng::new();

    group.bench_function(BenchmarkId::new("u8", ""), |b| {
        b.iter(|| {
            black_box(rng.mod_u8(u8::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("u16", ""), |b| {
        b.iter(|| {
            black_box(rng.mod_u16(u16::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("u32", ""), |b| {
        b.iter(|| {
            black_box(rng.mod_u32(u32::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("u64", ""), |b| {
        b.iter(|| {
            black_box(rng.mod_u64(u64::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("usize", ""), |b| {
        b.iter(|| {
            black_box(rng.mod_usize(usize::MAX - 1));
        })
    });

    group.finish();
}

pub fn range(c: &mut Criterion) {
    let mut group = c.benchmark_group("range");

    let rng = Rng::new();

    group.bench_function(BenchmarkId::new("u8", ""), |b| {
        b.iter(|| {
            black_box(rng.range_u8(..u8::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("u16", ""), |b| {
        b.iter(|| {
            black_box(rng.range_u16(..u16::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("u32", ""), |b| {
        b.iter(|| {
            black_box(rng.range_u32(..u32::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("u64", ""), |b| {
        b.iter(|| {
            black_box(rng.range_u64(..u64::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("usize", ""), |b| {
        b.iter(|| {
            black_box(rng.range_usize(..usize::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("i8", ""), |b| {
        b.iter(|| {
            black_box(rng.range_i8((i8::MIN + 1)..i8::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("i16", ""), |b| {
        b.iter(|| {
            black_box(rng.range_i16((i16::MIN + 1)..i16::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("i32", ""), |b| {
        b.iter(|| {
            black_box(rng.range_i32((i32::MIN + 1)..i32::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("i64", ""), |b| {
        b.iter(|| {
            black_box(rng.range_i64((i64::MIN + 1)..i64::MAX - 1));
        })
    });

    group.bench_function(BenchmarkId::new("isize", ""), |b| {
        b.iter(|| {
            black_box(rng.range_isize((isize::MIN + 1)..isize::MAX - 1));
        })
    });

    group.finish();
}

pub fn bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytes");

    for size in [128, 1024, 1024 * 1024] {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("Rng", size), &size, |b, &size| {
            b.iter_with_setup(
                || (Rng::new(), vec![0u8; size]),
                |(rng, mut buffer)| {
                    rng.fill_bytes(&mut buffer);
                    black_box(buffer);
                },
            );
        });

        group.bench_with_input(BenchmarkId::new("RngWide", size), &size, |b, &size| {
            b.iter_with_setup(
                || (RngWide::new(), vec![0u8; size]),
                |(mut rng, mut buffer)| {
                    rng.fill_bytes(&mut buffer);
                    black_box(buffer);
                },
            );
        });
    }

    group.finish();
}

#[cfg(feature = "tls")]
pub fn tls(c: &mut Criterion) {
    let mut group = c.benchmark_group("tls");

    let rng = Rng::new();
    romu::seed();

    group.bench_function(BenchmarkId::new("u64", "instanced"), |b| {
        b.iter(|| {
            black_box(rng.u32());
        })
    });

    group.bench_function(BenchmarkId::new("u64", "thread local"), |b| {
        b.iter(|| {
            black_box(romu::u32());
        })
    });

    let rng = Rng::new();
    romu::seed();

    let count = 1024 * 1024;
    group.throughput(Throughput::Bytes(count));

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("bytes", "instanced"), |b| {
        b.iter(|| {
            rng.fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    let mut buffer = vec![0u8; count as usize];
    group.bench_function(BenchmarkId::new("bytes", "thread local"), |b| {
        b.iter(|| {
            romu::fill_bytes(&mut buffer);
        })
    });
    black_box(buffer);

    group.finish();
}

#[cfg(feature = "unstable_simd")]
pub fn unstable_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("unstable_simd");

    for n in [128, 1024, 1024 * 1024] {
        const N_ELEMS_PER_NEXT: u64 = 8; // Output of `next` is `u64x8`
        group.throughput(Throughput::ElementsAndBytes {
            elements: n * N_ELEMS_PER_NEXT,
            bytes: n * size_of::<u64>() as u64 * N_ELEMS_PER_NEXT,
        });
        group.bench_with_input(BenchmarkId::new("u64x8", n), &n, |b, &_s| {
            b.iter_with_setup(
                || RngWide::new(),
                |mut rng| {
                    for _ in 0..n {
                        let _ = black_box(rng.u64x8());
                    }
                    black_box(rng);
                },
            );
        });
    }

    let size = 1024 * 1024; // 1 MiB
    let mut buffer = vec![0u8; size];
    let mut rng = RngWide::new();

    group.throughput(Throughput::Bytes(size as u64));
    group.bench_with_input(BenchmarkId::new("fill_bytes", size), &size, |b, &_s| {
        b.iter(|| rng.fill_bytes(&mut buffer));
    });
    black_box(buffer);
    group.finish();
}

pub fn benches() {
    let mut criterion: Criterion<_> = Criterion::default().configure_from_args();
    scalar(&mut criterion);
    mod_u(&mut criterion);
    range(&mut criterion);
    #[cfg(feature = "tls")]
    tls(&mut criterion);
    bytes(&mut criterion);
    #[cfg(feature = "unstable_simd")]
    unstable_simd(&mut criterion);
}

criterion_main!(benches);
