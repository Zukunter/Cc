#[cfg(target_family = "asmjs")]
pub const TARGET_FAMILY: &str = "asmjs";

#[cfg(target_family = "unix")]
pub const TARGET_FAMILY: &str = "unix";

#[cfg(target_family = "wasm")]
pub const TARGET_FAMILY: &str = "wasm";

#[cfg(target_family = "windows")]
pub const TARGET_FAMILY: &str = "windows";

#[cfg(not(any(
    target_family = "asmjs",
    target_family = "unix",
    target_family = "wasm",
    target_family = "windows",
)))]
pub const TARGET_FAMILY: &str = "";
