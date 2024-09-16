use horned_owl::model::ClassExpression;
use py_horned_owl_macros::pho_mapped;

#[pho_mapped]
pub struct PHOSubClassOf {
    #[pho_field("transparent")]
    pub sup: ClassExpression,
    pub sub: ClassExpression,
}