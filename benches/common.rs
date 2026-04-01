use std::hint::black_box;

use criterion::{BenchmarkGroup, BenchmarkId, measurement::WallTime};
use geometric_traits::impls::{CSR2D, SymmetricCSR2D};
use geometric_traits::prelude::*;

pub type Graph = SymmetricCSR2D<CSR2D<usize, usize, usize>>;

#[allow(dead_code)]
pub fn graph_label(g: &Graph) -> String {
    format!("V{}_E{}", g.order(), g.number_of_defined_values() / 2)
}

pub fn bench_exact_matchers(group: &mut BenchmarkGroup<'_, WallTime>, label: &str, g: &Graph) {
    group.bench_with_input(BenchmarkId::new("Blossom", label), g, |b, g| {
        b.iter(|| black_box(g.blossom()));
    });

    group.bench_with_input(BenchmarkId::new("Gabow1976", label), g, |b, g| {
        b.iter(|| black_box(g.gabow_1976()));
    });

    group.bench_with_input(BenchmarkId::new("MicaliVazirani", label), g, |b, g| {
        b.iter(|| black_box(g.micali_vazirani()));
    });
}
