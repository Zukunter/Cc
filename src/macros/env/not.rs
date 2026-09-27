macro_rules! make_not_env_macros {
    ($($macro_name:ident => $env:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not(target_env = stringify!($env)))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_env_macros!(
    not_gnu    => gnu,
    not_msvc   => msvc,
    not_musl   => musl,
    not_sgx    => sgx,
    not_uclibc => uclibc,
);
