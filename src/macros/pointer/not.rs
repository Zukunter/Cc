macro_rules! make_not_pointer_width_macros {
    ($($macro_name:ident => $width:expr),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not(target_pointer_width = $width))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_pointer_width_macros!(
    not_pointer_16 => "16",
    not_pointer_32 => "32",
    not_pointer_64 => "64",
);
