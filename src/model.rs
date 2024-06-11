use std::{borrow::Borrow, collections::BTreeSet, sync::Arc};
use std::fmt::{Display, Write};
use std::hash::{DefaultHasher, Hash, Hasher};

use horned_owl::model::ArcStr;
use paste::paste;
use pyo3::{exceptions::PyKeyError, prelude::*, PyObject, types::{IntoPyDict, PyType}};
use regex::Regex;

fn to_py_type_str(n: &str, m: String) -> String {
    let crate_regex = Regex::new(r"(?m)(?:\w+::)*(\w+)").unwrap();
    let tuple_regex = Regex::new(r"\(([^,]+),\s*([^,\)]+)\)").unwrap();
    let box_regex = Regex::new(r"BoxWrap<(.*)>").unwrap();
    let list_regex = Regex::new(r"VecWrap<(.*)>").unwrap();
    let set_regex = Regex::new(r"BTreeSetWrap<(.*)>").unwrap();
    let option_regex = Regex::new(r"Option<(.*)>").unwrap();

    let name = crate_regex.replace_all(n, "$1").to_string();

    let mut ma = tuple_regex.captures(&name);
    if ma.is_some() {
        let matc = ma.unwrap();
        let inner_0 = to_py_type_str(matc[1].borrow(), m.clone());
        let inner_1 = to_py_type_str(matc[2].borrow(), m);
        return format!("typing.Tuple[{}, {}]", inner_0, inner_1);
    }

    ma = box_regex.captures(&name);
    if ma.is_some() {
        return to_py_type_str(ma.unwrap()[1].borrow(), m);
    }

    ma = list_regex.captures(&name);
    if ma.is_some() {
        let inner = to_py_type_str(ma.unwrap()[1].borrow(), m);
        return format!("typing.List[{}]", inner);
    }

    ma = set_regex.captures(&name);
    if ma.is_some() {
        let inner = to_py_type_str(ma.unwrap()[1].borrow(), m);
        return format!("typing.Set[{}]", inner);
    }

    ma = option_regex.captures(&name);
    if ma.is_some() {
        let inner = to_py_type_str(ma.unwrap()[1].borrow(), m);
        return format!("typing.Optional[{}]", inner);
    }

    if name == "StringWrapper" || name == "&str" || name == "String" {
        return "str".to_string();
    }

    if name == "u32" {
        return "int".to_string();
    }

    if name == "u32" {
        return "int".to_string();
    }

    return format!("{}{}", m, name);
}

fn to_py_type<T>(m: String) -> String {
    let name = std::any::type_name::<T>();

    return to_py_type_str(name, m);
}

macro_rules! cond {
    ($x:ident, $($_:tt)+) => {
        $x
    };
    ($x:expr, $($_:tt)+) => {
        $x
    };
    ($x:ty, $($_:tt)+) => {
        $x
    };
}

macro_rules! wrapped_base {
    ($name:ident) => {
        impl FromCompatible<horned_owl::model::$name<ArcStr>> for $name {
            fn from_c(value: horned_owl::model::$name<ArcStr>) -> Self {
                $name::from(value)
            }
        }

        impl FromCompatible<&horned_owl::model::$name<ArcStr>> for $name {
            fn from_c(value: &horned_owl::model::$name<ArcStr>) -> Self {
                $name::from(value)
            }
        }

        impl FromCompatible<$name> for horned_owl::model::$name<ArcStr> {
            fn from_c(value: $name) -> Self {
                horned_owl::model::$name::<ArcStr>::from(value)
            }
        }

        impl FromCompatible<&$name> for horned_owl::model::$name<ArcStr> {
            fn from_c(value: &$name) -> Self {
                horned_owl::model::$name::<ArcStr>::from(value)
            }
        }

        impl From<horned_owl::model::$name<ArcStr>> for $name {
            fn from(value: horned_owl::model::$name<ArcStr>) -> Self {
                $name::from(value.borrow())
            }
        }

        impl From<$name> for horned_owl::model::$name<ArcStr> {
            fn from(value: $name) -> Self {
                horned_owl::model::$name::<ArcStr>::from(value.borrow())
            }
        }

        impl From<&BoxWrap<$name>> for Box<horned_owl::model::$name<ArcStr>> {
            fn from(value: &BoxWrap<$name>) -> Self {
                Box::new(((*value.0.clone()).into()))
            }
        }

        impl From<&Box<horned_owl::model::$name<ArcStr>>> for BoxWrap<$name> {
            fn from(value: &Box<horned_owl::model::$name<ArcStr>>) -> Self {
                BoxWrap(Box::new(Into::<$name>::into(*value.clone())))
            }
        }

        impl From<BoxWrap<$name>> for Box<horned_owl::model::$name<ArcStr>> {
            fn from(value: BoxWrap<$name>) -> Self {
                Into::<Box<horned_owl::model::$name<ArcStr>>>::into(value.borrow())
            }
        }

        impl From<Box<horned_owl::model::$name<ArcStr>>> for BoxWrap<$name> {
            fn from(value: Box<horned_owl::model::$name<ArcStr>>) -> Self {
                Into::<BoxWrap<$name>>::into(value.borrow())
            }
        }

        impl From<VecWrap<$name>> for Vec<horned_owl::model::$name<ArcStr>> {
            fn from(value: VecWrap<$name>) -> Self {
                Into::<Vec<horned_owl::model::$name<ArcStr>>>::into(value.borrow())
            }
        }

        impl From<Vec<horned_owl::model::$name<ArcStr>>> for VecWrap<$name> {
            fn from(value: Vec<horned_owl::model::$name<ArcStr>>) -> Self {
                Into::<VecWrap<$name>>::into(value.borrow())
            }
        }

        impl From<&VecWrap<$name>> for Vec<horned_owl::model::$name<ArcStr>> {
            fn from(value: &VecWrap<$name>) -> Self {
                value
                    .0
                    .iter()
                    .map(horned_owl::model::$name::<ArcStr>::from)
                    .collect()
            }
        }
        impl From<&Vec<horned_owl::model::$name<ArcStr>>> for VecWrap<$name> {
            fn from(value: &Vec<horned_owl::model::$name<ArcStr>>) -> Self {
                VecWrap(value.into_iter().map($name::from).collect())
            }
        }

        impl FromCompatible<&BoxWrap<$name>> for Box<horned_owl::model::$name<ArcStr>> {
            fn from_c(value: &BoxWrap<$name>) -> Self {
                Box::<horned_owl::model::$name<ArcStr>>::from(value)
            }
        }
        impl FromCompatible<&Box<horned_owl::model::$name<ArcStr>>> for BoxWrap<$name> {
            fn from_c(value: &Box<horned_owl::model::$name<ArcStr>>) -> Self {
                BoxWrap::<$name>::from(value)
            }
        }
        impl FromCompatible<BoxWrap<$name>> for Box<horned_owl::model::$name<ArcStr>> {
            fn from_c(value: BoxWrap<$name>) -> Self {
                Box::<horned_owl::model::$name<ArcStr>>::from(value)
            }
        }
        impl FromCompatible<Box<horned_owl::model::$name<ArcStr>>> for BoxWrap<$name> {
            fn from_c(value: Box<horned_owl::model::$name<ArcStr>>) -> Self {
                BoxWrap::<$name>::from(value)
            }
        }
        impl FromCompatible<VecWrap<$name>> for Vec<horned_owl::model::$name<ArcStr>> {
            fn from_c(value: VecWrap<$name>) -> Self {
                Vec::<horned_owl::model::$name<ArcStr>>::from(value)
            }
        }
        impl FromCompatible<Vec<horned_owl::model::$name<ArcStr>>> for VecWrap<$name> {
            fn from_c(value: Vec<horned_owl::model::$name<ArcStr>>) -> Self {
                VecWrap::<$name>::from(value)
            }
        }
        impl FromCompatible<&VecWrap<$name>> for Vec<horned_owl::model::$name<ArcStr>> {
            fn from_c(value: &VecWrap<$name>) -> Self {
                Vec::<horned_owl::model::$name<ArcStr>>::from(value)
            }
        }
        impl FromCompatible<&Vec<horned_owl::model::$name<ArcStr>>> for VecWrap<$name> {
            fn from_c(value: &Vec<horned_owl::model::$name<ArcStr>>) -> Self {
                VecWrap::<$name>::from(value)
            }
        }
    };
}

