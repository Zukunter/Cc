#[cfg(target_pointer_width = "16")]
pub const TARGET_POINTER_WIDTH: &str = "16";

#[cfg(target_pointer_width = "32")]
pub const TARGET_POINTER_WIDTH: &str = "32";

#[cfg(target_pointer_width = "64")]
pub const TARGET_POINTER_WIDTH: &str = "64";

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
)))]
pub const TARGET_POINTER_WIDTH: &str = "";
