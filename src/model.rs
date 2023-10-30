use std::{collections::BTreeSet, sync::Arc};

use horned_owl::model::ArcStr;

use pyo3::prelude::*;

use paste::paste;


macro_rules! cond {
    ($x:ident, $($_:tt)+) => {
        $x
    };
    ($x:ty, $($_:tt)+) => {
        $x
    };
    ($x:expr, $($_:tt)+) => {
        $x
    };
}

macro_rules! wrapped_base {
    ($name:ident) => {
        impl From<&horned_owl::model::$name<ArcStr>> for $name {
            fn from(value: &horned_owl::model::$name<ArcStr>) -> Self {
                value.into()
            }
        }


        impl From<&$name> for horned_owl::model::$name<ArcStr> {
            fn from(value: &$name) -> Self {
                value.into()
            }
        }

        impl From<BoxWrap<$name>> for Box<horned_owl::model::$name<ArcStr>> {
            fn from(value: BoxWrap<$name>) -> Self {
                Box::new((*value.0).into())
            }
        }

        impl From<Box<horned_owl::model::$name<ArcStr>>> for BoxWrap<$name> {
            fn from(value: Box<horned_owl::model::$name<ArcStr>>) -> Self {
                BoxWrap(Box::new((*value).into()))
            }
        }

        impl From<VecWrap<$name>> for Vec<horned_owl::model::$name<ArcStr>> {
            fn from(value: VecWrap<$name>) -> Self {
                value
                    .0
                    .into_iter()
                    .map(horned_owl::model::$name::<ArcStr>::from)
                    .collect()
            }
        }

        impl From<Vec<horned_owl::model::$name<ArcStr>>> for VecWrap<$name> {
            fn from(value: Vec<horned_owl::model::$name<ArcStr>>) -> Self {
                VecWrap(value.into_iter().map($name::from).collect())
            }
        }
    };
}

