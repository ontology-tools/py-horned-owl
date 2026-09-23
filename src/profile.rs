//! OWL 2 profile (EL/QL/RL/DL) conformance checking.
//!
//! Thin PyO3 wrapper over the [`horned_profile`] crate, which checks an
//! ontology's horned-owl model directly (no RDF/quad scan).

use crate::model::{
    AnnotatedComponent, ClassExpression, DataRange, ObjectProperty, ObjectPropertyExpression, IRI,
};
use crate::wrappers::{FromCompatible, IntoCompatible, VecWrap};
use crate::PyIndexedOntology;
use horned_owl::model::ArcStr;
use horned_profile::{Profile, Violation};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::str::FromStr;

/// OWL 2 profiles
///
/// OWL2DL: The [OWL 2 DL profile](https://www.w3.org/TR/owl2-syntax/#Global_Restrictions_on_Axioms_in_OWL_2_DL).
/// EL: The [OWL 2 EL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_EL).
/// QL: The [OWL 2 QL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_QL).
/// RL: The [OWL 2 RL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_RL).
#[pyclass(
    eq,
    eq_int,
    hash,
    frozen,
    name = "Profile",
    module = "pyhornedowl.profile",
    skip_from_py_object
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PyProfile {
    OWL2DL,
    EL,
    QL,
    RL,
}

impl From<Profile> for PyProfile {
    fn from(p: Profile) -> Self {
        match p {
            Profile::OWL2DL => PyProfile::OWL2DL,
            Profile::EL => PyProfile::EL,
            Profile::QL => PyProfile::QL,
            Profile::RL => PyProfile::RL,
        }
    }
}

impl From<PyProfile> for Profile {
    fn from(p: PyProfile) -> Self {
        match p {
            PyProfile::OWL2DL => Profile::OWL2DL,
            PyProfile::EL => Profile::EL,
            PyProfile::QL => Profile::QL,
            PyProfile::RL => Profile::RL,
        }
    }
}

impl FromStr for PyProfile {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "owl2dl" | "dl" => Ok(PyProfile::OWL2DL),
            "el" => Ok(PyProfile::EL),
            "ql" => Ok(PyProfile::QL),
            "rl" => Ok(PyProfile::RL),
            other => Err(PyValueError::new_err(format!("unknown profile '{other}'"))),
        }
    }
}

impl<'a, 'py> FromPyObject<'a, 'py> for PyProfile {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        // Cast rather than `obj.extract::<PyProfile>()`: this *is* the
        // `FromPyObject` impl for `PyProfile`, so extracting would recurse.
        match obj.cast::<PyProfile>() {
            Ok(profile) => Ok(*profile.get()),
            Err(_) => PyProfile::from_str(&obj.extract::<String>()?),
        }
    }
}

#[pymethods]
impl PyProfile {
    fn __str__(&self) -> &'static str {
        match self {
            PyProfile::OWL2DL => "OWL2DL",
            PyProfile::EL => "EL",
            PyProfile::QL => "QL",
            PyProfile::RL => "RL",
        }
    }

    fn __repr__(&self) -> String {
        format!("Profile.{}", self.__str__())
    }
}


impl FromCompatible<&Vec<&'static str>> for VecWrap<String> {
    fn from_c(value: &Vec<&'static str>) -> Self {
        VecWrap(value.iter().map(|s| s.to_string()).collect())
    }
}

impl FromCompatible<&&'static str> for String {
    fn from_c(value: &&'static str) -> Self {
        value.to_string()
    }
}

/// Base class for profile violations
#[pyclass(
    subclass,
    name = "Violation",
    module = "pyhornedowl.profile",
    skip_from_py_object
)]
pub struct PyViolation {}

