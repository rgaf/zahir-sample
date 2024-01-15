use std::ops::{AddAssign, Range, RangeInclusive};
use num_traits::{Float, PrimInt};
use rand::{Rng, RngCore, distributions::uniform::{UniformSampler, SampleUniform}};

// TODO: Remove Float & PrimInt; should be more specific

pub enum ExplodeResult<T> {
    Hit(T),
    Miss(T),
}

pub trait SampleExtendedRngRange<T> {
    fn sample_single<R: RngCore + ?Sized>(&self, rng: &mut R) -> T;
    fn sample_explosion<R: RngCore + ?Sized>(&self, rng: &mut R, target: T) -> ExplodeResult<T>;
    fn is_empty(&self) -> bool;
    fn max_value(&self) -> T;
}

impl<T: PrimInt + SampleUniform> SampleExtendedRngRange<T> for Range<T> {
    #[inline]
    fn sample_single<R: RngCore + ?Sized>(&self, rng: &mut R) -> T {
        T::Sampler::sample_single(&self.start, &self.end, rng)
    }

    #[inline]
    fn sample_explosion<R: RngCore + ?Sized>(&self, rng: &mut R, target: T) -> ExplodeResult<T> {
        let result = T::Sampler::sample_single(&self.start, &self.end, rng);

        if result >= target {
            ExplodeResult::Hit(result)
        } else {
            ExplodeResult::Miss(result)
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }

    #[inline]
    fn max_value(&self) -> T {
        self.end - T::one()
    }
}

impl<T: PrimInt + SampleUniform> SampleExtendedRngRange<T> for RangeInclusive<T> {
    #[inline]
    fn sample_single<R: RngCore + ?Sized>(&self, rng: &mut R) -> T {
        T::Sampler::sample_single_inclusive(self.start(), self.end(), rng)
    }

    #[inline]
    fn sample_explosion<R: RngCore + ?Sized>(&self, rng: &mut R, target: T) -> ExplodeResult<T> {
        let result = T::Sampler::sample_single_inclusive(self.start(), self.end(), rng);

        if result >= target {
            ExplodeResult::Hit(result)
        } else {
            ExplodeResult::Miss(result)
        }
    }

    #[inline]
    fn is_empty(&self) -> bool {
        !(self.start() <= self.end())
    }

    #[inline]
    fn max_value(&self) -> T {
        *self.end()
    }
}

pub trait ExtendedRng: Rng {
    /// Calls within a range multiple times, summing the result.
    ///
    /// A dice roll of 3d6 could be simulated with `multi_gen_range(3, 1..=6)`.
    fn multi_gen_range<T, R>(&mut self, mut num_rolls: usize, range: R) -> T
    where T: AddAssign + PrimInt + SampleUniform, R: SampleExtendedRngRange<T> {
        let mut total = T::zero();

        while num_rolls > 0 {
            let roll = range.sample_single(self);
            total += roll;
            num_rolls -= 1;
        };

        total
    }

    fn gen_explodable_range<T, R>(&mut self, range: R, target: T, mut max_depth: usize) -> T
    where T: AddAssign + PrimInt + SampleUniform, R: SampleExtendedRngRange<T> {
        let mut total = T::zero();

        loop {
            match range.sample_explosion(self, target) {
                ExplodeResult::Hit(result) => {
                    if max_depth == 0 {
                        return total + result;
                    } else {
                        total += result;
                        max_depth -= 1;
                    };
                },

                ExplodeResult::Miss(result) => {
                    return total + result;
                },
            };
        };
    }

    fn multi_gen_explodable_range<T, R>(
        &mut self,
        num_rolls: usize,
        range: R,
        target: T,
        mut max_depth: usize,
    ) -> T where T: AddAssign + PrimInt + SampleUniform, R: SampleExtendedRngRange<T> {
        let mut total = T::zero();

        loop {
            let mut local_total = T::zero();
            let mut local_num_rolls = num_rolls;

            while local_num_rolls > 0 {
                local_total += range.sample_single(self);
                local_num_rolls -= 1;
            };

            total += local_total;

            if max_depth == 0 || local_total < target {
                break;
            } else {
                max_depth -= 1;
            };
        };

        total
    }

    /// Randomly rounds a number to one of the two integer values bounding it, with the chance
    /// determined by proximity.
    ///
    /// For instance, `3.726` would have a 72.6% chance to be rounded to `4`, and a 17.4% chance to
    /// be rounded to `3`.
    fn round<F, N>(&mut self, fp_value: F) -> Option<N> where F: Float, N: PrimInt {
        let trunc: N = N::from(fp_value.trunc())?;
        let fract = fp_value.fract().abs().to_f64()?;

        if self.gen_bool(fract) {
            if fp_value.is_sign_negative() {
                Some(trunc - N::one())
            } else {
                Some(trunc + N::one())
            }
        } else {
            Some(trunc)
        }
    }
}

impl<R: Rng + ?Sized> ExtendedRng for R {}

#[cfg(test)]
mod tests {
    use rand_chacha::ChaCha8Rng;
    use crate::random::{ExtendedRng, Seed, Seedable};

