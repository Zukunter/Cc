#[macro_export]
macro_rules! aarch64 {
    ($body:block) => {{
        #[cfg(target_arch = "aarch64")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_arch = "aarch64"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! arm {
    ($body:block) => {{
        #[cfg(target_arch = "arm")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_arch = "arm"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! x86 {
    ($body:block) => {{
        #[cfg(target_arch = "x86")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_arch = "x86"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! x86_64 {
    ($body:block) => {{
        #[cfg(target_arch = "x86_64")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_arch = "x86_64"))]
        {
            $crate::Cfg::none()
        }
    }};
}
