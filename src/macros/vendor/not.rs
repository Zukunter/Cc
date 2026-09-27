macro_rules! make_not_vendor_macros {
    ($($macro_name:ident => $vendor:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not(target_vendor = stringify!($vendor)))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_vendor_macros!(
    not_apple    => apple,
    not_pc       => pc,
    not_unknown  => unknown,
    not_fortanix => fortanix,
);
