use py_horned_owl_macros::{model, model_enum};

use crate::model::protocol::entity_from_str;
use crate::wrappers::{BoxWrap, VecWrap};

use super::{Datatype, Facet, Literal};

// A `str` names a datatype.
#[model_enum(coerce = entity_from_str::<Datatype>, accepts = [String])]
pub enum DataRange {
    #[model(wraps)]
    Datatype(Datatype),
    DataIntersectionOf(DataIntersectionOf),
    DataUnionOf(DataUnionOf),
    DataComplementOf(DataComplementOf),
    DataOneOf(DataOneOf),
    DatatypeRestriction(DatatypeRestriction),
}

#[model(variant_of = DataRange)]
pub struct DataIntersectionOf {
    #[model(horned = 0)]
    pub operands: VecWrap<DataRange>,
}

#[model(variant_of = DataRange)]
pub struct DataUnionOf {
    #[model(horned = 0)]
    pub operands: VecWrap<DataRange>,
}

#[model(variant_of = DataRange)]
pub struct DataComplementOf {
    #[model(horned = 0)]
    pub operand: BoxWrap<DataRange>,
}

#[model(variant_of = DataRange)]
pub struct DataOneOf {
    #[model(horned = 0)]
    pub literals: VecWrap<Literal>,
}

#[model(variant_of = DataRange)]
pub struct DatatypeRestriction {
    #[model(horned = 0)]
    pub datatype: Datatype,
    #[model(horned = 1)]
    pub restrictions: VecWrap<FacetRestriction>,
}

#[model(render = functional)]
pub struct FacetRestriction {
    #[model(horned = f)]
    pub facet: Facet,
    #[model(horned = l)]
    pub literal: Literal,
}
