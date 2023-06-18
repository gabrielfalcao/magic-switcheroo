use crc::{Crc, CRC_32_ISCSI};
use hex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fmt;

pub const CAR_SIZE: usize = 32;
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);

pub fn hexdecs(data: &str) -> Vec<u8> {
    //eprintln!("{}", ac(0xac).paint(format!("hexdecs length: {}", data.len())));
    return hex::decode(data.clone()).expect(&format!("failed to hex-decode {:?}", data));
}
pub fn hexdeca(a: &[u8]) -> Vec<u8> {
    return hexdecs(&hex::encode(&a));
}
pub fn hexdecu32(v: u32) -> Vec<u8> {
    return hexdecs(&format!("{:x}", v));
}
pub fn crc32(data: &[u8]) -> Vec<u8> {
    return hexdecu32(CASTAGNOLI.checksum(&data));
}
pub fn usize_to_hex(value: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    return Ok(hex::decode(&if value < 16 {
        format!("0{:x}", value)
    } else {
        format!("{:x}", value)
    })?);
}
pub fn hex_to_usize(input: Vec<u8>, limit: usize) -> Result<usize, Box<dyn Error>> {
    let bytes = Vec::from(&input[..limit]);
    return Ok(bytes[0] as usize);
}

pub fn getmark() -> Vec<u8> {
    return hexdecs("c3bec3bf");
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetaMagic {
    tail_size: usize,    // arbitrary
    magic_size: usize,   // 12 (minimum)
    magic: Vec<u8>,      // 12 (presumed)
    mach0: [u8; 4],      // +4=16
    odigest: [u8; 4],    // +4=20
    ldigest: [u8; 4],    // +4=24
    rdigest: [u8; 4],    // +4=28
    car: [u8; CAR_SIZE], // +32= 60 // contains original magic numbers
    machf: [u8; 4],      // 64
    cdr: Vec<u8>,        //..tail_size
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct HumanizedMagic {
    tail_size: String,
    magic_size: String,
    magic: String,
    mach0: String,
    odigest: String,
    ldigest: String,
    rdigest: String,
    car: String,
    machf: String,
    cdr: String,
}

impl fmt::Display for HumanizedMagic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let json =
            serde_json::to_string(&self.clone()).expect("failed to json-serialize HumanizedMagic");
        write!(f, "{}", json)
    }
}

impl fmt::Debug for HumanizedMagic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string_pretty(&self.clone())
            .expect("failed to json-serialize HumanizedMagic");
        write!(f, "{}", json)
    }
}

impl HumanizedMagic {
    pub fn from_meta(meta: MetaMagic) -> HumanizedMagic {
        return HumanizedMagic {
            tail_size: format!("{:x}", meta.tail_size.clone()),
            magic_size: format!("{:x}", meta.magic_size.clone()),
            magic: hex::encode(meta.magic()),
            mach0: hex::encode(meta.mach0()),
            odigest: hex::encode(meta.odigest()),
            ldigest: hex::encode(meta.ldigest()),
            rdigest: hex::encode(meta.rdigest()),
            car: hex::encode(meta.car()),
            machf: hex::encode(meta.machf()),
            cdr: hex::encode(meta.cdr()),
        };
    }
}

