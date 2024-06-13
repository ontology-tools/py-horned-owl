use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};
use horned_owl::io::ParserConfiguration;
use horned_owl::io::rdf::reader::{RcRDFOntology, RDFOntology};
use horned_owl::model::{AnnotatedComponent, Build, MutableOntology, RcStr};
use horned_owl::ontology::set::{SetIndex, SetOntology};
use horned_owl::model::SubClassOf;
use horned_owl::ontology::declaration_mapped::DeclarationMappedIndex;
use horned_owl::ontology::indexed::ThreeIndexedOntology;
use horned_owl::ontology::logically_equal::LogicallyEqualIndex;

fn perf(path: &str) {
    println!("===={}====", path);

    println!("Opening...");

    let mut o: Option<RcRDFOntology> = None;
    let start = Instant::now();
    for _i in 0..3 {
        let file = File::open(path).ok().unwrap();
        let mut f = BufReader::new(file);
        let b = horned_owl::model::Build::new_rc();
        o = Some(horned_owl::io::rdf::reader::read_with_build(&mut f, &b, ParserConfiguration::default()).unwrap().0);
    }
    println!("Opened in {:.4?}s", start.elapsed() / 3);

    println!("Iterating...");
    let o = o.unwrap();
    let mut time: Duration = Duration::default();
    let c: SetOntology<RcStr> = o.into();
    for _i in 0..3 {
        let c = c.clone();
        let mut amount = 0;
        let start2 = Instant::now();
        for a in c.into_iter() {
            if a.ann.is_empty() {
                amount += 1;
            }
        }
        time += start2.elapsed();
    }
    println!("Iterated in {:.4?}s", time / 3);

    println!("Adding 100.000 axioms");
    let mut c = SetOntology::<RcStr>::default();
    let b = Build::new_rc();
    let start3 = Instant::now();
    for i in 0..100_000{
        let a = SubClassOf::new(b.class( format!("https://example.com/entity_{}", i)), b.class( format!("https://example.com/entity_{}", i)));
        c.insert(a)
    }
    println!("Added 100.000 axioms in {:.4?}", start3.elapsed())
}

fn main() {
    perf("go.owl");
    perf("chebi_core.owl");
}
