use std::{marker::PhantomData, ops};
use enum_map::Enum;

pub trait EnumIndex {
    const LENGTH: usize;

    fn from_usize(value: usize) -> Self;
    fn into_usize(self) -> usize;
}

impl<T> EnumIndex for T where T: Enum {
    const LENGTH: usize = T::LENGTH;

    fn from_usize(value: usize) -> Self {
        T::from_usize(value)
    }

    fn into_usize(self) -> usize {
        self.into_usize()
    }
}

pub const fn bit_width<T: EnumIndex>() -> usize {
    <T as EnumIndex>::LENGTH
}

pub const fn byte_width<T: EnumIndex>() -> usize {
    ((bit_width::<T>() + 7) & !7) / 8
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct EnumBitSet<T: EnumIndex> where [(); byte_width::<T>()]: {
    pub bytes: [u8; byte_width::<T>()],
    phantom: PhantomData<T>,
}

impl<T: EnumIndex> EnumBitSet<T> where [(); byte_width::<T>()]: {
    pub const NUM_BITS: usize = bit_width::<T>();
    pub const NUM_BYTES: usize = byte_width::<T>();

    pub const TAIL_IDX: usize = Self::NUM_BYTES - 1;
    pub const HAS_PROPER_TAIL: bool = Self::NUM_BITS % 8 > 0;
    pub const TAIL_MASK: u8 = if Self::HAS_PROPER_TAIL {
        0xFF >> (8 - (Self::NUM_BITS % 8))
    } else {
        0xFF
    };

    #[inline(always)]
    pub fn new(init_state: bool) -> Self {
        if init_state {
            Self::all()
        } else {
            Self::none()
        }
    }

    pub fn all() -> Self {
        let mut bytes = [0xFF; byte_width::<T>()];
        bytes[Self::TAIL_IDX] = Self::TAIL_MASK;

        Self { bytes, phantom: PhantomData }
    }

    pub fn none() -> Self {
        Self { bytes: [0x00; byte_width::<T>()], phantom: PhantomData }
    }

    pub fn include(&self, key: T) -> bool {
        let (byte_idx, bit_idx) = Self::indices_for(key);

        (self.bytes[byte_idx] & (0x01 << bit_idx)) > 0
    }

    pub fn set(&mut self, key: T) {
        let (byte_idx, bit_idx) = Self::indices_for(key);

        self.bytes[byte_idx] |= 0x01 << bit_idx;
    }

    pub fn clear(&mut self, key: T) {
        let (byte_idx, bit_idx) = Self::indices_for(key);

        self.bytes[byte_idx] &= !(0x01 << bit_idx);
    }

    pub fn toggle(&mut self, key: T) {
        let (byte_idx, bit_idx) = Self::indices_for(key);

        self.bytes[byte_idx] ^= 0x01 << bit_idx;
    }

    pub fn set_all(&mut self) {
        for idx in 0..Self::TAIL_IDX {
            self.bytes[idx] = 0xFF;
        };

        self.bytes[Self::TAIL_IDX] = Self::TAIL_MASK;
    }

    pub fn clear_all(&mut self) {
        for idx in 0..=Self::TAIL_IDX {
            self.bytes[idx] = 0x00;
        };
    }

    pub fn toggle_all(&mut self) {
        for idx in 0..Self::TAIL_IDX {
            self.bytes[idx] ^= 0xFF;
        };

        self.bytes[Self::TAIL_IDX] ^= Self::TAIL_MASK;
    }

    pub fn all_set(&self) -> bool {
        for idx in 0..Self::TAIL_IDX {
            if self.bytes[idx] != 0xFF {
                return false;
            };
        };

        self.bytes[Self::TAIL_IDX] == Self::TAIL_MASK
    }

    pub fn none_set(&self) -> bool {
        for idx in 0..=Self::TAIL_IDX {
            if self.bytes[idx] != 0x00 {
                return false;
            };
        };

        true
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline(always)]
    fn indices_for(key: T) -> (usize, usize) {
        let idx = key.into_usize();

        (idx / 8, idx % 8)
    }
}

// Bit operation impls {{{
impl<T: EnumIndex> ops::BitAnd for EnumBitSet<T> where [(); byte_width::<T>()]: {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for idx in 0..Self::NUM_BYTES {
            self.bytes[idx] &= rhs.bytes[idx];
        };

        self
    }
}

impl<T: EnumIndex> ops::BitAndAssign for EnumBitSet<T> where [(); byte_width::<T>()]: {
    fn bitand_assign(&mut self, rhs: Self) {
        for idx in 0..Self::NUM_BYTES {
            self.bytes[idx] &= rhs.bytes[idx];
        }
    }
}

