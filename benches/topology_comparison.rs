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

macro_rules! bench_named {
    ($group:expr, $name:expr, $graph:expr) => {{
        let g: Graph = $graph;
        eprintln!("  {}...", $name);
        let lbl = format!("{}_{}", $name, graph_label(&g));
        bench_all!($group, lbl, g);
    }};
}

fn bench_topology_v100(c: &mut Criterion) {
    eprintln!("[1/4] Running topology/V100 benchmarks...");
    let mut group = c.benchmark_group("topology/V100");

    bench_named!(group, "complete", complete_graph(100));
    bench_named!(group, "cycle", cycle_graph(100));
    bench_named!(group, "path", path_graph(100));
    bench_named!(group, "star", star_graph(100));
    bench_named!(group, "grid", grid_graph(10, 10));
    bench_named!(group, "torus", torus_graph(10, 10));
    bench_named!(group, "wheel", wheel_graph(99));
    bench_named!(group, "crown", crown_graph(50));
    bench_named!(
        group,
        "complete_bipartite",
        complete_bipartite_graph(50, 50)
    );
    bench_named!(group, "turan", turan_graph(100, 5));
    bench_named!(group, "friendship", friendship_graph(49));
    bench_named!(group, "windmill_k4", windmill_graph(33, 4));
    bench_named!(group, "hexagonal_lattice", hexagonal_lattice_graph(6, 6));
    bench_named!(
        group,
        "triangular_lattice",
        triangular_lattice_graph(10, 10)
    );
    bench_named!(group, "petersen", petersen_graph());
    bench_named!(group, "erdos_renyi", erdos_renyi_gnp(42, 100, 0.1));
    bench_named!(group, "barabasi_albert", barabasi_albert(42, 100, 3));
    bench_named!(group, "watts_strogatz", watts_strogatz(42, 100, 6, 0.3));

    group.finish();
}

fn bench_topology_v500(c: &mut Criterion) {
    eprintln!("[2/4] Running topology/V500 benchmarks...");
    let mut group = c.benchmark_group("topology/V500");
    group
        .sample_size(30)
        .measurement_time(Duration::from_secs(20));

    bench_named!(group, "cycle", cycle_graph(500));
    bench_named!(group, "path", path_graph(500));
    bench_named!(group, "star", star_graph(500));
    bench_named!(group, "grid", grid_graph(22, 23));
    bench_named!(group, "torus", torus_graph(22, 23));
    bench_named!(group, "wheel", wheel_graph(499));
    bench_named!(group, "friendship", friendship_graph(249));
    bench_named!(group, "windmill_k4", windmill_graph(166, 4));
    bench_named!(group, "hexagonal_lattice", hexagonal_lattice_graph(15, 15));
    bench_named!(
        group,
        "triangular_lattice",
        triangular_lattice_graph(22, 23)
    );
    bench_named!(group, "barbell", barbell_graph(50, 10));
    bench_named!(group, "erdos_renyi", erdos_renyi_gnp(42, 500, 0.02));
    bench_named!(group, "barabasi_albert", barabasi_albert(42, 500, 3));
    bench_named!(group, "watts_strogatz", watts_strogatz(42, 500, 6, 0.3));
    bench_named!(group, "crown", crown_graph(250));
    bench_named!(
        group,
        "complete_bipartite",
        complete_bipartite_graph(250, 250)
    );

    group.finish();
}

fn bench_topology_v1000(c: &mut Criterion) {
    eprintln!("[3/4] Running topology/V1000 benchmarks...");
    let mut group = c.benchmark_group("topology/V1000");
    group
        .sample_size(30)
        .measurement_time(Duration::from_secs(20));

    bench_named!(group, "cycle", cycle_graph(1000));
    bench_named!(group, "path", path_graph(1000));
    bench_named!(group, "star", star_graph(1000));
    bench_named!(group, "grid", grid_graph(32, 32));
    bench_named!(group, "torus", torus_graph(32, 32));
    bench_named!(group, "wheel", wheel_graph(999));
    bench_named!(group, "friendship", friendship_graph(499));
    bench_named!(group, "windmill_k4", windmill_graph(333, 4));
    bench_named!(group, "hexagonal_lattice", hexagonal_lattice_graph(21, 22));
    bench_named!(
        group,
        "triangular_lattice",
        triangular_lattice_graph(32, 32)
    );
    bench_named!(group, "erdos_renyi", erdos_renyi_gnp(42, 1000, 0.01));
    bench_named!(group, "barabasi_albert", barabasi_albert(42, 1000, 3));
    bench_named!(group, "watts_strogatz", watts_strogatz(42, 1000, 6, 0.3));
    bench_named!(group, "crown", crown_graph(500));
    bench_named!(
        group,
        "complete_bipartite",
        complete_bipartite_graph(500, 500)
    );

    group.finish();
}

fn bench_topology_v2000(c: &mut Criterion) {
    eprintln!("[4/4] Running topology/V2000 benchmarks...");
    let mut group = c.benchmark_group("topology/V2000");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(30));

    bench_named!(group, "cycle", cycle_graph(2000));
    bench_named!(group, "path", path_graph(2000));
    bench_named!(group, "star", star_graph(2000));
    bench_named!(group, "grid", grid_graph(45, 45));
    bench_named!(group, "torus", torus_graph(45, 45));
    bench_named!(group, "wheel", wheel_graph(1999));
    bench_named!(group, "friendship", friendship_graph(999));
    bench_named!(group, "windmill_k4", windmill_graph(666, 4));
    bench_named!(group, "hexagonal_lattice", hexagonal_lattice_graph(30, 31));
    bench_named!(
        group,
        "triangular_lattice",
        triangular_lattice_graph(45, 45)
    );
    bench_named!(group, "erdos_renyi", erdos_renyi_gnp(42, 2000, 0.005));
    bench_named!(group, "barabasi_albert", barabasi_albert(42, 2000, 3));
    bench_named!(group, "watts_strogatz", watts_strogatz(42, 2000, 6, 0.3));
    bench_named!(group, "crown", crown_graph(1000));
    bench_named!(
        group,
        "complete_bipartite",
        complete_bipartite_graph(1000, 1000)
    );

    group.finish();
}

criterion_group!(
    benches,
    bench_topology_v100,
    bench_topology_v500,
    bench_topology_v1000,
    bench_topology_v2000,
);
criterion_main!(benches);
