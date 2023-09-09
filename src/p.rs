use crate::errors::MSError;


pub fn str_to_u128(s: &str) -> Result<u128, MSError> {
    Ok(if s.starts_with("0x") || s.starts_with(r"\x") {
        u128::from_str_radix(&s[2..], 16)?
    } else if s.starts_with("0o") || s.starts_with(r"\o") {
        u128::from_str_radix(&s[2..], 8)?
    } else if s.starts_with("0b") || s.starts_with(r"\b") {
        u128::from_str_radix(&s[2..], 2)?
    } else {
        u128::from_str_radix(s, 10)?
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::MSError;
    use k9::assert_equal;

    #[test]
    fn test_str_to_u128() -> Result<(), MSError> {
        assert_equal!(str_to_u128("10")?, 10u128);
        assert_equal!(str_to_u128("0xa")?, 10u128);
        assert_equal!(str_to_u128("0o12")?, 10u128);
        assert_equal!(str_to_u128("0b1010")?, 10u128);
        Ok(())
    }
}
