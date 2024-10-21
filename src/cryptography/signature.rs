use k256::{ecdsa::{SigningKey, Signature, VerifyingKey}, elliptic_curve::rand_core::OsRng};

pub fn sign_message(message: &[u8]) -> (Signature, VerifyingKey) {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    let signature = signing_key.sign(message);
    (signature, verifying_key)
}

pub fn verify_signature(message: &[u8], signature: &Signature, verifying_key: &VerifyingKey) -> bool {
    verifying_key.verify(message, signature).is_ok()
}
