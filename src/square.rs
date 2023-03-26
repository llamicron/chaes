use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8, // 00 .. 07
    A7, B7, C7, D7, E7, F7, G7, H7, // 08 .. 15
    A6, B6, C6, D6, E6, F6, G6, H6, // 16 .. 23
    A5, B5, C5, D5, E5, F5, G5, H5, // 24 .. 31
    A4, B4, C4, D4, E4, F4, G4, H4, // 32 .. 39
    A3, B3, C3, D3, E3, F3, G3, H3, // 40 .. 47
    A2, B2, C2, D2, E2, F2, G2, H2, // 48 .. 55
    A1, B1, C1, D1, E1, F1, G1, H1, // 56 .. 63
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u8() {
        assert_eq!(Square::H1 as u8, 63);
        assert_eq!(Square::C8 as u8, 2);
        assert_eq!(Square::C7 as u8, 10);
        assert_eq!(Square::A1 as u8, 56);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Square::try_from(50).unwrap(), Square::C2);
        assert_eq!(Square::try_from(63).unwrap(), Square::H1);
        assert!(Square::try_from(69).is_err());
    }
}