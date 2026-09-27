macro_rules! make_not_arch_macros {
    ($($macro_name:ident => $arch:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(not(target_arch = stringify!($arch)))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_not_arch_macros!(
    not_x86 => x86,
    not_x86_64 => x86_64,
    not_arm => arm,
    not_aarch64 => aarch64,
    not_riscv32 => riscv32,
    not_riscv64 => riscv64,
    not_wasm32 => wasm32,
    not_wasm64 => wasm64,
    not_mips => mips,
    not_mips64 => mips64,
    not_powerpc => powerpc,
    not_powerpc64 => powerpc64,
    not_s390x => s390x,
    not_sparc64 => sparc64,
);
