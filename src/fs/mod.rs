use super::ram::{crc32, MetaMagic};
use crate::errors::MSError;
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
// #[cfg(test)]
// mod tests {
//     use crate::errors::MSError;
//     use super::enchant_file;
//     use super::restore_file;
//     use super::{read_file, write_file};
//     use hex;
//     use k9::{assert_equal};

//     fn test_image_data() -> Vec<u8> {
//         Vec::from([
//             // <car>
//             0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
//             0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
//             0x00, 0x28, 0xcb, 0x34,
//             // </car>
//             // <cdr>
//             0xbb, 0x00, 0x00, 0x00, 0x03, 0x50, 0x4c, 0x54, 0x45, 0xff, 0xff, 0xff, 0xa7, 0xc4,
//             0x1b, 0xc8, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x08, 0x99, 0x63, 0x60,
//             0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xf4, 0x71, 0x64, 0xa6, 0x00, 0x00, 0x00, 0x00,
//             0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
//             // </cdr>
//         ])
//     }
//     fn create_test_image_file(filename: String) -> Result<String, MSError> {
//         write_file(filename.clone(), test_image_data())?;
//         Ok(filename)
//     }
//     #[test]
//     fn test_enchant_file() -> Result<(), MSError> {
//         let name: String = "to-enchant.png".to_string();
//         let magic: String = "THISISMAGICO".to_string();

//         // Given an image file exists
//         let filename = create_test_image_file(name.clone())?;
//         assert_equal!("to-enchant.png", &filename);

//         // When I enchant it
//         enchant_file(filename, magic.clone())?;

//         // Then it should exist
//         let (enchanted_contents, enchanted_checksum) = read_file(&name);
//         assert_equal!("4b414078", hex::encode(enchanted_checksum));
//         assert_equal!(
//             hex::encode(enchanted_contents), "0c3d32245448495349534d414749434fc3bec3bfe0ffc57747e708639c7e1a7f34cb2800000003080100000001000000524448490d0000000a1a0a0d474e5089c3bec3bf826042ae444e454900000000a66471f401000200000060639908544144490a000000c81bc4a7ffffff45544c5003000000bb"
//         );

//         Ok(())
//     }
//     #[test]
//     fn test_restore_file() -> Result<(), MSError> {
//         let name: String = "to-restore.png".to_string();
//         let magic: String = "THISISMAGICO".to_string();

//         // Given an image file exists
//         let filename = create_test_image_file(name.clone())?;
//         assert_equal!("to-restore.png", &filename);

//         // And that it has been enchanted
//         enchant_file(filename.clone(), magic.clone())?;

//         // When I restore it
//         restore_file(filename.clone(), magic.clone())?;

//         // Then it should have the previous contents
//         let (read, _) = read_file(&filename);
//         assert_equal!(
//             hex::encode(read), hex::encode(test_image_data())
//         );

//         Ok(())
//     }
// }