macro_rules! wrapped_enum {
    (pub enum $name:ident {
        $(
            $(#[transparent] $v_name_transparent:ident ( $field_transparent:ty ))?
            $($v_name:ident as $v_name_full:ident $(( $field_t0:ty$(, $field_t1:ty)? ))?$({ $($field_s:ident : $type_s:ty,)+ })?)?
            ,
        )*
    }) => {
        paste! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            enum [<$name _ Inner>] {
                $($(
                    $v_name([<$v_name_full>]),
                )?)*
                $($(
                    $v_name_transparent($v_name_transparent),
                )?)*
            }

            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name([<$name _ Inner>]);

            impl ToPyi for $name {
                #[allow(unused_assignments)]
                fn pyi(module: Option<String>) -> String {
                    let mut res = String::new();
                    let mut first = true;

                    let m = module.map(|c| format!("{}.", c)).unwrap_or_default();

                    write!(&mut res, "typing.Union[").unwrap();
                    $($(

                        if (first) {
                            first = false;
                            write!(&mut res, "{}{}", m, stringify!($v_name_full)).unwrap();
                        } else {
                            write!(&mut res, ", {}{}", m, stringify!($v_name_full)).unwrap();
                        }
                    )*)?

                    $($(
                        if (first) {
                            first = false;
                            write!(&mut res, "{}{}", m, stringify!($v_name_transparent)).unwrap();
                        } else {
                            write!(&mut res, ", {}{}", m, stringify!($v_name_transparent)).unwrap();
                        }
                    )*)?
                    write!(&mut res, "]\n").unwrap();

                    res
                }
            }

            $($(
                #[doc = concat!(
                    stringify!($v_name_full),
                    "(",

                    $("first: ",
                    stringify!($field_t0),
                    $(concat!(", second: ", stringify!($field_t1)),)?)?

                    $($(concat!(stringify!($field_s), ": ", stringify!($type_s), ", ")),*,)?

                    ")",
                    "\n\n",doc!($v_name_full))]
                #[allow(non_camel_case_types)]
                #[pyclass(module="pyhornedowl.model")]
                #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct [<$v_name_full>]
                    $((
                        #[pyo3(get,set,name="first")]
                        pub $field_t0
                        $(,
                            #[pyo3(get,set,name="second")]
                            pub $field_t1
                        )?
                    );)?
                    $({
                        $(
                            #[pyo3(get,set)]
                            pub $field_s: $type_s,
                        )*
                    })?

                #[pymethods]
                impl [<$v_name_full >] {
                    #[new]
                    fn new(
                        $(first: $field_t0, $(second: $field_t1)?)?
                        $($($field_s: $type_s,)*)?
                    ) -> Self {
                        [<$v_name_full>]
                        $((
                            first.into(), $(cond! (second.into(), $field_t1))?
                        ))?
                        $({
                            $($field_s: $field_s.into(),)*
                        })?
                    }


                    fn __getitem__(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
                        match name {
                            $($(stringify!($field_s) => Ok(self.$field_s.clone().into_py(py)),)*)?
                            $("first" => Ok(cond!(self.0.clone().into_py(py), $field_t0)),)?
                            $($("second" => Ok(cond!(self.1.clone().into_py(py), $field_t1)),)?)?
                            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
                        }
                    }

                    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
                        match name {
                            $($(stringify!($field_s) => {
                                self.$field_s = value.extract()?;
                                Ok(())
                            },)*)?
                            $("first" => {
                                self.0 = cond!(value, $field_t0).extract()?;
                                Ok(())
                            })?
                            $($("second" => {
                                self.1 = cond!(value, $field_t1).extract()?;
                                Ok(())
                            })?)?
                            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
                        }
                    }

                    #[cfg(debug_assertions)]
                    #[classmethod]
                    fn __pyi__(_: &Bound<'_, PyType>) -> String {
                        let mut res = String::new();

                        write!(&mut res, "class {}:\n", stringify!($v_name_full)).unwrap();
                        $($(
                            write!(&mut res, "    {}: {}\n", stringify!($field_s), to_py_type::<$type_s>(String::new())).unwrap();
                        )*)?
                        $(
                            write!(&mut res, "    first: {}\n", to_py_type::<$field_t0>(String::new())).unwrap();
                        )?
                        $($(
                            write!(&mut res, "    second: {}\n", to_py_type::<$field_t1>(String::new())).unwrap();
                        )?)?

                        write!(&mut res, "    def __init__(self").unwrap();
                        $($(
                            write!(&mut res, ", {}: {}", stringify!($field_s), to_py_type::<$type_s>(String::new())).unwrap();
                        )*)?
                        $(write!(&mut res, ", first: {}", to_py_type::<$field_t0>(String::new())).unwrap();)?
                        $($(
                            write!(&mut res, ", second: {}", to_py_type::<$field_t1>(String::new())).unwrap();
                        )?)?
                        write!(&mut res, "):\n        ...\n").unwrap();
                        write!(&mut res, "    ...\n").unwrap();

                        res
                    }
                    
                    fn __hash__(&self) -> u64 {
                        let mut s = DefaultHasher::new();
                        Hash::hash(&self, &mut s);
                        s.finish()
                    }

                    fn __eq__(&self, other: &Self) -> bool {
                        self == other
                    }
                }
            )?)*

            impl From<&horned_owl::model::$name<ArcStr>> for $name {
                fn from(value: &horned_owl::model::$name<ArcStr>) -> Self {
                    match value {
                        $($(
                            horned_owl::model::$name::$v_name_transparent::<ArcStr>(f0) => $name(
                                [<$name _ Inner>]::$v_name_transparent(f0.into())),
                        )?)*
                        $($($(
                            horned_owl::model::$name::$v_name::<ArcStr>(f0 $(, cond!(f1, $field_t1))?) => $name(
                                [<$name _ Inner>]::$v_name([<$v_name_full>](
                                (f0).into() $(, Into::<$field_t1>::into(f1))?
                            ))),
                        )?)?)*
                        $($($(
                            horned_owl::model::$name::$v_name::<ArcStr>{
                                $($field_s, )*
                            } => $name([<$name _ Inner>]::$v_name([<$v_name_full>]{
                                $($field_s: IntoCompatible::<$type_s>::into_c($field_s),)*
                            })),
                        )?)?)*
                    }
                }
            }
            impl IntoPy<pyo3::PyObject> for $name {
                fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
                    match self.0 {
                        $($(
                            [<$name _ Inner>]::$v_name(val) => {
                                val.into_py(py)
                            },
                        )?)*
                        $($(
                            [<$name _ Inner>]::$v_name_transparent(val) => {
                                val.into_py(py)
                            },
                        )?)*
                    }
                }
            }

            impl From<&$name> for horned_owl::model::$name<ArcStr> {
                fn from(value: &$name) -> Self {
                    match value.0.borrow() {
                        $($(
                            [<$name _ Inner>]::$v_name_transparent(f0) => horned_owl::model::$name::<ArcStr>::$v_name_transparent(f0.into()),
                        )?

                        $($(
                            [<$name _ Inner>]::$v_name([<$v_name_full>](f0 $(, cond!(f1, $field_t1))?)) =>
                                horned_owl::model::$name::<ArcStr>::$v_name(f0.into() $(, cond!(f1.into(), $field_t1))?),
                        )?)?

                        $($(
                            [<$name _ Inner>]::$v_name([<$v_name_full>]{
                                $($field_s, )*
                            }) => horned_owl::model::$name::<ArcStr>::$v_name{
                                $($field_s: $field_s.into_c(),)*
                            },
                        )?)?)*
                    }
                }
            }

            impl <'source> FromPyObject<'source> for $name {
                fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
                    $($(
                        {
                            let r = [<$v_name_transparent>]::extract(ob);
                            if r.is_ok() {
                                let local = r.unwrap();
                                let inner = [<$name _ Inner>]::$v_name_transparent(local);
                                return Ok($name(inner));
                            }
                        }
                    )?

                    $(
                        {
                            let r = [<$v_name_full>]::extract(ob);
                            if r.is_ok() {
                                let local = r.unwrap();
                                let inner = [<$name _ Inner>]::$v_name(local);
                                return Ok($name(inner));
                            }
                        }
                    )?)*

                    Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to $name"))
                }
            }

            wrapped_base! {$name}
        }
    };
}

