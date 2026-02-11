//! Structural Reasoner for py-horned-owl
//!
//! A simple reasoner that performs no inference except climbing `SubClassOf` and
//! `SubObjectPropertyOf` chains. This is useful for querying the asserted class
//! and property hierarchies.

use horned_owl::model::{
    ArcAnnotatedComponent, ArcStr, Class, ClassExpression, Component, ComponentKind, DeclareClass,
    ObjectProperty, ObjectPropertyExpression, SubClassOf, SubObjectPropertyExpression,
    SubObjectPropertyOf,
};
use horned_owl::ontology::component_mapped::ComponentMappedIndex;
use horned_owl::ontology::indexed::OntologyIndex;
use horned_owl::ontology::set::SetOntology;
use horned_owl::vocab;
use pyhornedowlreasoner::{PyReasoner, Reasoner, ReasonerError};
use std::collections::HashSet;

/// A structural reasoner that traverses subclass and sub-property hierarchies.
///
/// This reasoner does not perform any logical inference beyond following
/// explicit SubClassOf and SubObjectPropertyOf axioms.
pub struct StructuralReasoner {
    component_index: ComponentMappedIndex<ArcStr, ArcAnnotatedComponent>,
}

#[allow(dead_code)]
impl StructuralReasoner {
    /// Returns direct subclasses of a class IRI.
    pub fn get_direct_subclasses_of_iri<'a>(
        &'a self,
        cls: &'a Class<ArcStr>,
    ) -> Box<dyn Iterator<Item = Class<ArcStr>> + 'a> {
        let mut subclass_axioms = self
            .component_index
            .component_for_kind(ComponentKind::SubClassOf);

        // Handle owl:Thing - return all root classes (classes without a superclass)
        if cls.is(&vocab::OWL::Thing) {
            let entities = self
                .component_index
                .component_for_kind(ComponentKind::DeclareClass);

            return Box::new(entities.filter_map(move |aax| match &aax.component {
                Component::DeclareClass(DeclareClass(decl))
                    if subclass_axioms.any(|aax| match &aax.component {
                        Component::SubClassOf(SubClassOf {
                            sub: ClassExpression::Class(sub),
                            sup: ClassExpression::Class(_sup),
                        }) if sub == decl => true,
                        _ => false,
                    }) =>
                {
                    None
                } // Skip classes that have a superclass
                Component::DeclareClass(DeclareClass(decl)) => Some(decl.clone()),
                _ => None,
            }));
        }

        Box::new(subclass_axioms.filter_map(move |aax| match &aax.component {
            Component::SubClassOf(SubClassOf {
                sub: ClassExpression::Class(sub),
                sup: ClassExpression::Class(sup),
            }) if sup == cls => Some(sub.clone()),
            _ => None,
        }))
    }

    /// Returns direct superclasses of a class IRI.
    pub fn get_direct_superclasses_of_iri<'a>(
        &'a self,
        cls: &'a Class<ArcStr>,
    ) -> Box<dyn Iterator<Item = Class<ArcStr>> + 'a> {
        // owl:Thing has no superclasses
        if cls.is(&vocab::OWL::Thing) {
            return Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Class<ArcStr>>>;
        }

        let subclass_axioms = self
            .component_index
            .component_for_kind(ComponentKind::SubClassOf);

        Box::new(subclass_axioms.filter_map(move |aax| match &aax.component {
            Component::SubClassOf(SubClassOf {
                sub: ClassExpression::Class(sub),
                sup: ClassExpression::Class(sup),
            }) if sub == cls => Some(sup.clone()),
            _ => None,
        }))
    }

    /// Recursively collects all descendants (subclasses) of a class.
    pub fn recurse_descendants(
        &self,
        superclass: &Class<ArcStr>,
        descendants: &mut HashSet<Class<ArcStr>>,
    ) {
        if superclass.is(&vocab::OWL::Nothing) {
            return; // owl:Nothing has no subclasses
        }

        if superclass.is(&vocab::OWL::Thing) {
            for c in self
                .component_index
                .component_for_kind(ComponentKind::DeclareClass)
                .filter_map(|aax| match &aax.component {
                    Component::DeclareClass(DeclareClass(decl)) => {
                        if decl.is(&vocab::OWL::Thing) {
                            None
                        } else {
                            Some(decl.clone())
                        }
                    }
                    _ => None,
                })
            {
                descendants.insert(c);
            }
            return;
        }

        let subclasses = self.get_direct_subclasses_of_iri(superclass);

        for cls in subclasses.into_iter() {
            if descendants.insert(cls.clone()) {
                self.recurse_descendants(&cls, descendants);
            }
        }
    }

    /// Recursively collects all ancestors (superclasses) of a class.
    pub fn recurse_ancestors(&self, subclass: &Class<ArcStr>, ancestors: &mut HashSet<Class<ArcStr>>) {
        if subclass.is(&vocab::OWL::Thing) {
            return; // owl:Thing has no superclasses
        }

        if subclass.is(&vocab::OWL::Nothing) {
            for c in self
                .component_index
                .component_for_kind(ComponentKind::DeclareClass)
                .filter_map(|aax| match &aax.component {
                    Component::DeclareClass(DeclareClass(decl)) => {
                        if decl.is(&vocab::OWL::Thing) {
                            None
                        } else {
                            Some(decl.clone())
                        }
                    }
                    _ => None,
                })
            {
                ancestors.insert(c);
            }
            return;
        }

        let superclasses = self.get_direct_superclasses_of_iri(subclass);

        for cls in superclasses.into_iter() {
            if ancestors.insert(cls.clone()) {
                self.recurse_ancestors(&cls, ancestors);
            }
        }
    }

    /// Returns direct sub-properties of an object property expression.
    fn get_direct_subobjectproperties<'a>(
        &'a self,
        prop: &'a ObjectProperty<ArcStr>,
    ) -> impl Iterator<Item = ObjectProperty<ArcStr>> + 'a {
        let subprop_axioms = self
            .component_index
            .component_for_kind(ComponentKind::SubObjectPropertyOf);

        subprop_axioms.filter_map(move |aax| match &aax.component {
            Component::SubObjectPropertyOf(SubObjectPropertyOf {
                sub:
                    SubObjectPropertyExpression::ObjectPropertyExpression(
                        ObjectPropertyExpression::ObjectProperty(sub),
                    ),
                sup: ObjectPropertyExpression::ObjectProperty(sup),
            }) if sup == prop => Some(sub.clone()),
            _ => None,
        })
    }

    /// Returns direct super-properties of an object property expression.
    fn get_direct_superobjectproperties<'a>(
        &'a self,
        prop: &'a ObjectProperty<ArcStr>,
    ) -> impl Iterator<Item = ObjectProperty<ArcStr>> + 'a {
        let subprop_axioms = self
            .component_index
            .component_for_kind(ComponentKind::SubObjectPropertyOf);

        subprop_axioms.filter_map(move |aax| match &aax.component {
            Component::SubObjectPropertyOf(SubObjectPropertyOf {
                sub:
                    SubObjectPropertyExpression::ObjectPropertyExpression(
                        ObjectPropertyExpression::ObjectProperty(sub),
                    ),
                sup: ObjectPropertyExpression::ObjectProperty(sup),
            }) if sub == prop => Some(sup.clone()),
            _ => None,
        })
    }

    /// Recursively collects all sub-properties (descendants) of an object property.
    fn recurse_subproperties(
        &self,
        superprop: &ObjectProperty<ArcStr>,
        descendants: &mut HashSet<ObjectProperty<ArcStr>>,
    ) {
        let subprops = self.get_direct_subobjectproperties(superprop);

        for prop in subprops.into_iter() {
            if descendants.insert(prop.clone()) {
                self.recurse_subproperties(&prop, descendants);
            }
        }
    }

    /// Recursively collects all super-properties (ancestors) of an object property.
    fn recurse_superproperties(
        &self,
        subprop: &ObjectProperty<ArcStr>,
        ancestors: &mut HashSet<ObjectProperty<ArcStr>>,
    ) {
        let superprops = self.get_direct_superobjectproperties(subprop);

        for prop in superprops.into_iter() {
            if ancestors.insert(prop.clone()) {
                self.recurse_superproperties(&prop, ancestors);
            }
        }
    }
}