impl MetaMagic {
    pub fn new(input: Vec<u8>, magic: &str) -> MetaMagic {
        let magic = magic.clone().to_string();
        let bom = getmark();
        let magic_size: usize = magic.len();
        assert!(magic_size == 12);
        let bom_size: usize = bom.len();
        let input_size = input.len();
        assert!(bom_size == 4);

        let odigest = crc32(&input);
        let car = Vec::from(&input[..CAR_SIZE]);
        assert_eq!(car.len(), CAR_SIZE);

        let cdr = Vec::from(&input[CAR_SIZE..]);
        assert_eq!(cdr.len(), input_size - CAR_SIZE);

        let ldigest = crc32(&car);
        let rdigest = crc32(&cdr);

        return MetaMagic {
            tail_size: cdr.len(),
            magic_size: magic_size,
            magic: magic.into(),
            mach0: <[u8; 4]>::try_from(bom.clone()).unwrap(),
            odigest: <[u8; 4]>::try_from(odigest).unwrap(),
            ldigest: <[u8; 4]>::try_from(ldigest).unwrap(),
            rdigest: <[u8; 4]>::try_from(rdigest).unwrap(),
            car: <[u8; CAR_SIZE]>::try_from(car).unwrap(),
            machf: <[u8; 4]>::try_from(bom).unwrap(),
            cdr: cdr.clone(),
        };
    }
    pub fn from_enchanted(input: Vec<u8>, magic: &str) -> MetaMagic {
        let digest_size: usize = 4;
        let mut magic_size: usize = magic.len();
        assert!(magic_size == 12);

        let mut input = input.clone();
        magic_size = hex_to_usize(Vec::from([input.remove(0)]), 1).unwrap();
        assert_eq!(magic_size, 12);
        let magic_suffix = hex_to_usize(Vec::from([input.remove(0)]), 1).unwrap();
        assert_eq!(magic_suffix, 0x3d);
        let tail_size = hex_to_usize(Vec::from([input.remove(0)]), 1).unwrap();
        assert_eq!(tail_size, 50);
        let tail_suffix = hex_to_usize(Vec::from([input.remove(0)]), 1).unwrap();
        assert_eq!(tail_suffix, 0x24);

        let magic: Vec<u8> = Vec::from(&input[..magic_size]);
        let input = Vec::from(&input[magic_size..]);

        let mach0: Vec<u8> = Vec::from(&input[..digest_size]);
        let input = Vec::from(&input[digest_size..]);
        let odigest: Vec<u8> = Vec::from(&input[..digest_size]);
        let input = Vec::from(&input[digest_size..]);
        let ldigest: Vec<u8> = Vec::from(&input[..digest_size]);
        let input = Vec::from(&input[digest_size..]);
        let rdigest: Vec<u8> = Vec::from(&input[..digest_size]);
        let input = Vec::from(&input[digest_size..]);
        let car: Vec<u8> = Vec::from(&input[..CAR_SIZE]);
        let input = Vec::from(&input[CAR_SIZE..]);
        let machf: Vec<u8> = Vec::from(&input[..digest_size]);
        let input = Vec::from(&input[digest_size..]);

        return MetaMagic {
            tail_size: tail_size,
            magic_size: magic.len(),
            magic: magic.into(),
            mach0: <[u8; 4]>::try_from(mach0).unwrap(),
            odigest: <[u8; 4]>::try_from(odigest).unwrap(),
            ldigest: <[u8; 4]>::try_from(ldigest).unwrap(),
            rdigest: <[u8; 4]>::try_from(rdigest).unwrap(),
            car: <[u8; CAR_SIZE]>::try_from(car).unwrap(),
            machf: <[u8; 4]>::try_from(machf).unwrap(),
            cdr: Vec::from(input),
        };
    }

    pub fn humanized(&self) -> HumanizedMagic {
        HumanizedMagic::from_meta(self.clone())
    }
    pub fn magic(&self) -> Vec<u8> {
        self.magic.clone()
    }
    pub fn magic_size_hex(&self) -> Vec<u8> {
        return usize_to_hex(self.magic_size).expect("failed to convert magic_size to hex");
    }
    pub fn tail_size_hex(&self) -> Vec<u8> {
        return usize_to_hex(self.tail_size).expect("failed to convert tail_size to hex");
    }
    pub fn odigest(&self) -> Vec<u8> {
        self.odigest.clone().to_vec()
    }
    pub fn ldigest(&self) -> Vec<u8> {
        self.ldigest.clone().to_vec()
    }
    pub fn rdigest(&self) -> Vec<u8> {
        self.rdigest.clone().to_vec()
    }
    pub fn mach0(&self) -> Vec<u8> {
        self.mach0.clone().to_vec()
    }
    pub fn machf(&self) -> Vec<u8> {
        self.machf.clone().to_vec()
    }
    pub fn car(&self) -> Vec<u8> {
        self.car.clone().to_vec()
    }
    pub fn cdr(&self) -> Vec<u8> {
        self.cdr.clone().to_vec()
    }
    pub fn head(&self) -> Vec<u8> {
        let mut helmet: Vec<u8> = Vec::new();
        // magic size
        helmet.extend(self.magic_size_hex());
        helmet.push(0x3d); // magic size suffix/tail size prefix
                           // tail size
        helmet.extend(self.tail_size_hex());
        helmet.push(0x24); // tail size suffix
        helmet.extend(&self.magic()); // Magic
        helmet.extend(&self.mach0()); // Mach0
        helmet.extend(&self.odigest()); // ODigest
        helmet.extend(&self.ldigest()); // LDigest
        helmet.extend(&self.rdigest()); // RDigest
        helmet.extend(&self.car()); // ORIG
        helmet.extend(&self.machf()); // Mach1
        return helmet;
    }

    pub fn orig(&self) -> Vec<u8> {
        let mut realigned: Vec<u8> = Vec::new();
        realigned.extend(&self.car());
        realigned.extend(&self.cdr());
        return realigned.clone();
    }

    pub fn body(&self) -> Vec<u8> {
        return self.cdr.clone();
    }

    pub fn restore(&self) -> Vec<u8> {
        // let magic_chunk_sizes = [self.magic_size, 4, 4, 4, 4, CAR_SIZE];
        return Vec::new();
    }

