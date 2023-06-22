use crate::errors::MSError;

#[cfg(target_pointer_width = "64")]
pub fn pad64(value: i64) -> Result<String, MSError> {
    if value > 0xffffffff {
        return Err(MSError::HexEncodingError(format!("value is excessively long: {}", value)))
    }

    let threshold = if value < 0xf {
        format!("0000000{:x}", value)
    } else if value < 0xff {
        format!("000000{:x}", value)
    } else if value < 0xfff {
        format!("00000{:x}", value)
    } else if value < 0xffff {
        format!("0000{:x}", value)
    } else if value < 0xfffff {
        format!("000{:x}", value)
    } else if value < 0xffffff {
        format!("00{:x}", value)
    } else if value < 0xfffffff {
        format!("0{:x}", value)
    } else {
        format!("{:x}", value)
    };
    Ok(if threshold.len() % 2 == 1 {
        format!("0{}", threshold.clone())
    } else {
        threshold.clone()
    })
}

#[cfg(any(target_pointer_width = "64", target_pointer_width = "32"))]
pub fn pad32(value: i32) -> Result<String, MSError> {
    if value > 0xfffffff {
        return Err(MSError::HexEncodingError(format!("value is excessively long: {}", value)))
    }

    let threshold = if value < 0xf {
        format!("00000{:x}", value)
    } else if value < 0xff {
        format!("0000{:x}", value)
    } else if value < 0xfff {
        format!("000{:x}", value)
    } else if value < 0xffff {
        format!("00{:x}", value)
    } else if value < 0xfffff {
        format!("0{:x}", value)
    } else {
        format!("{:x}", value)
    };
    Ok(if threshold.len() % 2 == 1 {
        format!("0{}", threshold.clone())
    } else {
        threshold.clone()
    })
}

#[cfg(test)]
mod test_arch64 {
    use super::*;

    #[test]
    fn lt_16() -> Result<(), MSError> {
        let result = pad64(0xf)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "0000000f");
        Ok(())
    }

    #[test]
    fn lt_0xf1() -> Result<(), MSError> {
        let result = pad64(0xf1)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "000000f1");
        Ok(())
    }

    #[test]
    fn lt_0xff2() -> Result<(), MSError> {
        let result = pad64(0xff2)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "00000ff2");
        Ok(())
    }

    #[test]
    fn lt_0xfeff() -> Result<(), MSError> {
        let result = pad64(0xfeff)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "0000feff");
        Ok(())
    }

    #[test]
    fn lt_0x5ffff() -> Result<(), MSError> {
        let result = pad64(0x5ffff)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "0005ffff");
        Ok(())
    }

    #[test]
    fn lt_0x6ffff() -> Result<(), MSError> {
        let result = pad64(0x6fffff)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "006fffff");
        Ok(())
    }

    #[test]
    fn lt_0x7ffffff() -> Result<(), MSError> {
        let result = pad64(0x7fffff)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "007fffff");
        Ok(())
    }

    #[test]
    fn lt_0x8fffffff() -> Result<(), MSError> {
        let result = pad64(0x8fffffff)?;
        assert_eq!(result.len(), 8);
        assert_eq!(result, "8fffffff");
        Ok(())
    }
}

#[cfg(test)]
mod test_arch32 {
    use super::*;

    #[test]
    fn lt_16() -> Result<(), MSError> {
        let result = pad32(0xf)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "00000f");
        Ok(())
    }

    #[test]
    fn lt_0xf1() -> Result<(), MSError> {
        let result = pad32(0xf1)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "0000f1");
        Ok(())
    }

    #[test]
    fn lt_0xff2() -> Result<(), MSError> {
        let result = pad32(0xff2)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "000ff2");
        Ok(())
    }

    #[test]
    fn lt_0xfeff() -> Result<(), MSError> {
        let result = pad32(0xfeff)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "00feff");
        Ok(())
    }

    #[test]
    fn lt_0x5ffff() -> Result<(), MSError> {
        let result = pad32(0x5ffff)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "05ffff");
        Ok(())
    }

    #[test]
    fn lt_0x6ffff() -> Result<(), MSError> {
        let result = pad32(0x6fffff)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "6fffff");
        Ok(())
    }

    #[test]
    fn lt_0xfffffff() -> Result<(), MSError> {
        let result = pad32(0xffffff)?;
        assert_eq!(result.len(), 6);
        assert_eq!(result, "ffffff");
        Ok(())
    }
}
