#[cfg(target_env = "gnu")]
pub const TARGET_ENV: &str = "gnu";

#[cfg(target_env = "msvc")]
pub const TARGET_ENV: &str = "msvc";

#[cfg(target_env = "musl")]
pub const TARGET_ENV: &str = "musl";

#[cfg(target_env = "sgx")]
pub const TARGET_ENV: &str = "sgx";

#[cfg(target_env = "uclibc")]
pub const TARGET_ENV: &str = "uclibc";

#[cfg(not(any(
    target_env = "gnu",
    target_env = "msvc",
    target_env = "musl",
    target_env = "sgx",
    target_env = "uclibc",
)))]
pub const TARGET_ENV: &str = "";
