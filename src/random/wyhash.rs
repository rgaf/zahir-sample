use byteorder::{ByteOrder, LittleEndian};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use crate::random::{HashFn, Seed};


#[derive(Clone, Debug)]
pub struct Wyhash {
    seed: u64,
    secret: [u64; 4],
}

impl Wyhash {
    #[inline(always)]
    fn wymum(x: u64, y: u64) -> (u64, u64) {
        let (lo, hi) = x.widening_mul(y);

        (x ^ lo, y ^ hi)
    }

    #[inline(always)]
    fn wymix(x: u64, y: u64) -> u64 {
        let (lo, hi) = Self::wymum(x, y);

        lo ^ hi
    }

    #[inline(always)]
    fn finish(&self, lhs: u64, rhs: u64, seed: u64, len: u64) -> u64 {
        let (lhs, rhs) = Self::wymum(lhs ^ self.secret[1], rhs ^ seed);

        Self::wymix(lhs ^ self.secret[0] ^ (len as u64), rhs ^ self.secret[1])
    }
}

impl HashFn for Wyhash {
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let seed = rng.next_u64();
        let secret_0 = rng.next_u64();
        let secret_1 = rng.next_u64();
        let secret_2 = rng.next_u64();
        let secret_3 = rng.next_u64();

        Self {
            seed: seed ^ Self::wymix(seed ^ secret_0, secret_1),
            secret: [
                secret_0,
                secret_1,
                secret_2,
                secret_3,
            ],
        }
    }

    fn from_seed(seed: &Seed) -> Self {
        use crate::random::Seedable;

        let mut rng = ChaCha8Rng::from_seed(seed);

        Self::from_rng(&mut rng)
    }

    fn hash_1u32(&self, x: u32) -> u64 {
        let mix = ((x as u64) << 32) | (x as u64);

        self.finish(mix, mix, self.seed, 4)
    }

    fn hash_2u32(&self, x: u32, y: u32) -> u64 {
        let lhs = ((x as u64) << 32) | (y as u64);
        let rhs = ((y as u64) << 32) | (x as u64);

        self.finish(lhs, rhs, self.seed, 8)
    }

    fn hash_3u32(&self, x: u32, y: u32, z: u32) -> u64 {
        let lhs = ((x as u64) << 32) | (y as u64);
        let rhs = ((z as u64) << 32) | (y as u64);

        self.finish(lhs, rhs, self.seed, 12)
    }

    fn hash_4u32(&self, x: u32, y: u32, z: u32, w: u32) -> u64 {
        let lhs = ((x as u64) << 32) | (z as u64);
        let rhs = ((w as u64) << 32) | (y as u64);

        self.finish(lhs, rhs, self.seed, 16)
    }

    fn hash_1u64(&self, x: u64) -> u64 {
        let lhs = x.rotate_left(32);
        let rhs = x;

        self.finish(lhs, rhs, self.seed, 8)
    }

    fn hash_2u64(&self, x: u64, y: u64) -> u64 {
        let lhs = (x << 32) | (y & 0x0000_0000_FFFF_FFFF);
        let rhs = (y & 0xFFFF_FFFF_0000_0000) | (x >> 32);

        self.finish(lhs, rhs, self.seed, 16)
    }

    fn hash_3u64(&self, x: u64, y: u64, z: u64) -> u64 {
        let seed = Self::wymix(x ^ self.secret[1], y ^ self.seed);

        self.finish(y, z, seed, 24)
    }

    fn hash_4u64(&self, x: u64, y: u64, z: u64, w: u64) -> u64 {
        let seed = Self::wymix(x ^ self.secret[1], y ^ self.seed);

        self.finish(z, w, seed, 32)
    }

    fn hash_bytes(&self, bytes: &[u8]) -> u64 {
        let len: usize = bytes.len();

        let mut seed = self.seed;
        let mut lhs: u64 = 0;
        let mut rhs: u64 = 0;

        if len <= 16 {
            if len >= 4 {
                let offset_1 = (len >> 3) << 2;
                let offset_2 = len - 4;
                let offset_3 = offset_2 - offset_1;

                let x = LittleEndian::read_u32(&bytes[0..]) as u64;
                let y = LittleEndian::read_u32(&bytes[offset_1..]) as u64;
                let z = LittleEndian::read_u32(&bytes[offset_2..]) as u64;
                let w = LittleEndian::read_u32(&bytes[offset_3..]) as u64;

                lhs = (x << 32) | y;
                rhs = (z << 32) | w;
            } else if len > 0 {
                let x = bytes[0] as u64;
                let y = bytes[len >> 1] as u64;
                let z = bytes[len - 1] as u64;

                lhs = (x << 16) | (y << 8) | z;
            };
        } else {
            let mut remaining = len;
            let mut offset = 0;

            if len >= 48 {
                let mut chunk = [0u64; 6];
                let mut seed_1 = seed;
                let mut seed_2 = seed;

                while remaining >= 48 {
                    LittleEndian::read_u64_into(&bytes[offset..(offset + 48)], &mut chunk);

                    seed   = Self::wymix(chunk[0] ^ self.secret[1], chunk[1] ^ seed);
                    seed_1 = Self::wymix(chunk[2] ^ self.secret[2], chunk[3] ^ seed_1);
                    seed_2 = Self::wymix(chunk[4] ^ self.secret[3], chunk[5] ^ seed_2);

                    remaining -= 48;
                    offset += 48;
                };

                seed ^= seed_1;
                seed ^= seed_2;
            };

            while remaining > 16 {
                let x = LittleEndian::read_u64(&bytes[offset..]);
                let y = LittleEndian::read_u64(&bytes[(offset + 8)..]);

                seed = Self::wymix(x ^ self.secret[1], y ^ seed);

                remaining -= 16;
                offset += 16;
            };

            lhs = LittleEndian::read_u64(&bytes[(offset + remaining - 16)..]);
            rhs = LittleEndian::read_u64(&bytes[(offset + remaining - 8)..]);
        };

        self.finish(lhs, rhs, seed, len as u64)
    }
}

