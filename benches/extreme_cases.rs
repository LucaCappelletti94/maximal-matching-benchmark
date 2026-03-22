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

fn bench_barbell(c: &mut Criterion) {
    eprintln!("[1/7] Running extreme/barbell benchmarks...");
    let mut group = c.benchmark_group("extreme/barbell");

    for (k, p) in [
        (10usize, 0usize),
        (20, 0),
        (50, 0),
        (100, 0),
        (10, 50),
        (20, 20),
        (50, 10),
    ] {
        let v = 2 * k + p;
        if v >= 200 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if v >= 100 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating barbell_graph(k={k}, p={p})...");
        let g: Graph = barbell_graph(k, p);
        let lbl = format!("k{k}_p{p}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_hypercube(c: &mut Criterion) {
    eprintln!("[2/7] Running extreme/hypercube benchmarks...");
    let mut group = c.benchmark_group("extreme/hypercube");

    for d in [4usize, 6, 8, 10, 12, 14] {
        let v = 1usize << d;
        if v >= 4096 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if v >= 256 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating hypercube_graph(d={d}), V={v}...");
        let g: Graph = hypercube_graph(d);
        let lbl = format!("d{d}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_star(c: &mut Criterion) {
    eprintln!("[3/7] Running extreme/star benchmarks...");
    let mut group = c.benchmark_group("extreme/star");

    for n in [50usize, 100, 500, 1000, 2000, 5000, 10000, 20000] {
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

        eprintln!("  Generating star_graph(V={n})...");
        let g: Graph = star_graph(n);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_path(c: &mut Criterion) {
    eprintln!("[4/7] Running extreme/path benchmarks...");
    let mut group = c.benchmark_group("extreme/path");

    for n in [50usize, 100, 500, 1000, 2000, 5000, 10000, 20000] {
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

        eprintln!("  Generating path_graph(V={n})...");
        let g: Graph = path_graph(n);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_cycle(c: &mut Criterion) {
    eprintln!("[5/7] Running extreme/cycle benchmarks...");
    let mut group = c.benchmark_group("extreme/cycle");

    for n in [50usize, 100, 500, 1000, 2000, 5000, 10000, 20000] {
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

        eprintln!("  Generating cycle_graph(V={n})...");
        let g: Graph = cycle_graph(n);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_crown(c: &mut Criterion) {
    eprintln!("[6/7] Running extreme/crown benchmarks...");
    let mut group = c.benchmark_group("extreme/crown");

    for n in [10usize, 25, 50, 75, 100, 150, 200] {
        if n >= 100 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 50 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating crown_graph(n={n}), V={}...", 2 * n);
        let g: Graph = crown_graph(n);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_complete_bipartite(c: &mut Criterion) {
    eprintln!("[7/7] Running extreme/complete_bipartite benchmarks...");
    let mut group = c.benchmark_group("extreme/complete_bipartite");

    for (m, n) in [
        (10usize, 10usize),
        (25, 25),
        (50, 50),
        (100, 100),
        (10, 100),
        (50, 200),
    ] {
        let v = m + n;
        if v >= 200 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if v >= 50 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating complete_bipartite_graph({m}, {n})...");
        let g: Graph = complete_bipartite_graph(m, n);
        let lbl = format!("{m}x{n}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_barbell,
    bench_hypercube,
    bench_star,
    bench_path,
    bench_cycle,
    bench_crown,
    bench_complete_bipartite,
);
criterion_main!(benches);
