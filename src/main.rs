use ed25519_dalek::{PublicKey, Signature, Verifier};
use ed25519::Error;

fn verify_signature() -> Result<(), Error> {
    let message: &[u8] = &[1, 2, 3, 4, 5];
    let public_key_bytes: [u8; 32] = [
        64, 62, 214, 61, 58, 61, 30, 176,
        50, 167, 89, 40, 14, 74, 55, 162,
        180, 239, 145, 111, 62, 251, 55, 20,
        244, 147, 168, 212, 182, 184, 143, 123
    ];
    let public_key: PublicKey = PublicKey::from_bytes(&public_key_bytes)?;

    let signature_bytes: [u8; 64] = [
        176, 149, 248, 112, 26, 7, 30, 203,
        255, 227, 148, 63, 19, 230, 136, 205,
        93, 114, 0, 103, 101, 67, 84, 120,
        3, 52, 141, 23, 252, 198, 208, 169,
        209, 134, 164, 172, 214, 131, 120, 63,
        105, 92, 76, 45, 155, 50, 196, 227,
        151, 241, 74, 77, 131, 104, 115, 22,
        220, 133, 41, 227, 4, 249, 66, 14
    ];
    let signature: Signature = Signature::from_bytes(&signature_bytes)?;
    public_key.verify(message, &signature)
}

fn main() {
    let res = verify_signature();
    println!("RESULT: {:?}", res);
}
