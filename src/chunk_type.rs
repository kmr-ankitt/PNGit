/*
* Chunk representation in PNG file
 *
 *  bLOb  <-- 32 bit chunk type code represented in text form
 *  ||||
 *  |||+- Safe-to-copy bit is 1 (lowercase letter; bit 5 is 1)
 *  ||+-- Reserved bit is 0     (uppercase letter; bit 5 is 0)
 *  |+--- Private bit is 0      (uppercase letter; bit 5 is 0)
 *  +---- Ancillary bit is 1    (lowercase letter; bit 5 is 1)
*/

pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return [self.0[0], self.0[1], self.0[2], self.0[3]];
    }

    pub fn is_critical(&self) -> bool {
        ChunkType::is_uppercase(self.0[0])
    }

    pub fn is_public(&self) -> bool {
        ChunkType::is_uppercase(self.0[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        ChunkType::is_uppercase(self.0[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        ChunkType::is_lowercase(self.0[3])
    }

    fn is_uppercase(byte: u8) -> bool {
        byte >= b'A' && byte <= b'Z'
    }

    fn is_lowercase(byte: u8) -> bool {
        byte >= b'a' && byte <= b'z'
    }

    fn is_valid_byte(byte: u8) -> bool {
        ChunkType::is_uppercase(byte) || ChunkType::is_lowercase(byte)
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }

        if !self.bytes().iter().all(|b| ChunkType::is_valid_byte(*b)) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