macro_rules! violation_field {
    (ty  axiom) => { AnnotatedComponent };
    (py  axiom) => { "AnnotatedComponent" };
    (doc axiom) => { "The axiom the violation was found in." };

    (ty  ce) => { ClassExpression };
    (py  ce) => { "ClassExpression" };
    (doc ce) => { "The offending class expression." };

    (ty  dr) => { DataRange };
    (py  dr) => { "DataRange" };
    (doc dr) => { "The offending data range." };

    (ty  ope) => { ObjectPropertyExpression };
    (py  ope) => { "ObjectPropertyExpression" };
    (doc ope) => { "The composite (non-simple) object property expression." };

    (ty  iri) => { IRI };
    (py  iri) => { "IRI" };
    (doc iri) => { "The IRI the violation is about." };

    (ty  cycle) => { VecWrap<ObjectProperty> };
    (py  cycle) => { "typing.List[ObjectProperty]" };
    (doc cycle) => { "The properties forming the cycle, the first repeated as the last to close the loop." };

    (ty  kinds) => { VecWrap<String> };
    (py  kinds) => { "typing.List[str]" };
    (doc kinds) => { "The mutually-exclusive entity kinds the IRI is declared as." };

    (ty  reason) => { String };
    (py  reason) => { "str" };
    (doc reason) => { "Which profile rule the axiom kind falls foul of." };

}

/// Generates the `Violation` subclass hierarchy.
///
/// Each entry is `Name { field, .. } = "message";`, where the fields are named
/// exactly as on the matching `horned_profile::Violation` variant
macro_rules! violations {
    ($(
        $(#[$meta:meta])*
        $name:ident {
            $($field:ident),* $(,)?
        } = $message:literal;
    )*) => {
        $(
            $(#[$meta])*
            #[pyclass(
                extends = PyViolation,
                module = "pyhornedowl.profile",
                skip_from_py_object
            )]
            pub struct $name {
                $(
                    #[doc = concat!(
                        stringify!($field), ": ",
                        violation_field!(py $field)
                    )]
                    #[doc = ""]
                    #[doc = concat!(violation_field!(doc $field))]
                    #[pyo3(get)]
                    pub $field: violation_field!(ty $field),
                )*
            }

            #[pymethods]
            impl $name {
                #[doc = "message: str"]
                #[doc = ""]
                /// A human readable description of the violation.
                #[getter]
                fn message(&self) -> &'static str {
                    $message
                }

                fn __str__(&self) -> &'static str {
                    $message
                }

                #[allow(unused_mut)]
                fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
                    let mut parts: Vec<String> = Vec::new();
                    $(
                        parts.push(format!(
                            "{}={}",
                            stringify!($field),
                            self.$field.clone().into_pyobject(py)?.repr()?
                        ));
                    )*
                    Ok(format!("{}({})", stringify!($name), parts.join(", ")))
                }
            }
        )*

        /// Converts one `horned_profile::Violation` into the matching
        /// `PyViolation` subclass instance.
        pub fn violation_to_py(
            py: Python<'_>,
            violation: &Violation<ArcStr>,
        ) -> PyResult<Py<PyViolation>> {
            let object = match violation {
                $(
                    Violation::$name { $($field),* } => Bound::new(
                        py,
                        PyClassInitializer::from(PyViolation {})
                            .add_subclass($name { $($field: $field.into_c()),* }),
                    )?
                    .into_super(),
                )*
            };

            Ok(object.unbind())
        }

        fn add_violation_classes(module: &Bound<'_, PyModule>) -> PyResult<()> {
            module.add_class::<PyViolation>()?;
            $( module.add_class::<$name>()?; )*
            Ok(())
        }
    };
}

