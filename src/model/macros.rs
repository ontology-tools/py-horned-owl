//! `conversions!`, used by the code `#[model]` generates (`lib/macros`).
//!
//! A macro rather than blanket impls, which coherence rules out: `impl<P, H>
//! From<BoxWrap<P>> for Box<H>` overlaps std's `impl<T> From<T> for Box<T>`.

/// The conversions between a Python class `$py` and its horned-owl type `$h`
/// that the rest of the model uses: owned, borrowed, boxed and in a `Vec`,
/// each as `From` and as `FromCompatible`. Needs `From<&$h> for $py` and
/// `From<&$py> for $h`.
macro_rules! conversions {
    ($py:ident => $h:ty) => {
        conversions!(@owned $h => $py);
        conversions!(@owned $py => $h);
        conversions!(@both Box<$h> => $crate::wrappers::BoxWrap<$py>,
            |v: Box<$h>| $crate::wrappers::BoxWrap(Box::new((&*v).into())),
            |v: &Box<$h>| $crate::wrappers::BoxWrap(Box::new((&**v).into())));
        conversions!(@both $crate::wrappers::BoxWrap<$py> => Box<$h>,
            |v: $crate::wrappers::BoxWrap<$py>| Box::new((&*v.0).into()),
            |v: &$crate::wrappers::BoxWrap<$py>| Box::new((&*v.0).into()));
        conversions!(@both Vec<$h> => $crate::wrappers::VecWrap<$py>,
            |v: Vec<$h>| $crate::wrappers::VecWrap(v.iter().map(Into::into).collect()),
            |v: &Vec<$h>| $crate::wrappers::VecWrap(v.iter().map(Into::into).collect()));
        conversions!(@both $crate::wrappers::VecWrap<$py> => Vec<$h>,
            |v: $crate::wrappers::VecWrap<$py>| v.0.iter().map(Into::into).collect(),
            |v: &$crate::wrappers::VecWrap<$py>| v.0.iter().map(Into::into).collect());
    };

    // Everything but `From<&$from>`, which the caller writes.
    (@owned $from:ty => $to:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self { (&value).into() }
        }
        impl $crate::wrappers::FromCompatible<$from> for $to {
            fn from_c(value: $from) -> Self { (&value).into() }
        }
        impl $crate::wrappers::FromCompatible<&$from> for $to {
            fn from_c(value: &$from) -> Self { value.into() }
        }
    };

    // `From` and `FromCompatible` from `$from` and `&$from` to `$to`.
    (@both $from:ty => $to:ty, $owned:expr, $borrowed:expr) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self { ($owned)(value) }
        }
        impl From<&$from> for $to {
            fn from(value: &$from) -> Self { ($borrowed)(value) }
        }
        impl $crate::wrappers::FromCompatible<$from> for $to {
            fn from_c(value: $from) -> Self { value.into() }
        }
        impl $crate::wrappers::FromCompatible<&$from> for $to {
            fn from_c(value: &$from) -> Self { value.into() }
        }
    };
}
