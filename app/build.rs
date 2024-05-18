
use rand::{distributions::Alphanumeric, Rng};

fn generate_secure_string(length: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn main() {
    let length = 42;
    let secure_string = generate_secure_string(length); 
    // Set an environment variable for the build process
    println!("{}", format_args!("cargo:rustc-env=LITCRYPT_ENCRYPT_KEY={secure_string}"));
}
