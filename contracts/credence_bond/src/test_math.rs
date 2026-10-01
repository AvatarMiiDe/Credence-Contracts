//! Tests for overflow-safe arithmetic helpers.

use crate::math;

#[test]
fn test_bps_basic() {
    let fee = math::bps(10_000_i128, 100_u32, "mul", "div");
    assert_eq!(fee, 100);
}

#[test]
#[should_panic(expected = "fee calculation overflow")]
fn test_bps_overflow_panics() {
    // i128::MAX * 10_000 overflows.
    let _ = math::bps(i128::MAX, 10_000_u32, "fee calculation overflow", "div");
}

#[test]
#[should_panic(expected = "sub underflow")]
fn test_sub_underflow_panics() {
    let _ = math::sub_i128(i128::MIN, 1, "sub underflow");
}

#[test]
#[should_panic(expected = "mul overflow")]
fn test_mul_overflow_panics() {
    let _ = math::mul_i128(i128::MAX, 2, "mul overflow");
}

#[test]
#[should_panic(expected = "attestation weight overflow")]
fn test_mul_u64_overflow_panics() {
    let _ = math::mul_u64(u64::MAX, 2, "attestation weight overflow");
}

#[test]
fn test_add_basic() {
    assert_eq!(math::add_i128(100, 200, "add fail"), 300);
}

#[test]
#[should_panic(expected = "add overflow")]
fn test_add_overflow_panics() {
    let _ = math::add_i128(i128::MAX, 1, "add overflow");
}

#[test]
fn test_sub_basic() {
    assert_eq!(math::sub_i128(500, 200, "sub fail"), 300);
}

#[test]
fn test_mul_basic() {
    assert_eq!(math::mul_i128(50, 2, "mul fail"), 100);
}

#[test]
fn test_mul_u64_basic() {
    assert_eq!(math::mul_u64(50, 2, "mul fail"), 100);
}

#[test]
fn test_div_basic() {
    assert_eq!(math::div_i128(100, 3, "div fail"), 33);
}

#[test]
#[should_panic(expected = "divide by zero")]
fn test_div_by_zero_panics() {
    let _ = math::div_i128(100, 0, "divide by zero");
}

#[test]
fn test_ceil_div_basic() {
    assert_eq!(math::ceil_div_i128(100, 3, "ceil div fail"), 34);
    assert_eq!(math::ceil_div_i128(100, 4, "ceil div fail"), 25);
}

#[test]
#[should_panic(expected = "divide by zero")]
fn test_ceil_div_by_zero_panics() {
    let _ = math::ceil_div_i128(100, 0, "divide by zero");
}

#[test]
fn test_sat_mul_bps_basic() {
    assert_eq!(math::sat_mul_bps(10_000, 5_000), 5_000); // 50% of 10000
}

#[test]
fn test_sat_mul_bps_saturation() {
    // Should saturate to i128::MAX instead of panicking, ensuring recovery and safety at limits
    assert_eq!(math::sat_mul_bps(i128::MAX, 20_000), i128::MAX);
}

#[test]
fn test_split_bps_basic() {
    let (part, remainder) = math::split_bps(1000, 2000, "mul fail", "div fail", "sub fail");
    assert_eq!(part, 200);
    assert_eq!(remainder, 800);
}

#[test]
fn test_split_bps_boundaries() {
    // 0%
    let (part_zero, rem_zero) = math::split_bps(1000, 0, "mul", "div", "sub");
    assert_eq!(part_zero, 0);
    assert_eq!(rem_zero, 1000);

    // 100%
    let (part_full, rem_full) = math::split_bps(1000, 10_000, "mul", "div", "sub");
    assert_eq!(part_full, 1000);
    assert_eq!(rem_full, 0);
}

#[test]
fn test_bps_u64_basic() {
    assert_eq!(math::bps_u64(10_000, 100, "mul fail", "div fail"), 100);
}

#[test]
#[should_panic(expected = "mul u64 overflow")]
fn test_bps_u64_overflow_panics() {
    let _ = math::bps_u64(u64::MAX, 10_000, "mul u64 overflow", "div fail");
}

#[test]
fn test_bps_denominator() {
    assert_eq!(math::BPS_DENOMINATOR, 10_000);
}
