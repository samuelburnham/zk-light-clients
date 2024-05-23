use glob::glob;
use std::fs;
use std::path::PathBuf;
use wp1_helper::build_program;

const PROGRAM_PATTERNS: [&str; 2] = ["../programs/*", "../programs/benchmarks/*"];
const TARGET_DIR: [&str; 2] = ["./artifacts", "./artifacts/benchmarks"];

fn main() {
    // Re-run if the core library changes
    let core_dir = std::path::Path::new("../core");
    println!("cargo:rerun-if-changed={}", core_dir.display());

    for (program_pattern, artifacts_folder) in PROGRAM_PATTERNS.iter().zip(TARGET_DIR.iter()) {
        // Create the target directory if it doesn't exist
        if !PathBuf::from(artifacts_folder).exists() {
            fs::create_dir_all(artifacts_folder).unwrap();
        }

        // Iterate over each directory that matches the pattern
        for entry in glob(program_pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    // Ignore the benchmarks folder
                    if path.ends_with("benchmarks") {
                        continue;
                    }

                    build_program(path.to_str().unwrap());

                    let dir_name = path.file_name().unwrap().to_str().unwrap();

                    let old_path = path.join("elf/riscv32im-succinct-zkvm-elf");
                    let new_path = format!("{}/{}-program", artifacts_folder, dir_name);

                    // If the file exists, move and rename it
                    if old_path.exists() {
                        fs::rename(old_path, new_path).unwrap();
                        fs::remove_dir_all(path.join("elf")).unwrap();
                    }
                }
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
