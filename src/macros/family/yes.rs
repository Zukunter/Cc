#[macro_export]
macro_rules! asmjs {
    ($body:block) => {{
        #[cfg(target_family = "asmjs")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_family = "asmjs"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! unix {
    ($body:block) => {{
        #[cfg(target_family = "unix")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_family = "unix"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! wasm_family {
    ($body:block) => {{
        #[cfg(target_family = "wasm")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_family = "wasm"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! windows_family {
    ($body:block) => {{
        #[cfg(target_family = "windows")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_family = "windows"))]
        {
            $crate::Cfg::none()
        }
    }};
}
