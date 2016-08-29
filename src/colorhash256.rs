#![no_std]

//! Like Chroma-Hash, but with ANSI terminal colors.
extern crate tiny_keccak;
use tiny_keccak::Keccak;

/// Hashes given bytes and encodes the result as ANSI terminal colors.
pub fn hash_as_ansi(bytes: &[u8]) -> [u16; 8] {
    let mut colors = [0; 8];
    let mut hash = [0; 8];

    let mut sha3 = Keccak::new_sha3_256();
    sha3.update(bytes);
    sha3.finalize(&mut hash);

    for i in 0..8 {
        colors[i] = 16 + (hash[i] as u16 % 216);
    }
    colors
}

/// Hashes given bytes and encodes the result as RGB colors.
pub fn hash_as_rgb(bytes: &[u8]) -> [(u8, u8, u8); 8] {
    let hash = hash_as_ansi(bytes);
    let mut colors: [(u8, u8, u8); 8] = [(0, 0, 0); 8];
    for i in 0..8 {
        let x = hash[i] - 16;
        colors[i] = ((x / 36 * 51) as u8, (x % 36 / 6 * 51) as u8, (x % 36 % 6 * 51) as u8);
    }
    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi() {
        assert_eq!(hash_as_ansi(b"Correct Horse Battery Staple"), [79, 131, 17, 194, 196, 52, 19, 54]);
    }

    #[test]
    fn test_rgb() {
        assert_eq!(hash_as_rgb(b"Correct Horse Battery Staple"), [(51, 204, 153), (153, 51, 51), (0, 0, 51), (204, 255, 204), (255, 0, 0), (51, 0, 0), (0, 0, 153), (51, 0, 102)]);
    }
}
