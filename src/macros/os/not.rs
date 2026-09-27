#[macro_export]
macro_rules! not_aix {
    ($body:block) => {{
        #[cfg(not(target_os = "aix"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "aix")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_android {
    ($body:block) => {{
        #[cfg(not(target_os = "android"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "android")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_amdhsa {
    ($body:block) => {{
        #[cfg(not(target_os = "amdhsa"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "amdhsa")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_cuda {
    ($body:block) => {{
        #[cfg(not(target_os = "cuda"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "cuda")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_cygwin {
    ($body:block) => {{
        #[cfg(not(target_os = "cygwin"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "cygwin")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_dragonfly {
    ($body:block) => {{
        #[cfg(not(target_os = "dragonfly"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "dragonfly")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_emscripten {
    ($body:block) => {{
        #[cfg(not(target_os = "emscripten"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "emscripten")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_espidf {
    ($body:block) => {{
        #[cfg(not(target_os = "espidf"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "espidf")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_fuchsia {
    ($body:block) => {{
        #[cfg(not(target_os = "fuchsia"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "fuchsia")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_haiku {
    ($body:block) => {{
        #[cfg(not(target_os = "haiku"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "haiku")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_helenos {
    ($body:block) => {{
        #[cfg(not(target_os = "helenos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "helenos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_hermit {
    ($body:block) => {{
        #[cfg(not(target_os = "hermit"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "hermit")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_horizon {
    ($body:block) => {{
        #[cfg(not(target_os = "horizon"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "horizon")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_hurd {
    ($body:block) => {{
        #[cfg(not(target_os = "hurd"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "hurd")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_illumos {
    ($body:block) => {{
        #[cfg(not(target_os = "illumos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "illumos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_ios {
    ($body:block) => {{
        #[cfg(not(target_os = "ios"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "ios")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_l4re {
    ($body:block) => {{
        #[cfg(not(target_os = "l4re"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "l4re")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_linux {
    ($body:block) => {{
        #[cfg(not(target_os = "linux"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "linux")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_lynxos178 {
    ($body:block) => {{
        #[cfg(not(target_os = "lynxos178"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "lynxos178")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_macos {
    ($body:block) => {{
        #[cfg(not(target_os = "macos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "macos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_managarm {
    ($body:block) => {{
        #[cfg(not(target_os = "managarm"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "managarm")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_motor {
    ($body:block) => {{
        #[cfg(not(target_os = "motor"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "motor")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_netbsd {
    ($body:block) => {{
        #[cfg(not(target_os = "netbsd"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "netbsd")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_nto {
    ($body:block) => {{
        #[cfg(not(target_os = "nto"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "nto")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_nuttx {
    ($body:block) => {{
        #[cfg(not(target_os = "nuttx"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "nuttx")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_openbsd {
    ($body:block) => {{
        #[cfg(not(target_os = "openbsd"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "openbsd")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_none {
    ($body:block) => {{
        #[cfg(not(target_os = "none"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "none")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_unknown {
    ($body:block) => {{
        #[cfg(not(target_os = "unknown"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "unknown")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_psp {
    ($body:block) => {{
        #[cfg(not(target_os = "psp"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "psp")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_psx {
    ($body:block) => {{
        #[cfg(not(target_os = "psx"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "psx")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_qurt {
    ($body:block) => {{
        #[cfg(not(target_os = "qurt"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "qurt")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_redox {
    ($body:block) => {{
        #[cfg(not(target_os = "redox"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "redox")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_rtems {
    ($body:block) => {{
        #[cfg(not(target_os = "rtems"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "rtems")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_solid_asp3 {
    ($body:block) => {{
        #[cfg(not(target_os = "solid_asp3"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "solid_asp3")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_solaris {
    ($body:block) => {{
        #[cfg(not(target_os = "solaris"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "solaris")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_teeos {
    ($body:block) => {{
        #[cfg(not(target_os = "teeos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "teeos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_tvos {
    ($body:block) => {{
        #[cfg(not(target_os = "tvos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "tvos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_trusty {
    ($body:block) => {{
        #[cfg(not(target_os = "trusty"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "trusty")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_uefi {
    ($body:block) => {{
        #[cfg(not(target_os = "uefi"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "uefi")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_vexos {
    ($body:block) => {{
        #[cfg(not(target_os = "vexos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "vexos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_visionos {
    ($body:block) => {{
        #[cfg(not(target_os = "visionos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "visionos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_vita {
    ($body:block) => {{
        #[cfg(not(target_os = "vita"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "vita")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_vxworks {
    ($body:block) => {{
        #[cfg(not(target_os = "vxworks"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "vxworks")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_wasi {
    ($body:block) => {{
        #[cfg(not(target_os = "wasi"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "wasi")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_watchos {
    ($body:block) => {{
        #[cfg(not(target_os = "watchos"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "watchos")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_windows {
    ($body:block) => {{
        #[cfg(not(target_os = "windows"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "windows")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_xous {
    ($body:block) => {{
        #[cfg(not(target_os = "xous"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "xous")]
        {
            $crate::Cfg::none()
        }
    }};
}

#[macro_export]
macro_rules! not_zkvm {
    ($body:block) => {{
        #[cfg(not(target_os = "zkvm"))]
        {
            $crate::Cfg::some($body)
        }

        #[cfg(target_os = "zkvm")]
        {
            $crate::Cfg::none()
        }
    }};
}
