macro_rules! make_pointer_width_macros {
    ($($macro_name:ident => $width:expr),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(target_pointer_width = $width)]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_pointer_width_macros!(
    pointer_16 => "16",
    pointer_32 => "32",
    pointer_64 => "64",
);
