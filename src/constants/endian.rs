#[cfg(target_endian = "little")]
pub const TARGET_ENDIAN: &str = "little";

#[cfg(target_endian = "big")]
pub const TARGET_ENDIAN: &str = "big";

#[cfg(not(any(
    target_endian = "little",
    target_endian = "big",
)))]
pub const TARGET_ENDIAN: &str = "";
