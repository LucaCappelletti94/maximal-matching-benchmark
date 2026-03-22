use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::{
    impls::{CSR2D, SymmetricCSR2D},
    prelude::{randomized_graphs::*, *},
};

type Graph = SymmetricCSR2D<CSR2D<usize, usize, usize>>;

macro_rules! bench_all {
    ($group:expr, $label:expr, $graph:expr) => {{
        let g: &Graph = &$graph;
        $group.bench_with_input(BenchmarkId::new("Blossom", &$label), g, |b, g| {
            b.iter(|| black_box(g.blossom()));
        });
        $group.bench_with_input(BenchmarkId::new("MicaliVazirani", &$label), g, |b, g| {
            b.iter(|| black_box(g.micali_vazirani()));
        });
        $group.bench_with_input(BenchmarkId::new("Blum", &$label), g, |b, g| {
            b.iter(|| black_box(g.blum()));
        });
    }};
}

fn graph_label(g: &Graph) -> String {
    format!("V{}_E{}", g.order(), g.number_of_defined_values() / 2)
}

fn bench_sparse_d6(c: &mut Criterion) {
    eprintln!("[1/5] Running size/sparse_d6 benchmarks...");
    let mut group = c.benchmark_group("size/sparse_d6");

    for n in [
        10usize, 20, 50, 100, 200, 500, 1000, 2000, 3000, 5000, 7500, 10000,
    ] {
        if n >= 5000 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 500 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating erdos_renyi_gnm(V={n}, E={})...", n * 3);
        let g: Graph = erdos_renyi_gnm(42, n, n * 3);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_medium_d20(c: &mut Criterion) {
    eprintln!("[2/5] Running size/medium_d20 benchmarks...");
    let mut group = c.benchmark_group("size/medium_d20");

    for n in [20usize, 50, 100, 200, 500, 1000, 2000, 3000] {
        if n >= 2000 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 500 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating erdos_renyi_gnm(V={n}, E={})...", n * 10);
        let g: Graph = erdos_renyi_gnm(42, n, n * 10);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_dense_half(c: &mut Criterion) {
    eprintln!("[3/5] Running size/dense_half benchmarks...");
    let mut group = c.benchmark_group("size/dense_half");

    for n in [20usize, 50, 100, 200, 500, 750, 1000] {
        if n >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 200 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        let m = n * n / 4;
        eprintln!("  Generating erdos_renyi_gnm(V={n}, E={m})...");
        let g: Graph = erdos_renyi_gnm(42, n, m);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_complete(c: &mut Criterion) {
    eprintln!("[4/5] Running size/complete benchmarks...");
    let mut group = c.benchmark_group("size/complete");

    for n in [10usize, 20, 50, 100, 200, 300, 400, 500] {
        if n >= 300 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 200 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating complete_graph(V={n})...");
        let g: Graph = complete_graph(n);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_grid(c: &mut Criterion) {
    eprintln!("[5/5] Running size/grid benchmarks...");
    let mut group = c.benchmark_group("size/grid");

    for k in [3usize, 5, 7, 10, 15, 22, 32, 45, 55, 70, 100] {
        let v = k * k;
        if v >= 5000 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if v >= 500 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating grid_graph({k}, {k})...");
        let g: Graph = grid_graph(k, k);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_sparse_d6,
    bench_medium_d20,
    bench_dense_half,
    bench_complete,
    bench_grid,
);
criterion_main!(benches);