impl<T: EnumIndex> ops::BitOr for EnumBitSet<T> where [(); byte_width::<T>()]: {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        for idx in 0..Self::NUM_BYTES {
            self.bytes[idx] |= rhs.bytes[idx];
        };

        self
    }
}

impl<T: EnumIndex> ops::BitOrAssign for EnumBitSet<T> where [(); byte_width::<T>()]: {
    fn bitor_assign(&mut self, rhs: Self) {
        for idx in 0..Self::NUM_BYTES {
            self.bytes[idx] |= rhs.bytes[idx];
        }
    }
}

impl<T: EnumIndex> ops::BitXor for EnumBitSet<T> where [(); byte_width::<T>()]: {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        for idx in 0..Self::NUM_BYTES {
            self.bytes[idx] ^= rhs.bytes[idx];
        };

        self
    }
}

impl<T: EnumIndex> ops::BitXorAssign for EnumBitSet<T> where [(); byte_width::<T>()]: {
    fn bitxor_assign(&mut self, rhs: Self) {
        for idx in 0..Self::NUM_BYTES {
            self.bytes[idx] ^= rhs.bytes[idx];
        }
    }
}

impl<T: EnumIndex> ops::Not for EnumBitSet<T> where [(); byte_width::<T>()]: {
    type Output = Self;

    fn not(mut self) -> Self {
        self.toggle_all();
        self
    }
}
// }}}

// Macros {{{
#[macro_export]
macro_rules! enum_bit_set {
    (for $enum:ty) => {{
        $crate::collections::EnumBitSet::<$enum>::none()
    }};

    (for $enum:ty; $val:expr) => {{
        $crate::collections::EnumBitSet::<$enum>::new($val)
    }};

    ($($key:expr),* $(,)?; $val:expr) => {{
        let mut bit_set = $crate::collections::EnumBitSet::new(!($val));

        $(bit_set.toggle($key);)*

        bit_set
    }};
}
// }}}

#[cfg(test)]
mod tests {
    use enum_map::Enum;
    use crate::collections::EnumBitSet;

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
    pub enum Enum1 {
        A, B, C, D, E, F,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
    pub enum Enum2 {
        A, B, C, D, E, F, G, H,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
    pub enum Enum3 {
        A, B, C, D, E, F, G, H,
        I, J, K,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
    pub enum Enum4 {
        A, B, C, D, E, F, G, H,
        I, J, K, L, M, N, O, P,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug, Enum)]
    enum Letter {
        A, B, C, D, E, F, G, H,
        I, J, K, L, M, N, O, P,
        Q, R, S, T, U, V, W, X,
        Y, Z
    }

    #[test]
    fn constants() {
        assert_eq!(EnumBitSet::<Enum1>::NUM_BYTES, 1);
        assert_eq!(EnumBitSet::<Enum2>::NUM_BYTES, 1);
        assert_eq!(EnumBitSet::<Enum3>::NUM_BYTES, 2);
        assert_eq!(EnumBitSet::<Enum4>::NUM_BYTES, 2);

        assert_eq!(EnumBitSet::<Enum1>::HAS_PROPER_TAIL, true);
        assert_eq!(EnumBitSet::<Enum2>::HAS_PROPER_TAIL, false);
        assert_eq!(EnumBitSet::<Enum3>::HAS_PROPER_TAIL, true);
        assert_eq!(EnumBitSet::<Enum4>::HAS_PROPER_TAIL, false);

        assert_eq!(EnumBitSet::<Enum1>::TAIL_MASK, 0b0011_1111);
        assert_eq!(EnumBitSet::<Enum2>::TAIL_MASK, 0b1111_1111);
        assert_eq!(EnumBitSet::<Enum3>::TAIL_MASK, 0b0000_0111);
        assert_eq!(EnumBitSet::<Enum4>::TAIL_MASK, 0b1111_1111);
    }

    #[test]
    fn layout() {
        let bs1 = EnumBitSet::<Enum1>::new(true);
        let bs2 = EnumBitSet::<Enum2>::new(true);
        let bs3 = EnumBitSet::<Enum3>::new(true);
        let bs4 = EnumBitSet::<Enum4>::new(true);

        assert_eq!(bs1.as_bytes(), &[0b0011_1111]);
        assert_eq!(bs2.as_bytes(), &[0b1111_1111]);
        assert_eq!(bs3.as_bytes(), &[0b1111_1111, 0b0000_0111]);
        assert_eq!(bs4.as_bytes(), &[0b1111_1111, 0b1111_1111]);
    }

