 macro_rules! make_not_family_macros {
    ($($macro_name:ident => $family:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not(target_family = stringify!($family)))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_family_macros!(
    not_asmjs => asmjs,
    not_unix => unix,
    not_wasm_family => wasm,
    not_windows_family => windows,
);
