macro_rules! make_not_build_cfg_macros {
    ($($macro_name:ident => $flag:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not($flag))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_build_cfg_macros!(
    not_debug => debug_assertions,
    not_test  => test,
);
