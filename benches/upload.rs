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
                fn $name(agent: &Agent, api_key: &str, public_url: &str) -> u16 {
                    agent
                        .post(&format!("{}/upload", public_url))
                        .set("x-api-key", api_key)
                        .set("x-file-name", "test.png")
                        .set("x-encrypted", if $encrypted { "true" } else { "false" })
                        .set("content-type", "image/png")
                        .send_bytes(&vec![0; $size_kb * 1024])
                        .expect("Failed to upload file")
                        .status()
                }

                c.bench_function(stringify!($name), |b| {
                    b.iter(|| {
                        $name(
                            &agent,
                            &api_key,
                            &public_url,
                        )
                    })
                });

                purge.clone().call().expect("Failed to purge files");
            )*
        }
    }

    generate_bench_functions! {
        upload_64kb_encrypted: 64, true,
        upload_128kb_encrypted: 128, true,
        upload_256kb_encrypted: 256, true,
        upload_512kb_encrypted: 512, true,
        upload_1mb_encrypted: 1024, true,

        upload_64kb_unencrypted: 64, false,
        upload_128kb_unencrypted: 128, false,
        upload_256kb_unencrypted: 256, false,
        upload_512kb_unencrypted: 512, false,
        upload_1mb_unencrypted: 1024, false
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