impl PyReasoner for StructuralReasoner {
    fn create_reasoner(ontology: SetOntology<ArcStr>) -> Self {
        let mut component_index = ComponentMappedIndex::<ArcStr, ArcAnnotatedComponent>::new();

        for cmp in ontology.iter() {
            component_index.index_insert(std::sync::Arc::new(cmp.clone()));
        }

        StructuralReasoner { component_index }
    }
}

impl OntologyIndex<ArcStr, ArcAnnotatedComponent> for StructuralReasoner {
    fn index_insert(&mut self, cmp: ArcAnnotatedComponent) -> bool {
        self.component_index.index_insert(cmp)
    }

    fn index_take(
        &mut self,
        cmp: &horned_owl::model::AnnotatedComponent<ArcStr>,
    ) -> Option<horned_owl::model::AnnotatedComponent<ArcStr>> {
        self.component_index.index_take(cmp)
    }

    fn index_remove(&mut self, cmp: &horned_owl::model::AnnotatedComponent<ArcStr>) -> bool {
        self.component_index.index_remove(cmp)
    }
}

impl Reasoner<ArcStr, ArcAnnotatedComponent> for StructuralReasoner {
    fn get_name(&self) -> String {
        "StructuralReasoner".to_string()
    }

    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn flush(&mut self) -> Result<(), ReasonerError> {
        // No-op for structural reasoner since it uses live index
        Ok(())
    }

