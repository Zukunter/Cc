 macro_rules! make_not_os_macros {
    ($($os:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $os {
                ($body:block) => {{
                    #[cfg(not(target_os = stringify!($os)))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_os_macros!(
    not_aix,
    not_android,
    not_amdhsa,
    not_cuda,
    not_cygwin,
    not_dragonfly,
    not_emscripten,
    not_espidf,
    not_fuchsia,
    not_haiku,
    not_helenos,
    not_hermit,
    not_horizon,
    not_hurd,
    not_illumos,
    not_ios,
    not_l4re,
    not_linux,
    not_lynxos178,
    not_macos,
    not_managarm,
    not_motor,
    not_netbsd,
    not_nto,
    not_nuttx,
    not_openbsd,
    not_none,
    not_unknown,
    not_psp,
    not_psx,
    not_qurt,
    not_redox,
    not_rtems,
    not_solid_asp3,
    not_solaris,
    not_teeos,
    not_tvos,
    not_trusty,
    not_uefi,
    not_vexos,
    not_visionos,
    not_vita,
    not_vxworks,
    not_wasi,
    not_watchos,
    not_windows,
    not_xous,
    not_zkvm,
);
