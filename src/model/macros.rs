//! `conversions!`, used by the code `#[model]` generates (`lib/macros`).

/// The conversions between a Python class `$py` and its horned-owl type `$h`
/// besides the borrowed `From`s, which the caller writes: owned `From`s for
/// the rest of the crate and borrowed `FromCompatible`s for the fields of
/// other classes. Boxes and collections convert through the generic
/// `FromCompatible` impls in `wrappers`.
macro_rules! conversions {
    ($py:ident => $h:ty) => {
        impl From<$h> for $py {
            fn from(value: $h) -> Self {
                (&value).into()
            }
        }
        impl From<$py> for $h {
            fn from(value: $py) -> Self {
                (&value).into()
            }
        }
        impl $crate::wrappers::FromCompatible<&$h> for $py {
            fn from_c(value: &$h) -> Self {
                value.into()
            }
        }
        impl $crate::wrappers::FromCompatible<&$py> for $h {
            fn from_c(value: &$py) -> Self {
                value.into()
            }
        }
    };
}