    fn inferred_axioms(&self) -> Box<dyn Iterator<Item = Component<ArcStr>>> {
        // Structural reasoner does not produce inferred axioms
        Box::new(std::iter::empty())
    }

    fn is_consistent(&self) -> Result<bool, ReasonerError> {
        // Structural reasoner cannot determine consistency
        Err(ReasonerError::NotImplemented)
    }

    fn get_subclasses<'a>(
        &'a self,
        cmp: &'a ClassExpression<ArcStr>,
    ) -> Result<Box<dyn Iterator<Item = Class<ArcStr>> + 'a>, ReasonerError> {
        match cmp {
            ClassExpression::Class(cls) => {
                let mut descendants = HashSet::new();
                self.recurse_descendants(cls, &mut descendants);
                Ok(Box::new(descendants.into_iter()))
            }
            _ => Err(ReasonerError::NotImplemented),
        }
    }

    fn get_superclasses<'a>(
        &'a self,
        cmp: &'a ClassExpression<ArcStr>,
    ) -> Result<Box<dyn Iterator<Item = Class<ArcStr>> + 'a>, ReasonerError> {
        match cmp {
            ClassExpression::Class(cls) => {
                let mut ancestors = HashSet::new();
                self.recurse_ancestors(cls, &mut ancestors);
                Ok(Box::new(ancestors.into_iter()))
            }
            _ => Err(ReasonerError::NotImplemented),
        }
    }

    fn get_subobjectproperties<'a>(
        &'a self,
        cmp: &'a ObjectPropertyExpression<ArcStr>,
    ) -> Result<Box<dyn Iterator<Item = ObjectProperty<ArcStr>> + 'a>, ReasonerError> {
        match cmp {
            ObjectPropertyExpression::ObjectProperty(prop) => {
                let mut descendants = HashSet::new();
                self.recurse_subproperties(prop, &mut descendants);
                Ok(Box::new(descendants.into_iter()))
            }
            _ => Err(ReasonerError::NotImplemented),
        }
    }

    fn get_superobjectproperties<'a>(
        &'a self,
        cmp: &'a ObjectPropertyExpression<ArcStr>,
    ) -> Result<Box<dyn Iterator<Item = ObjectProperty<ArcStr>> + 'a>, ReasonerError> {
        match cmp {
            ObjectPropertyExpression::ObjectProperty(prop) => {
                let mut ancestors = HashSet::new();
                self.recurse_superproperties(prop, &mut ancestors);
                Ok(Box::new(ancestors.into_iter()))
            }
            _ => Err(ReasonerError::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use horned_owl::model::{Build, DeclareClass, MutableOntology, SubClassOf};
    use horned_owl::ontology::set::SetOntology;

    fn create_test_ontology() -> SetOntology<ArcStr> {
        let build = Build::new();
        let mut ontology = SetOntology::new();

        // Create classes A -> B -> D and C (independent)
        let class_a = build.class("https://example.com/A");
        let class_b = build.class("https://example.com/B");
        let class_c = build.class("https://example.com/C");
        let class_d = build.class("https://example.com/D");

        ontology.insert(DeclareClass(class_a.clone()));
        ontology.insert(DeclareClass(class_b.clone()));
        ontology.insert(DeclareClass(class_c.clone()));
        ontology.insert(DeclareClass(class_d.clone()));

        // B subClassOf A
        ontology.insert(SubClassOf {
            sup: ClassExpression::Class(class_a.clone()),
            sub: ClassExpression::Class(class_b.clone()),
        });

        // D subClassOf B
        ontology.insert(SubClassOf {
            sup: ClassExpression::Class(class_b.clone()),
            sub: ClassExpression::Class(class_d.clone()),
        });

        ontology
    }

    #[test]
    fn test_direct_subclasses() {
        let ontology = create_test_ontology();
        let reasoner = StructuralReasoner::create_reasoner(ontology);

        let build = Build::new();
        let class_a = build.class("https://example.com/A");
        let class_a_expr = ClassExpression::Class(class_a);

        let result = reasoner.get_subclasses(&class_a_expr).unwrap();
        let subclasses: HashSet<_> = result.collect();

        assert_eq!(subclasses.len(), 1);
        assert!(subclasses.contains(&build.class("https://example.com/B")));
    }

    #[test]
    fn test_direct_superclasses() {
        let ontology = create_test_ontology();
        let reasoner = StructuralReasoner::create_reasoner(ontology);

        let build = Build::new();
        let class_b = build.class("https://example.com/B");
        let class_b_expr = ClassExpression::Class(class_b);

        let superclasses: Vec<_> = reasoner.get_superclasses(&class_b_expr).unwrap().collect();

        assert_eq!(superclasses.len(), 1);
        assert!(superclasses.contains(&build.class("https://example.com/A")));
    }

    #[test]
    fn test_owl_thing_subclasses() {
        let ontology = create_test_ontology();
        let reasoner = StructuralReasoner::create_reasoner(ontology);

        let build = Build::new();
        let owl_thing = build.class(vocab::OWL::Thing);
        let owl_thing_expr = ClassExpression::Class(owl_thing);

        let result = reasoner.get_subclasses(&owl_thing_expr).unwrap();
        let root_classes: HashSet<_> = result.collect();

        // A and C are root classes (have no superclass)
        assert_eq!(root_classes.len(), 2);
        assert!(root_classes.contains(&build.class("https://example.com/A")));
        assert!(root_classes.contains(&build.class("https://example.com/C")));
    }

    #[test]
    fn test_owl_nothing_subclasses() {
        let ontology = create_test_ontology();
        let reasoner = StructuralReasoner::create_reasoner(ontology);

        let build = Build::new();
        let owl_nothing = build.class(vocab::OWL::Nothing);
        let owl_nothing_expr = ClassExpression::Class(owl_nothing);

        let result = reasoner.get_subclasses(&owl_nothing_expr).unwrap();
        let subclasses: HashSet<_> = result.collect();

        assert!(subclasses.is_empty());
    }

    #[test]
    fn test_descendants() {
        let ontology = create_test_ontology();
        let reasoner = StructuralReasoner::create_reasoner(ontology);

        let build: Build<ArcStr> = Build::new();
        let class_a = build.class("https://example.com/A");

        let mut descendants = HashSet::new();
        reasoner.recurse_descendants(&class_a, &mut descendants);

        // A has descendants B and D
        assert_eq!(descendants.len(), 2);
        assert!(descendants.contains(&build.class("https://example.com/B")));
        assert!(descendants.contains(&build.class("https://example.com/D")));
    }

    #[test]
    fn test_ancestors() {
        let ontology = create_test_ontology();
        let reasoner = StructuralReasoner::create_reasoner(ontology);

        let build: Build<ArcStr> = Build::new();
        let class_d = build.class("https://example.com/D");

        let mut ancestors = HashSet::new();
        reasoner.recurse_ancestors(&class_d, &mut ancestors);

        // D has ancestors B and A
        assert_eq!(ancestors.len(), 2);
        assert!(ancestors.contains(&build.class("https://example.com/A")));
        assert!(ancestors.contains(&build.class("https://example.com/B")));
    }
}
