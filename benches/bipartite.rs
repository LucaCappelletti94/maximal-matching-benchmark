use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::{
    impls::{CSR2D, SymmetricCSR2D},
    prelude::{randomized_graphs::*, *},
    traits::{EdgesBuilder, HopcroftKarp},
};

type Graph = SymmetricCSR2D<CSR2D<usize, usize, usize>>;
type BiGraph = CSR2D<usize, usize, usize>;

/// Build a non-symmetric CSR2D for Hopcroft-Karp from a bipartite edge list.
/// `m` rows (left partition), `n` columns (right partition).
/// Edges are (left_idx, right_idx) with left_idx in 0..m, right_idx in 0..n.
fn build_bipartite_csr(m: usize, n: usize, edges: Vec<(usize, usize)>) -> BiGraph {
    let num_edges = edges.len();
    GenericEdgesBuilder::<_, BiGraph>::default()
        .expected_number_of_edges(num_edges)
        .expected_shape((m, n))
        .edges(edges.into_iter())
        .build()
        .unwrap()
}

fn graph_label(v: usize, e: usize) -> String {
    format!("V{v}_E{e}")
}

fn bench_complete_bipartite(c: &mut Criterion) {
    eprintln!("[1/4] Running bipartite/complete benchmarks...");
    let mut group = c.benchmark_group("bipartite/complete");

    for n in [10usize, 25, 50, 100, 200, 500] {
        if n >= 200 {
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

        eprintln!("  Generating complete_bipartite K_{{{n},{n}}}...");

        // Bipartite edges: left i -> right j for all i in 0..n, j in 0..n
        let edges: Vec<(usize, usize)> = (0..n).flat_map(|i| (0..n).map(move |j| (i, j))).collect();
        let num_edges = edges.len();

        let bi: BiGraph = build_bipartite_csr(n, n, edges);
        let sym: Graph = complete_bipartite_graph(n, n);

        let lbl = graph_label(2 * n, num_edges);

        group.bench_with_input(BenchmarkId::new("HopcroftKarp", &lbl), &bi, |b, g| {
            b.iter(|| black_box(g.hopcroft_karp()));
        });
        group.bench_with_input(BenchmarkId::new("Blossom", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blossom()));
        });
        group.bench_with_input(BenchmarkId::new("MicaliVazirani", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.micali_vazirani()));
        });
        group.bench_with_input(BenchmarkId::new("Blum", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blum()));
        });
    }

    group.finish();
}

fn bench_crown(c: &mut Criterion) {
    eprintln!("[2/4] Running bipartite/crown benchmarks...");
    let mut group = c.benchmark_group("bipartite/crown");

    for n in [10usize, 25, 50, 100, 200] {
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

        eprintln!("  Generating crown graph C_{n}...");

        // Crown = K_{n,n} minus a perfect matching.
        // Bipartite edges: left i -> right j for all j != i.
        let edges: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (0..n).filter(move |&j| j != i).map(move |j| (i, j)))
            .collect();
        let num_edges = edges.len();

        let bi: BiGraph = build_bipartite_csr(n, n, edges);
        let sym: Graph = crown_graph(n);

        let lbl = graph_label(2 * n, num_edges);

        group.bench_with_input(BenchmarkId::new("HopcroftKarp", &lbl), &bi, |b, g| {
            b.iter(|| black_box(g.hopcroft_karp()));
        });
        group.bench_with_input(BenchmarkId::new("Blossom", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blossom()));
        });
        group.bench_with_input(BenchmarkId::new("MicaliVazirani", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.micali_vazirani()));
        });
        group.bench_with_input(BenchmarkId::new("Blum", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blum()));
        });
    }

    group.finish();
}

fn bench_random_bipartite(c: &mut Criterion) {
    eprintln!("[3/4] Running bipartite/random benchmarks...");
    let mut group = c.benchmark_group("bipartite/random");

    // Sparse random bipartite: each left vertex connects to ~6 right vertices.
    for n in [50usize, 100, 200, 500, 1000] {
        if n >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(30));
        } else if n >= 200 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating random bipartite G({n},{n}, p=6/n)...");

        // Use a seeded RNG to generate edges with probability ~6/n.
        let mut rng_state: u64 = 42;
        let p_threshold = ((6.0 / n as f64) * u32::MAX as f64) as u64;
        let mut edges = Vec::new();
        for i in 0..n {
            for j in 0..n {
                // Simple xorshift64
                rng_state ^= rng_state << 13;
                rng_state ^= rng_state >> 7;
                rng_state ^= rng_state << 17;
                if (rng_state & 0xFFFF_FFFF) < p_threshold {
                    edges.push((i, j));
                }
            }
        }
        let num_edges = edges.len();

        // Build symmetric version: left vertices 0..n, right vertices n..2n
        let sym_edges: Vec<(usize, usize)> = edges.iter().map(|&(i, j)| (i, n + j)).collect();
        let sym: Graph = {
            let total = 2 * n;
            let ne = sym_edges.len();
            GenericUndirectedMonopartiteEdgesBuilder::<
                _,
                UpperTriangularCSR2D<CSR2D<usize, usize, usize>>,
                Graph,
            >::default()
            .expected_number_of_edges(ne)
            .expected_shape(total)
            .edges(sym_edges.into_iter())
            .build()
            .unwrap()
        };

        let bi: BiGraph = build_bipartite_csr(n, n, edges);

        let lbl = graph_label(2 * n, num_edges);

        group.bench_with_input(BenchmarkId::new("HopcroftKarp", &lbl), &bi, |b, g| {
            b.iter(|| black_box(g.hopcroft_karp()));
        });
        group.bench_with_input(BenchmarkId::new("Blossom", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blossom()));
        });
        group.bench_with_input(BenchmarkId::new("MicaliVazirani", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.micali_vazirani()));
        });
        group.bench_with_input(BenchmarkId::new("Blum", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blum()));
        });
    }

    group.finish();
}

fn bench_imbalanced_bipartite(c: &mut Criterion) {
    eprintln!("[4/4] Running bipartite/imbalanced benchmarks...");
    let mut group = c.benchmark_group("bipartite/imbalanced");

    for (m, n) in [(10usize, 100usize), (20, 200), (50, 500), (100, 1000)] {
        if m * n >= 50000 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if m * n >= 5000 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating complete bipartite K_{{{m},{n}}}...");

        let edges: Vec<(usize, usize)> = (0..m).flat_map(|i| (0..n).map(move |j| (i, j))).collect();
        let num_edges = edges.len();

        let bi: BiGraph = build_bipartite_csr(m, n, edges);
        let sym: Graph = complete_bipartite_graph(m, n);

        let lbl = format!("{m}x{n}_{}", graph_label(m + n, num_edges));

        group.bench_with_input(BenchmarkId::new("HopcroftKarp", &lbl), &bi, |b, g| {
            b.iter(|| black_box(g.hopcroft_karp()));
        });
        group.bench_with_input(BenchmarkId::new("Blossom", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blossom()));
        });
        group.bench_with_input(BenchmarkId::new("MicaliVazirani", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.micali_vazirani()));
        });
        group.bench_with_input(BenchmarkId::new("Blum", &lbl), &sym, |b, g| {
            b.iter(|| black_box(g.blum()));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_complete_bipartite,
    bench_crown,
    bench_random_bipartite,
    bench_imbalanced_bipartite,
);
criterion_main!(benches);
