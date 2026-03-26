use std::time::Duration;

mod common;

use common::{Graph, bench_exact_matchers, graph_label};
use criterion::{Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::randomized_graphs::*;

macro_rules! bench_all {
    ($group:expr, $label:expr, $graph:expr) => {{
        let g: &Graph = &$graph;
        bench_exact_matchers(&mut $group, &$label, g);
    }};
}

fn bench_sparse_d6(c: &mut Criterion) {
    eprintln!("[1/9] Running size/sparse_d6 benchmarks...");
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
    eprintln!("[2/9] Running size/medium_d20 benchmarks...");
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
    eprintln!("[3/9] Running size/dense_half benchmarks...");
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
    eprintln!("[4/9] Running size/complete benchmarks...");
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
    eprintln!("[5/9] Running size/grid benchmarks...");
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

fn bench_torus(c: &mut Criterion) {
    eprintln!("[6/9] Running size/torus benchmarks...");
    let mut group = c.benchmark_group("size/torus");

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

        eprintln!("  Generating torus_graph({k}, {k})...");
        let g: Graph = torus_graph(k, k);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_hexagonal_lattice(c: &mut Criterion) {
    eprintln!("[7/9] Running size/hexagonal_lattice benchmarks...");
    let mut group = c.benchmark_group("size/hexagonal_lattice");

    for (rows, cols) in [
        (1usize, 1usize),
        (2, 2),
        (3, 3),
        (6, 6),
        (10, 10),
        (15, 15),
        (21, 22),
        (30, 31),
        (38, 39),
        (50, 50),
        (70, 70),
    ] {
        let v = 2 * rows * cols + 2 * rows + 2 * cols;
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

        eprintln!("  Generating hexagonal_lattice_graph({rows}, {cols})...");
        let g: Graph = hexagonal_lattice_graph(rows, cols);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_triangular_lattice(c: &mut Criterion) {
    eprintln!("[8/9] Running size/triangular_lattice benchmarks...");
    let mut group = c.benchmark_group("size/triangular_lattice");

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

        eprintln!("  Generating triangular_lattice_graph({k}, {k})...");
        let g: Graph = triangular_lattice_graph(k, k);
        let lbl = graph_label(&g);
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_windmill_k4(c: &mut Criterion) {
    eprintln!("[9/9] Running size/windmill_k4 benchmarks...");
    let mut group = c.benchmark_group("size/windmill_k4");

    for num_cliques in [3usize, 6, 16, 33, 66, 166, 333, 666, 1000, 1666, 2500, 3333] {
        let v = 1 + 3 * num_cliques;
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

        eprintln!("  Generating windmill_graph(num_cliques={num_cliques}, clique_size=4)...");
        let g: Graph = windmill_graph(num_cliques, 4);
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
    bench_torus,
    bench_hexagonal_lattice,
    bench_triangular_lattice,
    bench_windmill_k4,
);
criterion_main!(benches);
