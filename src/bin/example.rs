use base64_secret::Base64;

fn main() {
    let engine = Base64::new(b"my secret key");
    let data = b"This is a secret message";

    let encoded_data = engine.encode(&data);

    println!("{}", encoded_data);

    let data = engine.decode(&encoded_data).unwrap();

    println!("{}", String::from_utf8_lossy(&data));
}
