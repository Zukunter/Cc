mod os;
mod arch;
mod family; 

#[macro_export]
macro_rules! always {
    ($body:block) => {{
        $crate::Cfg::some($body)
    }};
}
