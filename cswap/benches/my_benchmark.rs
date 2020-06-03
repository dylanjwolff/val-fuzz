use criterion::{criterion_group, criterion_main, Criterion};
use cswap::parser::rmv_comments;
use cswap::parser::script;
use std::fs;
use std::path::Path;

fn to_string_bench(c: &mut Criterion) {
    let filepath = Path::new("test/ooo.tag10.smt2");

    let contents: String = fs::read_to_string(&filepath).unwrap();
    let stripped_contents = &rmv_comments(&contents[..]).unwrap().1.join(" ")[..];
    let script = script(&stripped_contents[..]).unwrap().1;

    c.bench_function("script to str", |b| {
        b.iter(|| {
            script.to_string();
        })
    });
}

criterion_group!(benches, to_string_bench);
criterion_main!(benches);
