#[macro_export]
macro_rules! not_asmjs {
    ($body:block) => {{
        #[cfg(not(target_family = "asmjs"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_family = "asmjs")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_unix {
    ($body:block) => {{
        #[cfg(not(target_family = "unix"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_family = "unix")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_wasm_family {
    ($body:block) => {{
        #[cfg(not(target_family = "wasm"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_family = "wasm")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_windows_family {
    ($body:block) => {{
        #[cfg(not(target_family = "windows"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_family = "windows")]
        {
            $crate::Cfg::none()
        }
    }};
}
