#[cfg(target_os = "aix")]
pub const TARGET_OS: &str = "aix";

#[cfg(target_os = "android")]
pub const TARGET_OS: &str = "android";

#[cfg(target_os = "cuda")]
pub const TARGET_OS: &str = "cuda";

#[cfg(target_os = "dragonfly")]
pub const TARGET_OS: &str = "dragonfly";

#[cfg(target_os = "emscripten")]
pub const TARGET_OS: &str = "emscripten";

#[cfg(target_os = "espidf")]
pub const TARGET_OS: &str = "espidf";

#[cfg(target_os = "freebsd")]
pub const TARGET_OS: &str = "freebsd";

#[cfg(target_os = "fuchsia")]
pub const TARGET_OS: &str = "fuchsia";

#[cfg(target_os = "haiku")]
pub const TARGET_OS: &str = "haiku";

#[cfg(target_os = "hermit")]
pub const TARGET_OS: &str = "hermit";

#[cfg(target_os = "horizon")]
pub const TARGET_OS: &str = "horizon";

#[cfg(target_os = "illumos")]
pub const TARGET_OS: &str = "illumos";

#[cfg(target_os = "ios")]
pub const TARGET_OS: &str = "ios";

#[cfg(target_os = "l4re")]
pub const TARGET_OS: &str = "l4re";

#[cfg(target_os = "linux")]
pub const TARGET_OS: &str = "linux";

#[cfg(target_os = "macos")]
pub const TARGET_OS: &str = "macos";

#[cfg(target_os = "netbsd")]
pub const TARGET_OS: &str = "netbsd";

#[cfg(target_os = "none")]
pub const TARGET_OS: &str = "none";

#[cfg(target_os = "nto")]
pub const TARGET_OS: &str = "nto";

#[cfg(target_os = "openbsd")]
pub const TARGET_OS: &str = "openbsd";

#[cfg(target_os = "psp")]
pub const TARGET_OS: &str = "psp";

#[cfg(target_os = "redox")]
pub const TARGET_OS: &str = "redox";

#[cfg(target_os = "rtems")]
pub const TARGET_OS: &str = "rtems";

#[cfg(target_os = "solaris")]
pub const TARGET_OS: &str = "solaris";

#[cfg(target_os = "solid_asp3")]
pub const TARGET_OS: &str = "solid_asp3";

#[cfg(target_os = "tvos")]
pub const TARGET_OS: &str = "tvos";

#[cfg(target_os = "uefi")]
pub const TARGET_OS: &str = "uefi";

#[cfg(target_os = "unknown")]
pub const TARGET_OS: &str = "unknown";

#[cfg(target_os = "visionos")]
pub const TARGET_OS: &str = "visionos";

#[cfg(target_os = "vita")]
pub const TARGET_OS: &str = "vita";

#[cfg(target_os = "vxworks")]
pub const TARGET_OS: &str = "vxworks";

#[cfg(target_os = "wasi")]
pub const TARGET_OS: &str = "wasi";

#[cfg(target_os = "watchos")]
pub const TARGET_OS: &str = "watchos";

#[cfg(target_os = "windows")]
pub const TARGET_OS: &str = "windows";

#[cfg(target_os = "xous")]
pub const TARGET_OS: &str = "xous";

#[cfg(target_os = "zkvm")]
pub const TARGET_OS: &str = "zkvm";

#[cfg(not(any(
    target_os = "aix",
    target_os = "android",
    target_os = "cuda",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "espidf",
    target_os = "freebsd",
    target_os = "fuchsia",
    target_os = "haiku",
    target_os = "hermit",
    target_os = "horizon",
    target_os = "illumos",
    target_os = "ios",
    target_os = "l4re",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "none",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "psp",
    target_os = "redox",
    target_os = "rtems",
    target_os = "solaris",
    target_os = "solid_asp3",
    target_os = "tvos",
    target_os = "uefi",
    target_os = "unknown",
    target_os = "visionos",
    target_os = "vita",
    target_os = "vxworks",
    target_os = "wasi",
    target_os = "watchos",
    target_os = "windows",
    target_os = "xous",
    target_os = "zkvm",
)))]
pub const TARGET_OS: &str = "";