macro_rules! named {
    (pub struct $name:ident ( pub $type0:ty $(, pub $type1:ty)?)) => {
        wrapped!(pub struct $name ( pub $type0 $(, pub $type1)?));

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.0.to_string())
            }
        }

        #[pymethods]
        impl $name {
            fn __str__(&self) -> String {
                self.to_string()
            }
        }
    }
}

macro_rules! wrapped {
    (pub struct $name:ident { $(pub $field:ident: $type:ty,)* }) => {
        paste! {

            #[doc = concat!(
                stringify!($name),
                "(",
                $(concat!(stringify!($field), ": ", stringify!($type), ", ")),*,
                ")",
                "\n\n",
                doc!($name)
            )]
            #[pyclass(module="pyhornedowl.model",mapping)]
            #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name {
                $(
                    #[pyo3(get,set)]
                    pub $field: $type,
                )*
            }

            #[pymethods]
            impl $name {
                #[new]
                fn new($($field: $type),*) -> Self {
                    $name {
                        $($field,)*
                    }
                }

                fn __getitem__(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
                    match name {
                        $(stringify!($field) => Ok(self.$field.clone().into_py(py)),)*
                        &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
                    }
                }

                fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
                    match name {
                        $(stringify!($field) => {
                            self.$field = value.extract()?;
                            Ok(())
                        },)*
                        &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
                    }
                }

                #[cfg(debug_assertions)]
                #[classmethod]
                fn __pyi__(_: &Bound<'_, PyType>) -> String {
                    let mut res = String::new();

                    write!(&mut res, "class {}:\n", stringify!($name)).unwrap();
                    $(
                        write!(&mut res, "    {}: {}\n", stringify!($field), to_py_type::<$type>(String::new())).unwrap();
                    )*


                    write!(&mut res, "    def __init__(self").unwrap();
                    $(
                        write!(&mut res, ", {}: {}", stringify!($field), to_py_type::<$type>(String::new())).unwrap();
                    )*
                    write!(&mut res, "):\n        ...\n").unwrap();
                    write!(&mut res, "    ...\n").unwrap();

                    res
                }

                fn __hash__(&self) -> u64 {
                    let mut s = DefaultHasher::new();
                    Hash::hash(&self, &mut s);
                    s.finish()
                }

                fn __eq__(&self, other: &Self) -> bool {
                    self == other
                }
            }

            impl From<&horned_owl::model::$name<ArcStr>> for $name {
                fn from(value: &horned_owl::model::$name<ArcStr>) -> Self {
                    $name {
                        $($field: IntoCompatible::<$type>::into_c(value.$field.borrow()),)*
                    }
                }
            }


            impl From<&$name> for horned_owl::model::$name<ArcStr> {
                fn from(value: &$name) -> Self {
                    horned_owl::model::$name::<ArcStr> {
                        $($field: value.$field.borrow().into_c(),)*
                    }
                }
            }

            wrapped_base! {$name}
        }

    };
    (pub struct $name:ident ( pub $type0:ty $(, pub $type1:ty)?)) => { paste! {

        #[doc = concat!(
            stringify!($name),
            "(first: ",
            stringify!($type0),
            $(concat!("second: ", stringify!($type1)),)?
            ")",
            "\n\n",
            doc!($name)
        )]
        #[pyclass(module="pyhornedowl.model")]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name (
            #[pyo3(get,set,name="first")]
            pub $type0,
            $(
                #[pyo3(get,set,name="second")]
                pub $type1,
            )?
        );

        #[pymethods]
        impl $name {
            #[new]
            fn new(first: $type0$(, second: $type1)?) -> Self {
                $name (
                    first,
                    $(cond! (second, $type1))?
                )
            }

            #[cfg(debug_assertions)]
            #[classmethod]
            fn __pyi__(_: &Bound<'_, PyType>) -> String {
                let mut res = String::new();

                write!(&mut res, "class {}:\n", stringify!($name)).unwrap();
                write!(&mut res, "    first: {}\n", to_py_type::<$type0>(String::new())).unwrap();
                $(
                    write!(&mut res, "    second: {}\n", to_py_type::<$type1>(String::new())).unwrap();
                )?

                write!(&mut res, "    def __init__(self").unwrap();
                write!(&mut res, ", first: {}", to_py_type::<$type0>(String::new())).unwrap();
                $(
                    write!(&mut res, ", second: {}", to_py_type::<$type1>(String::new())).unwrap();
                )?
                write!(&mut res, "):\n        ...\n").unwrap();
                write!(&mut res, "    ...\n").unwrap();

                res
            }

            fn __hash__(&self) -> u64 {
                let mut s = DefaultHasher::new();
                Hash::hash(&self, &mut s);
                s.finish()
            }

            fn __eq__(&self, other: &Self) -> bool {
                self == other
            }
        }

        impl From<&horned_owl::model::$name<ArcStr>> for $name {
            fn from(value: &horned_owl::model::$name<ArcStr>) -> Self {

                $name (
                    IntoCompatible::<$type0>::into_c(&value.0),
                    $(IntoCompatible::<$type1>::into_c(&value.1))?
                )
            }
        }

        impl From<&$name> for horned_owl::model::$name<ArcStr> {
            fn from(value: &$name) -> Self {
                horned_owl::model::$name::<ArcStr> (
                    IntoCompatible::into_c(&value.0),
                    $(cond! (IntoCompatible::into_c(&value.1), $type1))?
                )
            }
        }

        wrapped_base! {$name}

    }};
    (transparent pub enum $name:ident {
        $($v_name:ident ( $field:ty ),)*
    }) => {
        #[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
            $(
                #[pyo3(transparent)]
                $v_name ($field),
            )*
        }

        impl ToPyi for $name {
            #[allow(unused_assignments)]
            fn pyi(module: Option<String>) -> String {
                let mut res = String::new();
                let mut first = true;

                let m = module.map(|c| format!("{}.", c)).unwrap_or_default();

                write!(&mut res, "typing.Union[").unwrap();
                $(

                    if (first) {
                        first = false;
                        write!(&mut res, "{}", to_py_type::<$field>(m.clone())).unwrap();
                    } else {
                        write!(&mut res, ", {}", to_py_type::<$field>(m.clone())).unwrap();
                    }
                )*

                write!(&mut res, "]\n").unwrap();

                res
            }
        }

        impl IntoPy<PyObject> for $name {
            fn into_py(self, py: Python<'_>) -> PyObject {
                match self {
                    $($name::$v_name(inner) => inner.into_py(py),)*
                }
            }
        }

        impl From<&$name> for horned_owl::model::$name<ArcStr> {
            fn from(value: &$name) -> Self {
                match value {
                    $($name::$v_name(inner) => horned_owl::model::$name::$v_name(inner.into()),)*
                }
            }
        }

        impl From<&horned_owl::model::$name<ArcStr>> for $name {

            fn from(value: &horned_owl::model::$name<ArcStr>) -> Self {
                match value {
                    $(horned_owl::model::$name::$v_name(inner) => $name::$v_name(inner.into()),)*
                }
            }
        }

        wrapped_base! {$name}
    };
    (pub enum $name:ident {
        $(
            $(#[transparent] $v_name_transparent:ident ( $field_transparent:ty ))?
            $($v_name:ident $(( $field_t0:ty$(, $field_t1:ty)? ))?$({ $($field_s:ident : $type_s:ty,)+ })?)?
            ,
        )*
    }) => {
        wrapped_enum! {
            pub enum $name {
                $(
                    $(#[transparent] $v_name_transparent ( $field_transparent ))?
                    $($v_name as $v_name $(( $field_t0 $(, $field_t1)? ))?$({ $($field_s : $type_s,)+ })?)?
                    ,
                )*
            }
        }
    };

    (#[suffixed] pub enum $name:ident {
        $(
            $v_name:ident $(( $field_t0:ty$(, $field_t1:ty)? ))?$({ $($field_s:ident : $type_s:ty,)+ })?
            ,
        )*
    }) => {
        paste! {
            wrapped_enum! {
                pub enum $name {
                    $(
                        $v_name as [<$v_name $name>] $(( $field_t0 $(, $field_t1)? ))?$({ $($field_s : $type_s,)+ })?,
                    )*
                }
            }
        }
    }
}