    fn build_rng(seed_str: &str) -> ChaCha8Rng {
        let seed = Seed::from_base58(seed_str).unwrap();

        ChaCha8Rng::from_seed(&seed)
    }

    #[test]
    fn multi_gen_range() {
        // Series: 6, 6, 4, 5, 6, 3, 4, 1, 2, 3, 6, 1, ...
        let mut rng = build_rng("Fx68WQF2NbDPkBQiwzsmQXNFBKxGt4BgAw5jJaLfnSq5");

        rng.set_word_pos(0);
        let result: u32 = rng.multi_gen_range(3, 1..=6); // 3d6
        let expected = 16; // 6 + 6 + 4

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.multi_gen_range(10, 1..=6); // 10d6
        let expected = 40; // 6 + 6 + 4 + 5 + 6 + 3 + 4 + 1 + 2 + 3

        assert_eq!(result, expected);
    }

    #[test]
    fn gen_explodable_range() {
        // Series: 6, 6, 4, 5, 6, 3, 4, 1, 2, 3, 6, 1, ...
        let mut rng = build_rng("Fx68WQF2NbDPkBQiwzsmQXNFBKxGt4BgAw5jJaLfnSq5");

        rng.set_word_pos(0);
        let result: u32 = rng.gen_explodable_range(1..=6, 6, 100);
        let expected = 16; // 6 + 6 + 4 [miss]

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.gen_explodable_range(1..=6, 4, 100);
        let expected = 30; // 6 + 6 + 4 + 5 + 6 + 3 [miss]

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.gen_explodable_range(1..=6, 4, 3);
        let expected = 21; // 6 + 6 + 4 + 5 [cutoff]

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.gen_explodable_range(1..=6, 6, 0);
        let expected = 6; // 6 [cutoff]

        assert_eq!(result, expected);
    }

    #[test]
    fn multi_gen_explodable_range() {
        // Series: 6, 6, 4, 5, 6, 3, 4, 1, 2, 3, 6, 1, ...
        let mut rng = build_rng("Fx68WQF2NbDPkBQiwzsmQXNFBKxGt4BgAw5jJaLfnSq5");

        rng.set_word_pos(0);
        let result: u32 = rng.multi_gen_explodable_range(2, 1..=6, 12, 100);
        let expected = 21; // (6 + 6) + (4 + 5) [miss]

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.multi_gen_explodable_range(2, 1..=6, 7, 100);
        let expected = 35; // (6 + 6) + (4 + 5) + (6 + 3) + (4 + 1) [miss]

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.multi_gen_explodable_range(3, 1..=6, 10, 100);
        let expected = 37; // (6 + 6 + 4) + (5 + 6 + 3) + (4 + 1 + 2) [miss]

        assert_eq!(result, expected);

        rng.set_word_pos(0);
        let result: u32 = rng.multi_gen_explodable_range(3, 1..=6, 10, 1);
        let expected = 30; // (6 + 6 + 4) + (5 + 6 + 3) [miss]

        assert_eq!(result, expected);
    }

    #[test]
    fn round() {
        // Series:
        //
        // rng.gen_bool(0.8125) -> true
        // rng.gen_bool(0.8125) -> true
        // rng.gen_bool(0.8125) -> false
        // rng.gen_bool(0.8125) -> false
        // rng.gen_bool(0.8125) -> true
        let mut rng = build_rng("6xfoUZmp2oJZcFLkA4oKuCPnsGBh5pk9hk53tCpntVQM");

        let positive: f64 = 4.8125;
        let negative: f64 = -positive;
        let large_fp: f64 = 99_999_999_999.8125;

        let result: Option<i32> = rng.round(positive);
        let expected = Some(5);

        assert_eq!(result, expected);

        let result: Option<i32> = rng.round(negative);
        let expected = Some(-5);

        assert_eq!(result, expected);

        let result: Option<i32> = rng.round(positive);
        let expected = Some(4);

        assert_eq!(result, expected);

        let result: Option<i32> = rng.round(negative);
        let expected = Some(-4);

        assert_eq!(result, expected);

        let result: Option<i32> = rng.round(large_fp);
        let expected = None;

        assert_eq!(result, expected);

        let result: Option<i64> = rng.round(large_fp);
        let expected = Some(100_000_000_000);

        assert_eq!(result, expected);

        let result: Option<u8> = rng.round(-5.0);
        let expected = None;

        assert_eq!(result, expected);
    }
}
