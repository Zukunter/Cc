macro_rules! make_build_cfg_macros {
    ($($macro_name:ident => $flag:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg($flag)]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_build_cfg_macros!(
    is_debug => debug_assertions,
    is_test  => test,
);
