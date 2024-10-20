use criterion::{criterion_group, criterion_main, Criterion};
use repro_tracing::LocalConnection;

pub fn criterion_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    c.bench_function("work", move |b| {
        let connection = runtime.block_on(LocalConnection::connect());
        b.to_async(&runtime).iter(|| async {
            connection.work().await;
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
