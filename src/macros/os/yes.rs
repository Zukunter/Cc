#[macro_export]
macro_rules! aix {
    ($body:block) => {{
        #[cfg(target_os = "aix")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "aix"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! android {
    ($body:block) => {{
        #[cfg(target_os = "android")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "android"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! amdhsa {
    ($body:block) => {{
        #[cfg(target_os = "amdhsa")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "amdhsa"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! cuda {
    ($body:block) => {{
        #[cfg(target_os = "cuda")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "cuda"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! cygwin {
    ($body:block) => {{
        #[cfg(target_os = "cygwin")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "cygwin"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! dragonfly {
    ($body:block) => {{
        #[cfg(target_os = "dragonfly")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "dragonfly"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! emscripten {
    ($body:block) => {{
        #[cfg(target_os = "emscripten")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "emscripten"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! espidf {
    ($body:block) => {{
        #[cfg(target_os = "espidf")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "espidf"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! fuchsia {
    ($body:block) => {{
        #[cfg(target_os = "fuchsia")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "fuchsia"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! haiku {
    ($body:block) => {{
        #[cfg(target_os = "haiku")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "haiku"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! helenos {
    ($body:block) => {{
        #[cfg(target_os = "helenos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "helenos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! hermit {
    ($body:block) => {{
        #[cfg(target_os = "hermit")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "hermit"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! horizon {
    ($body:block) => {{
        #[cfg(target_os = "horizon")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "horizon"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! hurd {
    ($body:block) => {{
        #[cfg(target_os = "hurd")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "hurd"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! illumos {
    ($body:block) => {{
        #[cfg(target_os = "illumos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "illumos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! ios {
    ($body:block) => {{
        #[cfg(target_os = "ios")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "ios"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! l4re {
    ($body:block) => {{
        #[cfg(target_os = "l4re")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "l4re"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! linux {
    ($body:block) => {{
        #[cfg(target_os = "linux")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "linux"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! lynxos178 {
    ($body:block) => {{
        #[cfg(target_os = "lynxos178")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "lynxos178"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! macos {
    ($body:block) => {{
        #[cfg(target_os = "macos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "macos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! managarm {
    ($body:block) => {{
        #[cfg(target_os = "managarm")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "managarm"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! motor {
    ($body:block) => {{
        #[cfg(target_os = "motor")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "motor"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! netbsd {
    ($body:block) => {{
        #[cfg(target_os = "netbsd")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "netbsd"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! nto {
    ($body:block) => {{
        #[cfg(target_os = "nto")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "nto"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! nuttx {
    ($body:block) => {{
        #[cfg(target_os = "nuttx")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "nuttx"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! openbsd {
    ($body:block) => {{
        #[cfg(target_os = "openbsd")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "openbsd"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! os_none {
    ($body:block) => {{
        #[cfg(target_os = "none")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "none"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! os_unknown {
    ($body:block) => {{
        #[cfg(target_os = "unknown")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "unknown"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! psp {
    ($body:block) => {{
        #[cfg(target_os = "psp")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "psp"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! psx {
    ($body:block) => {{
        #[cfg(target_os = "psx")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "psx"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! qurt {
    ($body:block) => {{
        #[cfg(target_os = "qurt")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "qurt"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! redox {
    ($body:block) => {{
        #[cfg(target_os = "redox")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "redox"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! rtems {
    ($body:block) => {{
        #[cfg(target_os = "rtems")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "rtems"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! solid_asp3 {
    ($body:block) => {{
        #[cfg(target_os = "solid_asp3")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "solid_asp3"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! solaris {
    ($body:block) => {{
        #[cfg(target_os = "solaris")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "solaris"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! teeos {
    ($body:block) => {{
        #[cfg(target_os = "teeos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "teeos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! tvos {
    ($body:block) => {{
        #[cfg(target_os = "tvos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "tvos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! trusty {
    ($body:block) => {{
        #[cfg(target_os = "trusty")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "trusty"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! uefi {
    ($body:block) => {{
        #[cfg(target_os = "uefi")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "uefi"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! vexos {
    ($body:block) => {{
        #[cfg(target_os = "vexos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "vexos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! visionos {
    ($body:block) => {{
        #[cfg(target_os = "visionos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "visionos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! vita {
    ($body:block) => {{
        #[cfg(target_os = "vita")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "vita"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! vxworks {
    ($body:block) => {{
        #[cfg(target_os = "vxworks")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "vxworks"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! wasi {
    ($body:block) => {{
        #[cfg(target_os = "wasi")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "wasi"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! watchos {
    ($body:block) => {{
        #[cfg(target_os = "watchos")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "watchos"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! windows {
    ($body:block) => {{
        #[cfg(target_os = "windows")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "windows"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! xous {
    ($body:block) => {{
        #[cfg(target_os = "xous")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "xous"))]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! zkvm {
    ($body:block) => {{
        #[cfg(target_os = "zkvm")]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(not(target_os = "zkvm"))]
        {
            $crate::Cfg::none()
        }
    }};
}