#[cfg(test)]
mod tests {
    use byteorder::{ByteOrder, LittleEndian};
    use crate::random::{HashFn, Wyhash};

    fn build_wyhash(seed: u64) -> Wyhash {
        Wyhash {
            seed: seed ^ Wyhash::wymix(seed ^ 0x2D358DCCAA6C78A5, 0x8BB84B93962EACC9),
            secret: [
                0x2D358DCCAA6C78A5,
                0x8BB84B93962EACC9,
                0x4B33A62ED433D4A3,
                0x4D5A2DA51DE1AA47,
            ],
        }
    }

    #[test]
    fn hash_1u32() {
        let wyhash = build_wyhash(0);

        let x: u32 = 0x125B9188;
        let mut bytes = [0u8; 4];

        LittleEndian::write_u32(&mut bytes[0..], x);

        let result = wyhash.hash_1u32(x);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_2u32() {
        let wyhash = build_wyhash(0);

        let x: u32 = 0x125B9188;
        let y: u32 = 0x7A6A7E34;
        let mut bytes = [0u8; 8];

        LittleEndian::write_u32(&mut bytes[0..], x);
        LittleEndian::write_u32(&mut bytes[4..], y);

        let result = wyhash.hash_2u32(x, y);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_3u32() {
        let wyhash = build_wyhash(0);

        let x: u32 = 0x125B9188;
        let y: u32 = 0x7A6A7E34;
        let z: u32 = 0x91A79E7E;
        let mut bytes = [0u8; 12];

        LittleEndian::write_u32(&mut bytes[0..], x);
        LittleEndian::write_u32(&mut bytes[4..], y);
        LittleEndian::write_u32(&mut bytes[8..], z);

        let result = wyhash.hash_3u32(x, y, z);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_4u32() {
        let wyhash = build_wyhash(0);

        let x: u32 = 0x125B9188;
        let y: u32 = 0x7A6A7E34;
        let z: u32 = 0x91A79E7E;
        let w: u32 = 0xFAE79714;
        let mut bytes = [0u8; 16];

        LittleEndian::write_u32(&mut bytes[ 0..], x);
        LittleEndian::write_u32(&mut bytes[ 4..], y);
        LittleEndian::write_u32(&mut bytes[ 8..], z);
        LittleEndian::write_u32(&mut bytes[12..], w);

        let result = wyhash.hash_4u32(x, y, z, w);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_1u64() {
        let wyhash = build_wyhash(0);

        let x: u64 = 0x7A6A7E34125B9188;
        let mut bytes = [0u8; 8];

        LittleEndian::write_u64(&mut bytes[0..], x);

        let result = wyhash.hash_1u64(x);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_2u64() {
        let wyhash = build_wyhash(0);

        let x: u64 = 0x7A6A7E34125B9188;
        let y: u64 = 0xFAE7971491A79E7E;
        let mut bytes = [0u8; 16];

        LittleEndian::write_u64(&mut bytes[0..], x);
        LittleEndian::write_u64(&mut bytes[8..], y);

        let result = wyhash.hash_2u64(x, y);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_3u64() {
        let wyhash = build_wyhash(0);

        let x: u64 = 0x7A6A7E34125B9188;
        let y: u64 = 0xFAE7971491A79E7E;
        let z: u64 = 0x8DD9BB8734213A60;
        let mut bytes = [0u8; 24];

        LittleEndian::write_u64(&mut bytes[ 0..], x);
        LittleEndian::write_u64(&mut bytes[ 8..], y);
        LittleEndian::write_u64(&mut bytes[16..], z);

        let result = wyhash.hash_3u64(x, y, z);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_4u64() {
        let wyhash = build_wyhash(0);

        let x: u64 = 0x7A6A7E34125B9188;
        let y: u64 = 0xFAE7971491A79E7E;
        let z: u64 = 0x8DD9BB8734213A60;
        let w: u64 = 0xE30DCD7F08BF2369;
        let mut bytes = [0u8; 32];

        LittleEndian::write_u64(&mut bytes[ 0..], x);
        LittleEndian::write_u64(&mut bytes[ 8..], y);
        LittleEndian::write_u64(&mut bytes[16..], z);
        LittleEndian::write_u64(&mut bytes[24..], w);

        let result = wyhash.hash_4u64(x, y, z, w);
        let expected = wyhash.hash_bytes(&bytes);

        assert_eq!(result, expected);
    }

    #[test]
    fn hash_bytes() {
        let wyhash = build_wyhash(0);
        let expected: [u64; 121] = [
            0x4C91B2FDB699FF5F,

            0x225DD3B25A4172AF, 0xD209885DE6C4146C, 0x4D4022C278AB4A60, 0xB2EC605A12DAD30F,
            0xD455230ADF70CF51, 0xBAC58D41FA541D57, 0x2253FA5AA481140D, 0xC86120583C64F685,
            0xE2D35682C50DE559, 0x4FB6E09528D8AF1D, 0xB40DBE3C10D6E5BD, 0xA4486169E7CE6105,
            0x6B4371F64D79454A, 0x0D97E23FEDB15466, 0x7642A92B46E0F625, 0xA48FF73120B09FEA,

            0xB05BF07F3F7FCF72, 0x1B7A5123A0CCF81A, 0x1077BF052393989E, 0x03F8FA9F4D2F9134,
            0xCAFE82FAEAB14887, 0x257C9111A2F04DE5, 0x3AB287966E4C3121, 0xC4D7033061FC6AF7,
            0x4B7DE72FB2BD532F, 0x5B42C3EE81BB21B7, 0xE85DD87C46C3A30C, 0x2242121B9556978C,
            0x2DADFC763C8DCFFE, 0xF4A04BEA7387CD3B, 0xA9F45A8B02E7A194, 0x02E9ED4F9CE1C724,

            0x4360BCC32C395F2F, 0x50CF7D3D5554B019, 0xCD561383E3FDDDC2, 0x14A6B5A45C2749B0,
            0x739D2F5A49FEE038, 0x0C539FF4CE2AB8C8, 0x2B94F79049D0AD0D, 0x217757321A914615,
            0x008286EB00B5677D, 0xF4CD61B7B9662463, 0xFFC3DDF9F5D3C2F6, 0x857DAC37A683F5FF,
            0x8B8E09D26B3432C3, 0x82F51F2BD0A9C8C7, 0xA4C7AD773FF2BD5E, 0x5A7355930C1B2063,

            0x58CA038714063EE3, 0x1AA09D9291324689, 0x842F493B7E493AF9, 0xC8A3F61F52698BA0,
            0xCCB686CDA9F90E00, 0x4BF4A12D0F5DB1DC, 0xD0569CC3CD6884CA, 0xB49C4E21779FCF5F,
            0x9BF8707D4B32BCE8, 0xCC89111C6B20900C, 0x2BBAB11A36788580, 0x838D74A1C5C1989F,
            0x9310F7D335097F93, 0x42E10A1CFBEF5D33, 0xF5ECC5A0E69FD5A4, 0x987C1F7FEBB17D9F,

            0x775424264DB45B70, 0xCAA9EE19DB4421DF, 0x8B4627591CBD4C1C, 0xFC8E12714EAEED42,
            0xEF14B4D77E5CDADA, 0xFB0749562FBA59A2, 0xC6DC0FAB24659D98, 0xB1EE829D7D468BCC,
            0x471C3FA29E4B82A1, 0x603ABC3EDC4688C2, 0x0D64D76F1DA7B7A9, 0x8033980E44D3A881,
            0x9465F9A1AB18792F, 0x4CE39ED87D0A0268, 0xF3F9599112F71BA3, 0xAF1F4A91953870E2,

            0x8145384E44FF5D48, 0xDE60E8E3E19C086F, 0xEB84EEE257CCC60C, 0xF0986C57B2CBD04A,
            0xAA58EC55D83F9D5C, 0xE6913D8A2E46C8D2, 0xD040862626E2E3E1, 0xB5771755975D2D5A,
            0x12B6A6979475A8DB, 0x18C77DD2BD51CBA6, 0xF0203BD355FF64BC, 0x9B1AA050289DC429,
            0xCB01C2B17EF2D1BE, 0x3CA8F0063E563DC0, 0x9A018FD075F000D9, 0xF06B2D49F7B391D3,

            0x85992F6E50311585, 0xB7B32188087BE782, 0x17071D1AFEB5A368, 0xFE313EB8E2B4C09C,
            0xA6C6216B65D08DD0, 0xF2BB16E761D41F73, 0x0B6DF943881BB360, 0x0C78FA4BD5124F27,
            0xAF51186DD7F02DE4, 0x12CF5A1C1B04492C, 0x23B5DC072AB3F6D7, 0x74F47589FA57AB2A,
            0x8E59074F6F142FA9, 0x585071EBEEE29E44, 0xDA825E2A75EC8FCF, 0x19EE2B8871029B8D,

            0x6ABF21F2867C4872, 0xB98070D385512B0C, 0xA417CBEA8C353F7E, 0x8F692229421EAB24,
            0x3E8D482A7FA5BFA9, 0xA649BD8A2574A32F, 0x6F011913AF8E737B, 0x01E9A4A61F99CFEA,
        ];

        let bytes: [u8; 120] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,

            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,

            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,

            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
            0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,

            0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47,
            0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,

            0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57,
            0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,

            0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
            0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F,

            0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        ];

        for n in 0..=120 {
            let result = wyhash.hash_bytes(&bytes[0..n]);

            assert_eq!(result, expected[n]);
        };
    }

    #[test]
    fn wyhash_message_0() {
        let wyhash = build_wyhash(0);
        let msg = "";

        let expected = 0x4C91B2FDB699FF5F;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn wyhash_message_1() {
        let wyhash = build_wyhash(1);
        let msg = "a";

        let expected = 0x9165230AED21D083;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn wyhash_message_2() {
        let wyhash = build_wyhash(2);
        let msg = "abc";

        let expected = 0xBA31EE45A25CB04F;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn wyhash_message_3() {
        let wyhash = build_wyhash(3);
        let msg = "message digest";

        let expected = 0x88AAF4CC14F9B6C9;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn wyhash_message_4() {
        let wyhash = build_wyhash(4);
        let msg = "abcdefghijklmnopqrstuvwxyz";

        let expected = 0x94725703203B71A5;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn wyhash_message_5() {
        let wyhash = build_wyhash(5);
        let msg = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let expected = 0x416BE917B4AD661A;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn wyhash_message_6() {
        let wyhash = build_wyhash(6);
        let msg = "1234567890123456789012345678901234567890\
                   1234567890123456789012345678901234567890";

        let expected = 0x899DEE2A86A08A8B;
        let result = wyhash.hash_bytes(msg.as_bytes());

        assert_eq!(result, expected);
    }
}