    pub fn enchant(&self) -> Vec<u8> {
        let mut enchanted: Vec<u8> = Vec::new();
        enchanted.extend(&self.head());
        enchanted.extend(&self.body());
        return enchanted.clone();
    }
}
#[cfg(test)]
mod tests {
    use super::crc32;
    use super::MetaMagic;
    use super::{hex_to_usize, usize_to_hex};
    use k9::assert_equal;
    use std::error::Error;

    #[test]
    fn test_usize_to_hex() {
        assert_equal!(hex::encode(usize_to_hex(12).unwrap()), "0c");
    }
    #[test]
    fn test_hex_to_u8() -> Result<(), Box<dyn Error>> {
        // 0x89 == 137
        assert_equal!(hex_to_usize(test_data(), 2)?, 137);
        Ok(())
    }

    fn test_data() -> Vec<u8> {
        let data: Vec<u8> = Vec::from([
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
        ]);
        data
    }
    fn test_string(data: &str) -> Vec<u8> {
        data.clone().as_bytes().to_vec()
    }
    #[test]
    fn test_metamagic_eq() {
        let meta0 = MetaMagic::new(test_data(), "THISISMAGICO");
        let meta1 = MetaMagic::new(test_data(), "THISISMAGICO");

        assert_equal!(test_data().len(), 82);
        assert_equal!(meta0, meta1);
        assert_equal!(meta0.magic_size, 12);
        assert_equal!(meta0.magic_size_hex(), [0x0c]);
        assert_equal!(meta0.tail_size, 50);
        assert_equal!(meta0.tail_size_hex(), [0x32]);
        assert_equal!(hex::encode(meta0.magic()), hex::encode(meta1.magic()));
        assert_equal!(hex::encode(meta0.car()), hex::encode(meta1.car()));
        assert_equal!(hex::encode(meta0.body()), hex::encode(meta1.cdr()));
        assert_equal!(hex::encode(meta0.cdr()), hex::encode(meta1.body()));
        assert_equal!(hex::encode(meta0.cdr()), hex::encode(meta1.cdr()));
        assert_equal!(&hex::encode(meta0.magic()), "5448495349534d414749434f");
        assert_equal!(&hex::encode(meta0.odigest()), "e0ffc577");
        assert_equal!(&hex::encode(meta0.ldigest()), "47e70863");
        assert_equal!(&hex::encode(meta0.rdigest()), "9c7e1a7f");
        assert_equal!(
            &hex::encode(meta0.car()),
            "89504e470d0a1a0a0000000d494844520000000100000001080300000028cb34"
        );
        assert_equal!(&hex::encode(meta0.cdr()), "bb00000003504c5445ffffffa7c41bc80000000a4944415408996360000000020001f47164a60000000049454e44ae426082");
        assert_equal!(&hex::encode(meta0.head()), "0c3d32245448495349534d414749434fc3bec3bfe0ffc57747e708639c7e1a7f89504e470d0a1a0a0000000d494844520000000100000001080300000028cb34c3bec3bf");
        assert_equal!(&hex::encode(meta0.body()), "bb00000003504c5445ffffffa7c41bc80000000a4944415408996360000000020001f47164a60000000049454e44ae426082");
        assert_equal!(&hex::encode(meta0.enchant()), "0c3d32245448495349534d414749434fc3bec3bfe0ffc57747e708639c7e1a7f89504e470d0a1a0a0000000d494844520000000100000001080300000028cb34c3bec3bfbb00000003504c5445ffffffa7c41bc80000000a4944415408996360000000020001f47164a60000000049454e44ae426082");
    }