violations! {
    /// A class expression is not *atomic* where OWL 2 DL requires one to be.
    UseOfNonAtomicClassExpression { axiom, ce }
        = "Use of a non-atomic class expression where an atomic one is required";

    /// `ce` is not legal in the profile's subclass-position grammar.
    UseOfNonSubClassExpression { axiom, ce }
        = "Use of a class expression that is not allowed in subclass position";

    /// `ce` is not legal in the profile's superclass-position grammar.
    UseOfNonSuperClassExpression { axiom, ce }
        = "Use of a class expression that is not allowed in superclass position";

    /// A class expression is not legal in OWL 2 DL at all, in any position.
    UseOfIllegalClassExpression { axiom, ce }
        = "Use of a class expression that is not allowed in this profile";

    /// `ObjectUnionOf`/`ObjectIntersectionOf` built with fewer than the two
    /// operands OWL 2 DL requires for these n-ary constructors.
    UseOfClassExpressionWithTooFewOperands { axiom, ce }
        = "Use of an n-ary class expression with fewer than two operands";

    /// `DataUnionOf`/`DataIntersectionOf` built with fewer than two operands.
    UseOfDataRangeWithTooFewOperands { axiom, dr }
        = "Use of an n-ary data range with fewer than two operands";

    /// A `DatatypeDefinition` whose subject is itself a built-in XSD/OWL 2
    /// datatype -- `DatatypeDefinition` introduces a *new* datatype, it cannot
    /// redefine an existing built-in one.
    UseOfBuiltinDatatypeInDatatypeDefinition { axiom }
        = "Use of a built-in datatype as the subject of a datatype definition";

    /// `ObjectHasSelf` is given a composite (non-simple) object property.
    UseOfNonSimplePropertyInObjectHasSelf { axiom, ope }
        = "Use of a non-simple property in an ObjectHasSelf restriction";

    /// An object-cardinality restriction is given a composite (non-simple)
    /// object property.
    UseOfNonSimplePropertyInCardinalityRestriction { axiom, ope }
        = "Use of a non-simple property in a cardinality restriction";

    /// `DisjointObjectProperties` includes a composite (non-simple) object
    /// property.
    UseOfNonSimplePropertyInDisjointPropertiesAxiom { axiom }
        = "Use of a non-simple property in a DisjointObjectProperties axiom";

    /// `IrreflexiveObjectProperty` is given a composite (non-simple) object
    /// property.
    UseOfNonSimplePropertyInIrreflexivePropertyAxiom { axiom }
        = "Use of a non-simple property in an IrreflexiveObjectProperty axiom";

    /// `AsymmetricObjectProperty` is given a composite (non-simple) object
    /// property.
    UseOfNonSimplePropertyInAsymmetricPropertyAxiom { axiom }
        = "Use of a non-simple property in an AsymmetricObjectProperty axiom";

    /// `FunctionalObjectProperty` is given a composite (non-simple) object
    /// property.
    UseOfNonSimplePropertyInFunctionalPropertyAxiom { axiom }
        = "Use of a non-simple property in a FunctionalObjectProperty axiom";

    /// `InverseFunctionalObjectProperty` is given a composite (non-simple)
    /// object property.
    UseOfNonSimplePropertyInInverseFunctionalPropertyAxiom { axiom }
        = "Use of a non-simple property in an InverseFunctionalObjectProperty axiom";

    /// A cycle in the role hierarchy's property-chain graph. Spans potentially
    /// many chain axioms, so -- unlike most violations -- it has no `axiom`.
    UseOfPropertyInChainCausingCycle { cycle }
        = "Use of a property in a property chain causing a cycle in the role hierarchy";

    /// `iri` is used as a class without a `DeclareClass` axiom.
    UseOfUndeclaredClass { iri }
        = "Use of an undeclared class";

    /// `iri` is used as an object property without a `DeclareObjectProperty`
    /// axiom.
    UseOfUndeclaredObjectProperty { iri }
        = "Use of an undeclared object property";

    /// `iri` is used as a data property without a `DeclareDataProperty` axiom.
    UseOfUndeclaredDataProperty { iri }
        = "Use of an undeclared data property";

    /// `iri` is used as an annotation property without a
    /// `DeclareAnnotationProperty` axiom.
    UseOfUndeclaredAnnotationProperty { iri }
        = "Use of an undeclared annotation property";

    /// `iri` is used as a datatype without a `DeclareDatatype` axiom.
    UseOfUndeclaredDatatype { iri }
        = "Use of an undeclared datatype";

    /// `iri` is declared as more than one mutually-exclusive entity kind.
    UseOfIllegalPunning { iri, kinds }
        = "Use of an IRI as more than one kind of entity (illegal punning)";

    /// A reserved `rdf:`/`rdfs:`/`owl:` structural vocabulary term is the
    /// subject of a `Declare*` axiom.
    UseOfReservedVocabulary { iri }
        = "Use of a reserved vocabulary IRI as a declared entity";

    /// EL-specific: `DataOneOf` with more than one literal.
    UseOfDataOneOfWithMultipleLiterals { axiom }
        = "Use of a DataOneOf with more than one literal";

    /// EL-specific: object property inverses are not permitted at all.
    UseOfObjectPropertyInverse { axiom }
        = "Use of an object property inverse";

    /// This whole axiom *kind* is not permitted in the profile, regardless of
    /// its content -- e.g. `FunctionalObjectProperty` in EL.
    UseOfIllegalAxiomKind { axiom, reason }
        = "Use of an axiom kind that is not allowed in this profile";
}

