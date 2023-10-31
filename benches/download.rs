use criterion::{criterion_group, criterion_main, Criterion};
use dotenvy::dotenv;
use ureq::Agent;

fn criterion_benchmark(c: &mut Criterion) {
    dotenv().ok();

    let agent = Agent::new();
    let api_key = std::env::var("TESTING_API_KEY").expect("TESTING_API_KEY not set in environment");
    let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL not set in environment");

    let purge = agent
        .post(&format!("{}/purge", public_url))
        .set("x-api-key", &api_key);

    macro_rules! generate_bench_functions {
        ($($name:ident: $size_kb:expr, $encrypted:expr),*) => {
            $(
                #[inline]
                fn $name(agent: &Agent, public_url: &str, id: &str, key: &str, nonce: &str) -> Vec<u8> {
                    let mut buffer = vec![0; $size_kb * 1024];

                    agent.get(&format!("{}/{}?key={}&nonce={}", public_url, id, key, nonce))
                        .call()
                        .expect("Failed to download file")
                        .into_reader()
                        .read_exact(&mut buffer).expect("Failed to read file");

                    buffer
                }

                let encrypted_file = agent
                    .post(&format!("{}/upload", public_url))
                    .set("x-api-key", &api_key)
                    .set("x-file-name", "test.png")
                    .set("x-encrypted", "true")
                    .set("content-type", "image/png")
                    .send_bytes(&vec![0; $size_kb * 1024])
                    .expect("Failed to upload file")
                    .into_json::<ureq::serde_json::Value>().expect("Failed to parse JSON");             

                c.bench_function(stringify!($name), |b| {
                    b.iter(|| {
                        $name(
                            &agent,
                            &public_url,
                            encrypted_file["id"].as_str().expect("Failed to get file ID"),
                            encrypted_file["key"].as_str().expect("Failed to get file key"),
                            encrypted_file["nonce"].as_str().expect("Failed to get file nonce"),
                        )
                    })
                });

                purge.clone().call().expect("Failed to purge files");
            )*
        }
    }

    generate_bench_functions! {
        download_64kb_encrypted: 64, true,
        download_128kb_encrypted: 128, true,
        download_256kb_encrypted: 256, true,
        download_512kb_encrypted: 512, true,
        download_1mb_encrypted: 1024, true,
        download_64kb_unencrypted: 64, false,
        download_128kb_unencrypted: 128, false,
        download_256kb_unencrypted: 256, false,
        download_512kb_unencrypted: 512, false,
        download_1mb_unencrypted: 1024, false
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);