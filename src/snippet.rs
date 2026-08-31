//! Per-element serialization: the machinery shared by the `write_snippet`
//! function and the `serialize` method on the model classes.
//!
//! The rules live here rather than on the classes because they are properties
//! of the syntax, not of the element:
//!
//! * horned-owl provides a per-element writer for OWL Functional (`ofn`) and
//!   OWL 2 Manchester (`omn`) syntax only. Its OWL/XML, RDF and OBO writers
//!   work on whole ontologies. OBO is the least sliceable of the three: it is
//!   stanza-oriented, so a component does not render to a string at all but to
//!   a clause line under some *other* entity's stanza, and which stanza that is
//!   depends on `oboInOwl:id` annotations gathered from the whole ontology.
//! * `ofn` renders an axiom's annotations and `omn` does not, because
//!   `AsFunctional` has an `AnnotatedComponent` impl and `AsManchester` does
//!   not.
//! * `AsManchester` covers a subset of the model (see
//!   `horned_owl::io::omn::writer::as_manchester`). Component types reach it by
//!   way of `Component`; a few wrappers have no Manchester rendering at all and
//!   say so.
//!
//! Rendering is exposed as macros rather than generic functions on purpose.
//! horned-owl blanket-implements `Display for Manchester<'t, &'t T, A>` in
//! terms of `Manchester<'t, T, A>`, so a `where Manchester<'t, T, ArcStr>:
//! Display` bound sends the trait solver down an unbounded `&&&&T` chain
//! (E0275). Expanding at the call site keeps every obligation concrete.

use pyo3::{exceptions::PyValueError, PyErr, PyResult};

/// The serializations that have a per-element writer in horned-owl.
pub enum SnippetSyntax {
    Manchester,
    Functional,
}

/// Resolve a `serialization` argument, rejecting the whole-ontology-only ones.
pub fn parse_syntax(serialization: &str) -> PyResult<SnippetSyntax> {
    match serialization {
        "omn" => Ok(SnippetSyntax::Manchester),
        "ofn" => Ok(SnippetSyntax::Functional),
        other => Err(PyValueError::new_err(format!(
            "Cannot write a snippet in {:?}. horned-owl has per-element writers only for \
             \"ofn\" and \"omn\"; its OWL/XML, RDF and OBO writers work on whole \
             ontologies only.",
            other
        ))),
    }
}

/// The error for a class horned-owl cannot render in Manchester syntax.
///
/// These are wrappers that are neither an OWL entity nor a component, so
/// Manchester syntax has no production for them standing alone.
pub fn no_manchester_rendering(class: &str) -> PyErr {
    PyValueError::new_err(format!(
        "horned-owl has no Manchester rendering for {}; it can only be written as part of \
         the element that contains it. Use \"ofn\".",
        class
    ))
}

/// Render a horned-owl value in Manchester syntax as a `String`, abbreviating
/// IRIs with `$prefix_mapping` (an `Option<&crate::prefix_mapping::PrefixMapping>`)
/// where possible.
#[macro_export]
macro_rules! as_omn {
    ($value:expr, $prefix_mapping:expr) => {{
        #[allow(unused_imports)]
        use ::horned_owl::io::omn::writer::AsManchester as _;
        match $prefix_mapping {
            Some(pm) => $value.as_manchester_with_prefixes(&pm.0).to_string(),
            None => $value.as_manchester().to_string(),
        }
    }};
}

/// Render a horned-owl value in functional syntax as a `String`, abbreviating
/// IRIs with `$prefix_mapping` where possible.
#[macro_export]
macro_rules! as_ofn {
    ($value:expr, $prefix_mapping:expr) => {{
        #[allow(unused_imports)]
        use ::horned_owl::io::ofn::writer::AsFunctional as _;
        match $prefix_mapping {
            Some(pm) => $value.as_functional_with_prefixes(&pm.0).to_string(),
            None => $value.as_functional().to_string(),
        }
    }};
}
