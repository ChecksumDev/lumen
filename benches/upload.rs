use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotenvy::dotenv;
use ureq::Agent;

#[inline]
fn upload_file(agent: &Agent, api_key: &str, public_url: &str, size_kb: usize, encrypted: bool) {
    agent
        .post(&format!("{}/upload", public_url))
        .set("x-api-key", api_key)
        .set("x-file-name", "test.png")
        .set("x-encrypted", if encrypted { "true" } else { "false" })
        .set("content-type", "image/png")
        .send_bytes(&vec![0; size_kb * 1024])
        .ok();
}

macro_rules! generate_upload_functions {
    ($($name:ident: $size_kb:expr, $encrypted:expr),*) => {
        $(
            #[inline]
            fn $name(agent: &Agent, api_key: &str, public_url: &str) {
                upload_file(agent, api_key, public_url, $size_kb, $encrypted);
            }
        )*
    };
}

generate_upload_functions! {
    // encrypted uploads
    upload_encrypted_1mb: 1024, true,
    upload_encrypted_512kb: 512, true,
    upload_encrypted_256kb: 256, true,
    upload_encrypted_128kb: 128, true,
    upload_encrypted_64kb: 64, true,

    // unencrypted uploads
    upload_unencrypted_1mb: 1024, false,
    upload_unencrypted_512kb: 512, false,
    upload_unencrypted_256kb: 256, false,
    upload_unencrypted_128kb: 128, false,
    upload_unencrypted_64kb: 64, false
}

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
                c.bench_function(stringify!($name), |b| {
                    b.iter(|| {
                        $name(
                            black_box(&agent),
                            black_box(&api_key),
                            black_box(&public_url),
                        )
                    })
                });

                purge.clone().call().expect("Failed to purge files");
            )*
        };
    }

    generate_bench_functions! {
        // encrypted uploads
        upload_encrypted_1mb: 1024, true,
        upload_encrypted_512kb: 512, true,
        upload_encrypted_256kb: 256, true,
        upload_encrypted_128kb: 128, true,
        upload_encrypted_64kb: 64, true,

        // unencrypted uploads
        upload_unencrypted_1mb: 1024, false,
        upload_unencrypted_512kb: 512, false,
        upload_unencrypted_256kb: 256, false,
        upload_unencrypted_128kb: 128, false,
        upload_unencrypted_64kb: 64, false
    }
}   

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
