// use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
// use rand::rngs::OsRng;
// use base64;

// pub fn encrypt(data: String, pub_key: RsaPublicKey) -> Result<Vec<u8>, rsa::Error>{
//     let mut rng = rand::thread_rng();
//     let enc_data = match pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data.as_bytes()){
//         Ok(value) => Ok(value),
//         Err(e) => {
//             println!("{:?}", e);
//             Err(e)
//         }
//     };
//     enc_data
// }

// pub fn decrypt(enc_data: Vec<u8>, priv_key: RsaPrivateKey) -> Result<Vec<u8>, rsa::Error> {
//     let dec_data = match priv_key.decrypt(Pkcs1v15Encrypt, &enc_data){
//         Ok(d) => Ok(d),
//         Err(e) =>{
//             println!("{:?}", e.to_string());
//             Err(e)
//         },
//     };
//     dec_data

// }

// pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
//     let mut rng = OsRng;
//     let bits = 2048;
//     let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//     let pub_key = RsaPublicKey::from(&priv_key);
//     (priv_key, pub_key)
// }

// pub fn to_utf(data: Vec<u8>) -> String{
//     base64::encode(data)
// }
// pub fn from_utf(data: String) -> Vec<u8>{
//     base64::decode(data).unwrap()
// }

// pub fn autogen_new_token() {

// }
