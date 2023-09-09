use crate::errors::MSError;
use crate::p::str_to_u128;
use crate::ram::{crc32, MetaMagic};
use hex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

// use magic_switcheroo::{hexdecs, CAR_SIZE};
pub fn read_file_into_vec(filename: &String, contents: &mut Vec<u8>) {
    let mut f = File::open(filename).unwrap();
    f.read_to_end(contents)
        .expect(&format!("failed to read file '{}'", filename));
}
pub fn read_file(filename: &String) -> Result<(Vec<u8>, Vec<u8>), MSError> {
    let mut contents = Vec::new();
    read_file_into_vec(filename, &mut contents);
    Ok((contents.clone(), crc32(&contents)?))
}

pub fn write_file(filename: String, data: Vec<u8>) -> Result<(), MSError> {
    let mut file = File::create(filename)?;
    Ok(file.write_all(&data)?)
}
pub fn enchant_file(filename: String, magic: String) -> Result<(), MSError> {
    let (read, _) = read_file(&filename)?;

    let meta = MetaMagic::new(read, &magic)?;
    Ok(write_file(filename, meta.enchant()?)?)
}

pub fn restore_file(filename: String, magic: String) -> Result<(), MSError> {
    let (raw, _) = read_file(&filename)?;
    let meta = MetaMagic::from_enchanted(raw, &magic)?;
    let restored = meta.orig();
    write_file(filename, restored)?;
    Ok(())
}

pub fn suffix_file(filename: String, prefix: Vec<String>) -> Result<(), MSError> {
    let (read, _) = read_file(&filename)?;
    let mut xdata = read.to_vec();
    for s in prefix {
        xdata.extend(hex::decode(&format!("{:02x}", str_to_u128(&s)?))?);
    }
    Ok(write_file(filename, xdata)?)
}

pub fn prefix_file(filename: String, prefix: Vec<String>) -> Result<(), MSError> {
    let (read, _) = read_file(&filename)?;
    let mut xdata = Vec::<u8>::new();
    for s in prefix {
        xdata.extend(hex::decode(&format!("{:02x}", str_to_u128(&s)?))?);
    }
    xdata.extend(read);
    Ok(write_file(filename, xdata)?)
}

pub fn delete_start_file(filename: String, amnt: usize) -> Result<Vec<u8>, MSError> {
    let (read, _) = read_file(&filename)?;
    let mut data = VecDeque::<u8>::from(read);
    let mut popped = Vec::<u8>::new();
    for _ in 0..amnt {
        match data.pop_front() {
            Some(b) => popped.push(b),
            None => break,
        }
    }
    write_file(filename, data.into())?;
    Ok(popped)
}

pub fn delete_end_file(filename: String, amnt: usize) -> Result<Vec<u8>, MSError> {
    let (read, _) = read_file(&filename)?;
    let mut data = VecDeque::<u8>::from(read);
    let mut popped = Vec::<u8>::new();
    for _ in 0..amnt {
        match data.pop_back() {
            Some(b) => popped.push(b),
            None => break,
        }
    }
    write_file(filename, data.into())?;
    Ok(popped)
}

pub fn read_start_file(filename: String, amnt: usize) -> Result<Vec<u8>, MSError> {
    let (read, _) = read_file(&filename)?;
    Ok(read[0..amnt].to_vec())
}

pub fn read_end_file(filename: String, amnt: usize) -> Result<Vec<u8>, MSError> {
    let (read, _) = read_file(&filename)?;
    let h = read.len() - amnt;
    Ok(read[h..].to_vec())
}

#[cfg(test)]
mod tests {
    use crate::errors::MSError;
    use crate::fs::delete_end_file;
    use crate::fs::delete_start_file;
    use crate::fs::enchant_file;
    use crate::fs::prefix_file;
    use crate::fs::restore_file;
    use crate::fs::suffix_file;
    use crate::fs::{read_file, write_file};
    use hex;
    use k9::assert_equal;

