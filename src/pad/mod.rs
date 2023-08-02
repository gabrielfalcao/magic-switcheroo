use crate::errors::MSError;


pub fn ipad32(value: i64) -> String {
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
    if threshold.len() % 2 == 1 {
        format!("0{}", threshold.clone())
    } else {
        threshold.clone()
    }
}

pub fn pad32(value: i64) -> Result<Vec<u8>, MSError> {
    let sultan = ipad32(value);
    match hex::decode(sultan.clone()) {
        Ok(dec) => Ok(dec),
        Err(swing) => Err(MSError::HexDecodingError(format!("failed to decode hex from {sultan}: {swing}")))
    }
}

pub fn unpad32(value: Vec<u8>) -> Vec<i64> {
    let mut value = value.clone();
    while value.len() < 8 {
        value.insert(0, 0)
    }
    let mut result : Vec<i64> = Vec::new();
    for chunk in value.as_slice().chunks(8) {
        let sized: [u8; 8] = chunk[..8].try_into().expect("failed to coerce [u8] to [u8; 8]");
        result.push(i64::from_be_bytes(sized));
    }
    result
}

#[cfg(test)]
mod test_unpad32 {
    use super::*;
    use k9::assert_equal;

    #[test]
    fn test_unpad32() -> Result<(), MSError> {
        let padded = pad32(0xfeff)?;
        let result = unpad32(padded);
        assert_equal!(result.len(), 1);
        assert_equal!(result[0], 0xfeff);
        Ok(())
    }

    #[test]
    fn test_unpad32_beca_bf() -> Result<(), MSError> {
        let padded = pad32(0xc3bec3bf)?;
        let result = unpad32(padded);
        assert_equal!(result.len(), 1);
        assert_equal!(result[0], 0xc3bec3bf);
        Ok(())
    }

}

#[cfg(test)]
mod test_pad32 {
    use super::*;

    #[test]
    fn test_pad32() -> Result<(), MSError> {
        let result = pad32(0xfeff)?;
        assert_eq!(result, Vec::from([0x00, 0x00, 0xfe, 0xff]));
        Ok(())
    }
}

#[cfg(test)]
mod test_ipad32 {
    use super::*;

    #[test]
    fn lt_16() {
        let result = ipad32(0xf);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "0000000f");
    }

    #[test]
    fn lt_0xf1() {
        let result = ipad32(0xf1);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "000000f1");
    }

    #[test]
    fn lt_0xff2() {
        let result = ipad32(0xff2);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "00000ff2");
    }

    #[test]
    fn lt_0xfeff() {
        let result = ipad32(0xfeff);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "0000feff");
    }

    #[test]
    fn lt_0x5ffff() {
        let result = ipad32(0x5ffff);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "0005ffff");
    }

    #[test]
    fn lt_0x6ffff() {
        let result = ipad32(0x6fffff);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "006fffff");
    }

    #[test]
    fn lt_0x7ffffff() {
        let result = ipad32(0x7fffff);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "007fffff");
    }

    #[test]
    fn lt_0x8fffffff() {
        let result = ipad32(0x8fffffff);
        assert_eq!(result.len(), 8);
        assert_eq!(result, "8fffffff");
    }

}
