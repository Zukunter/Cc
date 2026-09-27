macro_rules! make_os_macros {
    ($($os:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $os {
                ($body:block) => {{
                    #[cfg(target_os = stringify!($os))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_os_macros!(
    aix,
    android,
    amdhsa,
    cuda,
    cygwin,
    dragonfly,
    emscripten,
    espidf,
    fuchsia,
    haiku,
    helenos,
    hermit,
    horizon,
    hurd,
    illumos,
    ios,
    l4re,
    linux,
    lynxos178,
    macos,
    managarm,
    motor,
    netbsd,
    nto,
    nuttx,
    openbsd,
    none,
    unknown,
    psp,
    psx,
    qurt,
    redox,
    rtems,
    solid_asp3,
    solaris,
    teeos,
    tvos,
    trusty,
    uefi,
    vexos,
    visionos,
    vita,
    vxworks,
    wasi,
    watchos,
    windows,
    xous,
    zkvm,
);
