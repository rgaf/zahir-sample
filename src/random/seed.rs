use byteorder::{ByteOrder, LittleEndian};
use rand::{Rng, rngs::OsRng};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C, align(8))]
pub struct Seed {
    pub bytes: [u8; Self::NUM_BYTES],
}

impl Seed {
    const NUM_BYTES: usize = 32;
    const STR_LEN: usize = 44;

    pub const DEFAULT_SEED: Self = Self::from_bytes([
        0x12, 0xE3, 0xD9, 0x45, 0x3C, 0x6A, 0xBB, 0x33,
        0x9D, 0x6B, 0x2E, 0x6F, 0x53, 0x98, 0x7E, 0x7A,
        0xE8, 0xA3, 0x09, 0xE0, 0x8E, 0xA7, 0x39, 0xEF,
        0xF0, 0x3D, 0x48, 0x62, 0xB3, 0xED, 0x97, 0x80
    ]);

    pub const fn from_bytes(bytes: [u8; Self::NUM_BYTES]) -> Self {
        Self { bytes }
    }

    pub fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut bytes = [0; Self::NUM_BYTES];
        rng.fill_bytes(&mut bytes);

        Self { bytes }
    }

    pub fn from_entropy() -> Self {
        Self::from_rng(&mut OsRng)
    }

    pub fn to_base58(&self) -> String {
        fn digit_to_char(digit: u64) -> char {
            let ord = match digit {
                0..=8   => digit + 49,
                9..=16  => digit + 56,
                17..=21 => digit + 57,
                22..=32 => digit + 58,
                33..=43 => digit + 64,
                44..=57 => digit + 65,
                _ => panic!("Unexpected base-58 digit: {}", digit),
            };

            ord as u8 as char
        }

        let mut buffer = Vec::<char>::with_capacity(Self::STR_LEN);

        let mut x = LittleEndian::read_u64(&self.bytes[0..]);
        let mut y = LittleEndian::read_u64(&self.bytes[8..]);
        let mut z = LittleEndian::read_u64(&self.bytes[16..]);
        let mut w = LittleEndian::read_u64(&self.bytes[24..]);

        loop {
            let m = u128::from(w);
            let (mw, rem) = ((m / 58) as u64, (m % 58) as u64);

            let m = u128::from(z) + (u128::from(rem) << 64);
            let (mz, rem) = ((m / 58) as u64, (m % 58) as u64);

            let m = u128::from(y) + (u128::from(rem) << 64);
            let (my, rem) = ((m / 58) as u64, (m % 58) as u64);

            let m = u128::from(x) + (u128::from(rem) << 64);
            let (mx, rem) = ((m / 58) as u64, (m % 58) as u64);

            buffer.push(digit_to_char(rem));

            if mx < 58 && my == 0 && mz == 0 && mw == 0 {
                if mx > 0 {
                    buffer.push(digit_to_char(mx));
                };

                break;
            } else {
                (x, y, z, w) = (mx, my, mz, mw);
            };
        };

        while buffer.len() < Self::STR_LEN {
            buffer.push('1');
        };

        buffer.into_iter().rev().collect()
    }

    pub fn from_base58(s: &str) -> Result<Self, ParseSeedError> {
        if s.len() != Self::STR_LEN {
            return Err(ParseSeedError::InvalidLength { length: s.len() });
        };

        let mut sx: u64 = 0;
        let mut sy: u64 = 0;
        let mut sz: u64 = 0;
        let mut sw: u64 = 0;

        let mut bx: u64 = 58;
        let mut by: u64 = 0;
        let mut bz: u64 = 0;
        let mut bw: u64 = 0;

        for (idx, c) in s.chars().rev().enumerate() {
            let char_ord = c as u64;
            let char_value = match char_ord {
                49..=57     => char_ord - 49,
                65..=72     => char_ord - 56,
                74..=78     => char_ord - 57,
                80..=90     => char_ord - 58,
                97..=107    => char_ord - 64,
                109..=122   => char_ord - 65,
                _           => {
                    return Err(ParseSeedError::InvalidCharacter {
                        character: c,
                        index: Self::STR_LEN - idx - 1,
                    });
                },
            };

            if idx == 0 {
                sx = char_value;
                continue;
            };

            if idx != 1 {
                (bx, by, bz, bw) = mul_u256(58, bx, by, bz, bw)?;
            };

            let (ix, iy, iz, iw) = mul_u256(char_value, bx, by, bz, bw)?;

            let m = u128::from(sx) + u128::from(ix);
            let (mx, carry) = (m as u64, m >> 64);

            let m = u128::from(sy) + u128::from(iy) + carry;
            let (my, carry) = (m as u64, m >> 64);

            let m = u128::from(sz) + u128::from(iz) + carry;
            let (mz, carry) = (m as u64, m >> 64);

            let m = u128::from(sw) + u128::from(iw) + carry;
            let (mw, carry) = (m as u64, m >> 64);

            if carry > 0 {
                return Err(ParseSeedError::Overflow);
            } else {
                (sx, sy, sz, sw) = (mx, my, mz, mw);
            };
        };

        let mut seed = Self { bytes: [0; Self::NUM_BYTES] };

        LittleEndian::write_u64(&mut seed.bytes[ 0..], sx);
        LittleEndian::write_u64(&mut seed.bytes[ 8..], sy);
        LittleEndian::write_u64(&mut seed.bytes[16..], sz);
        LittleEndian::write_u64(&mut seed.bytes[24..], sw);

        Ok(seed)
    }
}

