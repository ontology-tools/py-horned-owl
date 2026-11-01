use py_horned_owl_macros::{model, model_enum};
use pyo3::prelude::*;

use crate::model::protocol::{entity_from_iri, IRI_OR_STR};
use crate::wrappers::{BoxWrap, VecWrap};

use super::{Class, DataProperty, DataRange, Individual, Literal, ObjectPropertyExpression};

// An `IRI` or a `str` names a class.
#[model_enum(coerce = entity_from_iri::<Class>, accepts = [IRI_OR_STR])]
pub enum ClassExpression {
    #[model(wraps)]
    Class(Class),
    ObjectIntersectionOf(ObjectIntersectionOf),
    ObjectUnionOf(ObjectUnionOf),
    ObjectComplementOf(ObjectComplementOf),
    ObjectOneOf(ObjectOneOf),
    ObjectSomeValuesFrom(ObjectSomeValuesFrom),
    ObjectAllValuesFrom(ObjectAllValuesFrom),
    ObjectHasValue(ObjectHasValue),
    ObjectHasSelf(ObjectHasSelf),
    ObjectMinCardinality(ObjectMinCardinality),
    ObjectMaxCardinality(ObjectMaxCardinality),
    ObjectExactCardinality(ObjectExactCardinality),
    DataSomeValuesFrom(DataSomeValuesFrom),
    DataAllValuesFrom(DataAllValuesFrom),
    DataHasValue(DataHasValue),
    DataMinCardinality(DataMinCardinality),
    DataMaxCardinality(DataMaxCardinality),
    DataExactCardinality(DataExactCardinality),
}

/// `&`, `|` and `~` build intersections, unions and complements.
#[pymethods]
impl ClassExpressionBase {
    fn __and__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectIntersectionOf> {
        Ok(slf.extract::<ClassExpression>()?.intersection(other))
    }

    fn __or__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectUnionOf> {
        Ok(slf.extract::<ClassExpression>()?.union(other))
    }

    fn __rand__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectIntersectionOf> {
        Ok(other.intersection(slf.extract()?))
    }

    fn __ror__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectUnionOf> {
        Ok(other.union(slf.extract()?))
    }

    fn __invert__(slf: &Bound<'_, Self>) -> PyResult<ClassExpression> {
        Ok(slf.extract::<ClassExpression>()?.complement())
    }

    /// Returns an equivalent, flatter class expression
    fn flatten(slf: &Bound<'_, Self>) -> PyResult<ClassExpression> {
        Ok(slf.extract::<ClassExpression>()?.flatten())
    }
}

macro_rules! flatten {
    ($variant:ident, $field:ident) => {{
        let mut flat = $field.into_iter().map(Self::flatten).collect::<Vec<_>>();

        if flat.len() == 1 {
            flat.pop().unwrap()
        } else {
            $variant {
                $field: flat.into(),
            }.into()
        }
    }};
}

impl ClassExpression {
    pub fn intersection(self, other: Self) -> ObjectIntersectionOf {
        ObjectIntersectionOf {
            operands: vec![self, other]
                .into_iter()
                .map(Self::flatten)
                .collect::<Vec<_>>()
                .into(),
        }
    }

    pub fn union(self, other: Self) -> ObjectUnionOf {
        ObjectUnionOf {
            operands: vec![self, other]
                .into_iter()
                .map(Self::flatten)
                .collect::<Vec<_>>()
                .into(),
        }
    }

    pub fn complement(self) -> Self {
        match self {
            Self::ObjectComplementOf(complement) => *complement.operand.0,
            operand => ObjectComplementOf {
                operand: Box::new(operand).into(),
            }
            .into(),
        }
    }

    pub fn flatten(self) -> Self {
        match self {
            Self::ObjectIntersectionOf(ObjectIntersectionOf {
                operands: VecWrap(operands),
            }) => flatten!(ObjectIntersectionOf, operands),
            Self::ObjectUnionOf(ObjectUnionOf {
                operands: VecWrap(operands),
            }) => flatten!(ObjectUnionOf, operands),
            Self::ObjectComplementOf(x) => x.operand.0.flatten().complement(),
            Self::ObjectSomeValuesFrom(x) => ObjectSomeValuesFrom {
                class_expression: x.class_expression.flatten(),
                ..x
            }
            .into(),
            Self::ObjectAllValuesFrom(x) => ObjectAllValuesFrom {
                class_expression: x.class_expression.flatten(),
                ..x
            }
            .into(),
            Self::ObjectMinCardinality(x) => ObjectMinCardinality {
                class_expression: x.class_expression.flatten(),
                ..x
            }
            .into(),
            Self::ObjectMaxCardinality(x) => ObjectMaxCardinality {
                class_expression: x.class_expression.flatten(),
                ..x
            }
            .into(),
            Self::ObjectExactCardinality(x) => ObjectExactCardinality {
                class_expression: x.class_expression.flatten(),
                ..x
            }
            .into(),
            other => other,
        }
    }
}

impl BoxWrap<ClassExpression> {
    fn flatten(self) -> Self {
        Box::new(self.0.flatten()).into()
    }
}

#[model(variant_of = ClassExpression)]
pub struct ObjectIntersectionOf {
    #[model(horned = 0)]
    pub operands: VecWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectUnionOf {
    #[model(horned = 0)]
    pub operands: VecWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectComplementOf {
    #[model(horned = 0)]
    pub operand: BoxWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectOneOf {
    #[model(horned = 0)]
    pub individuals: VecWrap<Individual>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectSomeValuesFrom {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = bce)]
    pub class_expression: BoxWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectAllValuesFrom {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = bce)]
    pub class_expression: BoxWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectHasValue {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = i)]
    pub individual: Individual,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectHasSelf {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectMinCardinality {
    #[model(horned = n)]
    pub cardinality: u32,
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = bce)]
    pub class_expression: BoxWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectMaxCardinality {
    #[model(horned = n)]
    pub cardinality: u32,
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = bce)]
    pub class_expression: BoxWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct ObjectExactCardinality {
    #[model(horned = n)]
    pub cardinality: u32,
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = bce)]
    pub class_expression: BoxWrap<ClassExpression>,
}

#[model(variant_of = ClassExpression)]
pub struct DataSomeValuesFrom {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = dr)]
    pub data_range: DataRange,
}

#[model(variant_of = ClassExpression)]
pub struct DataAllValuesFrom {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = dr)]
    pub data_range: DataRange,
}

#[model(variant_of = ClassExpression)]
pub struct DataHasValue {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = l)]
    pub literal: Literal,
}

#[model(variant_of = ClassExpression)]
pub struct DataMinCardinality {
    #[model(horned = n)]
    pub cardinality: u32,
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = dr)]
    pub data_range: DataRange,
}

#[model(variant_of = ClassExpression)]
pub struct DataMaxCardinality {
    #[model(horned = n)]
    pub cardinality: u32,
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = dr)]
    pub data_range: DataRange,
}

#[model(variant_of = ClassExpression)]
pub struct DataExactCardinality {
    #[model(horned = n)]
    pub cardinality: u32,
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = dr)]
    pub data_range: DataRange,
}
