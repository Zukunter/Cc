#[cfg(target_arch = "x86")]
pub const TARGET_ARCH: &str = "x86";

#[cfg(target_arch = "x86_64")]
pub const TARGET_ARCH: &str = "x86_64";

#[cfg(target_arch = "arm")]
pub const TARGET_ARCH: &str = "arm";

#[cfg(target_arch = "aarch64")]
pub const TARGET_ARCH: &str = "aarch64";

#[cfg(target_arch = "riscv32")]
pub const TARGET_ARCH: &str = "riscv32";

#[cfg(target_arch = "riscv64")]
pub const TARGET_ARCH: &str = "riscv64";

#[cfg(target_arch = "wasm32")]
pub const TARGET_ARCH: &str = "wasm32";

#[cfg(target_arch = "wasm64")]
pub const TARGET_ARCH: &str = "wasm64";

#[cfg(target_arch = "mips")]
pub const TARGET_ARCH: &str = "mips";

#[cfg(target_arch = "mips64")]
pub const TARGET_ARCH: &str = "mips64";

#[cfg(target_arch = "powerpc")]
pub const TARGET_ARCH: &str = "powerpc";

#[cfg(target_arch = "powerpc64")]
pub const TARGET_ARCH: &str = "powerpc64";

#[cfg(target_arch = "s390x")]
pub const TARGET_ARCH: &str = "s390x";

#[cfg(target_arch = "sparc64")]
pub const TARGET_ARCH: &str = "sparc64";

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "wasm32",
    target_arch = "wasm64",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "s390x",
    target_arch = "sparc64",
)))]
pub const TARGET_ARCH: &str = "";