/// The result of checking an ontology against one OWL 2 profile.
#[pyclass(
    name = "ProfileReport",
    module = "pyhornedowl.profile",
    skip_from_py_object
)]
pub struct PyProfileReport {
    /// profile: Profile
    ///
    /// The profile this report is for.
    #[pyo3(get)]
    pub profile: PyProfile,

    /// conformant: bool
    ///
    /// True if the ontology has no violations of this profile.
    #[pyo3(get)]
    pub conformant: bool,

    /// violations: typing.List[Violation]
    ///
    /// Every violation found, as `Violation` subclass instances.
    #[pyo3(get)]
    pub violations: Vec<Py<PyViolation>>,
}

#[pymethods]
impl PyProfileReport {
    fn __repr__(&self) -> String {
        format!(
            "ProfileReport(profile={}, conformant={})",
            self.profile.__repr__(),
            if self.conformant { "True" } else { "False" }
        )
    }
}

fn build_report<O: horned_owl::model::Ontology<ArcStr>>(
    py: Python<'_>,
    o: &O,
    p: Profile,
) -> PyResult<PyProfileReport> {
    let report = horned_profile::check(o, p);
    let violations = report
        .violations()
        .iter()
        .map(|v| violation_to_py(py, v))
        .collect::<PyResult<Vec<_>>>()?;

    Ok(PyProfileReport {
        profile: report.profile().into(),
        conformant: report.is_conformant(),
        violations,
    })
}

/// conformant_profiles(ontology: PyIndexedOntology) -> List[Profile]
///
/// Returns every OWL 2 profile the ontology conforms to, in declaration order
/// (OWL2DL, EL, QL, RL). The profiles overlap, so more than one may be returned.
///
/// :param PyIndexedOntology ontology: the ontology to check
#[pyfunction]
pub fn conformant_profiles(ontology: &PyIndexedOntology) -> Vec<PyProfile> {
    horned_profile::conformant_profiles::<ArcStr, _>(ontology)
        .into_iter()
        .map(Into::into)
        .collect()
}

/// check_profile(ontology: PyIndexedOntology, profile: typing.Union[Profile, Literal["DL", "EL", "QL", "RL"]]) -> ProfileReport
///
/// Checks `ontology` against a single profile and returns a `ProfileReport` with
/// conformance and the violations found.
///
/// :param PyIndexedOntology ontology: the ontology to check
/// :param typing.Union[Profile, Literal["DL", "EL", "QL", "RL"]] profile: the profile to check against, either a `Profile` member or one of the strings "DL"/"OWL2DL", "EL", "QL", "RL" (case-insensitive)
#[pyfunction]
pub fn check_profile(
    py: Python<'_>,
    ontology: &PyIndexedOntology,
    profile: PyProfile,
) -> PyResult<PyProfileReport> {
    build_report(py, ontology, profile.into())
}

pub fn py_module<'py>(py: Python<'py>) -> PyResult<Bound<'py, PyModule>> {
    let module = PyModule::new(py, "profile")?;

    module.add_function(wrap_pyfunction!(conformant_profiles, &module)?)?;
    module.add_function(wrap_pyfunction!(check_profile, &module)?)?;
    module.add_class::<PyViolation>()?;
    module.add_class::<PyProfile>()?;
    module.add_class::<PyProfileReport>()?;
    add_violation_classes(&module)?;

    Ok(module)
}
