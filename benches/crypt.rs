use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;

use almetica::crypt::CryptSession;

fn setup() -> CryptSession {
    let c1 = vec![0x11; 128];
    let c2 = vec![0x22; 128];
    let s1 = vec![0xfe; 128];
    let s2 = vec![0xff; 128];

    CryptSession::new([c1, c2], [s1, s2])
}

// Tests the crypto performance. Data in the TERA network procotol is at least 4 bytes in size (u16 length, u16 opcode).
fn crypt_benchmark(c: &mut Criterion<CyclesPerByte>) {
    let mut session = setup();

    let mut group = c.benchmark_group("crypt_benchmark");
    for data_size in [
        4u64, 6u64, 8u64, 12u64, 16u64, 32u64, 64u64, 128u64, 256u64, 512u64,
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(*data_size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(data_size),
            data_size,
            |b, &data_size| {
                let mut data = vec![0; data_size as usize];
                b.iter(|| session.crypt_client_data(data.as_mut_slice()));
            },
        );
    }
    group.finish();
}

criterion_group!(
    name = crypto_bench;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = crypt_benchmark
);
criterion_main!(crypto_bench);
