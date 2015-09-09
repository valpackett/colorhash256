//! Like Chroma-Hash, but with ANSI terminal colors.
extern crate sodiumoxide;
use sodiumoxide::crypto::hash::sha256;

/// Hashes given bytes and encodes the result as ANSI terminal colors.
pub fn hash_as_ansi(bytes: &[u8]) -> [u16; 8] {
    let sha256::Digest(hash) = sha256::hash(bytes);
    let mut colors: [u16; 8] = [0; 8];
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
        assert_eq!(hash_as_ansi(b"Correct Horse Battery Staple"), [191, 35, 175, 178, 148, 70, 82, 37]);
    }

    #[test]
    fn test_rgb() {
        assert_eq!(hash_as_rgb(b"Correct Horse Battery Staple"), [(204, 255, 51), (0, 153, 51), (204, 102, 153), (204, 153, 0), (153, 204, 0), (51, 153, 0), (51, 255, 0), (0, 153, 153)]);
    }
}
