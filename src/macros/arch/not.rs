#[macro_export]
macro_rules! not_aarch64 {
    ($body:block) => {{
        #[cfg(not(target_arch = "aarch64"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_arch = "aarch64")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_arm {
    ($body:block) => {{
        #[cfg(not(target_arch = "arm"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_arch = "arm")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_x86 {
    ($body:block) => {{
        #[cfg(not(target_arch = "x86"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_arch = "x86")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_x86_64 {
    ($body:block) => {{
        #[cfg(not(target_arch = "x86_64"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_arch = "x86_64")]
        {
            $crate::Cfg::none()
        }
    }};
}
