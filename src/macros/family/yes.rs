 macro_rules! make_family_macros {
    ($($macro_name:ident => $family:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(target_family = stringify!($family))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_family_macros!(
    asmjs => asmjs,
    unix => unix,
    wasm_family => wasm,
    windows_family => windows,
);
