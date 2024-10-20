use criterion::{criterion_group, criterion_main, Criterion};
use repro_tracing::{setup_tracing_one_subscriber, setup_tracing_two_subscribers, LocalConnection};

pub fn no_subscriber(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    c.bench_function("base_conn", move |b| {
        let connection = runtime.block_on(LocalConnection::connect());
        b.to_async(&runtime).iter(|| async {
            connection.work().await;
        });
    });
}

pub fn one_subscriber(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _outputs = setup_tracing_one_subscriber();
    c.bench_function("one_sub_conn", move |b| {
        let connection = runtime.block_on(LocalConnection::connect());
        b.to_async(&runtime).iter(|| async {
            connection.work().await;
        });
    });
}

pub fn two_subscriber(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _outputs = setup_tracing_two_subscribers();
    c.bench_function("two_sub_conn", move |b| {
        let connection = runtime.block_on(LocalConnection::connect());
        b.to_async(&runtime).iter(|| async {
            connection.work().await;
        });
    });
}

criterion_group!(benches, no_subscriber, one_subscriber, two_subscriber);
criterion_main!(benches);
