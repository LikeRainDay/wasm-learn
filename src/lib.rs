use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use base64::encode;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt_aes(key: &str, iv: &str) -> Result<String, JsValue> {
    // let key = GenericArray::from_slice(key.as_bytes());
    // let iv_bytes = GenericArray::from_slice(iv.as_bytes());
    // let cipher = Aes128::new(&key);
    // let mut block = iv_bytes.clone();
    // cipher.encrypt_block(&mut block);
    // let ciphertext_base64 = encode(&block);
    // Ok(ciphertext_base64)
    Ok("Hello".to_string())
}
