const F64_ONE_BITS: u64 = 0x3FF0_0000_0000_0000;

// All f64 values in the range [1.0, 2.0] differ only in the mantissa, allowing us to build one
// using a u64 value
pub fn f64_from_mantissa(mantissa: u64, min: f64, max: f64) -> f64 {
    let float = f64::from_bits(F64_ONE_BITS ^ (mantissa >> 12));

    float.mul_add(max - min, min * 2.0 - max)
}

// Linear interpolation
pub fn lerp(bias: f64, lhs: f64, rhs: f64) -> f64 {
    (rhs - lhs).mul_add(bias, lhs)
}

// Cosine interpolation
pub fn cerp(bias: f64, lhs: f64, rhs: f64) -> f64 {
    let bias = (1.0 - (bias * std::f64::consts::PI).cos()) / 2.0;

    (rhs - lhs).mul_add(bias, lhs)
}

// Return 6x^5 - 15x^4 + 10x^3
// Maps [0.0, 1.0] to [0.0, 1.0]
pub fn smoothstep(x: f64) -> f64 {
    x.powi(3) * x.mul_add(x.mul_add(6.0, -15.0), 10.0)
}

// Maps [0.0, 1.0] to [0.0, 1.0]
pub fn sigmoid(beta: f64, x: f64) -> f64 {
    1.0 / (1.0 + (x / (1.0 - x)).powf(beta))
}

// Maps [-1.0, 1.0] to [0.0, 1.0]
pub fn neg_unit_to_unit(x: f64) -> f64 {
    x.mul_add(0.5, 0.5)
}
