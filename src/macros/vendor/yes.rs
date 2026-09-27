macro_rules! make_vendor_macros {
    ($($macro_name:ident => $vendor:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(target_vendor = stringify!($vendor))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_vendor_macros!(
    apple     => apple,
    pc        => pc,
    unknown   => unknown,
    fortanix  => fortanix,
    sony      => sony,
    nintendo  => nintendo,
    uwin      => uwin,
    wrs       => wrs,
    espressif => espressif,
    kmc       => kmc,
);
