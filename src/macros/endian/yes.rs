macro_rules! make_endian_macros {
    ($($macro_name:ident => $endian:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(target_endian = stringify!($endian))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_endian_macros!(
    little_endian => little,
    big_endian    => big,
);
