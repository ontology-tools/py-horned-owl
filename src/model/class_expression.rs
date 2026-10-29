use py_horned_owl_macros::{model, model_enum};
use pyo3::prelude::*;

use crate::model::protocol::entity_from_str;
use crate::wrappers::{BoxWrap, VecWrap};

use super::{Class, DataProperty, DataRange, Individual, Literal, ObjectPropertyExpression};

// A `str` names a class.
#[model_enum(coerce = entity_from_str::<Class>, accepts = [String])]
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
        Ok(ObjectIntersectionOf {
            operands: intersection_operands(slf.extract()?, other).into(),
        })
    }

    fn __or__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectUnionOf> {
        Ok(ObjectUnionOf {
            operands: union_operands(slf.extract()?, other).into(),
        })
    }

    fn __rand__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectIntersectionOf> {
        Ok(ObjectIntersectionOf {
            operands: intersection_operands(other, slf.extract()?).into(),
        })
    }

    fn __ror__(slf: &Bound<'_, Self>, other: ClassExpression) -> PyResult<ObjectUnionOf> {
        Ok(ObjectUnionOf {
            operands: union_operands(other, slf.extract()?).into(),
        })
    }

    fn __invert__(slf: &Bound<'_, Self>) -> PyResult<ObjectComplementOf> {
        Ok(ObjectComplementOf {
            operand: Box::new(slf.extract::<ClassExpression>()?).into(),
        })
    }
}

// Flatten only the matching connective, preserving operand order and duplicates.
fn intersection_operands(left: ClassExpression, right: ClassExpression) -> Vec<ClassExpression> {
    fn append(value: ClassExpression, result: &mut Vec<ClassExpression>) {
        match value {
            ClassExpression::ObjectIntersectionOf(value) => {
                for operand in value.operands.0 {
                    append(operand, result);
                }
            }
            value => result.push(value),
        }
    }
    let mut result = Vec::new();
    append(left, &mut result);
    append(right, &mut result);
    result
}

fn union_operands(left: ClassExpression, right: ClassExpression) -> Vec<ClassExpression> {
    fn append(value: ClassExpression, result: &mut Vec<ClassExpression>) {
        match value {
            ClassExpression::ObjectUnionOf(value) => {
                for operand in value.operands.0 {
                    append(operand, result);
                }
            }
            value => result.push(value),
        }
    }
    let mut result = Vec::new();
    append(left, &mut result);
    append(right, &mut result);
    result
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