    #[test]
    fn macro_type_only() {
        let bs1 = enum_bit_set![for Letter; true];
        let bs2 = enum_bit_set![for Letter; false];

        assert_eq!(bs1.as_bytes(), &[0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0011]);
        assert_eq!(bs2.as_bytes(), &[0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000]);
    }

    #[test]
    fn macro_with_expr() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let bs1 = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b0101_1011, 0b0001_0011, 0b1010_1001, 0b0000_0001]
        let bs2 = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; false];

        assert_eq!(bs1.as_bytes(), &[0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]);
        assert_eq!(bs2.as_bytes(), &[0b0101_1011, 0b0001_0011, 0b1010_1001, 0b0000_0001]);
    }

    #[test]
    fn set() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        bs.set(Letter::I);
        bs.set(Letter::K);

        assert_eq!(bs.as_bytes(), &[0b1010_0100, 0b1110_1101, 0b0101_0110, 0b0000_0010]);
    }

    #[test]
    fn clear() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        bs.clear(Letter::I);
        bs.clear(Letter::K);

        assert_eq!(bs.as_bytes(), &[0b1010_0100, 0b1110_1000, 0b0101_0110, 0b0000_0010]);
    }

    #[test]
    fn toggle() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        bs.toggle(Letter::I);
        bs.toggle(Letter::K);

        assert_eq!(bs.as_bytes(), &[0b1010_0100, 0b1110_1001, 0b0101_0110, 0b0000_0010]);
    }

    #[test]
    fn set_all() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        bs.set_all();

        assert_eq!(bs.as_bytes(), &[0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0011]);
    }

    #[test]
    fn clear_all() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        bs.clear_all();

        assert_eq!(bs.as_bytes(), &[0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000]);
    }

    #[test]
    fn toggle_all() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        bs.toggle_all();

        assert_eq!(bs.as_bytes(), &[0b0101_1011, 0b0001_0011, 0b1010_1001, 0b0000_0001]);
    }

    #[test]
    fn bitand() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs1 = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1111_0000, 0b0011_1000, 0b0000_0001, 0b0000_0011]
        let bs2 = enum_bit_set![
            Letter::E, Letter::F, Letter::G, Letter::H,
            Letter::L, Letter::M, Letter::N,
            Letter::Q,
            Letter::Y, Letter::Z,
        ; true];

        let bs3 = bs1 & bs2;

        assert_eq!(bs1.as_bytes(), &[0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]);
        assert_eq!(bs3.as_bytes(), &[0b1010_0000, 0b0010_1000, 0b0000_0000, 0b0000_0010]);

        bs1 &= bs2;

        assert_eq!(bs1, bs3);
    }

    #[test]
    fn bitor() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs1 = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1111_0000, 0b0011_1000, 0b0000_0001, 0b0000_0011]
        let bs2 = enum_bit_set![
            Letter::E, Letter::F, Letter::G, Letter::H,
            Letter::L, Letter::M, Letter::N,
            Letter::Q,
            Letter::Y, Letter::Z,
        ; true];

        let bs3 = bs1 | bs2;

        assert_eq!(bs1.as_bytes(), &[0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]);
        assert_eq!(bs3.as_bytes(), &[0b1111_0100, 0b1111_1100, 0b0101_0111, 0b0000_0011]);

        bs1 |= bs2;

        assert_eq!(bs1, bs3);
    }

    #[test]
    fn bitxor() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let mut bs1 = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1111_0000, 0b0011_1000, 0b0000_0001, 0b0000_0011]
        let bs2 = enum_bit_set![
            Letter::E, Letter::F, Letter::G, Letter::H,
            Letter::L, Letter::M, Letter::N,
            Letter::Q,
            Letter::Y, Letter::Z,
        ; true];

        let bs3 = bs1 ^ bs2;

        assert_eq!(bs1.as_bytes(), &[0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]);
        assert_eq!(bs3.as_bytes(), &[0b0101_0100, 0b1101_0100, 0b0101_0111, 0b0000_0001]);

        bs1 ^= bs2;

        assert_eq!(bs1, bs3);
    }

    #[test]
    fn not() {
        //    HGFE DCBA    PONM LKJI    XWVU TSRQ           ZY
        // [0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]
        let bs1 = enum_bit_set![
            Letter::C, Letter::F, Letter::H,
            Letter::K, Letter::L, Letter::N, Letter::O, Letter::P,
            Letter::R, Letter::S, Letter::U, Letter::W,
            Letter::Z,
        ; true];

        let bs2 = !bs1;

        assert_eq!(bs1.as_bytes(), &[0b1010_0100, 0b1110_1100, 0b0101_0110, 0b0000_0010]);
        assert_eq!(bs2.as_bytes(), &[0b0101_1011, 0b0001_0011, 0b1010_1001, 0b0000_0001]);
    }
}