    #[test]
    fn test_metamagic_datum() {
        let magic = String::from("THISISMAGICO");
        let original = test_data();

        let meta = MetaMagic::new(original.clone(), &magic.clone());

        assert_equal!(meta.magic_size, 12);
        assert_equal!(meta.tail_size, 50);
        assert_equal!(
            hex::encode(meta.magic()),
            hex::encode(magic.as_bytes().to_vec())
        );
        assert_equal!(&hex::encode(meta.magic()), "5448495349534d414749434f");
        assert_equal!(&hex::encode(meta.mach0()), "c3bec3bf");
        assert_equal!(
            hex::encode(meta.odigest()),
            hex::encode(crc32(&original.clone()))
        );
        assert_equal!(&hex::encode(meta.odigest()), "e0ffc577");
        assert_equal!(&hex::encode(meta.ldigest()), "47e70863");
        assert_equal!(&hex::encode(meta.rdigest()), "9c7e1a7f");
        assert_equal!(meta.car(), original[..32]);
        assert_equal!(
            Vec::from([
                // magic_size
                0x0c, // magic_size suffix
                0x3d, // tail_size
                0x32, // tail_size suffix
                0x24, // <magic>
                0x54, 0x48, 0x49, 0x53, 0x49, 0x53, 0x4d, 0x41, 0x47, 0x49, 0x43, 0x4f,
                // <mach0>
                0xc3, 0xbe, 0xc3, 0xbf, // <odigest>
                0xe0, 0xff, 0xc5, 0x77, // <ldigest>
                0x47, 0xe7, 0x08, 0x63, // <rdigest>
                0x9c, 0x7e, 0x1a, 0x7f, // <car>
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x03, 0x00, 0x00,
                0x00, 0x28, 0xcb, 0x34, // <machf>
                0xc3, 0xbe, 0xc3, 0xbf,
            ]),
            meta.head()
        );
        assert_equal!(&hex::encode(meta.car()), &hex::encode(&original[..32]));
        assert_equal!(
            &hex::encode(meta.car()),
            "89504e470d0a1a0a0000000d494844520000000100000001080300000028cb34"
        );
        assert_equal!(meta.cdr(), original[32..]);
        assert_equal!(
            meta.cdr(),
            Vec::from([
                // <cdr>
                0xbb, 0x00, 0x00, 0x00, 0x03, 0x50, 0x4c, 0x54, 0x45, 0xff, 0xff, 0xff, 0xa7, 0xc4,
                0x1b, 0xc8, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x08, 0x99, 0x63, 0x60,
                0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xf4, 0x71, 0x64, 0xa6, 0x00, 0x00, 0x00, 0x00,
                0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
                // </cdr>
            ])
        );
        assert_equal!(&hex::encode(meta.cdr()), &hex::encode(&original[32..]));
        assert_equal!(&hex::encode(meta.cdr()), "bb00000003504c5445ffffffa7c41bc80000000a4944415408996360000000020001f47164a60000000049454e44ae426082");
        assert_equal!(&hex::encode(meta.machf()), "c3bec3bf");
    }
    #[test]
    fn test_enchant_string() {
        let ck = MetaMagic::new(
            test_string("ᎳᎡᎵᏓᎣᏅᎡ ᏔᎣ ᏅᏯ ᎳᎣᏛᎵᏗ, ᏔᎯᎡ ᎨᎡᎠᎤᏔᏯ ᎠᎾᏗ ᏔᎯᎡ ᎨᎠᎤᏗ"),
            "1CEB00DAFEFF",
        );
        assert_equal!(&hex::encode(ck.enchant()), "0c3d5224314345423030444146454646c3bec3bfe6be4e726bf0188194b224e3e18eb3e18ea1e18eb5e18f93e18ea3e18f85e18ea120e18f94e18ea320e18f85c3bec3bfe18faf20e18eb3e18ea3e18f9be18eb5e18f972c20e18f94e18eafe18ea120e18ea8e18ea1e18ea0e18ea4e18f94e18faf20e18ea0e18ebee18f9720e18f94e18eafe18ea120e18ea8e18ea0e18ea4e18f97");

        let ma = MetaMagic::new(
            test_string("њелцоме то мѕ њорлд, тхе беаутѕ анд тхе бауд"),
            "1CEB00DABA55",
        );
        assert_equal!(&hex::encode(ma.enchant()), "0c3d2f24314345423030444142413535c3bec3bfb4cc47bd6c6fecabac37934fd19ad0b5d0bbd186d0bed0bcd0b520d182d0be20d0bcd19520d19ad0bed180d0c3bec3bfbbd0b42c20d182d185d0b520d0b1d0b5d0b0d183d182d19520d0b0d0bdd0b420d182d185d0b520d0b1d0b0d183d0b4");

        let th = MetaMagic::new(
            test_string("ตยเลวสย รว ส่ ตวอเงะ รีย ทิย้ดร่ ท้คง รีย ทิ้ดง"),
            "B4BYL0N1AN42",
        );
        assert_equal!(&hex::encode(th.enchant()), "0c3d5d24423442594c304e31414e3432c3bec3bf75e551a112b79542489c61cde0b895e0b8a2e0b980e0b8a5e0b8a7e0b8aae0b8a220e0b8a3e0b8a720e0b8aac3bec3bfe0b98820e0b895e0b8a7e0b8ade0b980e0b887e0b8b020e0b8a3e0b8b5e0b8a220e0b897e0b8b4e0b8a2e0b989e0b894e0b8a3e0b98820e0b897e0b989e0b884e0b88720e0b8a3e0b8b5e0b8a220e0b897e0b8b4e0b989e0b894e0b887");
    }

    #[test]
    fn test_metamagic_restore() {
        let magic = String::from("THISISMAGICO");

        let meta0 = MetaMagic::new(test_data(), &magic.clone());
        let enchanted = meta0.enchant();

        assert_equal!(meta0.magic_size, 12);
        assert_equal!(meta0.tail_size, 50);

        let meta1 = MetaMagic::from_enchanted(enchanted, &magic.clone());
        assert_equal!(meta0, meta1);
    }
}
