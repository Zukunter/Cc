macro_rules! make_not_endian_macros {
    ($($macro_name:ident => $endian:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not(target_endian = stringify!($endian)))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_endian_macros!(
    not_little_endian => little,
    not_big_endian    => big,
);
