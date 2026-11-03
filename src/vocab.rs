use std::sync::LazyLock;

use horned_owl::model::Build;
use pyo3::prelude::*;

use crate::model::IRI;

/// Defines each namespace as a Python class whose class attributes are the
/// IRIs of its terms. From Rust, `XSD::boolean()` returns the shared IRI and
/// `XSD::NAMESPACE` the namespace.
macro_rules! vocabulary {
    ($($(#[$attr:meta])* $ns:ident = $namespace:literal { $($term:ident),* $(,)? })*) => {
        $(
            $(#[$attr])*
            ///
            $(#[doc = concat!(":cvar ", stringify!($term), ": ``", $namespace, stringify!($term), "``")])*
            #[pyclass(module = "pyhornedowl.vocab", frozen)]
            pub struct $ns;

            impl $ns {
                pub const NAMESPACE: &'static str = $namespace;
            }

            #[allow(non_snake_case)]
            #[pymethods]
            impl $ns {
                $(
                    #[classattr]
                    pub fn $term() -> IRI {
                        static TERM: LazyLock<IRI> = LazyLock::new(|| {
                            IRI::new(concat!($namespace, stringify!($term)), &Build::new_arc())
                        });
                        TERM.clone()
                    }
                )*
            }
        )*

        #[pymodule(name = "vocab")]
        pub mod py_vocab {
            $(
                #[pymodule_export]
                use super::$ns;
            )*
        }
    };
}

vocabulary! {
    /// The OWL 2 built-in entities and annotation properties.
    OWL = "http://www.w3.org/2002/07/owl#" {
        Thing, Nothing,
        topObjectProperty, bottomObjectProperty,
        topDataProperty, bottomDataProperty,
        real, rational,
        deprecated, versionInfo, priorVersion, backwardCompatibleWith, incompatibleWith,
    }

    /// The RDF datatypes of the OWL 2 datatype map.
    RDF = "http://www.w3.org/1999/02/22-rdf-syntax-ns#" {
        PlainLiteral, XMLLiteral, langString,
    }

    /// The RDFS annotation properties and datatype built into OWL 2.
    RDFS = "http://www.w3.org/2000/01/rdf-schema#" {
        label, comment, seeAlso, isDefinedBy,
        Literal,
    }

    /// The XML Schema datatypes of the OWL 2 datatype map.
    XSD = "http://www.w3.org/2001/XMLSchema#" {
        decimal, integer,
        nonNegativeInteger, nonPositiveInteger, positiveInteger, negativeInteger,
        long, int, short, byte,
        unsignedLong, unsignedInt, unsignedShort, unsignedByte,
        double, float,
        string, normalizedString, token, language, Name, NCName, NMTOKEN,
        boolean, hexBinary, base64Binary, anyURI,
        dateTime, dateTimeStamp,
    }
}
