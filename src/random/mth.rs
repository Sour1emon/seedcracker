pub const fn get_mask(bits: i32) -> i64 {
    if bits >= 64 {
        !0
    } else {
        (1 << bits) - 1
    }
}

pub const MASK_8: i32 = get_mask(8) as i32;
pub const MASK_16: i32 = get_mask(16) as i32;
pub const MASK_32: i64 = get_mask(32);
pub const MASK_48: i64 = get_mask(48);

#[inline(always)]
pub const fn mask(value: i64, bits: i32) -> i64 {
    value & if bits >= 64 { !0 } else { (1 << bits) - 1 }
}

#[inline(always)]
pub const fn mask_signed(value: i64, bits: i32) -> i64 {
    value << (64 - bits) >> (64 - bits)
}

#[inline(always)]
pub const fn mod_inverse(value: i64, bits: i32) -> i64 {
    let mut x = ((((value << 1) ^ value) & 4) << 1) ^ value;
    x *= 2 - value * x;
    x *= 2 - value * x;
    x *= 2 - value * x;
    x *= 2 - value * x;
    mask(x, bits)
}
