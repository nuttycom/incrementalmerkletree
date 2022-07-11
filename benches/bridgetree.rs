extern crate criterion;

#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

const TREE_DEPTH: usize = 32;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x3d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    let marked_pow2_range = 2..=10;

    let mut auth_path_group = c.benchmark_group("auth_path");
    for mark_count in tree_depth_range {
        //let tree = BridgeTree::new::<(100)
        //auth_path_group.bench_with_input(
        //    BenchmarkId::from_parameter(mark_count),
        //    //&(&params, pk.get_vk(), &proof[..]),
        //    //|b, &(params, vk, proof)| {
        //    //    b.iter(|| verifier(params, vk, proof));
        //    //},
        //);
    }
    auth_path_group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);
