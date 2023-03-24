use std::time::Instant;

use criterion::async_executor::AsyncStdExecutor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hypercore::{Builder, Hypercore, HypercoreError, Storage};
use random_access_memory::RandomAccessMemory;

async fn create_hypercore(
    page_size: usize,
) -> Result<Hypercore<RandomAccessMemory>, HypercoreError> {
    let storage = Storage::open(
        |_| Box::pin(async move { Ok(RandomAccessMemory::new(page_size)) }),
        false,
    )
    .await?;
    Builder::new(storage).build_new().await
}

fn create(c: &mut Criterion) {
    c.bench_function("create", move |b| {
        b.to_async(AsyncStdExecutor).iter(|| create_hypercore(1024));
    });
}

fn write(c: &mut Criterion) {
    c.bench_function("write", move |b| {
        b.to_async(AsyncStdExecutor)
            .iter_custom(|iters| async move {
                let mut hypercore = create_hypercore(1024).await.unwrap();
                let data = Vec::from("hello");
                let start = Instant::now();
                for _ in 0..iters {
                    black_box(hypercore.append(&data).await.unwrap());
                }
                start.elapsed()
            });
    });
}

fn read(c: &mut Criterion) {
    c.bench_function("read", move |b| {
        b.to_async(AsyncStdExecutor)
            .iter_custom(|iters| async move {
                let mut hypercore = create_hypercore(1024).await.unwrap();
                let data = Vec::from("hello");
                for _ in 0..iters {
                    hypercore.append(&data).await.unwrap();
                }
                let start = Instant::now();
                for i in 0..iters {
                    black_box(hypercore.get(i).await.unwrap());
                }
                start.elapsed()
            });
    });
}

criterion_group!(benches, create, write, read);
criterion_main!(benches);