    fn test_image_data() -> Vec<u8> {
        Vec::from([
            // <car>
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
            0x00, 0x28, 0xcb, 0x34, // </car>
            // <cdr>
            0xbb, 0x00, 0x00, 0x00, 0x03, 0x50, 0x4c, 0x54, 0x45, 0xff, 0xff, 0xff, 0xa7, 0xc4,
            0x1b, 0xc8, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x08, 0x99, 0x63, 0x60,
            0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xf4, 0x71, 0x64, 0xa6, 0x00, 0x00, 0x00, 0x00,
            0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
            // </cdr>
        ])
    }
    fn create_test_image_file(filename: String) -> Result<String, MSError> {
        write_file(filename.clone(), test_image_data())?;
        Ok(filename)
    }
    #[test]
    fn test_enchant_file() -> Result<(), MSError> {
        let name: String = "to-enchant.png".to_string();
        let magic: String = "THISISMAGICO".to_string();

        // Given an image file exists
        let filename = create_test_image_file(name.clone())?;
        assert_equal!("to-enchant.png", &filename);

        // When I enchant it
        enchant_file(filename, magic.clone())?;

        // Then it should exist
        let (enchanted_contents, enchanted_checksum) = read_file(&name)?;
        assert_equal!(hex::encode(enchanted_checksum), "8b86405c");
        assert_equal!(
            hex::encode(enchanted_contents), "0000000c3d00000032245448495349534d414749434fc3bec3bf487cad4daff0df6b2a00551c34cb2800000003080100000001000000524448490d0000000a1a0a0d474e5089c3bec3bf826042ae444e454900000000a66471f401000200000060639908544144490a000000c81bc4a7ffffff45544c5003000000bb"
        );
        Ok(())
    }

    #[test]
    fn test_restore_file() -> Result<(), MSError> {
        let name: String = "to-restore.png".to_string();
        let magic: String = "THISISMAGICO".to_string();

        // Given an image file exists
        let filename = create_test_image_file(name.clone())?;
        assert_equal!("to-restore.png", &filename);

        // And that it has been enchanted
        enchant_file(filename.clone(), magic.clone())?;

        // When I restore it
        restore_file(filename.clone(), magic.clone())?;

        // Then it should have the previous contents
        let (read, _) = read_file(&filename)?;
        assert_equal!(hex::encode(read), hex::encode(test_image_data()));

        Ok(())
    }

    #[test]
    fn test_suffix_file() -> Result<(), MSError> {
        let name: String = "to-suffix.png".to_string();

        // Given an image file exists
        write_file(
            name.clone(),
            Vec::<u8>::from([
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
                0x00, 0x28, 0xcb, 0x34,
            ]),
        )?;

        // When I suffix it
        suffix_file(
            name.clone(),
            "0x4f 0o44 0b100101"
                .to_string()
                .split(' ')
                .map(|x| x.to_string())
                .collect(),
        )?;

        // Then it should exist
        let (suffixed_contents, _) = read_file(&name)?;
        assert_equal!(
            suffixed_contents,
            Vec::<u8>::from([
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
                0x00, 0x28, 0xcb, 0x34, 0x4f, 0x24, 0x25,
            ])
        );

        Ok(())
    }
    #[test]
    fn test_prefix_file() -> Result<(), MSError> {
        let name: String = "to-prefix.png".to_string();

        // Given an image file exists
        write_file(
            name.clone(),
            Vec::<u8>::from([
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
                0x00, 0x28, 0xcb, 0x34,
            ]),
        )?;

        // When I prefix it
        prefix_file(
            name.clone(),
            "0x4f 0o44 0b100101"
                .to_string()
                .split(' ')
                .map(|x| x.to_string())
                .collect(),
        )?;

        // Then it should exist
        let (prefixed_contents, _) = read_file(&name)?;
        assert_equal!(
            prefixed_contents,
            Vec::<u8>::from([
                0x4f, 0x24, 0x25, 0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00,
                0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08,
                0x03, 0x00, 0x00, 0x00, 0x28, 0xcb, 0x34,
            ])
        );

        Ok(())
    }
    #[test]
    fn test_delete_start_file() -> Result<(), MSError> {
        let name: String = "dsf.png".to_string();

        // Given an image file exists
        write_file(
            name.clone(),
            Vec::<u8>::from([
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
                0x00, 0x28, 0xcb, 0x34,
            ]),
        )?;

        // When I delete the first N bytes
        delete_start_file(name.clone(), 4)?;

        // Then it should exist
        let (prefixed_contents, _) = read_file(&name)?;
        assert_equal!(
            prefixed_contents,
            Vec::<u8>::from([
                0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00,
                0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00, 0x00, 0x28, 0xcb, 0x34,
            ])
        );

        Ok(())
    }
    #[test]
    fn test_delete_end_file() -> Result<(), MSError> {
        let name: String = "def.png".to_string();

        // Given an image file exists
        write_file(
            name.clone(),
            Vec::<u8>::from([
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
                0x00, 0x28, 0xcb, 0x34,
            ]),
        )?;

        // When I delete the first N bytes
        delete_end_file(name.clone(), 4)?;

        // Then it should exist
        let (prefixed_contents, _) = read_file(&name)?;
        assert_equal!(
            prefixed_contents,
            Vec::<u8>::from([
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
            ])
        );

        Ok(())
    }
}