trait FromCompatible<T> {
    fn from_c(value: T) -> Self;
}

trait IntoCompatible<T> {
    fn into_c(self) -> T;
}

impl<T, U> IntoCompatible<U> for T
    where
        U: FromCompatible<T>,
{
    fn into_c(self) -> U {
        U::from_c(self)
    }
}

impl FromCompatible<&Arc<str>> for StringWrapper {
    fn from_c(value: &Arc<str>) -> Self {
        StringWrapper::from(value)
    }
}

impl FromCompatible<&StringWrapper> for Arc<str> {
    fn from_c(value: &StringWrapper) -> Self {
        Arc::<str>::from(value)
    }
}

impl FromCompatible<&String> for String {
    fn from_c(value: &String) -> Self {
        String::from(value)
    }
}

impl FromCompatible<&horned_owl::model::IRI<Arc<str>>> for IRI {
    fn from_c(value: &horned_owl::model::IRI<Arc<str>>) -> Self {
        IRI::from(value)
    }
}

impl FromCompatible<&IRI> for horned_owl::model::IRI<Arc<str>> {
    fn from_c(value: &IRI) -> Self {
        horned_owl::model::IRI::<Arc<str>>::from(value)
    }
}

impl FromCompatible<horned_owl::model::IRI<Arc<str>>> for IRI {
    fn from_c(value: horned_owl::model::IRI<Arc<str>>) -> Self {
        IRI::from(value.borrow())
    }
}

impl FromCompatible<IRI> for horned_owl::model::IRI<Arc<str>> {
    fn from_c(value: IRI) -> Self {
        horned_owl::model::IRI::<Arc<str>>::from(value.borrow())
    }
}

impl FromCompatible<&horned_owl::vocab::Facet> for Facet {
    fn from_c(value: &horned_owl::vocab::Facet) -> Self {
        Facet::from(value)
    }
}

impl FromCompatible<Facet> for horned_owl::vocab::Facet {
    fn from_c(value: Facet) -> Self {
        From::from(value.borrow())
    }
}

impl FromCompatible<&Facet> for horned_owl::vocab::Facet {
    fn from_c(value: &Facet) -> Self {
        horned_owl::vocab::Facet::from(value)
    }
}

impl FromCompatible<horned_owl::vocab::Facet> for Facet {
    fn from_c(value: horned_owl::vocab::Facet) -> Self {
        Facet::from(value.borrow())
    }
}

impl FromCompatible<&u32> for u32 {
    fn from_c(value: &u32) -> Self {
        value.clone()
    }
}

impl<'a, T: 'a, U> FromCompatible<&'a Option<T>> for Option<U>
    where
        U: FromCompatible<&'a T> {
    fn from_c(value: &'a Option<T>) -> Self {
        match value {
            None => None,
            Some(x) => Some(U::from_c(x))
        }
    }
}

impl<U, V, S, T> FromCompatible<(S, T)> for (U, V)
    where
        U: FromCompatible<S>,
        V: FromCompatible<T>
{
    fn from_c(value: (S, T)) -> Self {
        match value {
            (s, t) => (U::from_c(s), V::from_c(t))
        }
    }
}

impl<'a, U, V, S, T> FromCompatible<&'a (S, T)> for (U, V)
    where
        U: FromCompatible<&'a S>,
        V: FromCompatible<&'a T>
{
    fn from_c(value: &'a (S, T)) -> Self {
        match value {
            (s, t) => (U::from_c(s), V::from_c(t))
        }
    }
}

impl FromCompatible<&BTreeSet<horned_owl::model::Annotation<Arc<str>>>>
for BTreeSetWrap<Annotation>
{
    fn from_c(value: &BTreeSet<horned_owl::model::Annotation<Arc<str>>>) -> Self {
        BTreeSetWrap::<Annotation>::from(value)
    }
}

impl FromCompatible<&BTreeSetWrap<Annotation>>
for BTreeSet<horned_owl::model::Annotation<Arc<str>>>
{
    fn from_c(value: &BTreeSetWrap<Annotation>) -> Self {
        BTreeSet::<horned_owl::model::Annotation<Arc<str>>>::from(value)
    }
}

