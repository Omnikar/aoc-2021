#[macro_export]
macro_rules! parts {
    ($($part:ident)*) => {
        ::lazy_static::lazy_static! {
            pub static ref PARTS: ::std::collections::HashMap<
                &'static str,
                fn(::std::string::String) -> ::anyhow::Result<()>,
            > = [
                $((stringify!($part), $part as fn(::std::string::String) -> ::anyhow::Result<()>),)*
            ]
            .into_iter()
            .collect();
        }
    };
}

#[macro_export]
macro_rules! days {
    ($($day:ident)*) => {
        ::lazy_static::lazy_static! {
            pub static ref DAYS: ::std::collections::HashMap<
                &'static str,
                &'static ::std::collections::HashMap<&'static str, fn(::std::string::String) -> ::anyhow::Result<()>>,
            > = [
                $((stringify!($day), &*$day::PARTS),)*
            ]
            .into_iter()
            .collect();
        }
    };
}
