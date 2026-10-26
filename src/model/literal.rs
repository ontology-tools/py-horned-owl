use horned_owl::model::Build;
use py_horned_owl_macros::{model, model_enum};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyInt, PyString};

use super::IRI;

/// Besides the literal classes, a literal can be given as a `str` (a simple
/// literal), or a `bool`, `int` or `float` (typed as `xsd:boolean`,
/// `xsd:integer` and `xsd:double`).
fn literal_from_python(ob: &Bound<'_, PyAny>) -> PyResult<Option<Literal>> {
    if let Ok(s) = ob.cast::<PyString>() {
        return Ok(Some(
            SimpleLiteral {
                literal: s.to_str()?.to_owned(),
            }
            .into(),
        ));
    }
    // bool before int: Python's bool is a subclass of int.
    let (literal, datatype) = if let Ok(b) = ob.cast::<PyBool>() {
        (b.is_true().to_string(), "boolean")
    } else if ob.is_instance_of::<PyInt>() {
        (ob.str()?.to_string(), "integer")
    } else if let Ok(f) = ob.cast::<PyFloat>() {
        let v = f.value();
        let literal = if v.is_nan() {
            "NaN".to_owned()
        } else if v.is_infinite() {
            if v > 0.0 { "INF" } else { "-INF" }.to_owned()
        } else {
            ob.repr()?.to_string()
        };
        (literal, "double")
    } else {
        return Ok(None);
    };
    Ok(Some(
        DatatypeLiteral {
            literal,
            datatype_iri: IRI::new(
                format!("http://www.w3.org/2001/XMLSchema#{}", datatype),
                &Build::new_arc(),
            ),
        }
        .into(),
    ))
}

#[model_enum(coerce = literal_from_python, accepts = [String, bool, i64, f64])]
pub enum Literal {
    Simple(SimpleLiteral),
    Language(LanguageLiteral),
    Datatype(DatatypeLiteral),
}

#[model(variant_of = Literal::Simple)]
pub struct SimpleLiteral {
    pub literal: String,
}

#[model(variant_of = Literal::Language)]
pub struct LanguageLiteral {
    pub literal: String,
    #[model(horned = lang)]
    pub language: String,
}

#[model(variant_of = Literal::Datatype)]
pub struct DatatypeLiteral {
    pub literal: String,
    pub datatype_iri: IRI,
}
