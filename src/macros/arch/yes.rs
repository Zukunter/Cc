 macro_rules! make_arch_macros {
    ($($macro_name:ident => $arch:ident),* $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($body:block) => {{
                    #[cfg(target_arch = stringify!($arch))]
                    {
                        $body
                    }
                }};
            }
        )*
    };
}

make_arch_macros!(
    x86 => x86,
    x86_64 => x86_64,
    arm => arm,
    aarch64 => aarch64,
    riscv32 => riscv32,
    riscv64 => riscv64,
    wasm32 => wasm32,
    wasm64 => wasm64,
    mips => mips,
    mips64 => mips64,
    powerpc => powerpc,
    powerpc64 => powerpc64,
    s390x => s390x,
    sparc64 => sparc64,
);
