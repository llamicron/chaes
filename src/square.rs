use num_enum::TryFromPrimitive;

#[derive(Copy, Clone, PartialEq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Square {
    H1, H2, H3, H4, H5, H6, H7, H8, //  0 ..  7
    G1, G2, G3, G4, G5, G6, G7, G8, //  8 .. 15
    F1, F2, F3, F4, F5, F6, F7, F8, // 16 .. 23
    E1, E2, E3, E4, E5, E6, E7, E8, // 24 .. 31
    D1, D2, D3, D4, D5, D6, D7, D8, // 32 .. 39
    C1, C2, C3, C4, C5, C6, C7, C8, // 40 .. 47
    B1, B2, B3, B4, B5, B6, B7, B8, // 48 .. 55
    A1, A2, A3, A4, A5, A6, A7, A8, // 56 .. 63
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u8() {
        assert_eq!(Square::H1 as u8, 0);
        assert_eq!(Square::C8 as u8, 47);
        assert_eq!(Square::C7 as u8, 46);
        assert_eq!(Square::A1 as u8, 56);
    }

    #[test]
    fn test_from_u8() {
        assert_eq!(Square::try_from(41).unwrap(), Square::C2);
        assert_eq!(Square::try_from(0).unwrap(), Square::H1);
        assert!(Square::try_from(69).is_err());
    }
}