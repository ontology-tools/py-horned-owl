//! OWL 2 profile (EL/QL/RL/DL) conformance checking.
//!
//! Thin PyO3 wrapper over the [`horned_profile`] crate, which checks an
//! ontology's horned-owl model directly (no RDF/quad scan). Exposed as the
//! `pyhornedowl.profile` submodule, mirroring `reasoning`.
use crate::PyIndexedOntology;
use horned_owl::model::ArcStr;
use horned_owl::ontology::set::SetOntology;
use horned_profile::Profile;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

fn profile_name(p: Profile) -> &'static str {
    match p {
        Profile::OWL2DL => "OWL2DL",
        Profile::EL => "EL",
        Profile::QL => "QL",
        Profile::RL => "RL",
    }
}

fn parse_profile(s: &str) -> PyResult<Profile> {
    match s.to_ascii_uppercase().as_str() {
        "OWL2DL" | "DL" => Ok(Profile::OWL2DL),
        "EL" => Ok(Profile::EL),
        "QL" => Ok(Profile::QL),
        "RL" => Ok(Profile::RL),
        other => Err(PyValueError::new_err(format!(
            "unknown profile '{other}'; expected one of DL/OWL2DL, EL, QL, RL"
        ))),
    }
}

/// The result of checking an ontology against one OWL 2 profile.
#[pyclass(
    name = "ProfileReport",
    module = "pyhornedowl.profile",
    skip_from_py_object
)]
#[derive(Clone)]
pub struct PyProfileReport {
    /// The profile this report is for: "OWL2DL", "EL", "QL" or "RL".
    #[pyo3(get)]
    pub profile: String,
    /// True if the ontology has no violations of this profile.
    #[pyo3(get)]
    pub conformant: bool,
    /// Total number of violations.
    #[pyo3(get)]
    pub num_violations: usize,
    /// Violation counts grouped by kind (the `horned_profile::Violation`
    /// variant name, e.g. "UseOfNonSubClassExpression").
    #[pyo3(get)]
    pub violations_by_kind: HashMap<String, usize>,
}

#[pymethods]
impl PyProfileReport {
    fn __repr__(&self) -> String {
        format!(
            "ProfileReport(profile='{}', conformant={}, num_violations={})",
            self.profile,
            if self.conformant { "True" } else { "False" },
            self.num_violations
        )
    }
}

fn build_report(o: &SetOntology<ArcStr>, p: Profile) -> PyProfileReport {
    let report = horned_profile::check(o, p);
    let mut by_kind: HashMap<String, usize> = HashMap::new();
    for v in report.violations() {
        // Group by the Violation variant name. Deriving it from the Debug
        // representation keeps this correct as new variants are added upstream,
        // rather than an exhaustive match that would need updating in lockstep.
        let dbg = format!("{:?}", v);
        let kind = dbg
            .split(|c: char| c == ' ' || c == '{' || c == '(')
            .next()
            .unwrap_or("Violation")
            .to_string();
        *by_kind.entry(kind).or_insert(0) += 1;
    }
    PyProfileReport {
        profile: profile_name(p).to_string(),
        conformant: report.is_conformant(),
        num_violations: report.violations().len(),
        violations_by_kind: by_kind,
    }
}

/// conformant_profiles(ontology: PyIndexedOntology) -> List[str]
///
/// Returns every OWL 2 profile the ontology conforms to, in declaration order
/// (OWL2DL, EL, QL, RL). The profiles overlap, so more than one may be returned.
#[pyfunction]
pub fn conformant_profiles(ontology: PyIndexedOntology) -> Vec<String> {
    let o: SetOntology<ArcStr> = SetOntology::from(&ontology);
    horned_profile::conformant_profiles(&o)
        .into_iter()
        .map(|p| profile_name(p).to_string())
        .collect()
}

/// check_profile(ontology: PyIndexedOntology, profile: str) -> ProfileReport
///
/// Checks `ontology` against a single profile ("DL"/"OWL2DL", "EL", "QL", "RL",
/// case-insensitive) and returns a `ProfileReport` with conformance and
/// per-kind violation counts.
#[pyfunction]
pub fn check_profile(ontology: PyIndexedOntology, profile: String) -> PyResult<PyProfileReport> {
    let p = parse_profile(&profile)?;
    let o: SetOntology<ArcStr> = SetOntology::from(&ontology);
    Ok(build_report(&o, p))
}
