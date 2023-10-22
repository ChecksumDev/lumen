use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotenvy::dotenv;
use ureq::Agent;

#[inline]
fn upload(agent: &Agent, api_key: &str, public_url: &str) {
    agent
        .post(&format!("{}/upload", public_url))
        .set("x-api-key", api_key)
        .set("x-file-name", "test.png")
        .set("content-type", "image/png")
        .send_bytes(&vec![0; 1024 * 1024])
        .ok();
}

fn criterion_benchmark(c: &mut Criterion) {
    dotenv().ok();

    let agent = Agent::new();
    let api_key = std::env::var("TESTING_API_KEY").expect("TESTING_API_KEY not set in environment");
    let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL not set in environment");

    c.bench_function("upload", |b| {
        b.iter(|| upload(black_box(&agent), black_box(&api_key), black_box(&public_url)))
    });

    agent
        .post(&format!("{}/purge", public_url))
        .set("x-api-key", &api_key)
        .call()
        .unwrap();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
