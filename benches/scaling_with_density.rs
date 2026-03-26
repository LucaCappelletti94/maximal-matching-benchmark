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

fn bench_density_v100(c: &mut Criterion) {
    eprintln!("[1/5] Running density/V100 benchmarks...");
    let mut group = c.benchmark_group("density/V100");

    for p in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        eprintln!("  Generating erdos_renyi_gnp(V=100, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 100, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v200(c: &mut Criterion) {
    eprintln!("[2/5] Running density/V200 benchmarks...");
    let mut group = c.benchmark_group("density/V200");

    for p in [0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.9] {
        if p >= 0.3 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating erdos_renyi_gnp(V=200, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 200, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v500(c: &mut Criterion) {
    eprintln!("[3/5] Running density/V500 benchmarks...");
    let mut group = c.benchmark_group("density/V500");

    for p in [0.01, 0.02, 0.05, 0.1, 0.2, 0.3, 0.5] {
        if p >= 0.2 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if p >= 0.05 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating erdos_renyi_gnp(V=500, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 500, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v1000(c: &mut Criterion) {
    eprintln!("[4/5] Running density/V1000 benchmarks...");
    let mut group = c.benchmark_group("density/V1000");

    for p in [0.005, 0.01, 0.02, 0.05, 0.1, 0.2] {
        if p >= 0.1 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if p >= 0.02 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating erdos_renyi_gnp(V=1000, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 1000, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

fn bench_density_v2000(c: &mut Criterion) {
    eprintln!("[5/5] Running density/V2000 benchmarks...");
    let mut group = c.benchmark_group("density/V2000");

    for p in [0.005, 0.01, 0.02, 0.05, 0.1] {
        if p >= 0.05 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if p >= 0.01 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating erdos_renyi_gnp(V=2000, p={p})...");
        let g: Graph = erdos_renyi_gnp(42, 2000, p);
        let lbl = format!("p{p}_{}", graph_label(&g));
        bench_all!(group, lbl, g);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_density_v100,
    bench_density_v200,
    bench_density_v500,
    bench_density_v1000,
    bench_density_v2000,
);
criterion_main!(benches);
