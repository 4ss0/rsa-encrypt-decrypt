pub mod key_gen{
    extern crate openssl;

    use std::str;
    use std::string::String;
    use std::vec::Vec;
    use std::convert::TryInto;
    use std::io::Write;
    use openssl::error::ErrorStack;
    use openssl::rsa::{Rsa, Padding};
    use openssl::symm::Cipher;
    

    pub fn initialize_key_pair() -> (String, String) {
    
        

        let rsa = Rsa::generate(1024).unwrap();
        let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
        let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();
    
        (String::from_utf8(private_key).unwrap(), String::from_utf8(public_key).unwrap())

    }

    pub fn encrypt_with_rsa_public_key(pub_key: String, plaintext: String) -> Result<String, ErrorStack> {
        let rsa = Rsa::public_key_from_pem(pub_key.as_bytes())?;
        let mut ciphertext = vec![0; rsa.size() as usize];
        let len = rsa.public_encrypt(plaintext.as_bytes(), &mut ciphertext, Padding::PKCS1)?;
        ciphertext.truncate(len);
        Ok(base64::encode(&ciphertext))
    }

    pub fn decrypt_with_rsa_private_key(pvt_key: String, base64text: String) -> Result<String, ErrorStack> {
        let rsa = Rsa::private_key_from_pem(pvt_key.as_bytes())?;
        let plaintext = base64::decode(&base64text).unwrap();
        let mut ciphertext = vec![0; rsa.size() as usize];
        let len = rsa.private_decrypt(&plaintext, &mut ciphertext, Padding::PKCS1)?;
        ciphertext.truncate(len);
        Ok(base64::encode(&ciphertext))
    }

}