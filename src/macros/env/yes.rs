macro_rules! make_env_macros {
    ($($macro_name:ident => $env:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(target_env = stringify!($env))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_env_macros!(
    gnu    => gnu,
    msvc   => msvc,
    musl   => musl,
    sgx    => sgx,
    uclibc => uclibc,
);