impl FromCompatible<BTreeSet<horned_owl::model::Annotation<Arc<str>>>>
for BTreeSetWrap<Annotation>
{
    fn from_c(value: BTreeSet<horned_owl::model::Annotation<Arc<str>>>) -> Self {
        FromCompatible::from_c(value.borrow())
    }
}

impl FromCompatible<BTreeSetWrap<Annotation>>
for BTreeSet<horned_owl::model::Annotation<Arc<str>>>
{
    fn from_c(value: BTreeSetWrap<Annotation>) -> Self {
        FromCompatible::from_c(value.borrow())
    }
}

trait ToPyi {
    fn pyi(module: Option<String>) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecWrap<T>(Vec<T>);

impl<T> From<Vec<T>> for VecWrap<T> {
    fn from(value: Vec<T>) -> Self {
        VecWrap(value)
    }
}

impl<T> From<VecWrap<T>> for Vec<T> {
    fn from(value: VecWrap<T>) -> Self {
        value.0
    }
}

impl<'source, T: FromPyObject<'source>> FromPyObject<'source> for VecWrap<T> {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract().map(VecWrap)
    }
}

impl<T: IntoPy<pyo3::PyObject>> IntoPy<pyo3::PyObject> for VecWrap<T> {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.0.into_py(py)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoxWrap<T>(Box<T>);

impl<'source, T: FromPyObject<'source>> FromPyObject<'source> for BoxWrap<T> {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract::<T>().map(Box::new).map(BoxWrap)
    }
}

impl<T: IntoPy<pyo3::PyObject>> IntoPy<pyo3::PyObject> for BoxWrap<T> {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        (*self.0).into_py(py)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringWrapper(String);

impl From<String> for StringWrapper {
    fn from(value: String) -> Self {
        StringWrapper(value)
    }
}

impl From<StringWrapper> for String {
    fn from(value: StringWrapper) -> Self {
        value.0
    }
}

impl From<&Arc<str>> for StringWrapper {
    fn from(value: &Arc<str>) -> Self {
        StringWrapper(value.to_string())
    }
}

impl From<&StringWrapper> for Arc<str> {
    fn from(value: &StringWrapper) -> Self {
        Arc::<str>::from(value.0.clone())
    }
}

impl IntoPy<pyo3::PyObject> for StringWrapper {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.0.into_py(py)
    }
}

impl<'source> FromPyObject<'source> for StringWrapper {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract().map(StringWrapper)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(module = "pyhornedowl.model")]
pub struct IRI(horned_owl::model::IRI<ArcStr>);

impl From<IRI> for horned_owl::model::IRI<ArcStr> {
    fn from(value: IRI) -> Self {
        value.0
    }
}

impl From<horned_owl::model::IRI<ArcStr>> for IRI {
    fn from(value: horned_owl::model::IRI<ArcStr>) -> Self {
        IRI(value)
    }
}

impl From<&IRI> for horned_owl::model::IRI<ArcStr> {
    fn from(value: &IRI) -> Self {
        value.0.clone()
    }
}

impl From<&horned_owl::model::IRI<ArcStr>> for IRI {
    fn from(value: &horned_owl::model::IRI<ArcStr>) -> Self {
        IRI(value.clone())
    }
}

impl From<IRI> for String {
    fn from(value: IRI) -> Self {
        value.0.to_string()
    }
}

#[pymethods]
impl IRI {
    pub fn __repr__(&self) -> String {
        format!("IRI.parse(\"{}\")", self.0)
    }
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    #[classmethod]
    pub fn parse(_: &Bound<'_, PyType>, value: String) -> Self {
        let builder = horned_owl::model::Build::new_arc();
        IRI(builder.iri(value))
    }
}

impl IRI {
    pub fn new<A: Borrow<str>>(iri: A, build: &horned_owl::model::Build<ArcStr>) -> Self {
        IRI(build.iri(iri))
    }
}

#[doc = doc!(Facet)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(module = "pyhornedowl.model")]
pub enum Facet {
    Length = 1,
    MinLength = 2,
    MaxLength = 3,
    Pattern = 4,
    MinInclusive = 5,
    MinExclusive = 6,
    MaxInclusive = 7,
    MaxExclusive = 8,
    TotalDigits = 9,
    FractionDigits = 10,
    LangRange = 11,
}

