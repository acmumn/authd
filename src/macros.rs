/// Panics if any of the given environment variables are not defined.
#[macro_export]
macro_rules! require_env_vars {
    ($($var:ident),* $(,)*) => {{
        use std::env::var;
        $(
            var(stringify!($var)).expect(concat!(stringify!($var), " must be defined"));
        )*
    }}
}
