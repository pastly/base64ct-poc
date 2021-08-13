use base64ct::{Base64Unpadded, Encoding};

fn main() {
    for ex in &["Mi", "ppwthHXW8kXD0f9fE7UPYsOAAu4uj5ORwSomCMxaaaa"] {
        println!("--- {} ---", ex);
        let a = base64::decode(ex);
        let b = Base64Unpadded::decode_vec(ex);
        println!("base64:   {:?}", a);
        println!("base64ct: {:?}", b);
    }
}
