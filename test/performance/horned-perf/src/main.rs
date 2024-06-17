use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use horned_owl::io::ParserConfiguration;
use horned_owl::io::rdf::reader::RcRDFOntology;
use horned_owl::model::{Build, MutableOntology, RcStr};
use horned_owl::model::SubClassOf;
use horned_owl::ontology::iri_mapped::RcIRIMappedOntology;
use horned_owl::ontology::set::SetOntology;

fn perf(path: &str) {
    println!("===={}====", path);

    println!("Opening...");

    let file = File::open(&path).unwrap();
    let mut o: RcIRIMappedOntology = RcIRIMappedOntology::new_rc();
    let start = Instant::now();
    for _i in 0..3 {
        let mut f = BufReader::new(&file);
        o = SetOntology::<RcStr>::from(horned_owl::io::rdf::reader::read(&mut f, ParserConfiguration::default()).unwrap().0).into();
    }
    println!("Opened in {:.4?}", start.elapsed() / 3);

    println!("Iterating...");
    let o = o;
    let mut time: Duration = Duration::default();
    let c: RcIRIMappedOntology = o.into();
    for _i in 0..3 {
        let c = c.iter().clone();
        let mut amount = 0;
        let start2 = Instant::now();
        for a in c.into_iter() {
            if a.ann.is_empty() {
                amount += 1;
            }
        }
        time += start2.elapsed();
    }
    println!("Iterated in {:.4?}", time / 3);

    println!("Adding 100.000 axioms");
    let mut c = SetOntology::<RcStr>::default();
    let b = Build::new_rc();
    let start3 = Instant::now();
    for i in 0..100_000 {
        let a = SubClassOf::new(
            b.class(format!("https://example.com/entity_{}", i)).into(),
            b.class(format!("https://example.com/entity_{}", i)).into());
        c.insert(a);
    }
    println!("Added 100.000 axioms in {:.4?}", start3.elapsed())
}

fn main() {
    perf("../go.owl");
    perf("../chebi_core.owl");
}