#[cfg(debug_assertions)]
#[pymethods]
impl Facet {
    #[classmethod]
    fn __pyi__(_: &Bound<'_, PyType>) -> String {
        "class Facet:
    Length: Facet
    MinLength: Facet
    MaxLength: Facet
    Pattern: Facet
    MinInclusive: Facet
    MinExclusive: Facet
    MaxInclusive: Facet
    MaxExclusive: Facet
    TotalDigits: Facet
    FractionDigits: Facet
    LangRange: Facet
"
            .to_owned()
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&Facet> for horned_owl::vocab::Facet {
    fn from(value: &Facet) -> Self {
        match value {
            Facet::Length => horned_owl::vocab::Facet::Length,
            Facet::MinLength => horned_owl::vocab::Facet::MinLength,
            Facet::MaxLength => horned_owl::vocab::Facet::MaxLength,
            Facet::Pattern => horned_owl::vocab::Facet::Pattern,
            Facet::MinInclusive => horned_owl::vocab::Facet::MinInclusive,
            Facet::MinExclusive => horned_owl::vocab::Facet::MinExclusive,
            Facet::MaxInclusive => horned_owl::vocab::Facet::MaxInclusive,
            Facet::MaxExclusive => horned_owl::vocab::Facet::MaxExclusive,
            Facet::TotalDigits => horned_owl::vocab::Facet::TotalDigits,
            Facet::FractionDigits => horned_owl::vocab::Facet::FractionDigits,
            Facet::LangRange => horned_owl::vocab::Facet::LangRange,
        }
    }
}

impl From<&horned_owl::vocab::Facet> for Facet {
    fn from(value: &horned_owl::vocab::Facet) -> Self {
        match value {
            horned_owl::vocab::Facet::Length => Facet::Length,
            horned_owl::vocab::Facet::MinLength => Facet::MinLength,
            horned_owl::vocab::Facet::MaxLength => Facet::MaxLength,
            horned_owl::vocab::Facet::Pattern => Facet::Pattern,
            horned_owl::vocab::Facet::MinInclusive => Facet::MinInclusive,
            horned_owl::vocab::Facet::MinExclusive => Facet::MinExclusive,
            horned_owl::vocab::Facet::MaxInclusive => Facet::MaxInclusive,
            horned_owl::vocab::Facet::MaxExclusive => Facet::MaxExclusive,
            horned_owl::vocab::Facet::TotalDigits => Facet::TotalDigits,
            horned_owl::vocab::Facet::FractionDigits => Facet::FractionDigits,
            horned_owl::vocab::Facet::LangRange => Facet::LangRange,
        }
    }
}

impl From<Facet> for horned_owl::vocab::Facet {
    fn from(value: Facet) -> Self {
        value.borrow().into()
    }
}

impl From<horned_owl::vocab::Facet> for Facet {
    fn from(value: horned_owl::vocab::Facet) -> Self {
        (&value).into()
    }
}

named! { pub struct Class(pub IRI) }
named! { pub struct AnonymousIndividual(pub StringWrapper) }
named! { pub struct NamedIndividual(pub IRI) }
named! { pub struct ObjectProperty(pub IRI) }
named! { pub struct Datatype(pub IRI) }
named! { pub struct DataProperty(pub IRI) }

wrapped! { pub struct FacetRestriction {
    pub f: Facet,
    pub l: Literal,
} }

wrapped! {
    transparent
    pub enum Individual {
        Anonymous(AnonymousIndividual),
        Named(NamedIndividual),
    }
}

wrapped! {
    pub enum ObjectPropertyExpression {
        #[transparent] ObjectProperty(ObjectProperty),
        InverseObjectProperty(ObjectProperty),
    }
}

wrapped! {
    #[suffixed]
    pub enum Literal {
        Simple {
            literal: String,
        },
        Language {
            literal: String,
            lang: String,
        },
        Datatype {
            literal: String,
            datatype_iri: IRI,
        },
    }
}

wrapped! {
    pub enum DataRange {
        #[transparent] Datatype(Datatype),
        DataIntersectionOf(VecWrap<DataRange>),
        DataUnionOf(VecWrap<DataRange>),
        DataComplementOf(BoxWrap<DataRange>),
        DataOneOf(VecWrap<Literal>),
        DatatypeRestriction(Datatype, VecWrap<FacetRestriction>),
    }
}

wrapped! {
pub enum ClassExpression {
    #[transparent] Class(Class),
    ObjectIntersectionOf(VecWrap<ClassExpression>),
    ObjectUnionOf(VecWrap<ClassExpression>),
    ObjectComplementOf(BoxWrap<ClassExpression>),
    ObjectOneOf(VecWrap<Individual>),
    ObjectSomeValuesFrom {
        ope: ObjectPropertyExpression,
        bce: BoxWrap<ClassExpression>,
    },
    ObjectAllValuesFrom {
        ope: ObjectPropertyExpression,
        bce: BoxWrap<ClassExpression>,
    },
    ObjectHasValue {
        ope: ObjectPropertyExpression,
        i: Individual,
    },
    ObjectHasSelf(ObjectPropertyExpression),
    ObjectMinCardinality {
        n: u32,
        ope: ObjectPropertyExpression,
        bce: BoxWrap<ClassExpression>,
    },
    ObjectMaxCardinality {
        n: u32,
        ope: ObjectPropertyExpression,
        bce: BoxWrap<ClassExpression>,
    },
    ObjectExactCardinality {
        n: u32,
        ope: ObjectPropertyExpression,
        bce: BoxWrap<ClassExpression>,
    },
    DataSomeValuesFrom {
        dp: DataProperty,
        dr: DataRange,
    },
    DataAllValuesFrom {
        dp: DataProperty,
        dr: DataRange,
    },
    DataHasValue {
        dp: DataProperty,
        l: Literal,
    },
    DataMinCardinality {
        n: u32,
        dp: DataProperty,
        dr: DataRange,
    },
    DataMaxCardinality {
        n: u32,
        dp: DataProperty,
        dr: DataRange,
    },
    DataExactCardinality {
        n: u32,
        dp: DataProperty,
        dr: DataRange,
    },
}
}

wrapped! {
    transparent
    pub enum PropertyExpression {
        ObjectPropertyExpression(ObjectPropertyExpression),
        DataProperty(DataProperty),
        AnnotationProperty(AnnotationProperty),
    }
}

wrapped! {
    transparent
    pub enum AnnotationSubject {
        IRI(IRI),
        AnonymousIndividual(AnonymousIndividual),
    }
}

wrapped! {
    pub struct AnnotationProperty(pub IRI)
}

wrapped! {
    transparent
    pub enum AnnotationValue {
        Literal(Literal),
        IRI(IRI),
        AnonymousIndividual(AnonymousIndividual),
    }
}

wrapped! {
    pub struct Annotation {
        pub ap: AnnotationProperty,
        pub av: AnnotationValue,
    }
}

wrapped! {
    pub struct OntologyAnnotation(pub Annotation)
}

wrapped! {
    pub struct Import(pub IRI)
}

wrapped! {
    pub struct DeclareClass(pub Class)
}

wrapped! {
    pub struct DeclareObjectProperty(pub ObjectProperty)
}

wrapped! {
    pub struct DeclareAnnotationProperty(pub AnnotationProperty)
}

wrapped! {
    pub struct DeclareDataProperty(pub DataProperty)
}

wrapped! {
    pub struct DeclareNamedIndividual(pub NamedIndividual)
}

wrapped! {
    pub struct DeclareDatatype(pub Datatype)
}

wrapped! {
    pub struct SubClassOf {
        pub sup: ClassExpression,
        pub sub: ClassExpression,
    }
}

wrapped! {
    pub struct EquivalentClasses(pub VecWrap<ClassExpression>)
}

wrapped! {
    pub struct DisjointClasses(pub VecWrap<ClassExpression>)
}

wrapped! {
    pub struct DisjointUnion(pub Class, pub VecWrap<ClassExpression>)
}

wrapped! {
    transparent
    pub enum SubObjectPropertyExpression {
        ObjectPropertyChain(VecWrap<ObjectPropertyExpression>),
        ObjectPropertyExpression(ObjectPropertyExpression),
    }
}

wrapped! {
    pub struct SubObjectPropertyOf {
        pub sup: ObjectPropertyExpression,
        pub sub: SubObjectPropertyExpression,
    }
}

wrapped! {
    pub struct EquivalentObjectProperties(pub VecWrap<ObjectPropertyExpression>)
}

wrapped! {
    pub struct DisjointObjectProperties(pub VecWrap<ObjectPropertyExpression>)
}

wrapped! {
    pub struct InverseObjectProperties(pub ObjectProperty, pub ObjectProperty)
}

wrapped! {
    pub struct ObjectPropertyDomain {
        pub ope: ObjectPropertyExpression,
        pub ce: ClassExpression,
    }
}

wrapped! {
    pub struct ObjectPropertyRange {
        pub ope: ObjectPropertyExpression,
        pub ce: ClassExpression,
    }
}

wrapped! { pub struct FunctionalObjectProperty(pub ObjectPropertyExpression) }
wrapped! { pub struct InverseFunctionalObjectProperty(pub ObjectPropertyExpression) }
wrapped! { pub struct ReflexiveObjectProperty(pub ObjectPropertyExpression) }
wrapped! { pub struct IrreflexiveObjectProperty(pub ObjectPropertyExpression) }
wrapped! { pub struct SymmetricObjectProperty(pub ObjectPropertyExpression) }
wrapped! { pub struct AsymmetricObjectProperty(pub ObjectPropertyExpression) }
wrapped! { pub struct TransitiveObjectProperty(pub ObjectPropertyExpression) }

wrapped! {
    pub struct SubDataPropertyOf {
        pub sup: DataProperty,
        pub sub: DataProperty,
    }
}

wrapped! {
    pub struct EquivalentDataProperties(pub VecWrap<DataProperty>)
}

wrapped! {
    pub struct DisjointDataProperties(pub VecWrap<DataProperty>)
}

wrapped! {
    pub struct DataPropertyDomain {
        pub dp: DataProperty,
        pub ce: ClassExpression,
    }
}

wrapped! {
    pub struct DataPropertyRange {
    pub dp: DataProperty,
    pub dr: DataRange,
}
}

wrapped! {
    pub struct FunctionalDataProperty(pub DataProperty)
}

wrapped! {
    pub struct DatatypeDefinition {
    pub kind: Datatype,
    pub range: DataRange,
}
}

wrapped! {
    pub struct HasKey {
    pub ce: ClassExpression,
    pub vpe: VecWrap<PropertyExpression>,
}
}

wrapped! {
    pub struct SameIndividual(pub VecWrap<Individual>)
}

wrapped! {
    pub struct DifferentIndividuals(pub VecWrap<Individual>)
}

wrapped! {
    pub struct ClassAssertion {
    pub ce: ClassExpression,
    pub i: Individual,
}
}

wrapped! {
    pub struct ObjectPropertyAssertion {
    pub ope: ObjectPropertyExpression,
    pub from: Individual,
    pub to: Individual,
}
}

wrapped! {
    pub struct NegativeObjectPropertyAssertion {
    pub ope: ObjectPropertyExpression,
    pub from: Individual,
    pub to: Individual,
}
}

wrapped! {
    pub struct DataPropertyAssertion {
    pub dp: DataProperty,
    pub from: Individual,
    pub to: Literal,
}
}

wrapped! {
    pub struct NegativeDataPropertyAssertion {
    pub dp: DataProperty,
    pub from: Individual,
    pub to: Literal,
}
}

wrapped! {
    pub struct AnnotationAssertion {
    pub subject: AnnotationSubject,
    pub ann: Annotation,
}
}

wrapped! {
    pub struct SubAnnotationPropertyOf {
    pub sup: AnnotationProperty,
    pub sub: AnnotationProperty,
}
}

wrapped! {
    pub struct AnnotationPropertyDomain {
    pub ap: AnnotationProperty,
    pub iri: IRI,
}
}

wrapped! {
    pub struct AnnotationPropertyRange {
    pub ap: AnnotationProperty,
    pub iri: IRI,
}
}

wrapped! {
    pub struct DocIRI(pub IRI)
}

wrapped! {
    pub struct OntologyID {
        pub iri: Option<IRI>,
        pub viri: Option<IRI>,
    }
}

wrapped! {
    pub struct Variable(pub IRI)
}

wrapped! {
    transparent
    pub enum DArgument {
        Literal(Literal),
        Variable(Variable),
    }
}

wrapped! {
    transparent
    pub enum IArgument {
        Individual(Individual),
        Variable(Variable),
    }
}

wrapped! {
    pub enum Atom {
        BuiltInAtom {
            pred: IRI,
            args: VecWrap<DArgument>,
        },
        ClassAtom {
            pred: ClassExpression,
            arg: IArgument,
        },
        DataPropertyAtom {
            pred: DataProperty,
            args: (DArgument, DArgument),
        },
        DataRangeAtom {
            pred: DataRange,
            arg: DArgument,
        },
        DifferentIndividualsAtom(IArgument, IArgument),
        ObjectPropertyAtom {
            pred: ObjectPropertyExpression,
            args: (IArgument, IArgument),
        },
        SameIndividualAtom(IArgument, IArgument),
    }
}

wrapped! {
    pub struct Rule {
        pub head: VecWrap<Atom>,
        pub body: VecWrap<Atom>,
    }
}

wrapped! {
    transparent
    pub enum Component {
        OntologyID(OntologyID),
        DocIRI(DocIRI),
        OntologyAnnotation(OntologyAnnotation),
        Import(Import),
        DeclareClass(DeclareClass),
        DeclareObjectProperty(DeclareObjectProperty),
        DeclareAnnotationProperty(DeclareAnnotationProperty),
        DeclareDataProperty(DeclareDataProperty),
        DeclareNamedIndividual(DeclareNamedIndividual),
        DeclareDatatype(DeclareDatatype),
        SubClassOf(SubClassOf),
        EquivalentClasses(EquivalentClasses),
        DisjointClasses(DisjointClasses),
        DisjointUnion(DisjointUnion),
        SubObjectPropertyOf(SubObjectPropertyOf),
        EquivalentObjectProperties(EquivalentObjectProperties),
        DisjointObjectProperties(DisjointObjectProperties),
        InverseObjectProperties(InverseObjectProperties),
        ObjectPropertyDomain(ObjectPropertyDomain),
        ObjectPropertyRange(ObjectPropertyRange),
        FunctionalObjectProperty(FunctionalObjectProperty),
        InverseFunctionalObjectProperty(InverseFunctionalObjectProperty),
        ReflexiveObjectProperty(ReflexiveObjectProperty),
        IrreflexiveObjectProperty(IrreflexiveObjectProperty),
        SymmetricObjectProperty(SymmetricObjectProperty),
        AsymmetricObjectProperty(AsymmetricObjectProperty),
        TransitiveObjectProperty(TransitiveObjectProperty),
        SubDataPropertyOf(SubDataPropertyOf),
        EquivalentDataProperties(EquivalentDataProperties),
        DisjointDataProperties(DisjointDataProperties),
        DataPropertyDomain(DataPropertyDomain),
        DataPropertyRange(DataPropertyRange),
        FunctionalDataProperty(FunctionalDataProperty),
        DatatypeDefinition(DatatypeDefinition),
        HasKey(HasKey),
        SameIndividual(SameIndividual),
        DifferentIndividuals(DifferentIndividuals),
        ClassAssertion(ClassAssertion),
        ObjectPropertyAssertion(ObjectPropertyAssertion),
        NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion),
        DataPropertyAssertion(DataPropertyAssertion),
        NegativeDataPropertyAssertion(NegativeDataPropertyAssertion),
        AnnotationAssertion(AnnotationAssertion),
        SubAnnotationPropertyOf(SubAnnotationPropertyOf),
        AnnotationPropertyDomain(AnnotationPropertyDomain),
        AnnotationPropertyRange(AnnotationPropertyRange),
        Rule(Rule),
    }
}

wrapped! {
    pub struct AnnotatedComponent {
        pub component: Component,
        pub ann: BTreeSetWrap<Annotation>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTreeSetWrap<T>(BTreeSet<T>);

impl<T> From<BTreeSet<T>> for BTreeSetWrap<T> {
    fn from(value: BTreeSet<T>) -> Self {
        BTreeSetWrap(value)
    }
}

impl<T> From<BTreeSetWrap<T>> for BTreeSet<T> {
    fn from(value: BTreeSetWrap<T>) -> Self {
        value.0
    }
}

impl From<&BTreeSet<horned_owl::model::Annotation<ArcStr>>> for BTreeSetWrap<Annotation> {
    fn from(value: &BTreeSet<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BTreeSetWrap(value.iter().map(From::from).collect())
    }
}

impl From<&BTreeSetWrap<Annotation>> for BTreeSet<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: &BTreeSetWrap<Annotation>) -> Self {
        value.0.iter().map(From::from).collect()
    }
}

impl<'source> FromPyObject<'source> for BTreeSetWrap<Annotation> {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract::<BTreeSet<Annotation>>()
            .map(BTreeSetWrap::<Annotation>)
    }
}

