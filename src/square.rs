use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Square {
    H8, H7, H6, H5, H4, H3, H2, H1, //  0 ..  7
    G8, G7, G6, G5, G4, G3, G2, G1, //  8 .. 15
    F8, F7, F6, F5, F4, F3, F2, F1, // 16 .. 23
    E8, E7, E6, E5, E4, E3, E2, E1, // 24 .. 31
    D8, D7, D6, D5, D4, D3, D2, D1, // 32 .. 39
    C8, C7, C6, C5, C4, C3, C2, C1, // 40 .. 47
    B8, B7, B6, B5, B4, B3, B2, B1, // 48 .. 55
    A8, A7, A6, A5, A4, A3, A2, A1, // 56 .. 63
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u8() {
        assert_eq!(Square::H8 as u8, 0);
        assert_eq!(Square::C8 as u8, 40);
        assert_eq!(Square::C7 as u8, 41);
        assert_eq!(Square::A1 as u8, 63);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Square::try_from(41).unwrap(), Square::C7);
        assert_eq!(Square::try_from(0).unwrap(), Square::H8);
        assert!(Square::try_from(69).is_err());
    }
}