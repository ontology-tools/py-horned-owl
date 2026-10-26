use py_horned_owl_macros::{model, model_enum, model_union};
use pyo3::prelude::*;

use crate::model::protocol::entity_from_str;
use crate::wrappers::VecWrap;

use super::{
    AnnotationProperty, ClassExpression, DataProperty, Individual, ObjectAllValuesFrom,
    ObjectExactCardinality, ObjectHasSelf, ObjectHasValue, ObjectMaxCardinality,
    ObjectMinCardinality, ObjectProperty, ObjectSomeValuesFrom,
};

// A `str` names an object property.
#[model_enum(coerce = entity_from_str::<ObjectProperty>, accepts = [String])]
pub enum ObjectPropertyExpression {
    #[model(wraps)]
    ObjectProperty(ObjectProperty),
    InverseObjectProperty(InverseObjectProperty),
}

#[model(variant_of = ObjectPropertyExpression)]
pub struct InverseObjectProperty {
    #[model(horned = 0)]
    pub object_property: ObjectProperty,
}

/// Builders for the class expressions restricting this property, and `~`
/// for its inverse.
#[pymethods]
impl ObjectPropertyExpressionBase {
    fn some(
        slf: &Bound<'_, Self>,
        class_expression: ClassExpression,
    ) -> PyResult<ObjectSomeValuesFrom> {
        Ok(ObjectSomeValuesFrom {
            object_property_expression: slf.extract()?,
            class_expression: Box::new(class_expression).into(),
        })
    }

    fn only(
        slf: &Bound<'_, Self>,
        class_expression: ClassExpression,
    ) -> PyResult<ObjectAllValuesFrom> {
        Ok(ObjectAllValuesFrom {
            object_property_expression: slf.extract()?,
            class_expression: Box::new(class_expression).into(),
        })
    }

    fn has_value(slf: &Bound<'_, Self>, individual: Individual) -> PyResult<ObjectHasValue> {
        Ok(ObjectHasValue {
            object_property_expression: slf.extract()?,
            individual,
        })
    }

    fn has_self(slf: &Bound<'_, Self>) -> PyResult<ObjectHasSelf> {
        Ok(ObjectHasSelf {
            object_property_expression: slf.extract()?,
        })
    }

    fn min(
        slf: &Bound<'_, Self>,
        cardinality: u32,
        class_expression: ClassExpression,
    ) -> PyResult<ObjectMinCardinality> {
        Ok(ObjectMinCardinality {
            cardinality,
            object_property_expression: slf.extract()?,
            class_expression: Box::new(class_expression).into(),
        })
    }

    fn max(
        slf: &Bound<'_, Self>,
        cardinality: u32,
        class_expression: ClassExpression,
    ) -> PyResult<ObjectMaxCardinality> {
        Ok(ObjectMaxCardinality {
            cardinality,
            object_property_expression: slf.extract()?,
            class_expression: Box::new(class_expression).into(),
        })
    }

    fn exact(
        slf: &Bound<'_, Self>,
        cardinality: u32,
        class_expression: ClassExpression,
    ) -> PyResult<ObjectExactCardinality> {
        Ok(ObjectExactCardinality {
            cardinality,
            object_property_expression: slf.extract()?,
            class_expression: Box::new(class_expression).into(),
        })
    }

    fn __invert__(slf: &Bound<'_, Self>) -> PyResult<ObjectPropertyExpression> {
        Ok(match slf.extract()? {
            ObjectPropertyExpression::ObjectProperty(object_property) => {
                InverseObjectProperty { object_property }.into()
            }
            ObjectPropertyExpression::InverseObjectProperty(inverse) => {
                inverse.object_property.into()
            }
        })
    }
}

// The single expression first: a `str` is iterable too.
#[model_union]
pub enum SubObjectPropertyExpression {
    ObjectPropertyExpression(ObjectPropertyExpression),
    ObjectPropertyChain(VecWrap<ObjectPropertyExpression>),
}

#[model_union]
pub enum PropertyExpression {
    ObjectPropertyExpression(ObjectPropertyExpression),
    DataProperty(DataProperty),
    AnnotationProperty(AnnotationProperty),
}
