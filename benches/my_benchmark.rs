use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterated_rps;

fn bench_play_on_grid(c: &mut Criterion) {
    let a = iterated_rps::graph::generate_grid_graph::<iterated_rps::NodeData>(3_00, 3_00);
    let mut A = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [[0.0, 1.0, -1.0], [-1.0, 0.0, 1.0], [1.0, -1.0, 0.0]],
        },
        beta: 0.1,
    };
    c.bench_function("play on 300x300 grid", |b| b.iter(|| A.play()));
}

fn bench_play_on_cycle(c: &mut Criterion) {
    let a = iterated_rps::graph::generate_cycle_graph::<iterated_rps::NodeData>(300 * 300);
    let mut A = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [[0.0, 1.0, -1.0], [-1.0, 0.0, 1.0], [1.0, -1.0, 0.0]],
        },
        beta: 0.1,
    };
    c.bench_function("play on 90000 cycle", |b| b.iter(|| A.play()));
}

fn bench_update_srat_on_cycle(c: &mut Criterion) {
    let a = iterated_rps::graph::generate_cycle_graph::<iterated_rps::NodeData>(300 * 300);
    let mut A = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [[0.0, 1.0, -1.0], [-1.0, 0.0, 1.0], [1.0, -1.0, 0.0]],
        },
        beta: 0.1,
    };
    c.bench_function("update_strat on 90000 cycle", |b| {
        b.iter(|| A.update_strategies())
    });
}

fn bench_update_strat_on_grid(c: &mut Criterion) {
    let a = iterated_rps::graph::generate_grid_graph::<iterated_rps::NodeData>(3_00, 3_00);
    let mut A = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [[0.0, 1.0, -1.0], [-1.0, 0.0, 1.0], [1.0, -1.0, 0.0]],
        },
        beta: 0.1,
    };
    c.bench_function("update_strat on 300x300 grid", |b| {
        b.iter(|| A.update_strategies())
    });
}

fn bench_update_on_cycle(c: &mut Criterion) {
    let a = iterated_rps::graph::generate_cycle_graph::<iterated_rps::NodeData>(300 * 300);
    let mut A = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [[0.0, 1.0, -1.0], [-1.0, 0.0, 1.0], [1.0, -1.0, 0.0]],
        },
        beta: 0.1,
    };
    c.bench_function("update on 90000 cycle", |b| b.iter(|| A.update()));
}

fn bench_update_on_grid(c: &mut Criterion) {
    let a = iterated_rps::graph::generate_grid_graph::<iterated_rps::NodeData>(3_00, 3_00);
    let mut A = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [[0.0, 1.0, -1.0], [-1.0, 0.0, 1.0], [1.0, -1.0, 0.0]],
        },
        beta: 0.1,
    };
    c.bench_function("update on 300x300 grid", |b| b.iter(|| A.update()));
}

criterion_group!(
    benches,
    bench_play_on_grid,
    bench_play_on_cycle,
    bench_update_srat_on_cycle,
    bench_update_strat_on_grid,
    bench_update_on_cycle,
    bench_update_on_grid
);
criterion_main!(benches);
