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

fn bench_barabasi_albert_m2(c: &mut Criterion) {
    eprintln!("[1/7] Running realworld/barabasi_albert_m2 benchmarks...");
    let mut group = c.benchmark_group("realworld/barabasi_albert_m2");

    for n in [50usize, 100, 200, 500, 1000, 2000, 5000] {
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

        eprintln!("  Generating barabasi_albert(V={n}, m=2)...");
        let g: Graph = barabasi_albert(42, n, 2);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_barabasi_albert_m5(c: &mut Criterion) {
    eprintln!("[2/7] Running realworld/barabasi_albert_m5 benchmarks...");
    let mut group = c.benchmark_group("realworld/barabasi_albert_m5");

    for n in [50usize, 100, 200, 500, 1000, 2000, 5000] {
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

        eprintln!("  Generating barabasi_albert(V={n}, m=5)...");
        let g: Graph = barabasi_albert(42, n, 5);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_watts_strogatz_k6(c: &mut Criterion) {
    eprintln!("[3/7] Running realworld/watts_strogatz_k6 benchmarks...");
    let mut group = c.benchmark_group("realworld/watts_strogatz_k6");

    for n in [50usize, 100, 200, 500, 1000, 2000, 5000] {
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

        eprintln!("  Generating watts_strogatz(V={n}, k=6, beta=0.3)...");
        let g: Graph = watts_strogatz(42, n, 6, 0.3);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_watts_strogatz_k10(c: &mut Criterion) {
    eprintln!("[4/7] Running realworld/watts_strogatz_k10 benchmarks...");
    let mut group = c.benchmark_group("realworld/watts_strogatz_k10");

    for n in [50usize, 100, 200, 500, 1000, 2000, 5000] {
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

        eprintln!("  Generating watts_strogatz(V={n}, k=10, beta=0.5)...");
        let g: Graph = watts_strogatz(42, n, 10, 0.5);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_stochastic_block_model(c: &mut Criterion) {
    eprintln!("[5/7] Running realworld/stochastic_block_model benchmarks...");
    let mut group = c.benchmark_group("realworld/stochastic_block_model");

    for n in [50usize, 100, 200, 500, 1000, 2000] {
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

        eprintln!("  Generating stochastic_block_model(V={n})...");
        let g: Graph = stochastic_block_model(42, &[n / 2, n / 2], 0.3, 0.01);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_random_geometric(c: &mut Criterion) {
    eprintln!("[6/7] Running realworld/random_geometric benchmarks...");
    let mut group = c.benchmark_group("realworld/random_geometric");

    for n in [50usize, 100, 200, 500, 1000, 2000, 5000] {
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

        let radius = (6.0 / (std::f64::consts::PI * (n - 1) as f64)).sqrt();
        eprintln!("  Generating random_geometric_graph(V={n}, r={radius:.4})...");
        let g: Graph = random_geometric_graph(42, n, radius);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_random_regular_k4(c: &mut Criterion) {
    eprintln!("[7/7] Running realworld/random_regular_k4 benchmarks...");
    let mut group = c.benchmark_group("realworld/random_regular_k4");

    for n in [50usize, 100, 200, 500, 1000, 2000, 5000] {
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

        eprintln!("  Generating random_regular_graph(V={n}, k=4)...");
        let g: Graph = random_regular_graph(42, n, 4);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_barabasi_albert_m2,
    bench_barabasi_albert_m5,
    bench_watts_strogatz_k6,
    bench_watts_strogatz_k10,
    bench_stochastic_block_model,
    bench_random_geometric,
    bench_random_regular_k4,
);
criterion_main!(benches);
