use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn shivam(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    shivam(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn recover_pub_key() {
    let pair = ecdsa::Pair::from_string(&format!("//{}", 1), None).unwrap();
    let hash = keccak_256(b"Hello");
    let signature = pair.sign_prehashed(&hash);

    if let Ok(recovered_raw) = secp256k1_ecdsa_recover_compressed(&signature.0, &hash) {
        let recovered = ecdsa::Public::from_raw(recovered_raw);
        // Assert that we recovered the correct PK.
        assert_eq!(pair.public(), recovered);
    } else {
        panic!("recovery failed ...!");
    }
}