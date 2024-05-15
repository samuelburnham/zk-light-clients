use aptos_lc_core::aptos_test_utils::wrapper::AptosWrapper;
use aptos_lc_core::NBR_VALIDATORS;
use serde::Serialize;
use std::hint::black_box;
use std::time::Instant;
use wp1_sdk::utils::setup_logger;
use wp1_sdk::{ProverClient, SP1DefaultProof, SP1Stdin};

struct ProvingAssets {
    client: ProverClient,
    ledger_info_with_signature: Vec<u8>,
}

impl ProvingAssets {
    fn new() -> Self {
        let mut aptos_wrapper = AptosWrapper::new(2, NBR_VALIDATORS, NBR_VALIDATORS).unwrap();
        aptos_wrapper.generate_traffic().unwrap();
        aptos_wrapper.commit_new_epoch().unwrap();

        let ledger_info_with_signature = aptos_wrapper.get_latest_li_bytes().unwrap();

        let client = ProverClient::new();

        Self {
            client,
            ledger_info_with_signature,
        }
    }

    fn prove(&self) -> SP1DefaultProof {
        let mut stdin = SP1Stdin::new();

        setup_logger();

        stdin.write(&self.ledger_info_with_signature);

        let (pk, _) = self
            .client
            .setup(aptos_programs::bench::SIGNATURE_VERIFICATION_PROGRAM);
        self.client.prove(&pk, stdin).unwrap()
    }

    fn verify(&self, proof: &SP1DefaultProof) {
        let (_, vk) = self
            .client
            .setup(aptos_programs::bench::SIGNATURE_VERIFICATION_PROGRAM);
        self.client.verify(proof, &vk).expect("Verification failed");
    }
}

#[derive(Serialize)]
struct Timings {
    proving_time: u128,
    verifying_time: u128,
}

fn main() {
    let proving_assets = ProvingAssets::new();

    let start_proving = Instant::now();
    let proof = proving_assets.prove();
    let proving_time = start_proving.elapsed();

    let start_verifying = Instant::now();
    proving_assets.verify(black_box(&proof));
    let verifying_time = start_verifying.elapsed();

    // Print results in JSON format.
    let timings = Timings {
        proving_time: proving_time.as_millis(),
        verifying_time: verifying_time.as_millis(),
    };

    let json_output = serde_json::to_string(&timings).unwrap();
    println!("{}", json_output);
}