impl IntoPy<pyo3::PyObject> for BTreeSetWrap<Annotation> {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.0.into_py(py)
    }
}

macro_rules! add_type_alias {
    ($py:ident, $module:ident, $($name:ident),*) => {
        {
            let locals = [("typing", &$py.import_bound("typing")?), ("m", &$module)].into_py_dict_bound($py);

            let mut code: String;
            let mut ta: Bound<'_, PyAny>;

            $(
                code = $name::pyi(Some("m".to_string()));
                ta = $py.eval_bound(&code, None, Some(&locals))?;
                locals.set_item(stringify!($name), &ta)?;
                ta.setattr("__doc__",  doc!($name))?;
                $module.add(stringify!($name), &ta)?;
            )*
        }
    };
}

pub fn py_module(py: Python<'_>) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new_bound(py, "model")?;

    // To get all members to export on the documentation website for horned_ows::model execute the following javascript command
    // console.log([...(await Promise.all(Array.from(document.querySelectorAll("a.enum")).filter(x => ["ClassExpression", "ObjectPropertyExpression", "Literal", "DataRange", ""].indexOf(x.innerText) >= 0).map(async a => { html = await(await fetch(a.href)).text(); doc = document.createElement("html"); doc.innerHTML=html; return Array.from(doc.querySelectorAll(".variant")).map(x => x.id.replace("variant.", "")); }))).flatMap(arr => arr.map(x => `module.add_class::<${ x }>()?;`)), ...Array.from(document.querySelectorAll("a.struct")).map(x=>x.innerText).filter(x => ["Build", "OntologyID"].indexOf(x) < 0).map(x => `module.add_class::<${ x }>()?;`)].join("\n"))
    module.add_class::<Class>()?;
    module.add_class::<ObjectIntersectionOf>()?;
    module.add_class::<ObjectUnionOf>()?;
    module.add_class::<ObjectComplementOf>()?;
    module.add_class::<ObjectOneOf>()?;
    module.add_class::<ObjectSomeValuesFrom>()?;
    module.add_class::<ObjectAllValuesFrom>()?;
    module.add_class::<ObjectHasValue>()?;
    module.add_class::<ObjectHasSelf>()?;
    module.add_class::<ObjectMinCardinality>()?;
    module.add_class::<ObjectMaxCardinality>()?;
    module.add_class::<ObjectExactCardinality>()?;
    module.add_class::<DataSomeValuesFrom>()?;
    module.add_class::<DataAllValuesFrom>()?;
    module.add_class::<DataHasValue>()?;
    module.add_class::<DataMinCardinality>()?;
    module.add_class::<DataMaxCardinality>()?;
    module.add_class::<DataExactCardinality>()?;
    module.add_class::<Datatype>()?;
    module.add_class::<DataIntersectionOf>()?;
    module.add_class::<DataUnionOf>()?;
    module.add_class::<DataComplementOf>()?;
    module.add_class::<DataOneOf>()?;
    module.add_class::<DatatypeRestriction>()?;
    module.add_class::<SimpleLiteral>()?;
    module.add_class::<LanguageLiteral>()?;
    module.add_class::<DatatypeLiteral>()?;
    module.add_class::<ObjectProperty>()?;
    module.add_class::<InverseObjectProperty>()?;
    module.add_class::<AnnotatedComponent>()?;
    module.add_class::<Annotation>()?;
    module.add_class::<AnnotationAssertion>()?;
    module.add_class::<AnnotationProperty>()?;
    module.add_class::<AnnotationPropertyDomain>()?;
    module.add_class::<AnnotationPropertyRange>()?;
    module.add_class::<AnonymousIndividual>()?;
    module.add_class::<AsymmetricObjectProperty>()?;
    module.add_class::<Class>()?;
    module.add_class::<ClassAssertion>()?;
    module.add_class::<DataProperty>()?;
    module.add_class::<DataPropertyAssertion>()?;
    module.add_class::<DataPropertyDomain>()?;
    module.add_class::<DataPropertyRange>()?;
    module.add_class::<Datatype>()?;
    module.add_class::<DatatypeDefinition>()?;
    module.add_class::<DeclareAnnotationProperty>()?;
    module.add_class::<DeclareClass>()?;
    module.add_class::<DeclareDataProperty>()?;
    module.add_class::<DeclareDatatype>()?;
    module.add_class::<DeclareNamedIndividual>()?;
    module.add_class::<DeclareObjectProperty>()?;
    module.add_class::<DifferentIndividuals>()?;
    module.add_class::<DisjointClasses>()?;
    module.add_class::<DisjointDataProperties>()?;
    module.add_class::<DisjointObjectProperties>()?;
    module.add_class::<DisjointUnion>()?;
    module.add_class::<EquivalentClasses>()?;
    module.add_class::<EquivalentDataProperties>()?;
    module.add_class::<EquivalentObjectProperties>()?;
    module.add_class::<FacetRestriction>()?;
    module.add_class::<FunctionalDataProperty>()?;
    module.add_class::<FunctionalObjectProperty>()?;
    module.add_class::<HasKey>()?;
    module.add_class::<IRI>()?;
    module.add_class::<Import>()?;
    module.add_class::<InverseFunctionalObjectProperty>()?;
    module.add_class::<InverseObjectProperties>()?;
    module.add_class::<IrreflexiveObjectProperty>()?;
    module.add_class::<NamedIndividual>()?;
    module.add_class::<NegativeDataPropertyAssertion>()?;
    module.add_class::<NegativeObjectPropertyAssertion>()?;
    module.add_class::<ObjectProperty>()?;
    module.add_class::<ObjectPropertyAssertion>()?;
    module.add_class::<ObjectPropertyDomain>()?;
    module.add_class::<ObjectPropertyRange>()?;
    module.add_class::<OntologyAnnotation>()?;
    module.add_class::<ReflexiveObjectProperty>()?;
    module.add_class::<SameIndividual>()?;
    module.add_class::<SubAnnotationPropertyOf>()?;
    module.add_class::<SubClassOf>()?;
    module.add_class::<SubDataPropertyOf>()?;
    module.add_class::<SubObjectPropertyOf>()?;
    module.add_class::<SymmetricObjectProperty>()?;
    module.add_class::<TransitiveObjectProperty>()?;
    module.add_class::<OntologyID>()?;
    module.add_class::<DocIRI>()?;
    module.add_class::<Rule>()?;
    module.add_class::<Variable>()?;
    module.add_class::<BuiltInAtom>()?;
    module.add_class::<ClassAtom>()?;
    module.add_class::<DataPropertyAtom>()?;
    module.add_class::<DataRangeAtom>()?;
    module.add_class::<DifferentIndividualsAtom>()?;
    module.add_class::<ObjectPropertyAtom>()?;
    module.add_class::<SameIndividualAtom>()?;

    module.add_class::<Facet>()?;

    add_type_alias!(py, module,
        ClassExpression,
        ObjectPropertyExpression,
        SubObjectPropertyExpression,
        Literal,
        DataRange,
        Individual,
        PropertyExpression,
        AnnotationSubject,
        AnnotationValue,
        Component,
        Atom,
        IArgument,
        DArgument
    );

    Ok(module)
}
