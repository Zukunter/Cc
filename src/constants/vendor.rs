#[cfg(target_vendor = "apple")]
pub const TARGET_VENDOR: &str = "apple";

#[cfg(target_vendor = "pc")]
pub const TARGET_VENDOR: &str = "pc";

#[cfg(target_vendor = "unknown")]
pub const TARGET_VENDOR: &str = "unknown";

#[cfg(target_vendor = "fortanix")]
pub const TARGET_VENDOR: &str = "fortanix";

#[cfg(target_vendor = "sony")]
pub const TARGET_VENDOR: &str = "sony";

#[cfg(target_vendor = "nintendo")]
pub const TARGET_VENDOR: &str = "nintendo";

#[cfg(target_vendor = "uwin")]
pub const TARGET_VENDOR: &str = "uwin";

#[cfg(target_vendor = "wrs")]
pub const TARGET_VENDOR: &str = "wrs";

#[cfg(target_vendor = "espressif")]
pub const TARGET_VENDOR: &str = "espressif";

#[cfg(target_vendor = "kmc")]
pub const TARGET_VENDOR: &str = "kmc";

#[cfg(not(any(
    target_vendor = "apple",
    target_vendor = "pc",
    target_vendor = "unknown",
    target_vendor = "fortanix",
    target_vendor = "sony",
    target_vendor = "nintendo",
    target_vendor = "uwin",
    target_vendor = "wrs",
    target_vendor = "espressif",
    target_vendor = "kmc",
)))]
pub const TARGET_VENDOR: &str = "";