fn mul_u256(
    multiplicand: u64,
    x: u64,
    y: u64,
    z: u64,
    w: u64,
) -> Result<(u64, u64, u64, u64), ParseSeedError> {
    let multiplicand = u128::from(multiplicand);

    let m = u128::from(x) * multiplicand;
    let (x, carry) = (m as u64, m >> 64);

    let m = u128::from(y) * multiplicand + carry;
    let (y, carry) = (m as u64, m >> 64);

    let m = u128::from(z) * multiplicand + carry;
    let (z, carry) = (m as u64, m >> 64);

    let m = u128::from(w) * multiplicand + carry;
    let (w, carry) = (m as u64, m >> 64);

    if carry > 0 {
        Err(ParseSeedError::Overflow)
    } else {
        Ok((x, y, z, w))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ParseSeedError {
    InvalidLength { length: usize },
    InvalidCharacter { character: char, index: usize },
    Overflow,
}

#[cfg(test)]
mod tests {
    use crate::random::{ParseSeedError, Seed};

    const MIN_SEED: Seed = Seed { bytes: [u8::MIN; Seed::NUM_BYTES] };
    const MAX_SEED: Seed = Seed { bytes: [u8::MAX; Seed::NUM_BYTES] };

    const DEFAULT_STR: &'static str = "9eyYzoXRx7wTVRon6sF2EWNBUcg4bXBZQbV2dJrEq7A1";
    const MIN_STR: &'static str = "11111111111111111111111111111111111111111111";
    const MAX_STR: &'static str = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG";

    #[test]
    fn to_base58() {
        assert_eq!(Seed::DEFAULT_SEED.to_base58(), DEFAULT_STR);
        assert_eq!(MIN_SEED.to_base58(), MIN_STR);
        assert_eq!(MAX_SEED.to_base58(), MAX_STR);
    }

    #[test]
    fn from_base58() {
        assert_eq!(Seed::from_base58(DEFAULT_STR), Ok(Seed::DEFAULT_SEED));
        assert_eq!(Seed::from_base58(MIN_STR), Ok(MIN_SEED));
        assert_eq!(Seed::from_base58(MAX_STR), Ok(MAX_SEED));
    }

    #[test]
    fn from_base58_overflow() {
        let result = Seed::from_base58("JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFH");
        let expected = Err(ParseSeedError::Overflow);

        assert_eq!(result, expected);
    }

    #[test]
    fn from_base58_invalid_character() {
        let result = Seed::from_base58("2GjrFRmuRWVenL0TtQaZupNoTGvEa8DipMyhdyNYxcko");
        let expected = Err(ParseSeedError::InvalidCharacter { character: '0', index: 14 });

        assert_eq!(result, expected);
    }
}