macro_rules! wrapped {
    (pub struct $name:ident { $(pub $field:ident: $type:ty,)* }) => {
        paste! {
            #[pyclass(module="pyhornedowl.model")]
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name {
                $(
                    #[pyo3(get,set)]
                    $field: $type,
                )*
            }

            struct [<$name Wrap>](horned_owl::model::$name<ArcStr>);

            impl From<horned_owl::model::$name<ArcStr>> for $name {
                fn from(value: horned_owl::model::$name<ArcStr>) -> Self {
                    $name {
                        $($field: value.$field.into()),*
                    }
                }
            }

            impl IntoPy<PyObject> for [<$name Wrap>] {
                fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
                    $name::from(self.0).into_py(py)
                }
            }


            impl From<$name> for horned_owl::model::$name<ArcStr> {
                fn from(value: $name) -> Self {
                    horned_owl::model::$name::<ArcStr> {
                        $($field: value.$field.into(),)*
                    }
                }
            }

            impl <'source> FromPyObject<'source> for [<$name Wrap>] {
                fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
                    let local = $name::extract(ob)?;

                    Ok([<$name Wrap>](local.into()))
                }
            }

            wrapped_base! {$name}
        }

    };
    (pub struct $name:ident ( pub $type0:ty $(, pub $type1:ty)?)) => { paste! {
        #[pyclass(module="pyhornedowl.model")]
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name (
            #[pyo3(get,set,name="inner")]
            $type0,
            $(
                #[pyo3(get,set,name="inner")]
                $type1,
            )?
        );

        struct [<$name Wrap>](horned_owl::model::$name<ArcStr>);

        impl From<horned_owl::model::$name<ArcStr>> for $name {
            fn from(value: horned_owl::model::$name<ArcStr>) -> Self {
                $name (
                    value.0.into(),
                    $(cond! (value.1.into(), $type1))?
                )
            }
        }

        impl IntoPy<PyObject> for [<$name Wrap>] {
            fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
                $name::from(self.0).into_py(py)
            }
        }

        impl From<$name> for horned_owl::model::$name<ArcStr> {
            fn from(value: $name) -> Self {
                horned_owl::model::$name::<ArcStr> (
                    value.0.into(),
                    $(cond! (value.1.into(), $type1))?
                )
            }
        }

        impl <'source> FromPyObject<'source> for [<$name Wrap>] {
            fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
                let local = $name::extract(ob)?;
                Ok([<$name Wrap>](local.into()))
            }
        }

        wrapped_base! {$name}

    }};
    (transparent pub enum $name:ident {
        $($v_name:ident ( $field:ty ),)*
    }) => {
        #[derive(FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $(
                #[pyo3(transparent)]
                $v_name ($field),
            )*
        }

        impl IntoPy<PyObject> for $name {
            fn into_py(self, py: Python<'_>) -> PyObject {
                match self {
                    $($name::$v_name(inner) => inner.into_py(py),)*
                }
            }
        }

        impl From<$name> for horned_owl::model::$name<ArcStr> {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$v_name(inner) => horned_owl::model::$name::$v_name(inner.into()),)*
                }
            }
        }

        impl From<horned_owl::model::$name<ArcStr>> for $name {
            fn from(value: horned_owl::model::$name<ArcStr>) -> Self {
                match value {
                    $(horned_owl::model::$name::$v_name(inner) => $name::$v_name(inner.into()),)*
                }
            }
        }
        
        wrapped_base! {$name}
    };
    (pub enum $name:ident {
        $($v_name:ident $(( $field_t0:ty$(, $field_t1:ty)? ))?$({ $($field_s:ident : $type_s:ty,)+ })?,)*
    }) => {
        paste! {
            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
            enum [<$name _ Inner>] {
                $(
                    $v_name([<$name _ $v_name>]),
                )*
            }

            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name([<$name _ Inner>]);

            $(
                #[allow(non_camel_case_types)]
                // #[pyclass(extends=$name)]
                #[pyclass(module="pyhornedowl.model")]
                #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
                pub struct [<$name _ $v_name>]
                    $((
                        #[pyo3(get,set,name="first")]
                        $field_t0
                        $(,
                            #[pyo3(get,set,name="second")]
                            $field_t1
                        )?
                    );)?
                    $({
                        $(
                            #[pyo3(get,set)]
                            $field_s: $type_s,
                        )*
                    })?
            )*

            impl From<horned_owl::model::$name<ArcStr>> for $name {
                fn from(value: horned_owl::model::$name<ArcStr>) -> Self {
                    match value {
                        $($(
                            horned_owl::model::$name::$v_name::<ArcStr>(f0 $(, cond!(f1, $field_t1))?) => $name(
                                [<$name _ Inner>]::$v_name([<$name _ $v_name>](
                                f0.into() $(, cond!(f1.into(), $field_t1))?
                            ))),
                        )?)*
                        $($(
                            horned_owl::model::$name::$v_name::<ArcStr>{
                                $($field_s, )*
                            } => $name([<$name _ Inner>]::$v_name([<$name _ $v_name>]{
                                $($field_s: $field_s.into(),)*
                            })),
                        )?)*
                    }
                }
            }
            impl IntoPy<pyo3::PyObject> for $name {
                fn into_py(self, py: pyo3::Python) -> pyo3::PyObject {
                    match self.0 {
                        $(
                            [<$name _ Inner>]::$v_name(val) => {
                                val.into_py(py)
                            },
                        )*
                    }
                }
            }

            impl From<$name> for horned_owl::model::$name<ArcStr> {
                fn from(value: $name) -> Self {
                    match value.0 {
                        $($(
                            [<$name _ Inner>]::$v_name([<$name _ $v_name>](f0 $(, cond!(f1, $field_t1))?)) => horned_owl::model::$name::<ArcStr>::$v_name(f0.into() $(, cond!(f1.into(), $field_t1))?),
                        )?)*

                        $($(
                            [<$name _ Inner>]::$v_name([<$name _ $v_name>]{
                                $($field_s, )*
                            }) => horned_owl::model::$name::<ArcStr>::$v_name{
                                $($field_s: $field_s.into(),)*
                            },
                        )?)*
                    }
                }
            }

            impl <'source> FromPyObject<'source> for $name {
                fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
                    $(
                        {
                            let r = [<$name _ $v_name>]::extract(ob);
                            if r.is_ok() {
                                let local = r.unwrap();
                                let inner = [<$name _ Inner>]::$v_name(local);
                                return Ok($name(inner));
                            }
                        }
                    )*

                    Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to $name"))
                }
            }

            wrapped_base! {$name}
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BoxWrap<T>(Box<T>);

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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[pyclass(module="pyhornedowl.model")]
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

#[pymethods]
impl IRI {
    fn __repr__(&self) -> String {
        format!("IRI({})", self.0)
    }
    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StringWrapper(String);

impl From<Arc<str>> for StringWrapper {
    fn from(value: Arc<str>) -> Self {
        StringWrapper(value.to_string())
    }
}

impl From<StringWrapper> for Arc<str> {
    fn from(value: StringWrapper) -> Self {
        Arc::<str>::from(value.0)
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

wrapped! { pub struct Class(pub IRI) }
wrapped! { pub struct AnonymousIndividual(pub StringWrapper) }
wrapped! { pub struct NamedIndividual(pub IRI) }
wrapped! { pub struct ObjectProperty(pub IRI) }
wrapped! { pub struct Datatype(pub IRI) }
wrapped! { pub struct DataProperty(pub IRI) }
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
    ObjectProperty(ObjectProperty),
    InverseObjectProperty(ObjectProperty),
}
}

wrapped! {
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
        Datatype(Datatype),
        DataIntersectionOf(VecWrap<DataRange>),
        DataUnionOf(VecWrap<DataRange>),
        DataComplementOf(BoxWrap<DataRange>),
        DataOneOf(VecWrap<Literal>),
        DatatypeRestriction(Datatype, VecWrap<FacetRestriction>),
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
#[pyclass(module="pyhornedowl.model")]
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

// impl<'source> pyo3::FromPyObject<'source> for Facet {
//     fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
//         let py_str = ob.str()?;
//         let s: &str = py_str.extract()?;

//         match s {
//             "Length" => Ok(Facet::Length),
//             "MinLength" => Ok(Facet::MinLength),
//             "MaxLength" => Ok(Facet::MaxLength),
//             "Pattern" => Ok(Facet::Pattern),
//             "MinInclusive" => Ok(Facet::MinInclusive),
//             "MinExclusive" => Ok(Facet::MinExclusive),
//             "MaxInclusive" => Ok(Facet::MaxInclusive),
//             "MaxExclusive" => Ok(Facet::MaxExclusive),
//             "TotalDigits" => Ok(Facet::TotalDigits),
//             "FractionDigits" => Ok(Facet::FractionDigits),
//             "LangRange" => Ok(Facet::LangRange),
//             &_ => Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>(
//                 format!("Invalid facet '{s}'!"),
//             )),
//         }
//     }
// }

// impl pyo3::IntoPy<pyo3::PyObject> for Facet {
//     fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
//         let s = match self {
//             Facet::Length => "Length",
//             Facet::MinLength => "MinLength",
//             Facet::MaxLength => "MaxLength",
//             Facet::Pattern => "Pattern",
//             Facet::MinInclusive => "MinInclusive",
//             Facet::MinExclusive => "MinExclusive",
//             Facet::MaxInclusive => "MaxInclusive",
//             Facet::MaxExclusive => "MaxExclusive",
//             Facet::TotalDigits => "TotalDigits",
//             Facet::FractionDigits => "FractionDigits",
//             Facet::LangRange => "LangRange",
//         };

//         s.into_py(py)
//     }
// }

impl From<Facet> for horned_owl::model::Facet {
    fn from(value: Facet) -> Self {
        match value {
            Facet::Length => horned_owl::model::Facet::Length,
            Facet::MinLength => horned_owl::model::Facet::MinLength,
            Facet::MaxLength => horned_owl::model::Facet::MaxLength,
            Facet::Pattern => horned_owl::model::Facet::Pattern,
            Facet::MinInclusive => horned_owl::model::Facet::MinInclusive,
            Facet::MinExclusive => horned_owl::model::Facet::MinExclusive,
            Facet::MaxInclusive => horned_owl::model::Facet::MaxInclusive,
            Facet::MaxExclusive => horned_owl::model::Facet::MaxExclusive,
            Facet::TotalDigits => horned_owl::model::Facet::TotalDigits,
            Facet::FractionDigits => horned_owl::model::Facet::FractionDigits,
            Facet::LangRange => horned_owl::model::Facet::LangRange,
        }
    }
}
impl From<horned_owl::model::Facet> for Facet {
    fn from(value: horned_owl::model::Facet) -> Self {
        match value {
            horned_owl::model::Facet::Length => Facet::Length,
            horned_owl::model::Facet::MinLength => Facet::MinLength,
            horned_owl::model::Facet::MaxLength => Facet::MaxLength,
            horned_owl::model::Facet::Pattern => Facet::Pattern,
            horned_owl::model::Facet::MinInclusive => Facet::MinInclusive,
            horned_owl::model::Facet::MinExclusive => Facet::MinExclusive,
            horned_owl::model::Facet::MaxInclusive => Facet::MaxInclusive,
            horned_owl::model::Facet::MaxExclusive => Facet::MaxExclusive,
            horned_owl::model::Facet::TotalDigits => Facet::TotalDigits,
            horned_owl::model::Facet::FractionDigits => Facet::FractionDigits,
            horned_owl::model::Facet::LangRange => Facet::LangRange,
        }
    }
}

wrapped! {
pub enum ClassExpression {
    Class(Class),
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
    transparent
    pub enum Axiom {
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
    }
}

wrapped! {
    pub struct AnnotatedAxiom {
        pub axiom: Axiom,
        pub ann: BTreeSetWrap<Annotation>,
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BTreeSetWrap<T>(BTreeSet<T>);

impl From<BTreeSet<horned_owl::model::Annotation<ArcStr>>> for BTreeSetWrap<Annotation> {
    fn from(value: BTreeSet<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BTreeSetWrap(value.into_iter().map(From::from).collect())
    }
}

impl From<BTreeSetWrap<Annotation>> for BTreeSet<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: BTreeSetWrap<Annotation>) -> Self {
        value.0.into_iter().map(From::from).collect()
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

pub fn py_module(py: Python<'_>) -> PyResult<&PyModule> {
    let module = PyModule::new(py, "model")?;

    // To get all members to export on the documentation website for horned_ows::model execute the following javascript command 
    // console.log(Array.from(document.querySelectorAll("a.struct")).map(x=>x.innerText).filter(x => x != "Build" && x != "OntologyId").map(x => `module.add_class::<${ x.innerText }>()?;`).join("\n"))


    module.add_class::<AnnotatedAxiom>()?;
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

    Ok(module)
}