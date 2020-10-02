use criterion::{black_box, criterion_group, criterion_main, Criterion};
use resolution::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut clauses = Vec::new();
    for i in 0..5 {
        let mut literals = Vec::new();
        for j in 0..3 {
            literals.push(Literal{m: j + (i % 50), negated: ((j * i) % 1000 == 0)});
        }
        clauses.push(Clause::new(&literals));
    }
    let statement = Statement::new(&clauses);
    c.bench_function("big statement", |b| b.iter(|| black_box(statement.clone()).resolve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
