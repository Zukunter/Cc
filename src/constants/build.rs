#[cfg(debug_assertions)]
pub const DEBUG_ASSERTIONS: &str = "debug_assertions";

#[cfg(not(debug_assertions))]
pub const DEBUG_ASSERTIONS: &str = "";
